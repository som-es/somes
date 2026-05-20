<script lang="ts">
	import { justPostStatistics } from '$lib/api/api';
	import DelegateBarChartControl from '$lib/components/Statistics/DelegateBarChartControl.svelte';
	import type { StatisticsData } from '$lib/types';
	import { mapActivityDelegate, mapActivityCategory } from '$lib/api/statistics-adapter';
	import Container from '$lib/components/Layout/Container.svelte';

	export let selectedCategory: string = 'delegate';

	const chartDescriptions = {
		'delegate.normalized':
			'Gewichtete Initiativen je Abgeordneter oder Abgeordnetem, geteilt durch die Sitzungen der Periode.',
		'delegate.absolute': 'Summe der gewichteten Initiativen je Abgeordneter oder Abgeordnetem.',
		'party.normalized': 'Durchschnittliche Aktivitäts-Punkte pro Sitzung, je Partei.',
		'party.absolute': 'Durchschnittliche Aktivitäts-Punkte der Abgeordneten einer Partei.',
		'gender.normalized': 'Durchschnittliche Aktivitäts-Punkte pro Sitzung, nach Geschlecht.',
		'gender.absolute': 'Durchschnittliche Aktivitäts-Punkte nach Geschlecht.',
		'age.normalized': 'Durchschnittliche Aktivitäts-Punkte pro Sitzung, nach Altersgruppen.',
		'age.absolute': 'Durchschnittliche Aktivitäts-Punkte nach Altersgruppen.',
		'legis.normalized': 'Durchschnittliche Aktivitäts-Punkte pro Sitzung, je Legislaturperiode.',
		'legis.absolute': 'Durchschnittliche Aktivitäts-Punkte je Legislaturperiode.',
		'line.normalized':
			'Entwicklung der durchschnittlichen Aktivität pro Sitzung über die Perioden.',
		'line.absolute': 'Entwicklung der durchschnittlichen Aktivitäts-Punkte über die Perioden.',
		'donut.normalized': 'Anteil der höchsten Aktivitätswerte pro Sitzung in der aktuellen Auswahl.',
		'donut.absolute': 'Anteil der höchsten Aktivitäts-Punkte in der aktuellen Auswahl.'
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
	<title>Aktivitätsstatistiken</title>
	<meta
		name="description"
		content="Aktivitätsstatistiken über den Nationalrat und deren Abgeordnete"
	/>
</svelte:head>

<Container class="pb-12">
	<div class="mt-2 mb-6">
		<h1 class="text-3xl font-bold sm:text-4xl">Aktivitätsstatistiken</h1>
		<p class="mt-2 text-base text-gray-700 dark:text-gray-300">
			Parlamentarische Aktivität anhand eingebrachter Initiativen.
		</p>
	</div>

	<DelegateBarChartControl
		height={520}
		delegateMakeRequest={currentFunction}
		bind:selectedCategory
		valueLabel="Aktivitäts-Punkte"
		normalizedValueLabel="durchschnittliche Aktivitäts-Punkte (pro Sitzung)"
		{chartDescriptions}
		infoQuestion="Wie werden Aktivitäts-Punkte berechnet?"
		infoAnswer="
			<p><strong>Aktivitäts-Punkte:</strong> gewichtete Summe der eingebrachten parlamentarischen Initiativen.</p>
			<ul class='ml-4 space-y-1 text-xs'>
				<li>Anfragen (J): <strong>0,35 Punkte</strong></li>
				<li>Unselbständige Entschließungsanträge (UEA): <strong>0,75 Punkte</strong></li>
				<li>Abänderungsanträge (AA): <strong>0,9 Punkte</strong></li>
				<li>Selbständige Anträge (A): <strong>1 Punkt</strong></li>
				<li>Initiativen (I): <strong>1,25 Punkte</strong></li>
			</ul>
			<p class='mt-2'>Normalisiert wird durch die Anzahl der Sitzungen der jeweiligen Legislaturperiode geteilt.</p>
		"
	/>
</Container>
