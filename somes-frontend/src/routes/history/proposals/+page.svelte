<script lang="ts">
	import { errorToNull } from '$lib/api/api';
	import Container from '$lib/components/Layout/Container.svelte';
	import PaginationMinistrialProposals from '$lib/components/Proposals/PaginationMinistrialProposals.svelte';
	import type { GovProposalsWithMaxPage } from '$lib/types';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();

	let govProposals: GovProposalsWithMaxPage | null = $derived(errorToNull(data.govProposals))
</script>

<svelte:head>
    <title>Ministerialentwürfe</title>
    <meta name="description" content="Filterbare Liste an Ministerialentwürfe" />
</svelte:head>

<!-- <div class="mx-auto px-5"> -->
<Container>
	<h1 class="text-2xl sm:text-4xl font-bold">Ministerialentwürfe</h1>
	{#if govProposals}
		<PaginationMinistrialProposals {govProposals} selectedGp={data.selectedGp} />
	{/if}
</Container>
<!-- </div> -->
