<script lang="ts">
	import type { Delegate, GovProposal, VoteResult } from '$lib/types';
	import collapse from 'svelte-collapse';
	import rightArrowIcon from '$lib/assets/misc_icons/right-arrow.svg?raw';
	import VoteParliament2 from '$lib/components/Parliaments/VoteParliament2.svelte';
	import GovProposalExpanded from './GovProposalExpanded.svelte';
	import { currentVoteResultStore } from '$lib/stores/stores';
	import { gotoHistory } from '$lib/goto';

	export let govProposal: GovProposal;
	// export let dels: Delegate[];
	let clazz = '';
	export { clazz as class };
	let open = false;
	let duration = 0.35;

	function onShowDetails(voteResult: VoteResult | null) {
		currentVoteResultStore.set(voteResult);
		// $: if (browser) {
		gotoHistory('/vote_result', true);
		// }
	}
</script>

<div class="gap-3 mt-5">
	<div
		on:click={() => (open = !open)}
		on:keypress={() => (open = !open)}
		role="button"
		tabindex="0"
		class="entry dark:bg-primary-300 bg-primary-400 flex justify-between items-center text-black"
	>
		<div>
			<div id={open ? 'open' : 'closed'}>
				{@html rightArrowIcon}
			</div>
		</div>
		<div>{govProposal.ministrial_proposal.description}</div>
		<!-- <div>{voteResult.legislative_initiative.description}</div> -->

		{#if govProposal.vote_result}
			<button
				class="max-sm:hidden z-20 w-[7.5rem] bg-primary-100 dark:bg-primary-300 rounded-md"
				on:click={() => onShowDetails(govProposal.vote_result)}
			>
				<VoteParliament2 voteResult={govProposal.vote_result} preview={true} />
			</button>
		{:else}
			<div></div>
		{/if}
	</div>

	<div use:collapse={{ open, duration }}>
		<GovProposalExpanded {govProposal} bind:open />
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
