<script lang="ts">
	import type { Delegate, GovProposal } from '$lib/types';
	import ExtendInfoDialog from '../Delegates/ExtendInfoDialog.svelte';
	import AllProposalsModal from './AllProposalsModal.svelte';
	import GovProposalExpandableBar from './Latest/GovProposalExpandableBar.svelte';

	interface Props {
		govProposals: GovProposal[];
		delegate: Delegate;
	}

	let { govProposals, delegate }: Props = $props();

	let previewGovProposals = $derived(govProposals.slice(0, 2));
</script>

<div class="flex flex-wrap justify-between items-center">
	<div>
		<h1 class="font-bold text-lg sm:text-2xl">Ministerialentwürfe</h1>

		<h2 class="sm:text-lg">
			{govProposals.length}
			{govProposals.length == 1 ? 'Ministerialentwurf' : 'Ministerialentwürfe'} insgesamt
		</h2>
	</div>
	<ExtendInfoDialog title="Alle anzeigen">
		<AllProposalsModal govProposals={govProposals} delegate={delegate} />
	</ExtendInfoDialog>
</div>
{#each previewGovProposals as govProposal}
	<GovProposalExpandableBar
		govProposal={{ gov_proposal: govProposal, delegate }}
		coloring={'dark:bg-primary-300 bg-primary-400 text-black'}
	/>
{/each}
