<script lang="ts">
	import AllProposalsFiltering from './AllProposalsFiltering.svelte';
	import type { Delegate, GovProposal } from '$lib/types';
	import GovProposalExpandableBar from './Latest/GovProposalExpandableBar.svelte';
	import { Dialog } from 'bits-ui';
	import ModalCloseButton from '../UI/ModalCloseButton.svelte';
	import Pagination from '../Pagination.svelte';
	import { untrack } from 'svelte';

	interface Props {
		govProposals: GovProposal[];
		delegate: Delegate;
	}

	let { govProposals, delegate }: Props = $props();

	let page = $state(1);

	let filteredGovProposals: GovProposal[] = $state([]);

	const ENTRIES_PER_PAGE = 10;

	let paginatedGovProposals: GovProposal[] = $derived(
		filteredGovProposals.slice((page - 1) * ENTRIES_PER_PAGE, page * ENTRIES_PER_PAGE)
	);

	$effect(() => {
		void filteredGovProposals;
		untrack(() => {
			page = 1;
		});
	});
</script>

<div class="max-w-7xl card p-8">
	<div class="flex items-center justify-between">
		<h1 class="text-2xl font-bold">Ministerialentwürfe</h1>
		<Dialog.Close>
			<ModalCloseButton />
		</Dialog.Close>
	</div>

	<AllProposalsFiltering bind:filteredGovProposals allGovProposals={govProposals} />
	{#each paginatedGovProposals as govProposal}
		<GovProposalExpandableBar
			govProposal={{ gov_proposal: govProposal, delegate }}
			coloring={'dark:bg-primary-300 bg-primary-400 text-black'}
		/>
	{/each}
	<div class="float-right">
		<Pagination
			bind:dynPage={page}
			maxPage={Math.ceil(filteredGovProposals.length / ENTRIES_PER_PAGE)}
		/>
	</div>
</div>
