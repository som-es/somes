<script lang="ts">
	import VoteParliament from "$lib/components/Parliaments/VoteParliament.svelte";
	import Topics from "$lib/components/Topics/Topics.svelte";
	import SButton from "$lib/components/UI/SButton.svelte";
	import crossmarkIcon from '$lib/assets/misc_icons/crossmark.svg?raw';
	import checkmarkIcon from '$lib/assets/misc_icons/checkmark.svg?raw';
	import type { Delegate, VoteResult } from "$lib/types";
	import { type ConicStop } from "@skeletonlabs/skeleton";
	import SimpleDonut from "$lib/components/UI/SimpleDonut.svelte";
	import { partyToColor } from "$lib/partyColor";
	import Emphasis from "../Emphasis/Emphasis.svelte";
	import InfoTiles from "../InfoTiles/InfoTiles.svelte";

	export let voteResult: VoteResult;
	export let dels: Delegate[];

	const emphasis = voteResult.legislative_initiative.emphasis
		?.split('\n\t')
		.filter((x) => x.length > 0);

    const whichGridContainer = emphasis == null ? "grid-container-without-emphasis" : "grid-container-with-emphasis";
</script>
<div class="lg:!hidden entry bg-primary-200 dark:bg-primary-400 mt-3">
    <Emphasis emphasis={emphasis} /> 
    <div class="rounded-md w-full bg-primary-100 parliament-item mt-3 mb-3">
        <VoteParliament {dels} {voteResult} preview={true} />
    </div>

    <InfoTiles voteResult={voteResult} isCenter />

    <div class="flex justify-between mt-3"> 
        <div></div>
        <!-- <div class="accepted-item bg-primary-300">Angenommen: {voteResult.legislative_initiative.accepted}</div> -->
        <div class="ml-auto more-info-item"><SButton class="bg-tertiary-500">Details anzeigen</SButton></div>
    </div>

</div>
<div class="max-lg:!hidden entry bg-primary-200 dark:bg-primary-400 mt-3 {whichGridContainer}">
    <!-- Inneres Migration Frauen Klimaschutz -->

    <Emphasis emphasis={emphasis}></Emphasis>

    <div class="topics-item flex justify-center items-center bg-primary-300 px-3">
        <Topics topics={voteResult.topics} />
    </div>

    <div class="rounded-md min-w-full max-w-full ml-auto  parliament-item bg-primary-100">
        <VoteParliament {dels} {voteResult} preview={true} />
    </div>
    <InfoTiles voteResult={voteResult} />
    <div class="ml-auto details-item mt-auto"><SButton class="bg-tertiary-500">Details anzeigen</SButton></div>
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
		border-radius: 2rem;
	}

	.topics-item {
		grid-area: t;
		border-radius: 2rem;
        /* overflow: hidden; */
        /* min-width: 0;*/
	}

	.emphasis-item {
		grid-area: e;
		border-radius: 2rem;
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