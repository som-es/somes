<script lang="ts">
	import { errorToNull } from '$lib/api/api';
	import PaginationDecrees from '$lib/components/Decrees/PaginationDecrees.svelte';
	import type { DecreesWithMaxPage } from '$lib/components/Delegates/Decrees/types';
	import Container from '$lib/components/Layout/Container.svelte';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();

	let decrees: DecreesWithMaxPage | null = $derived(errorToNull(data.decrees))
	let departmentsPerGp: Record<string, string[]> | null = $derived(errorToNull(data.departmentsPerGp))
</script>

<svelte:head>
    <title>Verordnungen</title>
    <meta name="description" content="Filterbare Liste an Verordnungen" />
</svelte:head>

<!-- <div class="mx-auto px-5"> -->
<Container>
	<h1 class="text-2xl sm:text-4xl font-bold">Verordnungen</h1>
	{#if decrees && departmentsPerGp}
		<PaginationDecrees {decrees} selectedGp={data.selectedGp} {departmentsPerGp} />
	{/if}
</Container>
<!-- </div> -->
