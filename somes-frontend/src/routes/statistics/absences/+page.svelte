<script lang="ts">
	import { justPostStatistics } from '$lib/api/api';
	import DelegateBarChartControl from '$lib/components/Statistics/DelegateBarChartControl.svelte';
	import type { DelegateData } from '$lib/types';
	import { Select } from 'bits-ui';

	type AbsenceForDelegate = {
		delegate_name: string;
		delegate_party: string;
		total_absences: number;
		total_sessions: number;
		normalized_absences: number;
	};

	type AbsenceByCategory = {
		category: string;
		total_absences: number;
		total_sessions: number;
		normalized_absences: number;
	};

	// Category selection
	let selectedCategory: string = 'delegate';
	
	const categoryOptions = [
		{ value: 'delegate', label: 'Pro Abgeordneten' },
		{ value: 'party', label: 'Nach Parteien' },
		{ value: 'gender', label: 'Nach Gender' },
		{ value: 'age', label: 'Nach Alter' },
		{ value: 'legis', label: 'Nach Legislaturperiode' }
	];

	const delegateSimpleAbsences = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean,
		normalized: boolean
	): Promise<DelegateData[]> => {
		const response = await justPostStatistics<AbsenceForDelegate[]>('absences_per_delegate', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc,
			normalized: normalized
		});

		if ('error' in response) {
			return [];
		}

		return response.map((val) => {
			return {
				name: val.delegate_name,
				party: val.delegate_party,
				data: normalized ? val.normalized_absences : val.total_absences
			};
		});
	};

	const partySimpleAbsences = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean,
		normalized: boolean
	): Promise<DelegateData[]> => {
		const response = await justPostStatistics<AbsenceByCategory[]>('absences_per_party', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc,
			normalized: normalized
		});

		if ('error' in response) {
			return [];
		}

		return response.map((val) => {
			return {
				name: null,
				party: val.category,
				data: normalized ? val.normalized_absences : val.total_absences
			};
		});
	};

	const genderSimpleAbsences = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean,
		normalized: boolean
	): Promise<DelegateData[]> => {
		const response = await justPostStatistics<AbsenceByCategory[]>('absences_per_gender', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc,
			normalized: normalized
		});

		if ('error' in response) {
			return [];
		}

		return response.map((val) => {
			return {
				name: null,
				party: val.category,
				data: normalized ? val.normalized_absences : val.total_absences
			};
		});
	};

	const ageSimpleAbsences = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean,
		normalized: boolean
	): Promise<DelegateData[]> => {
		const response = await justPostStatistics<AbsenceByCategory[]>('absences_per_age', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc,
			normalized: normalized
		});

		if ('error' in response) {
			return [];
		}

		return response.map((val) => {
			return {
				name: null,
				party: val.category,
				data: normalized ? val.normalized_absences : val.total_absences
			};
		});
	};

	const legisSimpleAbsences = async (
		gp: string | null,
		gender: string | null,
		isDesc: boolean,
		normalized: boolean
	): Promise<DelegateData[]> => {
		const response = await justPostStatistics<AbsenceByCategory[]>('absences_per_legis', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc,
			normalized: normalized
		});

		if ('error' in response) {
			return [];
		}

		return response.map((val) => {
			return {
				name: null,
				party: val.category,
				data: normalized ? val.normalized_absences : val.total_absences
			};
		});
	};

	// Get the appropriate function based on selected category
	$: currentFunction = (() => {
		switch (selectedCategory) {
			case 'delegate': return delegateSimpleAbsences;
			case 'party': return partySimpleAbsences;
			case 'gender': return genderSimpleAbsences;
			case 'age': return ageSimpleAbsences;
			case 'legis': return legisSimpleAbsences;
			default: return delegateSimpleAbsences;
		}
	})();

	// Get the title based on selected category
	$: currentTitle = (() => {
		switch (selectedCategory) {
			case 'delegate': return 'Abwesenheiten pro Abgeordneten';
			case 'party': return 'Abwesenheiten nach Parteien';
			case 'gender': return 'Abwesenheiten nach Gender';
			case 'age': return 'Abwesenheiten nach Alter';
			case 'legis': return 'Abwesenheiten nach Legislaturperiode';
			default: return 'Abwesenheiten pro Abgeordneten';
		}
	})();
</script>

<svelte:head>
    <title>Abwesenheitsstatistiken</title>
    <meta name="description" content="Abwesenheitsstatistiken über den Nationalrat und deren Abgeordnete" />
</svelte:head>

<div class="container mx-auto px-4 py-8">
	<h1 class="text-3xl font-bold mb-8">Abwesenheitsstatistiken</h1>

	<!-- Category Selection -->
	<div class="bg-card rounded-xl p-6 shadow-sm mb-8">
		<div class="flex items-center gap-4">
			<label class="text-lg font-medium">Analyse-Kategorie:</label>
			<Select.Root
				type="single"
				bind:value={selectedCategory}
				items={categoryOptions}
			>
				<Select.Trigger class="w-64">
					<span>Kategorie wählen</span>
				</Select.Trigger>
				<Select.Portal>
					<Select.Content>
						<Select.Viewport>
							{#each categoryOptions as option}
								<Select.Item value={option.value} label={option.label}>
									{option.label}
								</Select.Item>
							{/each}
						</Select.Viewport>
					</Select.Content>
				</Select.Portal>
			</Select.Root>
		</div>
	</div>

	<!-- Dynamic Chart -->
	<div class="bg-card rounded-xl p-6 shadow-sm">
		<DelegateBarChartControl
			height={500}
			delegateMakeRequest={currentFunction}
			title={currentTitle}
		/>
	</div>
</div>
