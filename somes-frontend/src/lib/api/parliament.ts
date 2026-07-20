import { page } from '$app/state';

export type Parliament = 'at' | 'eu';

export function getParliament(): Parliament {
	try {

		const parl = (page.params as Record<string, string>).parliament === 'eu' ? 'eu' : 'at';
		return parl
	} catch {
		return 'at';
	}
}

export function plink(path: string): string {
	return `/${getParliament()}${path}`;
}
