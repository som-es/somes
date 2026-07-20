import {
	delegate_by_id,
	errorToNull,
	isHasError,
	latest_decrees,
	latest_ministrial_proposals,
	latest_session_activity_overview,
	latest_vote_results
} from '$lib/api/api';
import { fetchDelegates } from '$lib/api/fetch_delegates';
import { cachedAllSeats } from '$lib/caching/seats';
import type { DecreeDelegate } from '$lib/components/Delegates/Decrees/types';
import { next_plenar_date } from '$lib/components/PlenarySessions/api';
import type { Parliament } from '$lib/api/parliament';
import type { Delegate, HasError, VoteResult } from '$lib/types';
import type { PageServerLoad } from './$types';

const internalCache: Record<string, { data: any; timestamp: number }> = {};

const CACHE_DURATION_MS = 1000 * 60 * 10;

function hasDelegate(value: {
	decree: DecreeDelegate['decree'];
	delegate: Delegate | undefined;
}): value is DecreeDelegate {
	return value.delegate !== undefined;
}

async function fetchDelegatesFromVoteResult(
	latestVotes: VoteResult[] | HasError,
	fetcher: typeof fetch,
	parliament: Parliament
): Promise<Delegate[] | null> {
	if (isHasError(latestVotes)) {
		return [];
	}
	if (latestVotes.length == 0) return [];
	const date = latestVotes[0].legislative_initiative.nr_plenary_activity_date;
	const gp = latestVotes[0].legislative_initiative.gp;
	const dels = await fetchDelegates(date, gp, fetcher, parliament);
	return dels.delegates;
}

export const load: PageServerLoad = async ({ fetch, setHeaders, params }) => {
	const parliament = params.parliament as Parliament;
	const now = Date.now();
	const cached = internalCache[parliament];
	if (cached && now - cached.timestamp < CACHE_DURATION_MS) {
		return cached.data;
	}
	if (process.env.NODE_ENV === 'production') {
		setHeaders({
			'cache-control': 'max-age=1020'
		});
	}

	const [
		nextPlenarDate,
		latestVotes,
		latestMinisterialProposals,
		latestDecrees,
		allSeats,
		latestSessionActivity
	] = await Promise.all([
		next_plenar_date(fetch, parliament),
		latest_vote_results(fetch, parliament),
		latest_ministrial_proposals(30, fetch, parliament),
		latest_decrees(7, fetch, parliament),
		cachedAllSeats(false, fetch, parliament),
		latest_session_activity_overview(fetch, parliament)
	]);

	const delegates = await fetchDelegatesFromVoteResult(latestVotes, fetch, parliament);
	const res = errorToNull(latestDecrees)?.map(async (latestDecree) => {
		let delegate = delegates?.find((delegate) => delegate.id === latestDecree.gov_official_id);
		if (!delegate) {
			delegate =
				errorToNull(await delegate_by_id(latestDecree.gov_official_id, fetch, parliament)) ??
				undefined;
		}

		return { decree: latestDecree, delegate };
	});
	const latestDelegateDecrees: DecreeDelegate[] = (await Promise.all(res ?? [])).filter(
		hasDelegate
	);

	// TODO error handling

	const data = {
		nextPlenarDate,
		latestVotes,
		latestMinisterialProposals,
		latestDecrees,
		latestDelegateDecrees,
		latestSessionActivity,
		delegates,
		allSeats
	};

	internalCache[parliament] = {
		data,
		timestamp: now
	};

	return data;
};
