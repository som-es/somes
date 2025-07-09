<script lang="ts">
	import Topics from '$lib/components/Topics/Topics.svelte';
	import SButton from '$lib/components/UI/SButton.svelte';
	import { createVoteResultPath, type Delegate, type VoteResult } from '$lib/types';
	import Emphasis from '../Emphasis/Emphasis.svelte';
	import InfoTiles from '../InfoTiles/InfoTiles.svelte';
	import { currentDelegatesAtDateStore, currentVoteResultStore } from '$lib/stores/stores';
	import { gotoHistory } from '$lib/goto';
	import VoteParliament2 from '$lib/components/Parliaments/VoteParliament2.svelte';

	export let voteResult: VoteResult;
	export let dels: Delegate[];
	export let open: boolean = true;

	let delsAtDate: Delegate[] = [];

	function onShowDetails() {
		currentVoteResultStore.set(voteResult);
		currentDelegatesAtDateStore.set([
			voteResult.legislative_initiative.created_at.toString(),
			delsAtDate
		]);
		// $: if (browser) {
		gotoHistory(createVoteResultPath(voteResult), true);

		// }
	}

	$: rawEmphasis = voteResult.legislative_initiative.emphasis;

	$: whichGridContainer =
		rawEmphasis == null ? 'grid-container-without-emphasis' : 'grid-container-with-emphasis';
</script>

<div class="lg:!hidden entry bg-primary-200 dark:bg-primary-400 mt-3">
	<Emphasis
		{rawEmphasis}
		isAiGenerated={voteResult.legislative_initiative.is_emphasis_ai_generated ?? false}
		useTitleHover
	/>
	<div class="rounded-md w-full bg-primary-100 parliament-item mt-3 mb-3">
		<VoteParliament2 {voteResult} bind:delegates={delsAtDate} preview={true} />
	</div>
	<div class="topics-item flex rounded-xl justify-center items-center bg-primary-300 p-3 mb-3">
		<Topics
			topics={voteResult.topics.sort((a, b) => {
				return a.topic.length - b.topic.length;
			})}
		/>
	</div>
	<InfoTiles {voteResult} {dels} isCenter />

	<div class="flex justify-between mt-3">
		<SButton
			class="bg-primary-300 text-black"
			on:click={() => {
				open = false;
			}}>Einklappen</SButton
		>
		<!-- <div class="accepted-item bg-primary-300">Angenommen: {voteResult.legislative_initiative.accepted}</div> -->
		<div class="ml-auto more-info-item">
			<SButton class="bg-tertiary-500 text-black" on:click={onShowDetails}>Details anzeigen</SButton
			>
		</div>
	</div>
</div>
<div class="max-lg:!hidden entry bg-primary-200 dark:bg-primary-400 mt-3 {whichGridContainer}">
	<!-- Inneres Migration Frauen Klimaschutz -->

	<Emphasis
		{rawEmphasis}
		isAiGenerated={voteResult.legislative_initiative.is_emphasis_ai_generated ?? false}
		useTitleHover
	/>

	{#if voteResult.topics.length > 0}
		<div
			class="topics-item flex rounded-xl justify-center items-center bg-primary-300 p-3 max-h-[169px]"
		>
			<Topics
				topics={voteResult.topics.sort((a, b) => {
					return a.topic.length - b.topic.length;
				})}
			/>
		</div>
	{/if}

	{#if voteResult.legislative_initiative.accepted}
		<button
			class="rounded-xl min-w-full max-w-full ml-auto parliament-item bg-primary-100"
			on:click={onShowDetails}
		>
			<VoteParliament2 {voteResult} preview={true} />
		</button>
	{/if}
	<InfoTiles {voteResult} {dels} />
	<div class="ml-auto details-item mt-auto">
		<SButton class="bg-tertiary-500 text-black" on:click={onShowDetails}>Details anzeigen</SButton>
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
			'e e e e e p p p' /* e: emphasis, p: parliament */
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
			'i i i i i t p p' /* e: emphasis, p: parliament */
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

	.emphasis-item {
		grid-area: e;
	}

	.accepted-item {
		grid-area: a;
	}

	.majority-item {
		grid-area: m;
	}

	.date-item {
		grid-area: dt;
	}

	.details-item {
		grid-area: d;
	}

	.item {
		grid-column: 1fr;
	}
</style>
