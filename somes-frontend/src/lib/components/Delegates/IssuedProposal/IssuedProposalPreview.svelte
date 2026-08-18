<script lang="ts">
	import { t } from '$lib/i18n/i18n.svelte';
	import type { IssuedProposal } from '$lib/types';
	import ExtendInfoDialog from '../ExtendInfoDialog.svelte';
	import IssuedProposalBar from './IssuedProposalBar.svelte';
	import IssuedProposalModal from './IssuedProposalModal.svelte';

	interface Props {
		issuedProposals: IssuedProposal[];
	}

	let { issuedProposals }: Props = $props();

	let previewProposals = $derived(issuedProposals.slice(0, 2));
</script>

<div>
	<div class="flex items-start justify-between">
		<div>
			<h1 class="text-lg font-bold text-black xl:text-xl dark:text-white">
				{t('proposals.title')}
			</h1>
			<h2 class="text-sm text-gray-800 dark:text-gray-300">
				{issuedProposals.length}
				{issuedProposals.length == 1 ? t('proposals.proposal') : t('proposals.proposals')} {t('proposals.total')}
			</h2>
		</div>
		<ExtendInfoDialog title={t('proposals.showAll')}>
			<IssuedProposalModal {issuedProposals} />
		</ExtendInfoDialog>
	</div>
</div>
<div class="mt-5 flex flex-col gap-2">
	{#each previewProposals as issuedProposal}
		<IssuedProposalBar {issuedProposal} />
	{/each}
</div>
