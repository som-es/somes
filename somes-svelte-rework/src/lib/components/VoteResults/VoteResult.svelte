<script lang="ts">
	import { gotoHistory } from '$lib/goto';
	import { currentVoteResultStore } from '$lib/stores/stores';
	import { createVoteResultPath, type Delegate, type VoteResult } from '$lib/types';
	import VoteParliament2 from '../Parliaments/VoteParliament2.svelte';

	export let voteResult: VoteResult;
	export let dels: Delegate[];
	export let tabindex: number;

	function onClick() {
		currentVoteResultStore.set(voteResult);
		// $: if (browser) {

		gotoHistory(createVoteResultPath(voteResult), true);
		// }
	}
</script>

<span class="card tile hover:cursor-pointer"
	on:click={onClick}
	on:keypress={onClick}
	role="link"
	tabindex={10 + tabindex}
>
	<div class="tile-content">
		<div
			class="w-[360px]"
		>
			<VoteParliament2 showGovs {voteResult} preview={true} />
		</div>
		<div class="mx-3 text-left">
			<span>{voteResult.legislative_initiative.description}</span>
			{#if voteResult.legislative_initiative.is_law}
				<div class="badge bg-tertiary-400 text-black mx-2 my-1">Gesetz</div>
			{/if}
			{#if voteResult.legislative_initiative.ityp == "AA"}
				<div class="badge bg-tertiary-400 text-black mx-2 my-1">Abänderung</div>
			{/if}
		</div>
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
