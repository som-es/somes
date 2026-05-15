<script lang="ts">
	import { resolve } from '$app/paths';
	import Container from '$lib/components/Layout/Container.svelte';

	const statisticsSections = [
		{
			title: 'Redezeit',
			description: 'Gesamte und durchschnittliche Redezeiten nach Abgeordneten und Gruppen.',
			href: resolve('/statistics/speech_time')
		},
		{
			title: 'Gehaltene Reden',
			description: 'Anzahl der parlamentarischen Reden in den ausgewählten Perioden.',
			href: resolve('/statistics/total_speeches')
		},
		{
			title: 'Sprachkomplexität',
			description: 'Vergleich der berechneten Komplexitätswerte parlamentarischer Beiträge.',
			href: resolve('/statistics/complexity')
		},
		{
			title: 'Abwesenheiten',
			description: 'Absolute und normalisierte Abwesenheiten in Nationalratssitzungen.',
			href: resolve('/statistics/absences')
		},
		{
			title: 'Aktivität',
			description: 'Gewichtete parlamentarische Aktivität anhand eingebrachter Initiativen.',
			href: resolve('/statistics/activity')
		},
		{
			title: 'Ordnungsrufe',
			description: 'Ordnungsrufe nach Abgeordneten, Parteien und Legislaturperioden.',
			href: resolve('/statistics/call_to_orders')
		},
		{
			title: 'Alter',
			description: 'Altersstruktur und Durchschnittsalter der Abgeordneten.',
			href: resolve('/statistics/age')
		}
	];
</script>

<svelte:head>
	<title>Statistiken - Parlamentsinformationssystem</title>
	<meta name="description" content="Übersicht parlamentarischer Statistiken" />
</svelte:head>

<Container class="pb-12">
	<div class="mt-2 max-w-4xl">
		<h1 class="text-3xl font-bold sm:text-4xl">Statistiken</h1>
		<p class="mt-3 text-base text-gray-700 dark:text-gray-300">
			Wählen Sie eine Statistik aus der Seitenleiste oder aus der Übersicht. Die Detailseiten bieten
			Suche, Filter, Kategorien und mehrere Darstellungen.
		</p>
	</div>

	<div class="mt-8 grid gap-4 md:grid-cols-2 xl:grid-cols-3">
		{#each statisticsSections as section}
			<a
				href={section.href}
				class="rounded-xl border border-gray-300 bg-surface-50 p-5 shadow-sm transition hover:border-primary-400 hover:bg-primary-50 dark:border-surface-700 dark:bg-surface-700/60 dark:hover:bg-surface-700"
			>
				<h2 class="text-xl font-bold">{section.title}</h2>
				<p class="mt-2 text-sm leading-6 text-gray-700 dark:text-gray-300">{section.description}</p>
			</a>
		{/each}
	</div>
</Container>
