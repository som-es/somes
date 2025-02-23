<script lang="ts">
	import type { Delegate, VoteResult } from '$lib/types';
	import VoteResultComp from './VoteResult.svelte';
	import collapse from 'svelte-collapse';
	import SButton from '../UI/SButton.svelte';
	import { gotoHistory } from '$lib/goto';

	export let dels: Delegate[];

	export let voteResults: VoteResult[];
	export let showHistory: boolean = false;

	$: firstThreeVotes = voteResults.slice(0, 3);
	$: restVotes = voteResults.slice(3);

	let open = false;
</script>

{#if voteResults}
	{#if voteResults.length == 0}
		<p class="no-votes mb-5 mt-1 text-xl">Keine Abstimmungsergebnisse</p>
	{/if}
	<div class="card-container">
		{#each firstThreeVotes as voteResult, i}
			<VoteResultComp {dels} {voteResult} tabindex={i} />
		{/each}
	</div>
	<div class="flex justify-between px-3">
		{#if voteResults.length > 3}
			<SButton class="button offset-button bg-primary-500" on:click={() => (open = !open)}>
				{#if open}
					Einklappen
				{:else}
					Ausklappen
				{/if}
			</SButton>
		{:else}
			<div></div>
		{/if}

		{#if showHistory}
			<SButton
				class="button offset-button bg-secondary-500"
				on:click={() => gotoHistory('/vote_history')}
			>
				Vorherige anzeigen
			</SButton>
		{/if}
	</div>
	<hr />

	<div use:collapse={{ open }}>
		<div
			on:click={() => (open = !open)}
			on:keypress={() => (open = !open)}
			class="card-container z-0 mt-4"
			role="button"
			tabindex="0"
		>
			{#each restVotes as voteResult, i}
				<VoteResultComp {dels} {voteResult} tabindex={i} />
			{/each}
		</div>
	</div>
{/if}

<style>
	.card-container {
		margin: auto;
		display: flex;
		flex-direction: row;
		/* remove this maybe again */
		justify-content: start;
		flex-wrap: wrap;
	}
</style>
