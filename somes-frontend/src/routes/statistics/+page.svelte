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

<div class="text-base-font-color font-base dark:bg-surface-950 min-h-screen">
	<!-- Header -->
	<header class="sticky top-0 z-10 border-b border-surface-200 bg-surface-50/90 shadow-sm backdrop-blur-md dark:border-surface-700 dark:bg-surface-900/90">
		<div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8 py-6">
			<div class="flex items-center justify-between">
				<div>
					<h1 class="text-3xl font-bold text-surface-900 dark:text-surface-50">Statistiken</h1>
					<p class="mt-2 text-lg text-surface-600 dark:text-surface-400">
						Umfassende Analyse der parlamentarischen Daten und Aktivitäten
					</p>
				</div>
				<div class="flex items-center gap-3">
					<div class="w-12 h-12 bg-primary-500 rounded-xl flex items-center justify-center text-white text-xl shadow-lg">
						📈
					</div>
				</div>
			</div>
		</div>
	</header>

	<!-- Main Content -->
	<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
		<!-- Statistics Sections -->
		{#each statisticsSections as section}
			<div class="bg-gradient-to-br from-primary-50 to-secondary-50 dark:from-surface-800 dark:to-surface-900 rounded-2xl p-8 shadow-lg border border-surface-200 dark:border-surface-700 mb-8">
				<div class="flex flex-col lg:flex-row items-start lg:items-center justify-between gap-4 mb-6">
					<div class="flex items-center gap-3">
						<div class="w-12 h-12 bg-primary-500 rounded-xl flex items-center justify-center text-white text-xl shadow-lg">
							{section.icon}
						</div>
						<div>
							<h2 class="text-2xl font-bold text-surface-900 dark:text-surface-50">{section.title}</h2>
							<p class="text-sm text-surface-600 dark:text-surface-400">{section.description}</p>
						</div>
					</div>
					<!-- Category Selection -->
					<div class="flex flex-col items-end gap-2">
						<div class="flex items-center gap-2">
							<div class="w-1 h-4 bg-primary-500 rounded-full"></div>
							<h3 class="text-sm font-semibold text-surface-700 dark:text-surface-300">Kategorie auswählen</h3>
						</div>
						<div class="bg-white dark:bg-surface-800 rounded-xl p-1 shadow-md w-full lg:w-72">
							<Select.Root
								type="single"
								bind:value={section.selectedCategory}
								items={categoryOptions}
								onValueChange={section.onCategoryChange}
							>
								<Select.Trigger class="w-full h-10 bg-white dark:bg-surface-800 border-0 rounded-lg px-3 hover:bg-surface-50 dark:hover:bg-surface-700 transition-colors">
									<span class="flex items-center gap-2 text-surface-700 dark:text-surface-300">
										<span class="text-base">{categoryOptions.find(opt => opt.value === section.selectedCategory)?.icon || '📊'}</span>
										<span class="text-sm">{categoryOptions.find(opt => opt.value === section.selectedCategory)?.label || 'Kategorie auswählen'}</span>
									</span>
								</Select.Trigger>
								<Select.Portal>
									<Select.Content class="bg-white dark:bg-surface-800 border border-surface-200 dark:border-surface-700 rounded-xl shadow-xl">
										<Select.Viewport>
											{#each categoryOptions as option}
												<Select.Item value={option.value} label={option.label} class="px-4 py-3 hover:bg-surface-100 dark:hover:bg-surface-700 cursor-pointer flex items-center gap-3">
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
				<div class="bg-white dark:bg-surface-800 rounded-xl shadow-inner">
					<svelte:component this={section.component} selectedCategory={section.selectedCategory} />
				</div>
			</div>
		{/each}
	</div>
</div>
