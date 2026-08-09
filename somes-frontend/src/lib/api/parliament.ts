import { page } from '$app/state';

export type Parliament = 'at' | 'eu';

export function getParliament(): Parliament {
	try {
		const parl = (page.params as Record<string, string>).parliament === 'eu' ? 'eu' : 'at';
		return parl;
	} catch {
		return 'at';
	}
}
export function defaultGpByParliament(parliament: Parliament): string {
	switch (parliament) {
		case 'at':
			return 'XXVIII';
		case 'eu':
			return '10';
	}
}

export function defaultGp(): string {
	return defaultGpByParliament(getParliament());
}

export function plink(path: string): string {
	return `/${getParliament()}${path}`;
}
