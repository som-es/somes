<script lang="ts">
	import { justPostStatistics } from '$lib/api/api';
	import { mapOrientationCategory, mapOrientationDelegate } from '$lib/api/statistics-adapter';
	import Container from '$lib/components/Layout/Container.svelte';
	import DelegateBarChartControl from '$lib/components/Statistics/DelegateBarChartControl.svelte';
	import type { StatisticsData } from '$lib/types';

	type Orientation = 'left' | 'right' | 'liberal' | 'authoritarian';

	let selectedCategory = $state('delegate');

	const categoryOptions = [
		{ value: 'delegate', label: 'Abgeordnete' },
		{ value: 'party', label: 'Parteien' },
		{ value: 'gender', label: 'Geschlecht' },
		{ value: 'age', label: 'Alter' },
		{ value: 'legis', label: 'Legislaturperioden' }
	];

	const orientationOptions: {
		value: Orientation;
		label: string;
		valueLabel: string;
		description: string;
	}[] = [
		{
			value: 'left',
			label: 'Links',
			valueLabel: 'Links-Score',
			description: 'Abgeordnete mit den höchsten gespeicherten Linksorientierungswerten.'
		},
		{
			value: 'right',
			label: 'Rechts',
			valueLabel: 'Rechts-Score',
			description: 'Abgeordnete mit den höchsten gespeicherten Rechtsorientierungswerten.'
		},
		{
			value: 'liberal',
			label: 'Libertär',
			valueLabel: 'Libertärer Score',
			description: 'Abgeordnete mit den höchsten gespeicherten libertären Orientierungswerten.'
		},
		{
			value: 'authoritarian',
			label: 'Autoritär',
			valueLabel: 'Autoritärer Score',
			description: 'Abgeordnete mit den höchsten gespeicherten autoritären Orientierungswerten.'
		}
	];

	let selectedOrientation = $state<Orientation>('left');

	const selectedOrientationOption = $derived(
		orientationOptions.find((option) => option.value === selectedOrientation) ?? orientationOptions[0]
	);

	const loadOrientation = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean
	): Promise<StatisticsData[]> => {
		const response = await justPostStatistics<any[]>(
			`is_${selectedOrientation}_per_${selectedCategory}`,
			{
				legis_period: gp,
				party: null,
				gender,
				is_desc: isDesc
			}
		);

		if ('error' in response) {
			return [];
		}

		return selectedCategory === 'delegate'
			? mapOrientationDelegate(response)
			: mapOrientationCategory(response);
	};
</script>

<svelte:head>
	<title>Politische Positionen</title>
	<meta
		name="description"
		content="Statistiken zu politischen Positionswerten von Abgeordneten"
	/>
</svelte:head>

<Container class="pb-12">
	<div class="mt-2 mb-6">
		<h1 class="text-3xl font-bold sm:text-4xl">Politische Positionen</h1>
		<p class="mt-2 text-base text-gray-700 dark:text-gray-300">
			Ranglisten der gespeicherten politischen Positionswerte nach Abgeordneten und Gruppen.
		</p>
	</div>

	<section
		class="mb-5 rounded-xl border border-gray-300 bg-surface-50 p-4 shadow-sm dark:border-surface-700 dark:bg-surface-700/60"
	>
		<p class="text-sm font-semibold text-gray-600 dark:text-gray-300">Position</p>
		<div class="mt-2 flex flex-wrap gap-1 rounded-xl border border-primary-300 p-1 dark:border-primary-400">
			{#each orientationOptions as option}
				<button
					type="button"
					title={option.description}
					class="rounded-lg px-3 py-1.5 text-sm font-semibold transition {selectedOrientation ===
					option.value
						? 'bg-primary-300 text-black dark:bg-primary-400'
						: 'hover:bg-primary-100 dark:hover:bg-surface-500'}"
					onclick={() => (selectedOrientation = option.value)}
				>
					{option.label}
				</button>
			{/each}
		</div>
	</section>

	<DelegateBarChartControl
		height={520}
		delegateMakeRequest={loadOrientation}
		reloadKey={selectedOrientation}
		bind:selectedCategory
		{categoryOptions}
		valueLabel={selectedOrientationOption.valueLabel}
		normalizedValueLabel={selectedOrientationOption.valueLabel}
		filterConfig={{
			showNormalized: false,
			showPeriod: true,
			showGender: true,
			showParty: true
		}}
		infoQuestion="Was zeigt diese Statistik?"
		infoAnswer="<p>Die Werte stammen aus der Tabelle <strong>political_positions</strong>. Die Legislaturperiode filtert Abgeordnete danach, ob sie in dieser Periode ein Nationalrats- oder Regierungsmandat hatten; die Positionswerte selbst sind nicht zeitlich versioniert.</p>"
	/>
</Container>
