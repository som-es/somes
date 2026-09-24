<script lang="ts">
	import { justPostStatistics } from '$lib/api/api';
	import { t } from '$lib/i18n/i18n.svelte';
	import StatisticsChartControl from '$lib/components/Statistics/StatisticsChartControl.svelte';
	import Container from '$lib/components/Layout/Container.svelte';
	import {
		mapDivisionAccuracyDelegate,
		mapDivisionAccuracyCategory
	} from '$lib/api/statistics-adapter';
	import type { StatisticsData } from '$lib/types';

	let selectedCategory = $state('delegate');

	const endpoints: Record<string, string> = {
		delegate: 'division_accuracy_score_per_delegate',
		party: 'division_accuracy_score_per_party',
		gender: 'division_accuracy_score_per_gender',
		age: 'division_accuracy_score_per_age',
		legis: 'division_accuracy_score_per_legis'
	};

	const chartDescriptions = $derived({
		delegate: t('statistics.divisionAccuracy.desc.delegate'),
		party: t('statistics.divisionAccuracy.desc.party'),
		gender: t('statistics.divisionAccuracy.desc.gender'),
		age: t('statistics.divisionAccuracy.desc.age'),
		legis: t('statistics.divisionAccuracy.desc.legis'),
		line: t('statistics.divisionAccuracy.desc.line')
	});

	function requestFor(category: string) {
		return async (
			gp: string | null,
			gender: string | null,
			isDesc: boolean
		): Promise<StatisticsData[]> => {
			const response = await justPostStatistics<Parameters<typeof mapDivisionAccuracyDelegate>[0]>(
				endpoints[category] ?? endpoints.delegate,
				{ legis_period: gp, party: null, gender, is_desc: isDesc }
			);
			if ('error' in response) throw new Error(t('statistics.error.load'));
			const data =
				category === 'delegate'
					? mapDivisionAccuracyDelegate(response)
					: mapDivisionAccuracyCategory(response);
			return data;
		};
	}

	const currentFunction = $derived(requestFor(selectedCategory));
</script>

<svelte:head>
	<title>{t('statistics.divisionAccuracy.title')}</title>
	<meta name="description" content={t('statistics.divisionAccuracy.intro')} />
</svelte:head>

<Container class="pb-12">
	<div class="mt-2 mb-6">
		<h1 class="text-3xl font-bold sm:text-4xl">{t('statistics.divisionAccuracy.title')}</h1>
		<p class="mt-2 text-base text-gray-700 dark:text-gray-300">
			{t('statistics.divisionAccuracy.intro')}
		</p>
	</div>
	<StatisticsChartControl
		height={520}
		makeRequest={currentFunction}
		bind:selectedCategory
		valueLabel={t('statistics.divisionAccuracy.valueLabel')}
		normalizedValueLabel={t('statistics.divisionAccuracy.valueLabel')}
		filterConfig={{ showNormalized: false }}
		showDonutMode={false}
		{chartDescriptions}
		infoQuestion={t('statistics.divisionAccuracy.infoQuestion')}
		infoAnswer={t('statistics.divisionAccuracy.infoAnswer')}
	/>
</Container>
