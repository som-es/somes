<script lang="ts">
	import { filteredDelegates } from '$lib/caching/delegates.svelte';
	import Container from '$lib/components/Layout/Container.svelte';
	import PaginationVoteResults from '$lib/components/VoteResults/Expandable/PaginationVoteResults.svelte';
	import type { Delegate, Party, VoteResultsWithMaxPage } from '$lib/types';
	import { onMount } from 'svelte';
	import type { PageProps } from './$types';
	import { errorToNull } from '$lib/api/api';
	
	let { data }: PageProps = $props();

	let voteResults: VoteResultsWithMaxPage | null = $derived(errorToNull(data.voteResults))
	let partiesPerGp: Record<string, Party[]> | null = $derived(errorToNull(data.partiesPerGp))
	let selectedGp: string | null = $derived(data.selectedGp);
	let dels: Delegate[] | null = $state(null);
	onMount(async function () {
		dels = (await filteredDelegates())?.nr ?? null;
	});
</script>

<svelte:head>
    <title>Abstimmungen</title>
    <meta name="description" content="Filterbare Liste an Abstimmungen im Nationalrat" />
</svelte:head>

<!-- <div class="mx-auto px-5"> -->
<Container>
	<h1 class="text-3xl sm:text-4xl font-bold pt-2 px-1 sm:p-0 mt-2 sm:mt-0">Vergangene Abstimmungsergebnisse</h1>

	{#if dels && voteResults && partiesPerGp}
		<PaginationVoteResults {dels} {voteResults} {partiesPerGp} {selectedGp} />
	{/if}
</Container>
<!-- </div> -->