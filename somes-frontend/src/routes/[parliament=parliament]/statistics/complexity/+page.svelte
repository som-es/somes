<script lang="ts">
	import { justPostStatistics } from '$lib/api/api';
	import { t } from '$lib/i18n/i18n.svelte';
	import StatisticsChartControl from '$lib/components/Statistics/StatisticsChartControl.svelte';
	import Container from '$lib/components/Layout/Container.svelte';
	import { mapComplexityDelegate, mapComplexityCategory } from '$lib/api/statistics-adapter';
	import type { StatisticsData } from '$lib/types';

	let selectedCategory = $state('delegate');

	const endpoints: Record<string, string> = {
		delegate: 'complexity_per_delegate',
		party: 'complexity_per_party',
		gender: 'complexity_per_gender',
		age: 'complexity_at_age',
		legis: 'complexity_per_legis'
	};

	const chartDescriptions = $derived({
		delegate: t('statistics.complexity.desc.delegate'),
		party: t('statistics.complexity.desc.party'),
		gender: t('statistics.complexity.desc.gender'),
		age: t('statistics.complexity.desc.age'),
		legis: t('statistics.complexity.desc.legis'),
		line: t('statistics.complexity.desc.line')
	});

	function requestFor(category: string) {
		return async (
			gp: string | null,
			gender: string | null,
			isDesc: boolean
		): Promise<StatisticsData[]> => {
			const response = await justPostStatistics<Parameters<typeof mapComplexityDelegate>[0]>(
				endpoints[category] ?? endpoints.delegate,
				{ legis_period: gp, party: null, gender, is_desc: isDesc }
			);
			if ('error' in response) throw new Error(t('statistics.error.load'));
			const data =
				category === 'delegate' ? mapComplexityDelegate(response) : mapComplexityCategory(response);
			return data;
		};
	}

	const currentFunction = $derived(requestFor(selectedCategory));
</script>

<svelte:head>
	<title>{t('statistics.complexity.title')}</title>
	<meta name="description" content={t('statistics.complexity.intro')} />
</svelte:head>

<Container class="pb-12">
	<div class="mt-2 mb-6">
		<h1 class="text-3xl font-bold sm:text-4xl">{t('statistics.complexity.title')}</h1>
		<p class="mt-2 text-base text-gray-700 dark:text-gray-300">
			{t('statistics.complexity.intro')}
		</p>
	</div>
	<StatisticsChartControl
		height={520}
		makeRequest={currentFunction}
		bind:selectedCategory
		valueLabel={t('statistics.complexity.valueLabel')}
		normalizedValueLabel={t('statistics.complexity.valueLabel')}
		filterConfig={{ showNormalized: false }}
		showDonutMode={false}
		lineValueDomain={[1, 1.3]}
		valuePrecision={3}
		{chartDescriptions}
		infoQuestion={t('statistics.complexity.infoQuestion')}
		infoAnswer={t('statistics.complexity.infoAnswer')}
	/>
</Container>
