export function ministerialInfoBadges(
	issuer: string | null,
	date: string,
	gp: string | null,
	locale?: string
): string[] {
	return [
		issuer,
		new Date(date).toLocaleDateString(locale, { timeZone: 'Europe/Vienna' }),
		gp
	].filter((x) => x !== null) as string[];
}
