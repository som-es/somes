import { page } from '$app/state';
import { plink } from '$lib/api/parliament';
import type { DelegateQuestionFilter } from '$lib/types';

export function convertDelegateQuestionFilterToUrl(
	filter: DelegateQuestionFilter | null,
	searchValue: string,
	currentUrl: URL | undefined,
	sort: 'Desc' | 'Asc' | 'relevance' = 'relevance',
	topicIdByName: Map<string, string> = new Map()
): URL {
	const nextUrl = currentUrl ? currentUrl : new URL(plink('/questions'), page.url.origin);
	nextUrl.search = '';

	nextUrl.searchParams.set('page', '1');

	if (!filter) {
		nextUrl.searchParams.set('sort', 'Desc');
		return nextUrl;
	}

	nextUrl.searchParams.set('page', filter.page?.toString() ?? '1');
	
	filter.topics?.forEach((topic, i) => {
		nextUrl.searchParams.set(`filter_topics[${i}]`, topicIdByName.get(topic) ?? topic);
	});

	if (filter.date_from) {
		nextUrl.searchParams.set('date_from', filter.date_from);
	}
	if (filter.date_to) {
		nextUrl.searchParams.set('date_to', filter.date_to);
	}

	// enforce with frontend => add user sorting
	if (searchValue.length === 0 || sort === 'Desc') {
		nextUrl.searchParams.set('sort', 'Desc');
	} else if (sort === 'Asc') {
		nextUrl.searchParams.set('sort', 'Asc');
	}
	// else relevance: no sort param, backend uses relevance ranking

	nextUrl.searchParams.set('search', searchValue);

	return nextUrl;
}
