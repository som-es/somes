import { describe, expect, it } from 'vitest';
import { with_open_graph } from './opengraph';

const page = (head: string) =>
	`<!doctype html><html lang="de"><head>${head}</head><body></body></html>`;

describe('with_open_graph', () => {
	it('copies title, description and canonical into Open Graph tags', () => {
		const html = with_open_graph(
			page(
				'<title>Langthaler Monika (GRÜNE) – "Test" | Somes</title><meta name="description" content="Beschreibung &amp; mehr" class="svelte-1"><link rel="canonical" href="https://somes.at/at/delegates?delegate=785">'
			)
		);
		expect(html).toContain(
			'<meta property="og:title" content="Langthaler Monika (GRÜNE) – &quot;Test&quot; | Somes" />'
		);
		expect(html).toContain('<meta property="og:description" content="Beschreibung &amp; mehr" />');
		expect(html).toContain(
			'<meta property="og:url" content="https://somes.at/at/delegates?delegate=785" />'
		);
		expect(html).toContain('<meta property="og:locale" content="de_AT" />');
		expect(html).toContain('<meta property="og:image" content="https://somes.at/og-image.png" />');
		expect(html.indexOf('og:title')).toBeLessThan(html.indexOf('</head>'));
	});

	it('leaves out tags whose source is missing', () => {
		const html = with_open_graph(page('<title>Somes</title>'));
		expect(html).not.toContain('og:description');
		expect(html).not.toContain('og:url');
	});

	it('does nothing for chunks without a head', () => {
		expect(with_open_graph('<div>body chunk</div>')).toBe('<div>body chunk</div>');
	});

	it('keeps tags the page set itself and fills in the rest', () => {
		const html = with_open_graph(
			page(
				'<title>Abstimmung | Somes</title><meta property="og:image" content="https://somes.at/og/at/vote_result/XXVIII/A/4.png">'
			)
		);
		expect(html.match(/property="og:image"/g)).toHaveLength(1);
		expect(html).toContain('https://somes.at/og/at/vote_result/XXVIII/A/4.png');
		expect(html).toContain('<meta property="og:title" content="Abstimmung | Somes" />');
	});
});
