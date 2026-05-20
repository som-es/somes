<script lang="ts">
	import { justPostStatistics } from '$lib/api/api';
	import { mapOrientationCategory, mapOrientationDelegate } from '$lib/api/statistics-adapter';
	import Container from '$lib/components/Layout/Container.svelte';
	import DelegateBarChartControl from '$lib/components/Statistics/DelegateBarChartControl.svelte';
	import type { StatisticsData } from '$lib/types';

	type Orientation = 'left' | 'right' | 'liberal' | 'authoritarian';
	type ChartMode = 'bar' | 'donut' | 'line' | 'spectrum';

	let selectedCategory = $state('delegate');
	let selectedChartMode = $state<ChartMode>('bar');

	const categoryOptions = [
		{ value: 'delegate', label: 'Abgeordnete' },
		{ value: 'party', label: 'Parteien' },
		{ value: 'gender', label: 'Geschlecht' },
		{ value: 'age', label: 'Alter' }
	];

	const chartDescriptions = {
		delegate: 'Gespeicherter Positionswert der einzelnen Abgeordneten.',
		party: 'Durchschnittliche Positionswerte je Partei.',
		gender: 'Durchschnittliche Positionswerte nach Geschlecht.',
		age: 'Durchschnittliche Positionswerte nach Altersgruppen.',
		spectrum: 'Wirtschaftliche und gesellschaftliche Positionen in einer gemeinsamen Ansicht.',
		donut: 'Anteil der höchsten Positionswerte in der aktuellen Auswahl.'
	};

	const orientationOptions: {
		value: Orientation;
		label: string;
		valueLabel: string;
		description: string;
	}[] = [
		{
			value: 'left',
			label: 'Sozialistisch',
			valueLabel: 'Sozialismus-Score',
			description: 'Abgeordnete mit den höchsten gespeicherten sozialistischen Positionswerten.'
		},
		{
			value: 'right',
			label: 'Kapitalistisch',
			valueLabel: 'Kapitalismus-Score',
			description: 'Abgeordnete mit den höchsten gespeicherten kapitalistischen Positionswerten.'
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
		orientationOptions.find((option) => option.value === selectedOrientation) ??
			orientationOptions[0]
	);

	function orientationValue(item: any) {
		return Number(item.orientation_score ?? item.average_orientation ?? 0);
	}

	function spectrumKey(item: any) {
		if (selectedCategory === 'delegate') {
			return `${item.delegate_name}\u0000${item.delegate_party}`;
		}
		return item.category;
	}

	async function loadSpectrum(
		gp: string | null,
		gender: string | null,
		isDesc: boolean
	): Promise<StatisticsData[]> {
		const body = {
			legis_period: gp,
			party: null,
			gender,
			is_desc: true
		};
		const [socialist, capitalist, liberal, authoritarian] = await Promise.all([
			justPostStatistics<any[]>(`is_left_per_${selectedCategory}`, body),
			justPostStatistics<any[]>(`is_right_per_${selectedCategory}`, body),
			justPostStatistics<any[]>(`is_liberal_per_${selectedCategory}`, body),
			justPostStatistics<any[]>(`is_authoritarian_per_${selectedCategory}`, body)
		]);

		if (
			'error' in socialist ||
			'error' in capitalist ||
			'error' in liberal ||
			'error' in authoritarian
		) {
			return [];
		}

		const capitalistByKey = new Map(capitalist.map((item) => [spectrumKey(item), item]));
		const liberalByKey = new Map(liberal.map((item) => [spectrumKey(item), item]));
		const authoritarianByKey = new Map(authoritarian.map((item) => [spectrumKey(item), item]));

		const data = socialist.flatMap((socialistItem): StatisticsData[] => {
			const key = spectrumKey(socialistItem);
			const capitalistItem = capitalistByKey.get(key);
			const liberalItem = liberalByKey.get(key);
			const authoritarianItem = authoritarianByKey.get(key);

			if (!capitalistItem || !liberalItem || !authoritarianItem) {
				return [];
			}

			const leftRightScore = orientationValue(capitalistItem) - orientationValue(socialistItem);
			const liberalAuthoritarianScore =
				orientationValue(authoritarianItem) - orientationValue(liberalItem);
			const spectrumMagnitude = Math.hypot(leftRightScore, liberalAuthoritarianScore);
			const isDelegate = selectedCategory === 'delegate';

			return [
				{
					type: isDelegate ? 'delegate' : 'category',
					label: isDelegate ? socialistItem.delegate_name : socialistItem.category,
					value: spectrumMagnitude,
					party: isDelegate ? socialistItem.delegate_party : undefined,
					metadata: {
						left_right_score: leftRightScore,
						liberal_authoritarian_score: liberalAuthoritarianScore,
						spectrum_magnitude: spectrumMagnitude,
						total_votes: socialistItem.total_votes,
						delegate_count: socialistItem.delegate_count
					}
				}
			];
		});

		return data.sort((a, b) => (isDesc ? b.value - a.value : a.value - b.value));
	}

	const loadOrientation = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean,
		_normalized: boolean,
		chartMode?: ChartMode
	): Promise<StatisticsData[]> => {
		if (chartMode === 'spectrum') {
			return loadSpectrum(gp, gender, isDesc);
		}

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
	<meta name="description" content="Statistiken zu politischen Positionswerten von Abgeordneten" />
</svelte:head>

<Container class="pb-12">
	<div class="mt-2 mb-6">
		<h1 class="text-3xl font-bold sm:text-4xl">Politische Positionen</h1>
		<p class="mt-2 text-base text-gray-700 dark:text-gray-300">
			Politische Positionswerte nach Abgeordneten, Parteien und weiteren Gruppen.
		</p>
	</div>

	{#if selectedChartMode !== 'spectrum'}
		<section
			class="mb-5 rounded-xl border border-gray-300 bg-surface-50 p-4 shadow-sm dark:border-surface-700 dark:bg-surface-700/60"
		>
			<p class="text-sm font-semibold text-gray-600 dark:text-gray-300">Position</p>
			<div
				class="mt-2 flex flex-wrap gap-1 rounded-xl border border-primary-300 p-1 dark:border-primary-400"
			>
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
	{/if}

	<DelegateBarChartControl
		height={520}
		delegateMakeRequest={loadOrientation}
		reloadKey={selectedOrientation}
		bind:selectedCategory
		{categoryOptions}
		valueLabel={selectedOrientationOption.valueLabel}
		normalizedValueLabel={selectedOrientationOption.valueLabel}
		{chartDescriptions}
		filterConfig={{
			showNormalized: false,
			showPeriod: false,
			showGender: true,
			showParty: true
		}}
		showSpectrumMode={true}
		bind:selectedChartMode
		infoQuestion="Was zeigt diese Statistik?"
		infoAnswer="<p>Die Werte stammen aus der Tabelle <strong>political_positions</strong>. Diese Positionswerte sind aktuell nicht nach Legislaturperioden versioniert, daher wird hier keine Periodenfilterung angeboten.</p>"
	/>
</Container>
