<script lang="ts">
	import { page } from '$app/stores';
	import { errorToNull } from '$lib/api/api';
	import Container from '$lib/components/Layout/Container.svelte';
	import { gov_proposal_by_path } from '$lib/components/Proposals/api';
	import GovProposalView from '$lib/components/Proposals/GovProposalView.svelte';
	import SButton from '$lib/components/UI/SButton.svelte';
	import ExpandablePlaceholder from '$lib/components/VoteResults/Expandable/Placeholders/ExpandablePlaceholder.svelte';
	import { currentGovProposalDelegateStore, hasGoBackStore } from '$lib/stores/stores';
	import type { GovProposalDelegate } from '$lib/types';
	import { onMount } from 'svelte';
	import { get } from 'svelte/store';

	let govProposalDelegate: GovProposalDelegate | null = null;
	let currentlyUpdating = false;

	$: gp = $page.params.gp;
	$: inr = $page.params.inr;

	const loadGovProposal = async () => {
		currentlyUpdating = true;
		govProposalDelegate = errorToNull(await gov_proposal_by_path(gp, inr));
		currentGovProposalDelegateStore.set(govProposalDelegate);
		currentlyUpdating = false;
	};

	const runGovProposalUpdate = () => {
		govProposalDelegate = get(currentGovProposalDelegateStore);
		const storedGp = govProposalDelegate?.gov_proposal.ministrial_proposal?.gp;
		const storedInr = govProposalDelegate?.gov_proposal.ministrial_proposal?.inr?.toString();
		if (gp != storedGp || inr != storedInr) {
			loadGovProposal();
		}
	};

	onMount(runGovProposalUpdate);

	const goBack = () => {
		history.back();
	};
</script>

<Container>
	{#if currentlyUpdating}
		<!-- <CenterPrograssRadial /> -->
	{:else}
		{#if get(hasGoBackStore)}
			<SButton class="bg-primary-500" on:click={goBack}>Zurück</SButton>
		{/if}

		{#if govProposalDelegate}
			<GovProposalView govProposal={govProposalDelegate} />
		{:else}
			{#each { length: 10 } as _}
				<ExpandablePlaceholder />
			{/each}
		{/if}
	{/if}
</Container>
