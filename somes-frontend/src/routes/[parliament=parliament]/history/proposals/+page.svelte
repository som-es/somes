<script lang="ts">
	import { errorToNull } from '$lib/api/api';
	import Container from '$lib/components/Layout/Container.svelte';
	import PaginationMinistrialProposals from '$lib/components/Proposals/PaginationMinistrialProposals.svelte';
	import type { GovProposalsWithMaxPage } from '$lib/types';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();

	let govProposals: GovProposalsWithMaxPage | null = $derived(errorToNull(data.govProposals));
	let departmentsPerGp: Record<string, string[]> | null = $derived(
		errorToNull(data.departmentsPerGp)
	);
</script>

<svelte:head>
	<title>Ministerialentwürfe</title>
	<meta name="description" content="Filterbare Liste an Ministerialentwürfe" />
</svelte:head>

<!-- <div class="mx-auto px-5"> -->
<Container>
	<h1 class="mt-2 px-1 pt-2 text-3xl font-bold sm:mt-0 sm:p-0 sm:text-4xl">Ministerialentwürfe</h1>
	{#if govProposals && departmentsPerGp}
		<PaginationMinistrialProposals {govProposals} selectedGp={data.selectedGp} {departmentsPerGp} />
	{/if}
</Container>
<!-- </div> -->
