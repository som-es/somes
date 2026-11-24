import { SITE_URL } from './sitemap';

export interface Crumb {
	name: string;
	path?: string;
}

export function breadcrumb_json_ld(crumbs: Crumb[]): Record<string, unknown> {
	return {
		'@context': 'https://schema.org',
		'@type': 'BreadcrumbList',
		itemListElement: crumbs.map((crumb, index) => ({
			'@type': 'ListItem',
			position: index + 1,
			name: crumb.name,
			...(crumb.path ? { item: `${SITE_URL}${crumb.path}` } : {})
		}))
	};
}
