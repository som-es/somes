<script lang="ts">
	import Topics from '$lib/components/Topics/Topics.svelte';
	import SButton from '$lib/components/UI/SButton.svelte';
	import type { Delegate, GovProposal, VoteResult } from '$lib/types';
	import { currentDelegatesAtDateStore, currentVoteResultStore } from '$lib/stores/stores';
	import { gotoHistory } from '$lib/goto';
	import VoteParliament2 from '$lib/components/Parliaments/VoteParliament2.svelte';
	import Emphasis from '$lib/components/VoteResults/Emphasis/Emphasis.svelte';
	import GovProposalInfoTiles from '$lib/components/VoteResults/InfoTiles/GovProposalInfoTiles.svelte';
	import DelegateCard from '$lib/components/Delegates/DelegateCard.svelte';
	import { getModalStore } from '@skeletonlabs/skeleton';

	export let govProposal: GovProposal;
	export let delegate: Delegate | null = null;
	// export let dels: Delegate[];
	export let open: boolean = true;

	let delsAtDate: Delegate[] = [];

	const modalStore = getModalStore();

	function onShowDetails(voteResult: VoteResult | null) {
		if (!voteResult) return;

		modalStore.close();
		currentVoteResultStore.set(voteResult);
		currentDelegatesAtDateStore.set([
			voteResult.legislative_initiative.created_at.toString(),
			delsAtDate
		]);
		// $: if (browser) {
		gotoHistory('/vote_result', true);
		// }
	}

	$: rawEmphasis = govProposal.ministrial_proposal.emphasis;
</script>

<div class="sm:hidden entry bg-primary-200 dark:bg-primary-400 mt-3">
	<Emphasis {rawEmphasis} isAiGenerated={false} useTitleHover />

	{#if govProposal.vote_result}
		<div class="rounded-md w-full bg-primary-100 parliament-item mt-3 mb-3">
			<VoteParliament2
				voteResult={govProposal.vote_result}
				bind:delegates={delsAtDate}
				preview={true}
			/>
		</div>
	{/if}
	{#if govProposal.topics.length > 0}
		<div
			class="topics-item flex rounded-xl justify-center items-center bg-primary-300 p-3 mb-3 mt-1"
		>
			<Topics
				topics={govProposal.topics.sort((a, b) => {
					return a.topic.length - b.topic.length;
				})}
			/>
		</div>
	{/if}
	<!-- <InfoTiles voteResult={govProposal} {dels} isCenter /> -->
	<GovProposalInfoTiles {govProposal} isCenter />

	<div class="flex justify-between mt-3">
		<SButton
			class="bg-primary-300 text-black"
			on:click={() => {
				open = false;
			}}>Einklappen</SButton
		>
		<!-- <div class="accepted-item bg-primary-300">Angenommen: {voteResult.legislative_initiative.accepted}</div> -->
		{#if govProposal.vote_result}
			<div class="ml-auto more-info-item">
				<SButton
					class="bg-tertiary-500 text-black"
					on:click={() => onShowDetails(govProposal.vote_result)}>Details anzeigen</SButton
				>
			</div>
		{/if}
	</div>
</div>

<div class="max-sm:!hidden entry bg-primary-200 dark:bg-primary-400 mt-3 flex flex-wrap">
	<!-- Inneres Migration Frauen Klimaschutz -->

	<div class="flex flex-wrap">
		<div class="emphasis">
			<Emphasis {rawEmphasis} isAiGenerated={false} useTitleHover />
		</div>
		{#if govProposal.vote_result}
			<button
				class="rounded-xl ml-auto parliament-item bg-primary-100"
				on:click={() => onShowDetails(govProposal.vote_result)}
			>
				<VoteParliament2 voteResult={govProposal.vote_result} preview={true} />
			</button>
		{/if}

		{#if delegate}
			<div class="delegate-card">
				<DelegateCard {delegate} onlyTop showMoreDetailsBtn showImg={false} />
			</div>
		{/if}
	</div>

	<div class="flex flex-wrap gap-3">
		<GovProposalInfoTiles {govProposal} />

		{#if govProposal.topics.length > 0}
			<div
				class="topics-item flex-row flex rounded-xl justify-center items-center bg-primary-300 p-3 max-h-[169px]"
			>
				<Topics
					topics={govProposal.topics.sort((a, b) => {
						return a.topic.length - b.topic.length;
					})}
				/>
			</div>
		{/if}
	</div>
	<!-- <InfoTiles voteResult={govProposal} {dels} /> -->

	{#if govProposal.vote_result}
		<div class="ml-auto details-item mt-auto">
			<SButton
				class="bg-tertiary-500 text-black"
				on:click={() => onShowDetails(govProposal.vote_result)}>Betreffende Abstimmung</SButton
			>
		</div>
	{/if}
</div>

<style>
	.entry {
		border-radius: 0.9rem;
		box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
		padding: 20px;
		gap: 10px;
	}

	.emphasis {
		flex-basis: 70%;
	}

	.delegate-card {
		flex-basis: 30%;
	}

	.parliament-item {
		flex-basis: 24%;
	}
	.topics-item {
		/* flex-basis: 20%; */
		max-width: 390px;
	}
</style>
