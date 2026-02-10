import { getWithRoute } from "$lib/api/api";
import { deleteWithAuth, postWithAuth, putWithAuth } from "$lib/api/authed";
import type { HasError } from "$lib/types";

export function events(fetcher: typeof fetch = fetch): Promise<SomesEvent[] | HasError> {
	return getWithRoute('v1/events', "at/", fetcher);
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
