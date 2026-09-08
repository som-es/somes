<script lang="ts">
	import { justPostStatistics } from '$lib/api/api';
	import { t } from '$lib/i18n/i18n.svelte';
	import StatisticsChartControl from '$lib/components/Statistics/StatisticsChartControl.svelte';
	import type { StatisticsData } from '$lib/types';
	import { mapSpeechTimeDelegate, mapSpeechTimeCategory } from '$lib/api/statistics-adapter';
	import Container from '$lib/components/Layout/Container.svelte';

	export let selectedCategory: string = 'delegate';

	const chartDescriptions = {
		// Descriptions will be translated via i18n in the chart control; keep keys
		'delegate.normalized': t('statistics.speechTime.desc.delegateNormalized'),
		'delegate.absolute': t('statistics.speechTime.desc.delegateAbsolute'),
		'party.normalized': t('statistics.speechTime.desc.partyNormalized'),
		'party.absolute': t('statistics.speechTime.desc.partyAbsolute'),
		'gender.normalized': t('statistics.speechTime.desc.genderNormalized'),
		'gender.absolute': t('statistics.speechTime.desc.genderAbsolute'),
		'age.normalized': t('statistics.speechTime.desc.ageNormalized'),
		'age.absolute': t('statistics.speechTime.desc.ageAbsolute'),
		'legis.normalized': t('statistics.speechTime.desc.legisNormalized'),
		'legis.absolute': t('statistics.speechTime.desc.legisAbsolute'),
		'line.normalized': t('statistics.speechTime.desc.lineNormalized'),
		'line.absolute': t('statistics.speechTime.desc.lineAbsolute'),
		'donut.normalized': t('statistics.speechTime.desc.donutNormalized'),
		'donut.absolute': t('statistics.speechTime.desc.donutAbsolute')
	};

	const delegateSimpleSpeechTime = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean,
		normalized: boolean
	): Promise<StatisticsData[]> => {
		const response = await justPostStatistics<any[]>('speechtime_per_delegate', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc,
			normalized
		});

		if ('error' in response) {
			return [];
		}

		return mapSpeechTimeDelegate(response, normalized);
	};

	const partySimpleSpeechTime = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean,
		normalized: boolean
	): Promise<StatisticsData[]> => {
		const response = await justPostStatistics<any[]>('speechtime_per_party', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc,
			normalized
		});

		if ('error' in response) {
			return [];
		}

		return mapSpeechTimeCategory(response, normalized);
	};

	const genderSimpleSpeechTime = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean,
		normalized: boolean
	): Promise<StatisticsData[]> => {
		const response = await justPostStatistics<any[]>('speechtime_per_gender', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc,
			normalized
		});

		if ('error' in response) {
			return [];
		}

		return mapSpeechTimeCategory(response, normalized);
	};

	const ageSimpleSpeechTime = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean,
		normalized: boolean
	): Promise<StatisticsData[]> => {
		const response = await justPostStatistics<any[]>('speechtime_per_age', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc,
			normalized
		});

		if ('error' in response) {
			return [];
		}

		return mapSpeechTimeCategory(response, normalized);
	};

	const legisSimpleSpeechTime = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean,
		normalized: boolean
	): Promise<StatisticsData[]> => {
		const response = await justPostStatistics<any[]>('speechtime_per_legis', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc,
			normalized
		});

		if ('error' in response) {
			return [];
		}

		return mapSpeechTimeCategory(response, normalized);
	};

	// Get the appropriate function based on selected category
	$: currentFunction = (() => {
		switch (selectedCategory) {
			case 'delegate':
				return delegateSimpleSpeechTime;
			case 'party':
				return partySimpleSpeechTime;
			case 'gender':
				return genderSimpleSpeechTime;
			case 'age':
				return ageSimpleSpeechTime;
			case 'legis':
				return legisSimpleSpeechTime;
			default:
				return delegateSimpleSpeechTime;
		}
	})();
</script>

<svelte:head>
	<title>{t('statistics.speechTime.title')}</title>
	<meta name="description" content={t('statistics.speechTime.description')} />
</svelte:head>

<Container class="pb-12">
	<div class="mt-2 mb-6">
		<h1 class="text-3xl font-bold sm:text-4xl">{t('statistics.speechTime.h1')}</h1>
		<p class="mt-2 text-base text-gray-700 dark:text-gray-300">
			Gesamte Redezeit und durchschnittliche Rededauer im Parlament.
		</p>
	</div>

	<StatisticsChartControl
		makeRequest={currentFunction}
		height={560}
		bind:selectedCategory
		valueLabel={t('statistics.speechTime.valueLabel')}
		normalizedValueLabel={t('statistics.speechTime.normalizedValueLabel')}
		{chartDescriptions}
	/>
</Container>
