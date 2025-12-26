<script lang="ts">
	import Topics from '$lib/components/Topics/Topics.svelte';
	import SButton from '$lib/components/UI/SButton.svelte';
	import {
		createVoteResultPath,
		type Delegate,
		type GovProposal,
		type VoteResult
	} from '$lib/types';
	import {
		currentDelegatesAtDateStore,
		currentGovProposalDelegateStore,
		currentVoteResultStore
	} from '$lib/stores/stores';
	import { gotoHistory } from '$lib/goto';
	import VoteParliament2 from '$lib/components/Parliaments/VoteParliament2.svelte';
	import Emphasis from '$lib/components/VoteResults/Emphasis/Emphasis.svelte';
	import GovProposalInfoTiles from '$lib/components/VoteResults/InfoTiles/GovProposalInfoTiles.svelte';
	import DelegateCard from '$lib/components/Delegates/DelegateCard.svelte';
	import { getModalStore } from '@skeletonlabs/skeleton';
	import { createGovProposalPath } from '../types';

	export let govProposal: GovProposal;
	export let delegate: Delegate;
	export let showDelegate: boolean = false;
	// export let dels: Delegate[];
	export let open: boolean = true;

	let delsAtDate: Delegate[] = [];

	const modalStore = getModalStore();
	function onShowDetails(gov_proposal: GovProposal, delegate: Delegate) {
		modalStore.close();
		currentGovProposalDelegateStore.set({ gov_proposal, delegate });
		gotoHistory(createGovProposalPath(gov_proposal.ministrial_proposal), true);
	}
	function onShowDetailsVoteResult(voteResult: VoteResult | null) {
		if (!voteResult) return;

		modalStore.close();
		currentVoteResultStore.set(voteResult);
		currentDelegatesAtDateStore.set([
			voteResult.legislative_initiative.created_at.toString(),
			delsAtDate
		]);
		gotoHistory(createVoteResultPath(voteResult), true);
	}

	$: aiSummary = govProposal.ai_summary;

	$: whichGridContainer =
		aiSummary == null ? 'grid-container-without-emphasis' : 'grid-container-with-emphasis';
</script>

<div class="sm:hidden entry bg-primary-200 dark:bg-primary-400 mt-3">
	{#if aiSummary}
		<Emphasis emphasis={aiSummary.full_summary.key_points} useTitleHover glossary={aiSummary.full_summary.glossary} />
	{/if}

	{#if govProposal.vote_result}
		<div class="rounded-md w-full bg-primary-100 parliament-item mt-3 mb-3">
			<VoteParliament2
				voteResult={govProposal.vote_result}
				bind:delegates={delsAtDate}
				showGovs
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
					on:click={() => onShowDetailsVoteResult(govProposal.vote_result)}
					>Details anzeigen</SButton
				>
			</div>
		{/if}
	</div>
</div>

<div class="max-lg:!hidden entry bg-primary-200 dark:bg-primary-400 mt-3 {whichGridContainer}">
	<!--  -->
	{#if aiSummary}
		<Emphasis emphasis={aiSummary.full_summary.key_points} useTitleHover glossary={aiSummary.full_summary.glossary} />
	{/if}

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

	{#if govProposal.vote_result}
		<button
			class="rounded-xl ml-auto parliament-item bg-primary-100"
			on:click={() => onShowDetailsVoteResult(govProposal.vote_result)}
		>
			<VoteParliament2 showGovs voteResult={govProposal.vote_result} preview={true} />
		</button>
	{/if}

	<GovProposalInfoTiles {govProposal} />
	{#if delegate && showDelegate}
		<div class="delegate-card gov-official">
			<DelegateCard {delegate} onlyTop showMoreDetailsBtn showImg={false} />
		</div>
	{/if}
	<div class="ml-auto details-item mt-auto">
		<SButton
			class="bg-tertiary-500 text-black"
			on:click={() => onShowDetails(govProposal, delegate)}>Details anzeigen</SButton
		>
	</div>
</div>

<style>
	.entry {
		border-radius: 0.9rem;
		box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
		padding: 20px;
		gap: 10px;
	}

	.grid-container-with-emphasis {
		box-sizing: border-box;
		display: grid;
		grid-template-areas:
			'e e e e e m m m' /* e: emphasis, p: parliament */
			'e e e e e p p p'
			'e e e e e p p p'
			'i i i i i t t d'; /* a: accepted, m: majority? 2/3, 1/2, dt: date, d: details */
		/* "i i i a"; */
		padding: 10px;
	}

	.grid-container-without-emphasis {
		box-sizing: border-box;
		display: grid;
		grid-template-areas:
			'i i i i i t p m' /* e: emphasis, p: parliament */
			'. . . . . . d d'; /* a: accepted, m: majority? 2/3, 1/2, dt: date, d: details */
		/* "i i i a"; */
		padding: 10px;
	}
	.parliament-item {
		grid-area: p;
	}

	.topics-item {
		grid-area: t;
		/* overflow: hidden; */
		/* min-width: 0;*/
	}

	.gov-official {
		grid-area: m;
	}

	.emphasis-item {
		grid-area: e;
	}

	.details-item {
		grid-area: d;
	}

	.topics-item {
		/* flex-basis: 20%; */
		max-width: 490px;
	}
</style>
