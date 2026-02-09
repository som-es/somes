<script lang="ts">
	import {
		createVoteResultPath,
		type Delegate,
		type GovProposal,
		type GovProposalDelegate,
		type VoteResult
	} from '$lib/types';
	import { slide } from 'svelte/transition';
	import VoteParliament2 from '$lib/components/Parliaments/VoteParliament2.svelte';
	import { currentVoteResultStore } from '$lib/stores/stores';
	import { gotoHistory } from '$lib/goto';
	import GovProposalExpanded from '../ExpandableAtDelegate/GovProposalExpanded.svelte';
	import { address } from '$lib/api/api';
	import { dashDateToDotDate } from '$lib/date';
	import { currentDelegatesAtDateStore, currentGovProposalDelegateStore } from '$lib/stores/stores';
	import { browser } from '$app/environment';
	import { createGovProposalPath } from '../types';

	export let govProposal: GovProposalDelegate;
	export let showDelegate: boolean = false;
	export let coloring: string = 'bg-primary-300 dark:bg-primary-500 text-black dark:text-white';
	// export let dels: Delegate[];
	let clazz = '';
	export { clazz as class };
	let open = false;
	let duration = 0.35;



	function onShowDetails(govProposal: GovProposal, delegate: Delegate) {
		currentGovProposalDelegateStore.value = { gov_proposal: govProposal, delegate };
		gotoHistory(createGovProposalPath(govProposal.ministrial_proposal), true);
	}


	function toggleOpen(e: Event) {
		e.preventDefault();
		if (typeof window !== 'undefined' && window.innerWidth < 1024) {
			onShowDetails(govProposal.gov_proposal, govProposal.delegate);
		} else {
			open = !open;
		}
	}

	$: date = dashDateToDotDate(
		govProposal.gov_proposal.ministrial_proposal.raw_data_created_at.toString().split('T')[0]
	);
</script>

{#if govProposal}
<div class="mt-5">
	<a
		href="{createGovProposalPath(govProposal.gov_proposal.ministrial_proposal)}"
		onclick={toggleOpen}
		onkeypress={toggleOpen}
		role="button"
		tabindex="0"
				class="entry flex justify-between items-center {coloring}"
	>
		<!-- <div>
			<div id={open ? 'open' : 'closed'}>
				{@html rightArrowIcon}
			</div>
		</div> -->


		<div class="flex flex-col gap-1">
			{#if govProposal.gov_proposal.ai_summary}
				<span class="text-xl font-semibold" style="hyphens: auto; word-break: normal; overflow-wrap: break-word;">
					{govProposal.gov_proposal.ai_summary.short_title}
				</span>
				<span class="text-sm sm:text-md">
					{govProposal.gov_proposal.ai_summary.short_summary}
				</span>
			{:else}
				<span>{govProposal.gov_proposal.ministrial_proposal.description.slice(30)}</span>
			{/if}
			<div class="flex flex-wrap gap-1 md:mt-4 mt-2">
				<span class="badge bg-tertiary-400 text-black text-wrap"
					>{govProposal.gov_proposal.ministrial_proposal.ressort}</span
				>
				<span class="badge bg-tertiary-400 text-black"
					>{date}</span
				>
				<span class="badge bg-tertiary-400 text-black"
					>{govProposal.gov_proposal.ministrial_proposal.gp}</span
				>
			</div>
		</div>
		<!-- <div>{voteResult.legislative_initiative.description}</div> -->

		<!-- {#if browser && govProposal.gov_proposal.vote_result && govProposal.gov_proposal.vote_result.legislative_initiative.accepted !== null}
			<button
				class="max-sm:hidden w-30 bg-primary-100 dark:bg-primary-300 rounded-md"
				on:click={() => onShowDetails(govProposal.gov_proposal.vote_result)}
			>
				<VoteParliament2
					voteResult={govProposal.gov_proposal.vote_result}
					showGovs
					preview={true}
				/>
			</button>
		{:else}
			<div></div>
		{/if} -->
		{#if showDelegate}
			<div class="hidden sm:block">
				<img
					class="min-w-[80px] max-h-[80px] rounded-full mx-1"
					src={`${address}/assets/${govProposal.delegate.id}.jpg`}
					title={govProposal.delegate.name}
					alt="Image of delegate {govProposal.delegate.name}"
				/>
			</div>
		{/if}
	</a>

	{#if open}
		<div transition:slide={{ duration: 240 }}>
			<GovProposalExpanded
				govProposal={govProposal.gov_proposal}
				delegate={govProposal.delegate}
				{showDelegate}
				bind:open
			/>
		</div>
	{/if}
</div>
{/if}

<style>
	.entry {
		border-radius: 0.9rem;
		box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
		padding: 20px;
		gap: 10px;
	}
</style>
