import { SITE_URL } from './sitemap';

export const OG_IMAGE = `${SITE_URL}/og-image.png`;

function attr(value: string): string {
	return value.replaceAll('"', '&quot;');
}

export function with_open_graph(html: string): string {
	const headEnd = html.indexOf('</head>');
	if (headEnd === -1) return html;
	const head = html.slice(0, headEnd);

	const title = /<title>([^<]*)<\/title>/.exec(head)?.[1]?.trim();
	const description = /<meta name="description" content="([^"]*)"/.exec(head)?.[1];
	const url = /<link rel="canonical" href="([^"]*)"/.exec(head)?.[1];
	const lang = /<html lang="([a-z]+)"/.exec(head)?.[1];

	const tags = [
		['og:site_name', 'Somes'],
		['og:type', 'website'],
		['og:locale', lang === 'en' ? 'en_US' : 'de_AT'],
		['og:image', OG_IMAGE],
		['og:image:width', '1200'],
		['og:image:height', '630'],
		...(title ? [['og:title', title]] : []),
		...(description ? [['og:description', description]] : []),
		...(url ? [['og:url', url]] : [])
	]
		.filter(([property]) => !head.includes(`property="${property}"`))
		.map(([property, content]) => `<meta property="${property}" content="${attr(content)}" />`);

	if (!head.includes('name="twitter:card"')) {
		tags.push('<meta name="twitter:card" content="summary_large_image" />');
	}
	if (tags.length === 0) return html;

	return `${html.slice(0, headEnd)}${tags.join('')}${html.slice(headEnd)}`;
}
