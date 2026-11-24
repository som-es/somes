export function shorten(text: string, max = 160): string {
	const clean = text.replace(/\s+/g, ' ').trim();
	if (clean.length <= max) return clean;
	return `${clean.slice(0, clean.lastIndexOf(' ', max - 1))}…`;
}
