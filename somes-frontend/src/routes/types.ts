import { getWithRoute } from '$lib/api/api';
import { deleteWithAuth, postWithAuth, putWithAuth } from '$lib/api/authed';
import { getParliament, type Parliament } from '$lib/api/parliament';
import type { Document, HasError } from '$lib/types';

export function events(fetcher: typeof fetch = fetch): Promise<SomesEvent[] | HasError> {
	return getWithRoute('v1/events', 'at', fetcher);
}

export interface EventId {
	id: number;
}

export function createEvent(event: SomesEvent): Promise<EventId | HasError> {
	return postWithAuth('v1/events/create', event);
}

export function updateEvent(event: SomesEvent): Promise<null | HasError> {
	return putWithAuth('v1/events/update', event);
}

export function deleteEvent(eventId: number): Promise<null | HasError> {
	return deleteWithAuth('v1/events/delete', { id: eventId });
}

export type PlatformItemType = 'vote' | 'proposal' | 'decree';

export interface PlatformItem {
	id: number;
	path?: string;
	type: PlatformItemType;
	title: string;
	date: string;
	status?: 'accepted' | 'rejected' | 'pending';
}

export interface DialogEvent {
	event: SomesEvent;
	dialogOpen: boolean;
	hidden: boolean;
}

export interface SomesEvent {
	id: number | null;
	title: string;
	location: string;
	event_date: string; // ISO format YYYY-MM-DD
	start_time: string;
	description: string;
	image: string | null;
	requires_membership: boolean;
	requires_registration: boolean;
}

export interface Volksbg {
	id: number;
	slug: string;
	title: string;
	description: string | null;
	overview_url: string;
	state: string | null;
	ruling_date: string;
	cut_off_date: string;
	eintragungswoche: number | null;
	documents: Document[];
}

export interface VolksbgEintragungswoche {
	id: number | null;
	start_date: string | null;
	end_date: string | null;
	cut_off_date: string | null;
	online_deadline_utc: string | null;
	polling_stations_url: string | null;
	volksbgs: Volksbg[] | null;
}

export function volksbg_weeks(
	fetcher: typeof fetch = fetch,
	parliament: Parliament = getParliament()
): Promise<VolksbgEintragungswoche[] | HasError> {
	return getWithRoute<VolksbgEintragungswoche[]>('v1/volksbg/weeks', parliament, fetcher);
}
