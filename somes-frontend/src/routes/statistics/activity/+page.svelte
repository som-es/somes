<script lang="ts">
	import { justPostStatistics } from '$lib/api/api';
	import DelegateBarChartControl from '$lib/components/Statistics/DelegateBarChartControl.svelte';
	import type { StatisticsData } from '$lib/types';
	import { mapActivityDelegate, mapActivityCategory } from '$lib/api/statistics-adapter';

	export let selectedCategory: string = 'delegate';

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
			case 'delegate': return delegateSimpleActivity;
			case 'party': return partySimpleActivity;
			case 'gender': return genderSimpleActivity;
			case 'age': return ageSimpleActivity;
			case 'legis': return legisSimpleActivity;
			default: return delegateSimpleActivity;
		}
	})();

	// Static title
	$: currentTitle = 'Aktivitätsstatistiken';
</script>

<svelte:head>
    <title>Aktivitätsstatistiken</title>
    <meta name="description" content="Aktivitätsstatistiken über den Nationalrat und deren Abgeordnete" />
</svelte:head>

<div class="container mx-auto px-4 py-8">
	<!-- Chart -->
	<div class="bg-card rounded-xl p-6 shadow-sm">
		<DelegateBarChartControl
			height={500}
			delegateMakeRequest={currentFunction}
			selectedCategory={selectedCategory}
			valueLabel="Aktivitäts-Punkte"
			normalizedValueLabel="durchschnittliche Aktivitäts-Punkte (pro Sitzung)"
			infoQuestion="Wie werden Aktivitäts-Punkte berechnet?"
			infoAnswer="
				<p><strong>Aktivitäts-Punkte:</strong> Summe der gewichteten Vorstösse</p>
				<ul class='ml-4 space-y-1 text-xs'>
					<li>• Motion: <strong>1 Punkt</strong></li>
					<li>• Postulat: <strong>1.2 x Anzahl</strong></li>
					<li>• Anfrage: <strong>1.2 x Anzahl</strong></li>
					<li>• Dringliche Anfrage: <strong>1.15 x Anzahl</strong></li>
					<li>• Initiative: <strong>1.3 x Anzahl</strong></li>
				</ul>
			"
		/>
	</div>
</div>
