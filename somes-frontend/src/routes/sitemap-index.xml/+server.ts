import type { RequestHandler } from './$types';
import { isHasError } from '$lib/api/api';
import { PARLIAMENTS } from '$lib/api/parliament';
import { sitemap_summary } from '$lib/api/sitemap';
import {
	absolute,
	sitemap_child_path,
	STATIC_SITEMAP_PATH,
	to_index_xml,
	xml_response
} from '$lib/seo/sitemap';

export const GET: RequestHandler = async ({ fetch }) => {
	const summaries = await Promise.all(
		PARLIAMENTS.map(async (parliament) => ({
			parliament,
			summary: await sitemap_summary(parliament, fetch)
		}))
	);

	const children = [absolute(STATIC_SITEMAP_PATH)];

	for (const { parliament, summary } of summaries) {
		if (isHasError(summary)) continue;

		if (summary.decrees > 0) children.push(absolute(sitemap_child_path('decrees', parliament)));
		if (summary.gov_proposals > 0)
			children.push(absolute(sitemap_child_path('gov_proposals', parliament)));
		if (summary.questions > 0) children.push(absolute(sitemap_child_path('questions', parliament)));

		for (const gp of summary.vote_results) {
			if (gp.entry_count > 0)
				children.push(absolute(sitemap_child_path('vote_results', parliament, gp.gp)));
		}
	}

	return xml_response(to_index_xml(children));
};
