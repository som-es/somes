<script lang="ts">
	import Container from '$lib/components/Layout/Container.svelte';
	import PaginationVoteResults from '$lib/components/VoteResults/Expandable/PaginationVoteResults.svelte';
	import type { Party, PartyStates, VoteResultsWithMaxPage } from '$lib/types';
	import type { PageProps } from './$types';
	import { errorToNull } from '$lib/api/api';

	let { data }: PageProps = $props();

	let voteResults: VoteResultsWithMaxPage | null = $derived(errorToNull(data.voteResults));
	let partiesPerGp: Record<string, Party[]> | null = $derived(errorToNull(data.partiesPerGp));
	let coalitionPartiesPerGp: Record<string, PartyStates> | null = $derived(
		errorToNull(data.coalitionPartiesPerGp)
	);
	let selectedGp: string | null = $derived(data.selectedGp);
</script>

<svelte:head>
	<title>Abstimmungen</title>
	<meta name="description" content="Filterbare Liste an Abstimmungen im Nationalrat" />
</svelte:head>

<!-- <div class="mx-auto px-5"> -->
<Container>
	<h1 class="mt-2 px-1 pt-2 text-3xl font-bold sm:mt-0 sm:p-0 sm:text-4xl">
		Vergangene Abstimmungsergebnisse
	</h1>

	{#if partiesPerGp && coalitionPartiesPerGp}
		<PaginationVoteResults
			{voteResults}
			{partiesPerGp}
			{selectedGp}
			{coalitionPartiesPerGp}
			showAcceptedFilter
			showNamedVoteFilter
			showPartyFilter
			showReqMajorityFilter
			showIsUrgentFilter
		/>
	{/if}
</Container>
<!-- </div> -->
