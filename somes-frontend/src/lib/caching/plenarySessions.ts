import { isHasError, plenary_sessions_per_gp } from '$lib/api/api';
import { getParliament, type Parliament } from '$lib/api/parliament';
import type { PlenarySession } from '$lib/types';

const allPlenarySessions: Record<string, Record<string, PlenarySession[]>> = {};

export async function cachedPlenarySessions(
	refetch: boolean = false,
	fetcher: typeof fetch = fetch,
	parliament: Parliament = getParliament()
): Promise<Record<string, PlenarySession[]> | null> {
	if (allPlenarySessions[parliament] == null || refetch) {
		const fetched = await plenary_sessions_per_gp(fetcher, parliament);
		if (!isHasError(fetched)) {
			allPlenarySessions[parliament] = fetched;
		}
	}
	return allPlenarySessions[parliament] ?? null;
}
