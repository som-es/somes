<script lang="ts">
	import { justPostStatistics } from '$lib/api/api';
import { t } from '$lib/i18n/i18n.svelte';
	import StatisticsChartControl from '$lib/components/Statistics/StatisticsChartControl.svelte';
	import type { StatisticsData } from '$lib/types';
	import { mapCallToOrdersDelegate, mapCallToOrdersCategory } from '$lib/api/statistics-adapter';
	import Container from '$lib/components/Layout/Container.svelte';

	export let selectedCategory: string = 'delegate';

	const chartDescriptions = {
		'delegate.normalized': t('statistics.callToOrders.desc.delegateNormalized'),
		'delegate.absolute': t('statistics.callToOrders.desc.delegateAbsolute'),
		'party.normalized': t('statistics.callToOrders.desc.partyNormalized'),
		'party.absolute': t('statistics.callToOrders.desc.partyAbsolute'),
		'gender.normalized': t('statistics.callToOrders.desc.genderNormalized'),
		'gender.absolute': t('statistics.callToOrders.desc.genderAbsolute'),
		'age.normalized': t('statistics.callToOrders.desc.ageNormalized'),
		'age.absolute': t('statistics.callToOrders.desc.ageAbsolute'),
		'legis.normalized': t('statistics.callToOrders.desc.legisNormalized'),
		'legis.absolute': t('statistics.callToOrders.desc.legisAbsolute'),
		'line.normalized': t('statistics.callToOrders.desc.lineNormalized'),
		'line.absolute': t('statistics.callToOrders.desc.lineAbsolute'),
		'donut.normalized': t('statistics.callToOrders.desc.donutNormalized'),
		'donut.absolute': t('statistics.callToOrders.desc.donutAbsolute')
	};

	const delegateSimpleCallsToOrder = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean,
		normalized: boolean
	): Promise<StatisticsData[]> => {
		const response = await justPostStatistics<any[]>('call_to_orders_by_delegate', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc,
			normalized: normalized
		});

		if ('error' in response) {
			return [];
		}

		return mapCallToOrdersDelegate(response, normalized);
	};

	const partySimpleCallsToOrder = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean,
		normalized: boolean
	): Promise<StatisticsData[]> => {
		const response = await justPostStatistics<any[]>('call_to_orders_per_party', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc,
			normalized: normalized
		});

		if ('error' in response) {
			return [];
		}

		return mapCallToOrdersCategory(response, normalized);
	};

	const genderSimpleCallsToOrder = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean,
		normalized: boolean
	): Promise<StatisticsData[]> => {
		const response = await justPostStatistics<any[]>('call_to_orders_per_gender', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc,
			normalized: normalized
		});

		if ('error' in response) {
			return [];
		}

		return mapCallToOrdersCategory(response, normalized);
	};

	const ageSimpleCallsToOrder = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean,
		normalized: boolean
	): Promise<StatisticsData[]> => {
		const response = await justPostStatistics<any[]>('call_to_orders_per_age', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc,
			normalized: normalized
		});

		if ('error' in response) {
			return [];
		}

		return mapCallToOrdersCategory(response, normalized);
	};

	const legisSimpleCallsToOrder = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean,
		normalized: boolean
	): Promise<StatisticsData[]> => {
		const response = await justPostStatistics<any[]>('call_to_orders_per_legis', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc,
			normalized: normalized
		});

		if ('error' in response) {
			return [];
		}

		return mapCallToOrdersCategory(response, normalized);
	};

	// Get the appropriate function based on selected category
	$: currentFunction = (() => {
		switch (selectedCategory) {
			case 'delegate':
				return delegateSimpleCallsToOrder;
			case 'party':
				return partySimpleCallsToOrder;
			case 'gender':
				return genderSimpleCallsToOrder;
			case 'age':
				return ageSimpleCallsToOrder;
			case 'legis':
				return legisSimpleCallsToOrder;
			default:
				return delegateSimpleCallsToOrder;
		}
	})();
</script>

<svelte:head>
	<title>{t('statistics.callToOrders.title')}</title>
	<meta
		name="description"
		content={t('statistics.callToOrders.description')}
	/>
</svelte:head>

<Container class="pb-12">
	<div class="mt-2 mb-6">
		<h1 class="text-3xl font-bold sm:text-4xl">{t('statistics.callToOrders.h1')}</h1>
		<p class="mt-2 text-base text-gray-700 dark:text-gray-300">
			{t('statistics.callToOrders.intro')}
		</p>
	</div>

	<StatisticsChartControl
		makeRequest={currentFunction}
		height={520}
		bind:selectedCategory
		valueLabel={t('statistics.callToOrders.valueLabel')}
		normalizedValueLabel={t('statistics.callToOrders.normalizedValueLabel')}
		{chartDescriptions}
	/>
</Container>
