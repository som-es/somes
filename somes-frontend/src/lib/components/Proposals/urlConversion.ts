import { resolve } from "$app/paths";
import { page } from "$app/state";
import type { GovPropFilter } from "$lib/types";

export function convertGovPropFilterToUrl(
	filter: GovPropFilter | null,
	searchValue: string,
	currentUrl: URL | undefined,
): URL {
	const nextUrl = currentUrl
		? currentUrl
		: new URL(
				resolve('/history/proposals'),
				page.url.origin
			);
    nextUrl.search = "";
    nextUrl.searchParams.set('page', '1');

    if (!filter) {
        return nextUrl
    }

    if (filter.has_vote_result) {
        nextUrl.searchParams.set(
            'gov_proposal[ministrial_proposal][has_vote_result][eq]',
            filter.has_vote_result.toString()
        );
    }
    if (filter.legis_period !== null) {
        nextUrl.searchParams.set('gov_proposal[ministrial_proposal][gp][in][0]', filter.legis_period);
    }
    filter.topics?.forEach((topic, i) => {
        nextUrl.searchParams.set(`gov_proposal[eurovoc_topics][${i}][topic][cn]`, topic);
    });
    filter.departments?.forEach((department, i) => {
        nextUrl.searchParams.set(`gov_proposal[ministrial_proposal][ressort][in][${i}]`, department);
    });

    nextUrl.searchParams.set('search', searchValue);

    return nextUrl;
}