<script lang="ts">
	import { justPostStatistics } from '$lib/api/api';
import { t } from '$lib/i18n/i18n.svelte';
	import StatisticsChartControl from '$lib/components/Statistics/StatisticsChartControl.svelte';
	import type { StatisticsData } from '$lib/types';
	import { mapActivityDelegate, mapActivityCategory } from '$lib/api/statistics-adapter';
	import Container from '$lib/components/Layout/Container.svelte';

	export let selectedCategory: string = 'delegate';

	const chartDescriptions = {
		'delegate.normalized':
			t('statistics.activity.desc.delegateNormalized'),
		'delegate.absolute': t('statistics.activity.desc.delegateAbsolute'),
		'party.normalized': t('statistics.activity.desc.partyNormalized'),
		'party.absolute': t('statistics.activity.desc.partyAbsolute'),
		'gender.normalized': t('statistics.activity.desc.genderNormalized'),
		'gender.absolute': t('statistics.activity.desc.genderAbsolute'),
		'age.normalized': t('statistics.activity.desc.ageNormalized'),
		'age.absolute': t('statistics.activity.desc.ageAbsolute'),
		'legis.normalized': t('statistics.activity.desc.legisNormalized'),
		'legis.absolute': t('statistics.activity.desc.legisAbsolute'),
		'line.normalized':
			t('statistics.activity.desc.lineNormalized'),
		'line.absolute': t('statistics.activity.desc.lineAbsolute'),
		'donut.normalized': t('statistics.activity.desc.donutNormalized'),
		'donut.absolute': t('statistics.activity.desc.donutAbsolute')
	};

	const delegateSimpleActivity = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean,
		normalized: boolean
	): Promise<StatisticsData[]> => {
		const response = await justPostStatistics<any[]>('activity_per_delegate', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc,
			normalized: normalized
		});

		if ('error' in response) {
			return [];
		}

		return mapActivityDelegate(response, normalized);
	};

	const partySimpleActivity = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean,
		normalized: boolean
	): Promise<StatisticsData[]> => {
		const response = await justPostStatistics<any[]>('activity_per_party', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc,
			normalized: normalized
		});

		if ('error' in response) {
			return [];
		}

		return mapActivityCategory(response, normalized);
	};

	const genderSimpleActivity = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean,
		normalized: boolean
	): Promise<StatisticsData[]> => {
		const response = await justPostStatistics<any[]>('activity_per_gender', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc,
			normalized: normalized
		});

		if ('error' in response) {
			return [];
		}

		return mapActivityCategory(response, normalized);
	};

	const ageSimpleActivity = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean,
		normalized: boolean
	): Promise<StatisticsData[]> => {
		const response = await justPostStatistics<any[]>('activity_per_age', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc,
			normalized: normalized
		});

		if ('error' in response) {
			return [];
		}

		return mapActivityCategory(response, normalized);
	};

	const legisSimpleActivity = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean,
		normalized: boolean
	): Promise<StatisticsData[]> => {
		const response = await justPostStatistics<any[]>('activity_per_legis', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc,
			normalized: normalized
		});

		if ('error' in response) {
			return [];
		}

		return mapActivityCategory(response, normalized);
	};

	// Get the appropriate function based on selected category
	$: currentFunction = (() => {
		switch (selectedCategory) {
			case 'delegate':
				return delegateSimpleActivity;
			case 'party':
				return partySimpleActivity;
			case 'gender':
				return genderSimpleActivity;
			case 'age':
				return ageSimpleActivity;
			case 'legis':
				return legisSimpleActivity;
			default:
				return delegateSimpleActivity;
		}
	})();
</script>

<svelte:head>
	<title>{t('statistics.activity.title')}</title>
	<meta
		name="description"
		content={t('statistics.activity.description')}
	/>
</svelte:head>

<Container class="pb-12">
	<div class="mt-2 mb-6">
		<h1 class="text-3xl font-bold sm:text-4xl">{t('statistics.activity.h1')}</h1>
		<p class="mt-2 text-base text-gray-700 dark:text-gray-300">
			{t('statistics.activity.intro')}
		</p>
	</div>

	<StatisticsChartControl
		height={520}
		makeRequest={currentFunction}
		bind:selectedCategory
		valueLabel={t('statistics.activity.valueLabel')}
		normalizedValueLabel={t('statistics.activity.normalizedValueLabel')}
		{chartDescriptions}
		infoQuestion={t('statistics.activity.infoQuestion')}
		infoAnswer={t('statistics.activity.infoAnswer')}
	/>
</Container>
