import satori from 'satori';
import { html } from 'satori-html';
import { Resvg } from '@resvg/resvg-js';
import { render } from 'svelte/server';
import type { Component } from 'svelte';
import regular from '@fontsource/noto-sans/files/noto-sans-latin-400-normal.woff?inline';
import bold from '@fontsource/noto-sans/files/noto-sans-latin-700-normal.woff?inline';
import type { ShareIcons } from '$lib/components/ShareCards/types';
import somesIcon from '$lib/assets/somes_icon.svg?raw';
import checkmark from '$lib/assets/misc_icons/checkmark_small.svg?raw';
import crossmark from '$lib/assets/misc_icons/crossmark_small.svg?raw';

const fromDataUrl = (url: string) => Buffer.from(url.slice(url.indexOf(',') + 1), 'base64');

const fonts = [
	{ name: 'Noto Sans', data: fromDataUrl(regular), weight: 400 as const, style: 'normal' as const },
	{ name: 'Noto Sans', data: fromDataUrl(bold), weight: 700 as const, style: 'normal' as const }
];

export const svgDataUrl = (svg: string) =>
	`data:image/svg+xml;base64,${Buffer.from(svg).toString('base64')}`;

export const SHARE_ICONS: ShareIcons = {
	logo: svgDataUrl(somesIcon.replace('<svg ', '<svg fill="#ffffff" stroke="#ffffff" ')),
	check: svgDataUrl(checkmark.replace('<path ', '<path stroke="#00a63e" ')),
	cross: svgDataUrl(crossmark),
	minus: svgDataUrl(
		'<svg width="100%" height="100%" viewBox="0 0 80 80" xmlns="http://www.w3.org/2000/svg"><line x1="20" y1="40" x2="60" y2="40" stroke="#2b7fff" stroke-width="8" stroke-linecap="round"/></svg>'
	)
};

const ENTITIES: Record<string, string> = {
	'&amp;': '&',
	'&lt;': '<',
	'&gt;': '>',
	'&quot;': '"',
	'&#39;': "'",
	'&nbsp;': ' '
};

function decode(node: unknown): unknown {
	if (typeof node === 'string') {
		return node
			.replace(/&(amp|lt|gt|quot|#39|nbsp);/g, (entity) => ENTITIES[entity])
			.replace(/[‐‑]/g, '-');
	}
	if (Array.isArray(node)) return node.map(decode);
	if (node && typeof node === 'object' && 'props' in node) {
		const { props } = node as { props: { children?: unknown } };
		return { ...node, props: { ...props, children: decode(props.children) } };
	}
	return node;
}

export async function render_share_image<Props extends Record<string, unknown>>(
	component: Component<Props>,
	props: Props
): Promise<Uint8Array> {
	const { body } = render(component, { props });
	const markup = body.replace(/<!--[\s\S]*?-->/g, '').replace(/>\s+</g, '><');
	const svg = await satori(decode(html(markup)) as Parameters<typeof satori>[0], {
		width: 1200,
		height: 630,
		fonts
	});
	return new Resvg(svg, { fitTo: { mode: 'width', value: 1200 } }).render().asPng();
}
