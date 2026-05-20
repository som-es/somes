<script lang="ts">
	import { justPostStatistics } from '$lib/api/api';
	import DelegateBarChartControl from '$lib/components/Statistics/DelegateBarChartControl.svelte';
	import type { StatisticsData } from '$lib/types';
	import { mapSpeechTimeDelegate, mapSpeechTimeCategory } from '$lib/api/statistics-adapter';
	import Container from '$lib/components/Layout/Container.svelte';

	export let selectedCategory: string = 'delegate';

	const chartDescriptions = {
		'delegate.normalized': 'Durchschnittliche Dauer einer Rede je Abgeordneter oder Abgeordnetem.',
		'delegate.absolute': 'Gesamte Redezeit je Abgeordneter oder Abgeordnetem.',
		'party.normalized': 'Durchschnittliche Rededauer der Abgeordneten einer Partei.',
		'party.absolute': 'Gesamte Redezeit, nach Parteien zusammengefasst.',
		'gender.normalized': 'Durchschnittliche Rededauer im Vergleich nach Geschlecht.',
		'gender.absolute': 'Gesamte Redezeit im Vergleich nach Geschlecht.',
		'age.normalized': 'Durchschnittliche Rededauer nach Altersgruppen.',
		'age.absolute': 'Gesamte Redezeit nach Altersgruppen.',
		'legis.normalized': 'Durchschnittliche Rededauer je Legislaturperiode.',
		'legis.absolute': 'Gesamte Redezeit je Legislaturperiode.',
		'line.normalized': 'Entwicklung der durchschnittlichen Rededauer über die Perioden.',
		'line.absolute': 'Entwicklung der gesamten Redezeit über die Perioden.',
		'donut.normalized':
			'Anteil der höchsten durchschnittlichen Rededauern in der aktuellen Auswahl.',
		'donut.absolute': 'Anteil der höchsten Redezeiten in der aktuellen Auswahl.'
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
	<title>Redezeitstatistiken - Parlamentsinformationssystem</title>
	<meta name="description" content="Redezeiten im Parlament nach Personen und Gruppen" />
</svelte:head>

<Container class="pb-12">
	<div class="mt-2 mb-6">
		<h1 class="text-3xl font-bold sm:text-4xl">Redezeitstatistiken</h1>
		<p class="mt-2 text-base text-gray-700 dark:text-gray-300">
			Gesamte Redezeit und durchschnittliche Rededauer im Parlament.
		</p>
	</div>

	<DelegateBarChartControl
		delegateMakeRequest={currentFunction}
		height={560}
		bind:selectedCategory
		valueLabel="Redezeit"
		normalizedValueLabel="Durchschnittliche Rededauer"
		{chartDescriptions}
	/>
</Container>
