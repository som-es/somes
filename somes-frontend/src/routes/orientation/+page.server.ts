import type { PageServerLoad } from './$types';
import { getWithRoute, isHasError } from '$lib/api/api';
import type { Delegate, DelegatesWithMaxPage, StanceTopicScore } from '$lib/types';

export const load: PageServerLoad = async ({ fetch }) => {
	const questions = await getWithRoute('orientation_questions', 'at', fetch);
	const delegates = await getWithRoute<DelegatesWithMaxPage>(
		'v1/delegates/search?search=&page=1&entries_per_page=600&mandates[0][is_nr][eq]=true&mandates[0][start_date][gte]=2017-01-01',
		'at',
		fetch
	);
	const govOfficials = await getWithRoute<DelegatesWithMaxPage>(
		'v1/delegates/search?search=&page=1&entries_per_page=600&mandates[0][is_gov_official][eq]=true&mandates[0][start_date][gte]=2017-01-01',
		'at',
		fetch
	);

	let filteredDelegates: Delegate[] = [];
	if (!isHasError(delegates)) {
		filteredDelegates = delegates.delegates.filter(
			(delegate) =>
				(delegate.mandates ?? []).findIndex(
					(mandate) => mandate.is_nr && mandate.start_date! >= '2017-01-01'
				) != -1
		);
	}

	let filteredGovOfficials: Delegate[] = [];
	if (!isHasError(govOfficials)) {
		filteredGovOfficials = govOfficials.delegates.filter(
			(delegate) =>
				(delegate.mandates ?? []).findIndex(
					(mandate) => mandate.is_gov_official && mandate.start_date! >= '2017-01-01'
				) != -1
		);
	}

	// Combine both lists and deduplicate by delegate.id
	const combinedDelegatesMap = new Map<number, Delegate>();
	[...filteredDelegates, ...filteredGovOfficials].forEach((d) => combinedDelegatesMap.set(d.id, d));
	const combinedDelegates = Array.from(combinedDelegatesMap.values());

	const fetchTopicPoliticalScores = async (delegates: Delegate[]) => {
		const results = await Promise.all(
			delegates.map(async (delegate) => {
				const scores = await getWithRoute<StanceTopicScore[]>(
					`v1/delegates/political_analysis/left_right_topic_score?delegate_id=${delegate.id}`,
					'at',
					fetch
				);
				const position = await getWithRoute<any>(
					`v1/delegates/political_analysis/political_position?delegate_id=${delegate.id}`,
					'at',
					fetch
				);
				return {
					delegate,
					scores: isHasError(scores) ? [] : scores,
					position: isHasError(position) ? null : position
				};
			})
		);
		return results;
	};

	const delegateScores = await fetchTopicPoliticalScores(combinedDelegates);

	return { questions, delegateScores };
};
