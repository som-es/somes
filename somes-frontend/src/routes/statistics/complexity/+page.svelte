<script lang="ts">
	import { resolve } from '$app/paths';
	import Container from '$lib/components/Layout/Container.svelte';
</script>

<svelte:head>
	<title>Sprachkomplexität deaktiviert</title>
	<meta
		name="description"
		content="Sprachkomplexitätsstatistiken sind derzeit deaktiviert"
	/>
</svelte:head>

<Container class="pb-12">
	<div class="mt-2 max-w-3xl">
		<h1 class="text-3xl font-bold sm:text-4xl">Sprachkomplexität</h1>
		<p class="mt-3 text-base leading-7 text-gray-700 dark:text-gray-300">
			Diese Statistik ist vorübergehend deaktiviert, weil aktuell keine belastbaren
			Sprachkomplexitätsdaten verfügbar sind.
		</p>
		<a
			href={resolve('/statistics')}
			class="mt-6 inline-flex rounded-lg bg-primary-500 px-4 py-2 text-sm font-semibold text-white hover:bg-primary-600"
		>
			Zur Statistikübersicht
		</a>
	</div>
</Container>
