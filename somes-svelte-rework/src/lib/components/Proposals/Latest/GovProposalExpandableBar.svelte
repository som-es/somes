<script lang="ts">
	import { createVoteResultPath, type Delegate, type GovProposal, type GovProposalDelegate, type VoteResult } from '$lib/types';
	import collapse from 'svelte-collapse';
	import rightArrowIcon from '$lib/assets/misc_icons/right-arrow.svg?raw';
	import VoteParliament2 from '$lib/components/Parliaments/VoteParliament2.svelte';
	import { currentVoteResultStore } from '$lib/stores/stores';
	import { gotoHistory } from '$lib/goto';
	import GovProposalExpanded from '../ExpandableAtDelegate/GovProposalExpanded.svelte';
	import { address } from '$lib/api/api';
	import { dashDateToDotDate } from '$lib/date';

	export let govProposal: GovProposalDelegate;
	// export let dels: Delegate[];
	let clazz = '';
	export { clazz as class };
	let open = false;
	let duration = 0.35;

	function onShowDetails(voteResult: VoteResult | null) {
		if (!voteResult) return;
		currentVoteResultStore.set(voteResult);
		gotoHistory(createVoteResultPath(voteResult), true);
	}
</script>

<div class="gap-3 mt-5">
	<div
		on:click={() => (open = !open)}
		on:keypress={() => (open = !open)}
		role="button"
		tabindex="0"
		class="entry bg-primary-300 dark:bg-primary-500 flex justify-between items-center text-black dark:text-white"
	>
		<!-- <div>
			<div id={open ? 'open' : 'closed'}>
				{@html rightArrowIcon}
			</div>
		</div> -->


		<div class="flex flex-col gap-1">
			<span>{govProposal.gov_proposal.ministrial_proposal.description.slice(30)}</span>
			<div class="flex flex-wrap gap-1">
				<span class="badge bg-tertiary-400 text-black text-wrap">{govProposal.gov_proposal.ministrial_proposal.ressort}</span>
				<span class="badge bg-tertiary-400 text-black">{dashDateToDotDate(govProposal.gov_proposal.ministrial_proposal.created_at.split('T')[0].toString())}</span>
				<span class="badge bg-tertiary-400 text-black">{govProposal.gov_proposal.ministrial_proposal.gp}</span>
			</div>
		</div>
		<!-- <div>{voteResult.legislative_initiative.description}</div> -->

		{#if govProposal.gov_proposal.vote_result}
			<button
				class="max-sm:hidden z-20 w-[7.5rem] bg-primary-100 dark:bg-primary-300 rounded-md"
				on:click={() => onShowDetails(govProposal.gov_proposal.vote_result)}
			>
				<VoteParliament2 voteResult={govProposal.gov_proposal.vote_result} preview={true} />
			</button>
		{:else}
			<div></div>
		{/if}
		<div>
			<img
				class="min-w-[80px] max-h-[80px] rounded-full mx-1"
				src={`${address}/assets/${govProposal.delegate.id}.jpg`}
				alt="Image of delegate {govProposal.delegate.name}"
			/>
		</div>
	</div>

	<div use:collapse={{ open, duration }}>
		<GovProposalExpanded
			govProposal={govProposal.gov_proposal}
			delegate={govProposal.delegate}
			bind:open
		/>
	</div>
</div>

<style>
	.entry {
		border-radius: 0.9rem;
		box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
		padding: 20px;
		gap: 10px;
	}
	/* .grid-container {
		display: grid;
		grid-template-columns: 2fr 1fr;
	} */

	#open :global(.right-arrow) {
		transform: rotate(90deg);
		transition: transform 0.35s;
	}

	#closed :global(.right-arrow) {
		transform: rotate(0deg);
		transition: transform 0.35s;
	}
</style>
