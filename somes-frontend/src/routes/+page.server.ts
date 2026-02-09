import {
	isHasError,
	latest_decrees,
	latest_ministrial_proposals,
	latest_vote_results
} from '$lib/api/api';
import { next_plenar_date } from '$lib/components/PlenarySessions/api';
import type { PageServerLoad } from './$types';
import type { PlatformItem } from './types';

let internalCache: {
	data: any;
	timestamp: number;
} | null = null;

const CACHE_DURATION_MS = 1000 * 60 * 10;

export const load: PageServerLoad = async ({ fetch, setHeaders }) => {
	const now = Date.now();
	if (internalCache && now - internalCache.timestamp < CACHE_DURATION_MS) {
		return internalCache.data;
	}
	if (process.env.NODE_ENV === 'production') {
		setHeaders({
			'cache-control': 'max-age=320'
		});
	}

	const [nextPlenarDate, latestVotes, latestMinisterialProposals, latestDecrees] =
		await Promise.all([
			next_plenar_date(fetch),
			latest_vote_results(fetch),
			latest_ministrial_proposals(21, fetch),
			latest_decrees(7, fetch)
		]);

	const platformItems: PlatformItem[] = [];

	const randomId = () => Math.random() * 10000000;

	if (!isHasError(latestVotes)) {
		latestVotes.forEach((vote) => {
			platformItems.push({
				id: randomId(),
				type: 'vote',
				title: vote.ai_summary?.short_title ?? vote.legislative_initiative.title,
				date: vote.legislative_initiative.nr_plenary_activity_date,
				status:
					vote.legislative_initiative.accepted == null
						? 'pending'
						: vote.legislative_initiative.accepted == 'a'
							? 'accepted'
							: 'rejected'
			});
		});
	}

	if (!isHasError(latestMinisterialProposals)) {
		latestMinisterialProposals.forEach((proposal) => {
			platformItems.push({
				id: randomId(),
				type: 'proposal',
				title:
					proposal.gov_proposal.ai_summary?.short_title ??
					proposal.gov_proposal.ministrial_proposal.title,
				date: proposal.gov_proposal.ministrial_proposal.raw_data_created_at,
				status: 'pending'
			});
		});
	}

	if (!isHasError(latestDecrees)) {
		latestDecrees.forEach((decree) => {
			platformItems.push({
				id: randomId(),
				type: 'decree',
				title: decree.short_title,
				date: decree.publication_date
			});
		});
	}

	platformItems.sort((a, b) => a.id - b.id);

	const data = {
		nextPlenarDate,
		platformItems
	};

	internalCache = {
		data,
		timestamp: now
	};

	return data;
};
