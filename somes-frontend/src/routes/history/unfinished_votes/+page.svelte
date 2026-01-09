<script lang="ts">
	import { filteredDelegates } from '$lib/caching/delegates.svelte';
	import Container from '$lib/components/Layout/Container.svelte';
	import PaginationVoteResults from '$lib/components/VoteResults/Expandable/PaginationVoteResults.svelte';
	import type { Delegate } from '$lib/types';
	import { onMount } from 'svelte';

	let dels: Delegate[] | null = null;
	onMount(async function () {
		dels = (await filteredDelegates())?.nr ?? null;
	});
</script>

<svelte:head>
    <title>Eingebrachte Anträge</title>
    <meta name="description" content="Filterbare Liste an Anträgen, die eingebracht wurden und bald zur Abstimmung stehen" />
</svelte:head>

<!-- <div class="mx-auto px-5"> -->
<Container>
	<h1 class="text-2xl sm:text-4xl font-bold">Anträge zur Abstimmung</h1>

	{#if dels}
		<PaginationVoteResults
			{dels}
			storeIdx={1}
			isFinished={false}
			showAcceptedFilter={false}
			showVoteTypeFilter={false}
		/>
	{/if}
</Container>
<!-- </div> -->
