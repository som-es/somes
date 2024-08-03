<script lang="ts">
	import SimpleDonut from "$lib/components/UI/SimpleDonut.svelte";
	import crossmarkIcon from '$lib/assets/misc_icons/crossmark.svg?raw';
	import checkmarkIcon from '$lib/assets/misc_icons/checkmark.svg?raw';
	import type { ConicStop } from "@skeletonlabs/skeleton";
	import { partyToColor } from "$lib/partyColor";
	import type { VoteResult } from "$lib/types";
	import Square from "$lib/components/UI/Square.svelte";

    export let voteResult: VoteResult;
    export let isCenter: boolean = false;
    
    function dashDateToDotDate(date: string): string {
        const dateParts = date.split('-');

        return `${dateParts[2]}.${dateParts[1]}.${dateParts[0]}`
    }

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

<div class="flex flex-wrap {isCenter ? "justify-center" : ""} info-item gap-3">
    <Square class="accepted-item">
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
    </Square>
    <Square class="majority-item">
        <SimpleDonut stops={voteResult.legislative_initiative.requires_simple_majority ? conicStopsSimpleMajority : conicStopsOtherMajority} />
        <div>
            Notwendige
        </div>
        <div>
            Mehrheit
        </div>
    </Square>
    <Square>
        <SimpleDonut stops={conicsStopsAchievedVotes} />
        <div>
            Erreichte
        </div>
        <div>
            Mehrheit
        </div>
        <!-- {voteResult.legislative_initiative.requires_simple_majority ? "1/2" : "2/3" } -->
    </Square>
    <Square>
        <div class="font-bold text-lg">
            {dashDateToDotDate(voteResult.legislative_initiative.created_at.toString())}
        </div>
        <div>
            Abgestimmt am 
        </div>
    </Square>
</div>

<style>
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

	:global(.accepted-item) {
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

    /* @media not all and (min-width: 1254px) {
        .responsive-accepted-hidden {
            display: none !important;
        }
    } */

</style>
