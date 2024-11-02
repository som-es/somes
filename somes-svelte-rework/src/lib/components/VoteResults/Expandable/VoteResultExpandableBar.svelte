<script lang="ts">
	import type { Delegate, VoteResult } from '$lib/types';
	import VoteParliament from '../../Parliaments/VoteParliament.svelte';
	import collapse from 'svelte-collapse';
	import rightArrowIcon from '$lib/assets/misc_icons/right-arrow.svg?raw';
	import VoteResultExpanded from './VoteResultExpanded.svelte';
	import VoteParliament2 from '$lib/components/Parliaments/VoteParliament2.svelte';

	export let voteResult: VoteResult;
	export let dels: Delegate[];
	let clazz;
	export { clazz as class };
	let open = false;
	let duration = 0.35;
</script>

<div class="gap-3 mt-5">
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
		<div class="w-20 bg-primary-100 dark:bg-primary-300 rounded-md">
			<VoteParliament2 {voteResult} preview={true} />
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
