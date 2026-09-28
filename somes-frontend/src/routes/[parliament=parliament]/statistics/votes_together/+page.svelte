<script lang="ts">
	import { justPostStatistics } from '$lib/api/api';
	import { t } from '$lib/i18n/i18n.svelte';
	import Container from '$lib/components/Layout/Container.svelte';
	import StatisticsChartControl from '$lib/components/Statistics/StatisticsChartControl.svelte';
	import type { StatisticsData } from '$lib/types';
	import { partyToColor } from '$lib/partyColor';

	type VotesTogetherRow = {
		party_1: string;
		party_2: string;
		parties?: string[];
		group_size?: number;
		same_votes: number;
		total_votes: number;
		agreement_percentage: number;
	};
	let parties = $state<string[]>([]);
	let selectedParties = $state<string[]>([]);
	let selectedParty = $derived(selectedParties[0] ?? '');
	let partyChartMode = $state<'bar' | 'donut'>('bar');
	let selectedGroupSize = $state<'all' | '2' | '3'>('all');
	const votesTogetherCache = new Map<string, Promise<VotesTogetherRow[]>>();

	$effect(() => {
		if (partyChartMode === 'donut' && selectedParties.length === 0 && parties.length > 0) {
			selectedParties = [parties[0]];
		}
	});

	async function getVotesTogether(gp: string | null, isDesc: boolean): Promise<VotesTogetherRow[]> {
		const key = `${gp ?? 'all'}:${isDesc ? 'desc' : 'asc'}`;
		const cached = votesTogetherCache.get(key);
		if (cached) return cached;

		const request = (async () => {
			const response = await justPostStatistics<VotesTogetherRow[]>('votes_together', {
				legis_period: gp,
				is_desc: isDesc
			});
			if ('error' in response) throw new Error(t('statistics.error.load'));
			return response;
		})();
		votesTogetherCache.set(key, request);
		try {
			return await request;
		} catch (error) {
			votesTogetherCache.delete(key);
			throw error;
		}
	}

	const loadVotesTogether = async (
		gp: string | null,
		_gender: string | null,
		isDesc: boolean,
		normalized: boolean,
		_chartMode?: 'bar' | 'donut' | 'line' | 'spectrum'
	): Promise<StatisticsData[]> => {
		const responseRows = await getVotesTogether(gp, isDesc);
		// Older running API versions only return party_1 and party_2.
		const compatibleRows = responseRows.map((row) => {
			const group = row.parties ?? [row.party_1, row.party_2];
			return { ...row, parties: group, group_size: row.group_size ?? group.length };
		});
		const rows =
			selectedGroupSize === 'all'
				? compatibleRows
				: compatibleRows.filter((row) => row.group_size === Number(selectedGroupSize));
		if (
			normalized &&
			rows.some((row) => !Number.isFinite(row.agreement_percentage) || !Number.isFinite(row.total_votes))
		) {
			throw new Error(t('statistics.error.load'));
		}
		const availableParties = [...new Set(rows.flatMap((row) => row.parties))].sort(
			(a, b) => a.localeCompare(b, 'de-AT')
		);
		if (parties.join('\0') !== availableParties.join('\0')) parties = availableParties;
		const activeParty = availableParties.includes(selectedParty) ? selectedParty : '';
		if (selectedParty && !activeParty) selectedParties = [];

		const relevantRows = activeParty
			? rows.filter((row) => row.parties.includes(activeParty))
			: rows;

		return relevantRows.map(({ parties: group, group_size, same_votes, agreement_percentage, total_votes }) => {
			const label = activeParty
				? group.filter((party) => party !== activeParty).join(' + ')
				: group.join(' + ');
			return {
				type: 'category',
				label,
				value: normalized ? agreement_percentage : same_votes,
				party: activeParty ? label : undefined,
				metadata: { parties: group, group_size, same_votes, total_votes, agreement_percentage }
			};
		});
	};

</script>

<Container class="pb-12">
	<div class="mt-2 mb-6">
		<h1 class="text-3xl font-bold sm:text-4xl">{t('statistics.votesTogether.title')}</h1>
		<p class="mt-2 text-base text-gray-700 dark:text-gray-300">
			{t('statistics.votesTogether.intro')}
		</p>
	</div>

	<StatisticsChartControl
		height={520}
		categoryLabelWidth="14rem"
		makeRequest={loadVotesTogether}
		bind:selectedCategory={selectedGroupSize}
		valueLabel={t('statistics.votesTogether.valueLabel')}
		normalizedValueLabel={t('statistics.votesTogether.agreementLabel')}
		valuePrecision={1}
		reloadKey={(selectedParties[0] ?? '') + selectedGroupSize}
		filterConfig={{ showNormalized: true, showPeriod: true, showGender: false, showParty: false }}
		categoryOptions={[
			{ value: 'all', label: t('statistics.votesTogether.allGroups') },
			{ value: '2', label: t('statistics.votesTogether.twoParties') },
			{ value: '3', label: t('statistics.votesTogether.threeParties') }
		]}
		partyFilterOptions={parties.map((name) => ({ name, color: partyToColor(name) }))}
		bind:selectedPartyFilter={selectedParties}
		bind:selectedChartMode={partyChartMode}
	/>
</Container>
