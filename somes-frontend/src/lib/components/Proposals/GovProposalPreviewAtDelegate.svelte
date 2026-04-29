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

<div>
	<div class="flex items-start justify-between">
		<div>
			<h1 class="text-lg font-bold text-black xl:text-xl dark:text-white">Ministerialentwürfe</h1>
			<h2 class="text-sm text-gray-800 dark:text-gray-300">
				{govProposals.length}
				{govProposals.length == 1 ? 'Ministerialentwurf' : 'Ministerialentwürfe'} insgesamt
			</h2>
		</div>
		<ExtendInfoDialog title="Alle anzeigen">
			<AllProposalsModal {govProposals} {delegate} />
		</ExtendInfoDialog>
	</div>
</div>
{#each previewGovProposals as govProposal}
	<GovProposalExpandableBar
		govProposal={{ gov_proposal: govProposal, delegates: [delegate] }}
		coloring={'dark:bg-primary-300 bg-primary-400 text-black'}
	/>
{/each}
