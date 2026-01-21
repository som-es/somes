<script lang="ts">
	import Topics from '$lib/components/Topics/Topics.svelte';
	import SButton from '$lib/components/UI/SButton.svelte';
	import { createVoteResultPath, type Delegate, type VoteResult } from '$lib/types';
	import Emphasis from '../Emphasis/Emphasis.svelte';
	import InfoTiles from '../InfoTiles/InfoTiles.svelte';
	import { currentDelegatesAtDateStore, currentVoteResultStore } from '$lib/stores/stores';
	import { gotoHistory } from '$lib/goto';
	import VoteTypeBadge from '../VoteTypeBadge.svelte';

	export let voteResult: VoteResult;
	export let dels: Delegate[];
	export let open: boolean = true;

	let delsAtDate: Delegate[] = [];

	function onShowDetails() {
		currentVoteResultStore.set(voteResult);
		currentDelegatesAtDateStore.set([
			voteResult.legislative_initiative.nr_plenary_activity_date.toString(),
			delsAtDate
		]);
		gotoHistory(createVoteResultPath(voteResult), true);
	}
</script>

<<<<<<< HEAD
<div class="entry bg-primary-200 dark:bg-primary-400 mt-3 p-2 lg:block hidden">
	<div class="flex gap-2">
		{#if voteResult.ai_summary}
			<div class="flex-grow basis-3/4">
				<Emphasis
					emphasis={voteResult.ai_summary.full_summary.key_points}
					glossary={voteResult.ai_summary.full_summary.glossary}
				/>
			</div>
			<div class="flex flex-col justify-between flex-grow basis-1/4 min-w-[320px]">
				<!-- <div class="flex gap-2">
					<div class="flex flex-1">
						<InfoTiles {voteResult} showRequiredMajority={false} showAchievedVotes={true} {dels} />
					</div>
					<div class="xl:flex flex-1 hidden">
						<InfoTiles {voteResult} showRequiredMajority={true} showAchievedVotes={false} {dels} />
					</div>
				</div> -->
				<div class="pt-2">
					<Topics
						topics={voteResult.eurovoc_topics.sort((a, b) => {
							return a.topic.length - b.topic.length;
						})}
					/>
				</div>
				<div class="flex items-center justify-end h-8 rounded-xl px-2">
					<button on:click={onShowDetails} class="rounded-lg px-2 py-1 bg-primary-300">
						<span class="text-base font-semibold">Mehr Details &#8594;</span>
					</button>
				</div>
			</div>
		{/if}
	</div>
</div>
=======
<div class="lg:hidden! entry bg-primary-200 dark:bg-primary-400 mt-3">
	{#if voteResult.ai_summary}
		<Emphasis
			emphasis={voteResult.ai_summary.full_summary.key_points}
			glossary={voteResult.ai_summary.full_summary.glossary}
		/>
	{/if}
	<div class="rounded-md w-full bg-primary-100 parliament-item mt-3 mb-3">
		<VoteParliament2 {voteResult} bind:delegates={delsAtDate} preview={true} />
	</div>
	<div class="topics-item flex rounded-xl justify-center items-center bg-primary-300 p-3 mb-3">
		<Topics
			topics={voteResult.eurovoc_topics.sort((a, b) => {
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
<div class="max-lg:hidden! entry bg-primary-200 dark:bg-primary-400 mt-3 {whichGridContainer}">
	<!-- Inneres Migration Frauen Klimaschutz -->
	{#if voteResult.ai_summary}
		<Emphasis
			emphasis={voteResult.ai_summary.full_summary.key_points}
			glossary={voteResult.ai_summary.full_summary.glossary}
		/>
	{/if}

	{#if voteResult.topics.length > 0}
		<div
			class="topics-item flex rounded-xl justify-center items-center bg-primary-300 p-3 max-h-[169px]"
		>
			<Topics
				topics={voteResult.eurovoc_topics.sort((a, b) => {
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
			<VoteParliament2 showGovs {voteResult} preview={true} />
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
	.grid-container-without-voting {
		box-sizing: border-box;
		display: grid;
		grid-template-areas:
			'e e e e e t t t' /* e: emphasis, p: parliament */
			'e e e e e i i i';
		/*'e e e e e p p p'*/
		/*'i i i i i t t d'; */
		/* a: accepted, m: majority? 2/3, 1/2, dt: date, d: details */
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
>>>>>>> sveltev5
