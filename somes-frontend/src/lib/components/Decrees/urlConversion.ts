import { resolve } from "$app/paths";
import { page } from "$app/state";
import type { DecreeFilter } from "../Delegates/Decrees/types";

export function convertDecreeFilterToUrl(
	filter: DecreeFilter | null,
	searchValue: string,
	currentUrl: URL | undefined,
): URL {
	const nextUrl = currentUrl
		? currentUrl
		: new URL(
				resolve('/history/proposals'),
				page.url.origin
			);
    nextUrl.search = ""
    nextUrl.searchParams.set('page', '1');

    if (!filter) {
        return nextUrl;
    }

    if (filter.legis_period !== null) {
        nextUrl.searchParams.set('decree[gp][in][0]', filter.legis_period);
    }
    filter.topics?.forEach((topic, i) => {
        nextUrl.searchParams.set(`decree[ai_summary][full_summary][topics][in][${i}]`, topic);
    });

    filter.departments?.forEach((department, i) => {
        nextUrl.searchParams.set(`decree[ministrial_issuer][in][${i}]`, department);
    });

    nextUrl.searchParams.set('search', searchValue);

    return nextUrl;
}