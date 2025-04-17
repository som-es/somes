<script lang="ts">
	import { createVoteResultPath, type Delegate, type VoteResult } from '$lib/types';
	import collapse from 'svelte-collapse';
	import rightArrowIcon from '$lib/assets/misc_icons/right-arrow.svg?raw';
	import VoteResultExpanded from './VoteResultExpanded.svelte';
	import VoteParliament2 from '$lib/components/Parliaments/VoteParliament2.svelte';
	import SButton from '$lib/components/UI/SButton.svelte';

	import { currentDelegatesAtDateStore, currentVoteResultStore } from '$lib/stores/stores';
	import { gotoHistory } from '$lib/goto';
	import InfoTiles from '../InfoTiles/InfoTiles.svelte';

	export let voteResult: VoteResult;
	export let dels: Delegate[];
	let clazz;
	export { clazz as class };
	let open = false;
	let duration = 0.35;

	function onShowDetails() {
		currentVoteResultStore.set(voteResult);
		// $: if (browser) {
		// gotoHistory('/vote_result', true);
		gotoHistory(createVoteResultPath(voteResult), true);
		// }
	}
</script>

<div class="gap-3 mt-5 {clazz}">
	<div
		on:click={() => (open = !open)}
		on:keypress={() => (open = !open)}
		role="button"
		tabindex="0"
		class="entry bg-primary-300 dark:bg-primary-500 flex justify-between items-center"
	>
		<div>
			<div id={open ? 'open' : 'closed'}>
				{@html rightArrowIcon}
			</div>
		</div>
		<div>{voteResult.legislative_initiative.description}</div>
		<div class="flex items-center gap-4">
			{#if voteResult.legislative_initiative.is_law}
				<div class="badge bg-tertiary-400 text-black">Gesetz</div>
			{/if}

			{#if voteResult.legislative_initiative.ityp == "AA"}
				<div class="badge bg-tertiary-400 text-black ">Abänderung</div>
			{/if}
			<div class="max-sm:hidden">
				<InfoTiles
					squareSize={'10px'}
					squareClasses={'min-w-[60px] min-h-[60px] max-w-[60px] max-h-[60px] h-[60px] w-[60px] w-16 mx-4'}
					{voteResult}
					{dels}
					isCenter
					showText={true}
					showRequiredMajority={false}
					showDate={false}
					showAchievedVotes={false}
				/>
			</div>

			<!-- {#if voteResult} -->

			{#if voteResult.legislative_initiative.accepted}
				<button
					class="max-sm:hidden z-20 w-[7.5rem] bg-primary-100 dark:bg-primary-300 rounded-md"
					on:click={onShowDetails}
				>
					<VoteParliament2 {voteResult} preview={true} />
				</button>
			{/if}
		</div>
	</div>

	<div use:collapse={{ open, duration }}>
		<VoteResultExpanded {voteResult} {dels} bind:open />
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
