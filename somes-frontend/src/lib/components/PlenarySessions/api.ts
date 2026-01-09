import { getWithRoute } from '$lib/api/api';
import type { HasError } from '$lib/types';

export interface PlenarDate {
	date_and_time: string;
}

export function next_plenar_date(fetcher: typeof fetch = fetch): Promise<PlenarDate | HasError> {
	return getWithRoute<PlenarDate>('next_plenar_date', "at/", fetcher);
}

export function plenar_dates(date: string): Promise<PlenarDate[] | HasError> {
	return getWithRoute<PlenarDate[]>(`plenar_dates?at=${date}`);
}
