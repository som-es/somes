import { isHasError, plenary_sessions_per_gp } from '$lib/api/api';
import type { PlenarySession } from '$lib/types';

let allPlenarySessions: Record<string, PlenarySession[]> | null = null;

export async function cachedPlenarySessions(
	refetch: boolean = false,
	fetcher: typeof fetch = fetch
): Promise<Record<string, PlenarySession[]> | null> {
	if (allPlenarySessions == null || refetch) {
		const fetched = await plenary_sessions_per_gp(fetcher);
		if (!isHasError(fetched)) {
			allPlenarySessions = fetched;
		}
	}
	return allPlenarySessions;
}
