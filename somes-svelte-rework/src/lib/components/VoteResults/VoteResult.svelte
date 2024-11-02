<script lang="ts">
	import { gotoHistory } from '$lib/goto';
	import { currentVoteResultStore } from '$lib/stores/stores';
	import type { Delegate, VoteResult } from '$lib/types';
	import VoteParliament from '../Parliaments/VoteParliament.svelte';
	import VoteParliament2 from '../Parliaments/VoteParliament2.svelte';

	export let voteResult: VoteResult;
	export let dels: Delegate[];
	export let tabindex: number;

	function onClick() {
		currentVoteResultStore.set(voteResult);
		// $: if (browser) {
		gotoHistory('/vote_result', true);
		// }
	}
</script>

<span class="card tile hover:cursor-pointer">
	<div class="tile-content">
		<div
			class="w-[360px]"
			on:click={onClick}
			on:keypress={onClick}
			role="link"
			tabindex={10 + tabindex}
		>
			<VoteParliament2 {voteResult} preview={true} />
		</div>
		<span class="mx-3 text-left">{voteResult.legislative_initiative.description}</span>
	</div>
</span>

<style>
	.tile {
		box-sizing: border-box;
		padding: 0;
		border-radius: 25px;
		position: relative;
		z-index: 1;
		overflow: hidden;
		width: 25rem;
		margin: 0.8rem;
	}

	.tile-content {
		display: flex;
		justify-content: center;
		flex-direction: column;
		align-items: center;
	}
</style>
