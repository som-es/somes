<script lang="ts">
	import { justPostStatistics } from '$lib/api/api';
	import { t } from '$lib/i18n/i18n.svelte';
	import { mapOrientationCategory, mapOrientationDelegate } from '$lib/api/statistics-adapter';
	import Container from '$lib/components/Layout/Container.svelte';
	import StatisticsChartControl from '$lib/components/Statistics/StatisticsChartControl.svelte';
	import type { StatisticsData } from '$lib/types';

	type Orientation = 'left' | 'right' | 'liberal' | 'authoritarian';
	type ChartMode = 'bar' | 'donut' | 'line' | 'spectrum';

	let selectedCategory = $state('delegate');
	let selectedChartMode = $state<ChartMode>('bar');

	const categoryOptions = [
		{ value: 'delegate', label: t('statistics.orientation.category.delegate') },
		{ value: 'party', label: t('statistics.orientation.category.party') },
		{ value: 'gender', label: t('statistics.orientation.category.gender') },
		{ value: 'age', label: t('statistics.orientation.category.age') }
	];

	const chartDescriptions = {
		delegate: t('statistics.orientation.desc.delegate'),
		party: t('statistics.orientation.desc.party'),
		gender: t('statistics.orientation.desc.gender'),
		age: t('statistics.orientation.desc.age'),
		spectrum: t('statistics.orientation.desc.spectrum'),
		donut: t('statistics.orientation.desc.donut')
	};

	const orientationOptions: {
		value: Orientation;
		label: string;
		valueLabel: string;
		description: string;
	}[] = [
		{
			value: 'left',
			label: t('statistics.orientation.axis.left'),
			valueLabel: t('statistics.orientation.axis.leftValue'),
			description: t('statistics.orientation.axis.leftDesc')
		},
		{
			value: 'right',
			label: t('statistics.orientation.axis.right'),
			valueLabel: t('statistics.orientation.axis.rightValue'),
			description: t('statistics.orientation.axis.rightDesc')
		},
		{
			value: 'liberal',
			label: t('statistics.orientation.axis.liberal'),
			valueLabel: t('statistics.orientation.axis.liberalValue'),
			description: t('statistics.orientation.axis.liberalDesc')
		},
		{
			value: 'authoritarian',
			label: t('statistics.orientation.axis.authoritarian'),
			valueLabel: t('statistics.orientation.axis.authoritarianValue'),
			description: t('statistics.orientation.axis.authoritarianDesc')
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
	<title>{t('statistics.orientation.title')}</title>
	<meta name="description" content={t('statistics.orientation.description')} />
</svelte:head>

<Container class="pb-12">
	<div class="mt-2 mb-6">
		<h1 class="text-3xl font-bold sm:text-4xl">{t('statistics.orientation.h1')}</h1>
		<p class="mt-2 text-base text-gray-700 dark:text-gray-300">
			{t('statistics.orientation.intro')}
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

	<StatisticsChartControl
		height={520}
		makeRequest={loadOrientation}
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
		extraReservedHeight={selectedChartMode === 'spectrum' ? 0 : 112}
		infoQuestion={t('statistics.orientation.infoQuestion')}
		infoAnswer={t('statistics.orientation.infoAnswer')}
	/>
</Container>
