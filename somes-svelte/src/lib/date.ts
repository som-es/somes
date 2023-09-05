export function toTSDate(apiDate: string): Date {
	return new Date(apiDate);
}

export function toAPIDate(TSDate: Date): string {
	return TSDate.toISOString();
}

export function getAge(date: Date): number {
	return Math.abs(new Date(Date.now() - date.getTime()).getUTCFullYear() - 1970);
}

export function format(date: Date): string {
	// TODO: add localisation (i18n doesn't work here)
	return `Am ${date.toLocaleDateString()} um ${date.toLocaleTimeString()}`;
}
