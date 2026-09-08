import { getWithRoute } from '$lib/api/api';
import { getParliament, type Parliament } from '$lib/api/parliament';
import type { HasError } from '$lib/types';

export interface PlenarDate {
	date_and_time: string;
}

export function next_plenar_date(
	fetcher: typeof fetch = fetch,
	parliament: Parliament = getParliament()
): Promise<PlenarDate | HasError> {
	return getWithRoute<PlenarDate>('next_plenar_date', parliament, fetcher);
}

export function plenar_dates(date: string): Promise<PlenarDate[] | HasError> {
	return getWithRoute<PlenarDate[]>(`plenar_dates?at=${date}`);
}
