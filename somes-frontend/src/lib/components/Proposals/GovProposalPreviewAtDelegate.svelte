<script lang="ts">
	import type { Delegate, GovProposal } from '$lib/types';
	import { t } from '$lib/i18n/i18n.svelte';
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
			<h1 class="text-lg font-bold text-black xl:text-xl dark:text-white">{t('proposals.all.title')}</h1>
			<h2 class="text-sm text-gray-800 dark:text-gray-300">
				{t(govProposals.length == 1 ? 'proposals.preview.countOne' : 'proposals.preview.countOther', { count: govProposals.length })}
			</h2>
		</div>
		<ExtendInfoDialog title={t('ui.showAll')}>
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
