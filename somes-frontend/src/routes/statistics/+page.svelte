<script lang="ts">
	import { onMount } from 'svelte';
	import DelegateBarChartControl from '$lib/components/Statistics/DelegateBarChartControl.svelte';
	import type { StatisticsData } from '$lib/types';
	import { Select } from 'bits-ui';

	// Independent category selection for each statistics type
	let selectedCallsCategory: string = 'delegate';
	let selectedAbsencesCategory: string = 'delegate';
	let selectedActivityCategory: string = 'delegate';
	let selectedAgeCategory: string = 'delegate';
	let selectedComplexityCategory: string = 'delegate';
	let selectedSpeechTimeCategory: string = 'delegate';
	let selectedTotalSpeechesCategory: string = 'delegate';

	const categoryOptions = [
		{ value: 'delegate', label: 'Pro Abgeordneten', icon: '👤' },
		{ value: 'party', label: 'Nach Parteien', icon: '🏛️' },
		{ value: 'gender', label: 'Nach Gender', icon: '👥' },
		{ value: 'age', label: 'Nach Alter', icon: '📅' },
		{ value: 'legis', label: 'Nach Legislaturperiode', icon: '📋' }
	];

	// Import all statistics pages
	import CallToOrdersPage from './call_to_orders/+page.svelte';
	import AbsencesPage from './absences/+page.svelte';
	import ActivityPage from './activity/+page.svelte';
	import AgePage from './age/+page.svelte';
	import SpeechComplexityPage from './complexity/+page.svelte';
	import SpeechTimePage from './speech_time/+page.svelte';
	import TotalSpeechesPage from './total_speeches/+page.svelte';

	// Statistics sections data
	const statisticsSections = [
		{
			id: 'call_to_orders',
			title: 'Ordnungsrufstatistiken',
			description: 'Analyse der Ordnungsrufe im Parlament',
			icon: '🔔',
			component: CallToOrdersPage,
			selectedCategory: selectedCallsCategory,
			onCategoryChange: (value: string) => selectedCallsCategory = value
		},
		{
			id: 'absences',
			title: 'Abwesenheitsstatistiken',
			description: 'Analyse der Abwesenheiten im Parlament',
			icon: '📋',
			component: AbsencesPage,
			selectedCategory: selectedAbsencesCategory,
			onCategoryChange: (value: string) => selectedAbsencesCategory = value
		},
		{
			id: 'activity',
			title: 'Aktivitätsstatistiken',
			description: 'Analyse der parlamentarischen Aktivitäten',
			icon: '📊',
			component: ActivityPage,
			selectedCategory: selectedActivityCategory,
			onCategoryChange: (value: string) => selectedActivityCategory = value
		},
		{
			id: 'age',
			title: 'Altersstatistiken',
			description: 'Altersstruktur der Abgeordneten',
			icon: '👥',
			component: AgePage,
			selectedCategory: selectedAgeCategory,
			onCategoryChange: (value: string) => selectedAgeCategory = value
		},
		{
			id: 'speech_complexity',
			title: 'Komplexitätsstatistiken',
			description: 'Analyse der Komplexität von parlamentarischen Reden',
			icon: '🧠',
			component: SpeechComplexityPage,
			selectedCategory: selectedComplexityCategory,
			onCategoryChange: (value: string) => selectedComplexityCategory = value
		},
		{
			id: 'speech_time',
			title: 'Redezeitstatistiken',
			description: 'Analyse der Redezeiten im Parlament',
			icon: '🎤',
			component: SpeechTimePage,
			selectedCategory: selectedSpeechTimeCategory,
			onCategoryChange: (value: string) => selectedSpeechTimeCategory = value
		},
		{
			id: 'total_speeches',
			title: 'Anzahl der Reden',
			description: 'Analyse der Anzahl der parlamentarischen Reden',
			icon: '🎙',
			component: TotalSpeechesPage,
			selectedCategory: selectedTotalSpeechesCategory,
			onCategoryChange: (value: string) => selectedTotalSpeechesCategory = value
		}
	];

	onMount(() => {
		// Main statistics page loaded
	});
</script>

<svelte:head>
	<title>Statistiken - Parlamentsinformationssystem</title>
	<meta name="description" content="Übersicht aller parlamentarischen Statistiken" />
</svelte:head>

<div class="min-h-screen bg-gradient-to-br from-slate-50 to-blue-50 dark:from-slate-900 dark:to-slate-800">
	<!-- Header -->
	<div class="bg-white dark:bg-slate-900 border-b border-slate-200 dark:border-slate-700 shadow-sm">
		<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6">
			<div class="flex items-center justify-between">
				<div>
					<h1 class="text-3xl font-bold text-slate-900 dark:text-slate-100">Statistiken</h1>
					<p class="mt-2 text-lg text-slate-600 dark:text-slate-400">
						Umfassende Analyse der parlamentarischen Daten und Aktivitäten
					</p>
				</div>
				<div class="flex items-center gap-3">
					<div class="w-12 h-12 bg-blue-500 rounded-xl flex items-center justify-center text-white text-xl shadow-lg">
						📈
					</div>
				</div>
			</div>
		</div>
	</div>

	<!-- Main Content -->
	<div class="max-w-[100rem] mx-auto px-4 sm:px-6 lg:px-8 py-8">
		<!-- Statistics Sections -->
		{#each statisticsSections as section}
			<div class="bg-gradient-to-br from-blue-50 to-indigo-100 dark:from-blue-900/20 dark:to-indigo-900/20 rounded-2xl p-8 shadow-lg border border-blue-200/50 dark:border-blue-800/50 mb-8">
				<div class="flex flex-col lg:flex-row items-start lg:items-center justify-between gap-4 mb-6">
					<div class="flex items-center gap-3">
						<div class="w-12 h-12 bg-blue-500 rounded-xl flex items-center justify-center text-white text-xl shadow-lg">
							{section.icon}
						</div>
						<div>
							<h2 class="text-2xl font-bold text-slate-800 dark:text-slate-200">{section.title}</h2>
							<p class="text-sm text-slate-600 dark:text-slate-400">{section.description}</p>
						</div>
					</div>
					<!-- Category Selection -->
					<div class="flex flex-col items-end gap-2">
						<div class="flex items-center gap-2">
							<div class="w-1 h-4 bg-blue-500 rounded-full"></div>
							<h3 class="text-sm font-semibold text-slate-700 dark:text-slate-300">Kategorie auswählen</h3>
						</div>
						<div class="bg-white dark:bg-slate-800 rounded-xl p-1 shadow-md w-full lg:w-72">
							<Select.Root
								type="single"
								bind:value={section.selectedCategory}
								items={categoryOptions}
								onValueChange={section.onCategoryChange}
							>
								<Select.Trigger class="w-full h-10 bg-white dark:bg-slate-800 border-0 rounded-lg px-3 hover:bg-slate-50 dark:hover:bg-slate-700 transition-colors">
									<span class="flex items-center gap-2 text-slate-700 dark:text-slate-300">
										<span class="text-base">{categoryOptions.find(opt => opt.value === section.selectedCategory)?.icon || '📊'}</span>
										<span class="text-sm">{categoryOptions.find(opt => opt.value === section.selectedCategory)?.label || 'Kategorie auswählen'}</span>
									</span>
								</Select.Trigger>
								<Select.Portal>
									<Select.Content class="bg-white dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-xl shadow-xl">
										<Select.Viewport>
											{#each categoryOptions as option}
												<Select.Item value={option.value} label={option.label} class="px-4 py-3 hover:bg-slate-100 dark:hover:bg-slate-700 cursor-pointer flex items-center gap-3">
													<span class="text-lg">{option.icon}</span>
													<div>
														<div class="font-medium">{option.label}</div>
													</div>
												</Select.Item>
											{/each}
										</Select.Viewport>
									</Select.Content>
								</Select.Portal>
							</Select.Root>
						</div>
					</div>
				</div>

				<!-- Statistics Component -->
				<div class="bg-white dark:bg-slate-800 rounded-xl shadow-inner">
					<svelte:component this={section.component} selectedCategory={section.selectedCategory} />
				</div>
			</div>
		{/each}
	</div>
</div>
