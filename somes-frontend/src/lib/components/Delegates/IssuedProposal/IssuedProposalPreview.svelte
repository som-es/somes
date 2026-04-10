<script lang="ts">
	import type { IssuedProposal } from '$lib/types';
	import ExtendInfoDialog from '../ExtendInfoDialog.svelte';
	import IssuedProposalBar from './IssuedProposalBar.svelte';
	import IssuedProposalModal from './IssuedProposalModal.svelte';

	interface Props {
		issuedProposals: IssuedProposal[];
		delegateId: number;
	}

	let { issuedProposals, delegateId }: Props = $props();

	let previewProposals = $derived(issuedProposals.slice(0, 2));
</script>

<div>
	<div class="flex items-start justify-between">
		<div>
			<h1 class="text-lg font-bold text-black xl:text-xl dark:text-white">
				Letzte eingebrachte Anträge
			</h1>
			<h2 class="text-sm text-primary-600 dark:text-primary-300">
				{issuedProposals.length}
				{issuedProposals.length == 1 ? 'Antrag' : 'Anträge'} insgesamt
			</h2>
		</div>
		<ExtendInfoDialog title="Alle anzeigen">
			<IssuedProposalModal {issuedProposals} />
		</ExtendInfoDialog>
	</div>
</div>
<div>
	{#each previewProposals as issuedProposal}
		<IssuedProposalBar {issuedProposal} />
	{/each}
</div>
