<script lang="ts">
	import type { IssuedProposal } from '$lib/types';
	import Pagination from '$lib/components/Pagination.svelte';
	import IssuedProposalBar from './IssuedProposalBar.svelte';
	import { Dialog } from 'bits-ui';
	import ModalCloseButton from '$lib/components/UI/ModalCloseButton.svelte';

	interface Props {
		issuedProposals: IssuedProposal[];
	}

	let { issuedProposals }: Props = $props();

	const ENTRIES = 14;

	let page = $state(1);

	let currentProposals: IssuedProposal[] = $derived(
		issuedProposals.slice((page - 1) * ENTRIES, page * ENTRIES)
	);
</script>

<div class="card px-4">
	<div class="flex items-center justify-between p-8">
		<h1 class="text-2xl font-bold">Alle eingebrachten Anträge</h1>
		<Dialog.Close>
			<ModalCloseButton />
		</Dialog.Close>
	</div>

	<div class="flex flex-col gap-2">
		{#each currentProposals as issuedProposal (issuedProposal.legis_init_id)}
			<IssuedProposalBar {issuedProposal} />
		{/each}
	</div>

	<div class="float-right">
		<Pagination bind:dynPage={page} maxPage={Math.ceil(issuedProposals.length / ENTRIES)} />
	</div>
</div>
