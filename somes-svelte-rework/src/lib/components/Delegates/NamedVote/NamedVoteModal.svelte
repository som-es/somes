<script lang="ts">
	import { getModalStore, type PopupSettings } from '@skeletonlabs/skeleton';
	import type { GovProposal, NamedVote, Speech } from '$lib/types';
	import Pagination from '$lib/components/Pagination.svelte';
	import NamedVoteBar from './NamedVoteBar.svelte';
	import ExpandablePlaceholder from '$lib/components/VoteResults/Expandable/Placeholders/ExpandablePlaceholder.svelte';

	const modalStore = getModalStore();
	export let parent;

	const ENTRIES = 14;

	let currentPageSpeeches: NamedVote[] = [];

	$: if ($modalStore.length > 0 && $modalStore[0].meta) {
		currentPageSpeeches = $modalStore[0].meta.namedVotes;
	}

	let page = 1;

	$: if (page && $modalStore.length > 0 && $modalStore[0].meta) {
		currentPageSpeeches = $modalStore[0].meta.namedVotes.slice(
			(page - 1) * ENTRIES,
			page * ENTRIES
		);
	}

	// $: speeches =
</script>

{#if $modalStore.length > 0 && $modalStore[0].meta}
	<div class="card p-8 max-w-7xl w-[80rem]">
		<h1 class="font-bold text-2xl">Letzte namentliche Abstimmungen</h1>
		<button
			on:click={() => {
				modalStore.close();
			}}
			style="font-size: 34px"
			class="w-5 unselectable float-right"
		>
			&#x2715
		</button>
		<Pagination bind:page maxPage={Math.ceil($modalStore[0].meta.namedVotes.length / ENTRIES)} />
		<!-- <AllProposalsFiltering bind:filteredGovProposals={filteredGovProposals} allGovProposals={$modalStore[0].meta.govProposals} /> -->
		{#each currentPageSpeeches as namedVote}
			<NamedVoteBar {namedVote} />
			<!-- <GovProposalExpandableBar {govProposal} /> -->
		{/each}
		{#if currentPageSpeeches.length == 0}
			{#each { length: 15 } as _}
				<ExpandablePlaceholder class="min-w-7xl w-7xl" />
			{/each}
		{/if}
		<div class="float-right">
			<Pagination bind:page maxPage={Math.ceil($modalStore[0].meta.namedVotes.length / ENTRIES)} />
		</div>
	</div>
{/if}
