import type { LegisPeriod } from './types';

export function dashDateToDotDate(date: string): string {
	const dateParts = date.split('-');

	return `${dateParts[2]}.${dateParts[1]}.${dateParts[0]}`;
}

export function legisDurationString(
	legisPeriod: LegisPeriod,
	next: LegisPeriod | undefined
): string {
	const startDate = dashDateToDotDate(legisPeriod.start_date.toString());
	if (next == undefined) return `${startDate} -`;
	return `${startDate} - ${dashDateToDotDate(next.start_date.toString())}`;
}

export function isEntryNew(date: Date) {
	const today = new Date();
	const diffMs = today.getTime() - date.getTime();
	const diffDays = Math.abs(diffMs) / (1000 * 60 * 60 * 24);

	return diffDays <= 7;
}
