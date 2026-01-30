<script lang="ts">
	import Container from '$lib/components/Layout/Container.svelte';
	import PaginationVoteResults from '$lib/components/VoteResults/Expandable/PaginationVoteResults.svelte';
	import type { Party, VoteResultsWithMaxPage } from '$lib/types';
	import type { PageProps } from './$types';
	import { errorToNull } from '$lib/api/api';

	let { data }: PageProps = $props();

	let voteResults: VoteResultsWithMaxPage | null = $derived(errorToNull(data.voteResults))
	let partiesPerGp: Record<string, Party[]> | null = $derived(errorToNull(data.partiesPerGp))
	let selectedGp: string | null = $derived(data.selectedGp);
</script>

<svelte:head>
    <title>Eingebrachte Anträge</title>
    <meta name="description" content="Filterbare Liste an Anträgen, die eingebracht wurden und bald zur Abstimmung stehen" />
</svelte:head>

<!-- <div class="mx-auto px-5"> -->
<Container>
	<h1 class="text-2xl sm:text-4xl font-bold">Anträge zur Abstimmung</h1>
	{#if voteResults && partiesPerGp}
		<PaginationVoteResults 
			{voteResults} 
			{partiesPerGp} 
			{selectedGp} 
			storeIdx={1}
		/>
	{/if}
</Container>
<!-- </div> -->
