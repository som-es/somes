<script lang="ts">
		import Pagination from '$lib/components/Pagination.svelte';
	import ExpandablePlaceholder from '$lib/components/VoteResults/Expandable/Placeholders/ExpandablePlaceholder.svelte';
	import AbsenceBar from './AbsenceBar.svelte';
	import type { Absence } from '$lib/types';

	const modalStore = getModalStore();
	export let parent;

	const ENTRIES = 15;

	let currentPageAbsences: Absence[] = [];
	let page = 1;

	$: if ($modalStore.length > 0 && $modalStore[0].meta) {
		currentPageAbsences = $modalStore[0].meta.absences.slice((page - 1) * ENTRIES, page * ENTRIES);
	}

	// $: speeches =
</script>

{#if $modalStore.length > 0 && $modalStore[0].meta}
	<div class="card p-8 max-w-7xl w-7xl">
		<h1 class="font-bold text-2xl">Letzte Abwesenheiten</h1>
		<button
			on:click={() => {
				modalStore.close();
			}}
			style="font-size: 34px"
			class="w-5 unselectable float-right"
		>
			✕
		</button>

		<Pagination bind:page maxPage={Math.ceil($modalStore[0].meta.absences.length / ENTRIES)} />
		<!-- <AllProposalsFiltering bind:filteredGovProposals={filteredGovProposals} allGovProposals={$modalStore[0].meta.govProposals} /> -->
		{#each currentPageAbsences as absence}
			<AbsenceBar {absence} {page} />
			<!-- <GovProposalExpandableBar {govProposal} /> -->
		{/each}
		{#if currentPageAbsences.length == 0}
			{#each { length: 15 } as _}
				<ExpandablePlaceholder class="min-w-7xl w-7xl" />
			{/each}
		{/if}
		<div class="float-right">
			<Pagination bind:page maxPage={Math.ceil($modalStore[0].meta.absences.length / ENTRIES)} />
		</div>
	</div>
{/if}
