<script lang="ts">
	import { type Delegate, type GovProposal, type GovProposalDelegate } from '$lib/types';
	import { slide } from 'svelte/transition';
	import { aiViewEnabledStore } from '$lib/stores/stores';
	import { gotoHistory } from '$lib/goto';
	import GovProposalExpanded from '../ExpandableAtDelegate/GovProposalExpanded.svelte';
	import { url } from '$lib/api/api';
	import { dashDateToDotDate } from '$lib/date';
	import { currentGovProposalDelegateStore } from '$lib/stores/stores';
	import { createGovProposalPath } from '../types';
	import NewBadge from '$lib/components/UI/NewBadge.svelte';
	import { getParliament, type Parliament } from '$lib/api/parliament';

	export let govProposal: GovProposalDelegate;
	export let showDelegate: boolean = false;
	export let coloring: string =
		'bg-primary-300 hover:bg-primary-400 dark:bg-primary-500 dark:hover:bg-primary-600 text-black dark:text-white';
	export let parliament: Parliament = getParliament();
	// export let dels: Delegate[];
	let clazz = '';
	export { clazz as class };
	let open = false;
	let duration = 0.35;

	function onShowDetails(govProposal: GovProposal, delegates: Delegate[]) {
		currentGovProposalDelegateStore.value = { gov_proposal: govProposal, delegates };
		gotoHistory(createGovProposalPath(govProposal.ministrial_proposal), true);
	}

	$: date = dashDateToDotDate(
		govProposal.gov_proposal.ministrial_proposal.raw_data_created_at.toString().split('T')[0]
	);

	$: delegate = govProposal.delegates?.at(0);

	function toggleOpen(e: Event) {
		e.preventDefault();
		if (typeof window !== 'undefined' && window.innerWidth < 1024) {
			onShowDetails(govProposal.gov_proposal, govProposal.delegates ?? []);
		} else {
			open = !open;
		}
	}
</script>

{#if govProposal}
	<div class="mt-5">
		<a
			href={createGovProposalPath(govProposal.gov_proposal.ministrial_proposal)}
			onclick={toggleOpen}
			onkeypress={toggleOpen}
			role="button"
			tabindex="0"
			class="entry flex items-center justify-between transition-colors {coloring}"
		>
			<!-- <div>
			<div id={open ? 'open' : 'closed'}>
				{@html rightArrowIcon}
			</div>
		</div> -->

			<div class="flex w-full flex-col gap-1">
				{#if aiViewEnabledStore.value && govProposal.gov_proposal.ai_summary}
					<span
						class="text-xl font-semibold"
						style="hyphens: auto; word-break: normal; overflow-wrap: break-word;"
					>
						{govProposal.gov_proposal.ai_summary.short_title}
					</span>
					<span class="sm:text-md text-sm">
						{govProposal.gov_proposal.ai_summary.short_summary}
					</span>
				{:else}
					<span>{govProposal.gov_proposal.ministrial_proposal.title.split('|')[0]}</span>
				{/if}
				<div class="flex items-center justify-between">
					<div class="mt-2 flex flex-wrap gap-1 md:mt-4">
						<span class="badge bg-tertiary-400 text-wrap text-black"
							>{govProposal.gov_proposal.ministrial_proposal.ressort}</span
						>
						<span class="badge bg-tertiary-400 text-black">{date}</span>
						<span class="badge bg-tertiary-400 text-black"
							>{govProposal.gov_proposal.ministrial_proposal.gp}</span
						>
					</div>
					{#if !showDelegate}
						<NewBadge date={govProposal.gov_proposal.ministrial_proposal.raw_data_created_at} />
					{/if}
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
			{#if showDelegate && delegate}
				<div class="hidden flex-col sm:flex">
					<img
						class="mx-1 max-h-[80px] min-w-[80px] rounded-full"
						src={parliament == 'at' ? `${url}assets/${delegate.id}.jpg` : delegate.image_url}
						title={delegate.name}
						alt="Image of delegate {delegate.name}"
					/>
					<span class="bottom-0 rounded text-[8px]">
						{#if delegate.image_copyright}
							&copy {delegate.image_copyright}
						{:else}
							&copy Parlamentsdirektion
						{/if}
					</span>
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
