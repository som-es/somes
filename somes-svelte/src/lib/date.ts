export function toTSDate(apiDate: string, endOfDay: boolean = false): Date {
	const date = new Date(apiDate);
	if (endOfDay) date.setHours(23, 59, 59, 999);

	return date;
}

// yyyy-mm-dd
export function toAPIDate(TSDate: Date): string {
	return TSDate.toISOString().substring(0, 10);
}

export function getAge(date: Date): number {
	return Math.abs(new Date(Date.now() - date.getTime()).getUTCFullYear() - 1970);
}

export function format(date: Date): string {
	// TODO: add localisation (i18n doesn't work here)
	return `Am ${date.toLocaleDateString()} um ${date.toLocaleTimeString()}`;
}
