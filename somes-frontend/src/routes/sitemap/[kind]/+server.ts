import { error } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { PARLIAMENTS, type Parliament } from '$lib/api/parliament';
import { sitemap_routes } from '$lib/api/sitemap';
import {
	absolute,
	route_path,
	static_sitemap_urls,
	SITEMAP_CHILD_KINDS,
	type SitemapChildKind,
	to_urlset_xml,
	xml_response,
	type SitemapUrl
} from '$lib/seo/sitemap';

export const GET: RequestHandler = async ({ params, url, fetch }) => {
	const kind = params.kind.replace(/\.xml$/, '');

	if (!isChildKind(kind)) error(404, 'Unknown sitemap');

	if (kind === 'static') return xml_response(to_urlset_xml(static_sitemap_urls()));

	const parliamentParam = url.searchParams.get('parliament');
	if (!isParliamentParam(parliamentParam)) error(400, 'Unknown parliament');

	const gp = kind === 'vote_results' ? (url.searchParams.get('gp') ?? undefined) : undefined;
	const routes = await sitemap_routes(kind, parliamentParam, fetch, gp);

	const urls: SitemapUrl[] = routes.map((route) => ({
		loc: absolute(route_path(parliamentParam, kind, route)),
		lastmod: route.lastmod
	}));

	return xml_response(to_urlset_xml(urls));
};

function isChildKind(kind: string): kind is SitemapChildKind {
	return (SITEMAP_CHILD_KINDS as readonly string[]).includes(kind);
}

function isParliamentParam(parliament: string | null): parliament is Parliament {
	return PARLIAMENTS.includes(parliament as Parliament);
}
