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

	export let voteResult: VoteResult;
	export let dels: Delegate[];

	const emphasis = voteResult.legislative_initiative.emphasis
		?.split('\n\t')
		.filter((x) => x.length > 0);

    function dashDateToDotDate(date: string): string {
        const dateParts = date.split('-');

        return `${dateParts[2]}.${dateParts[1]}.${dateParts[0]}`
    }

    const whichGridContainer = emphasis == null ? "grid-container-without-emphasis" : "grid-container-with-emphasis";

    const NOT_REACHED_COLOR = 'rgb(var(--color-primary-600))';

    const conicStopsSimpleMajority: ConicStop[] = [
        { color: 'rgb(var(--color-secondary-400))', start: 0, end: 180 },
        { color: NOT_REACHED_COLOR, start: 180, end: 360 },
    ];

    const conicStopsOtherMajority: ConicStop[] = [
        { color: 'rgb(var(--color-secondary-400))', start: 0, end: 240 },
        { color: NOT_REACHED_COLOR, start: 240, end: 360 },
    ];

    function generateConicStopsForAchievedVotes(): ConicStop[] {
        voteResult.votes.sort((a, b) => b.fraction - a.fraction);
        let fractionSum = 0;
        voteResult.votes.forEach((vote) => {
            fractionSum += vote.fraction;
        });
        let currentStart = 0;

        let conicStops = [];

        for (let i = 0; i < voteResult.votes.length; i++) {
            let vote = voteResult.votes[i];
            if (!vote.infavor) {
                continue
            }
            const share = (vote.fraction / fractionSum) * 360;
            const prevStart = currentStart;
            currentStart += share;
            conicStops.push({color: partyToColor(vote.party), start: prevStart, end: currentStart});
        }

        if (conicStops.length != voteResult.votes.length) {
            conicStops.push({color: NOT_REACHED_COLOR, start: currentStart, end: 360 - currentStart});
        }

        return conicStops;
    }
    const conicsStopsAchievedVotes = generateConicStopsForAchievedVotes();

</script>
<div class="lg:!hidden entry bg-primary-200 dark:bg-primary-400 mt-3">
    <Emphasis emphasis={emphasis} /> 
    <div class="rounded-md w-full">
        <VoteParliament {dels} {voteResult} preview={true} />
    </div>

    <div class="flex justify-between"> 
        <div class="accepted-item bg-primary-300">Angenommen: {voteResult.legislative_initiative.accepted}</div>
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
    <div class="flex flex-wrap info-item gap-3">

        <div class="responsive-accepted-hidden accepted-item square bg-primary-300">
            <div class="flex flex-col items-center justify-center">
                {#if voteResult.legislative_initiative.accepted}	
                    {@html checkmarkIcon}
                    <div>
                        Angenommen
                    </div>
                {:else}
                    {@html crossmarkIcon}
                    <div>
                        Abgelehnt
                    </div>
                {/if}
            </div>
            
            <!-- Angenommen: {voteResult.legislative_initiative.accepted} -->
        </div>
        <div class="majority-item square flex bg-primary-300">
            <div class="flex flex-col items-center justify-center">
                <SimpleDonut stops={voteResult.legislative_initiative.requires_simple_majority ? conicStopsSimpleMajority : conicStopsOtherMajority} />
                <div>
                    Notwendige
                </div>
                <div>
                    Mehrheit
                </div>

            </div> 
        </div>
        <div class="square flex bg-primary-300">
            <div class="flex flex-col items-center justify-center">
                <SimpleDonut stops={conicsStopsAchievedVotes} />
                <div>
                    Erreichte
                </div>
                <div>
                    Mehrheit
                </div>
                <!-- {voteResult.legislative_initiative.requires_simple_majority ? "1/2" : "2/3" } -->

            </div> 
        </div>
        <div class="accepted-item square bg-primary-300">
            <div class="flex flex-col items-center justify-center">
                <div class="bold font-bold text-lg">
                    {dashDateToDotDate(voteResult.legislative_initiative.created_at.toString())}
                </div>
                <div>
                    Abgestimmt am 
                </div>
            </div>
        </div>
    </div>
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

	.square {
		aspect-ratio: 1/ 1;
        min-width: 140px;
        min-height: 140px;
        max-height: 140px;
		/* padding: 5%; */
        display: flex;
        justify-content: center;
        align-content: center;
        border-radius: 1rem;
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

	.info-item {
		grid-area: i;
        overflow: hidden;  /* NEW */
        min-width: 0;      /* NEW; needed for Firefox */
	}

	.details-item {
		grid-area: d;
	}

	.item {
		grid-column: 1fr;
	}
    @media not all and (min-width: 1254px) {
    .responsive-accepted-hidden {
        display: none !important;
    }
}

</style>