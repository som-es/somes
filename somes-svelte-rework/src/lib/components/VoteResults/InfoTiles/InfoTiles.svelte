<script lang="ts">
	import SimpleDonut from '$lib/components/UI/SimpleDonut.svelte';
	import crossmarkIcon from '$lib/assets/misc_icons/crossmark.svg?raw';
	import checkmarkIcon from '$lib/assets/misc_icons/checkmark.svg?raw';
	import type { ConicStop } from '@skeletonlabs/skeleton';
	import { partyToColor } from '$lib/partyColor';
	import type { Delegate, Vote, VoteResult } from '$lib/types';
	import Square from '$lib/components/UI/Square.svelte';
	import { dashDateToDotDate } from '$lib/date';

	export let voteResult: VoteResult;
	export let dels: Delegate[];
	export let isCenter: boolean = false;

	const NOT_REACHED_COLOR = 'rgb(var(--color-primary-600))';
	const REACHED_COLOR = 'rgb(var(--color-secondary-300))';

	const conicStopsSimpleMajority: ConicStop[] = [
		{ color: 'rgb(var(--color-secondary-400))', start: 0, end: 180 },
		{ color: NOT_REACHED_COLOR, start: 180, end: 360 }
	];

	const conicStopsOtherMajority: ConicStop[] = [
		{ color: 'rgb(var(--color-secondary-400))', start: 0, end: 240 },
		{ color: NOT_REACHED_COLOR, start: 240, end: 360 }
	];
    
    function findDelegateById(id: number): Delegate | undefined {
		return dels.find((del) => del.id === id);
	}

    function generateConicStopsWithVoteForAchiedVotesWithVoteSum(votes: Vote[], voteSum: number): ConicStop[] {
        votes.sort((a, b) => b.fraction - a.fraction);
		let currentStart = 0;

		let conicStops = [];

		for (let i = 0; i < votes.length; i++) {
			let vote = votes[i];
			if (!vote.infavor) {
				continue;
			}
			const share = (vote.fraction / voteSum) * 360;
			const prevStart = currentStart;
			currentStart += share;
			conicStops.push({ color: partyToColor(vote.party), start: prevStart, end: currentStart });
		}

		if (conicStops.length != votes.length) {
			conicStops.push({ color: NOT_REACHED_COLOR, start: currentStart, end: 360 - currentStart });
		}

		return conicStops;
    }

	function generateConicStopsForAchievedVotes(): ConicStop[] {
		if (voteResult.named_votes !== null) {
			const voteInfo = voteResult.named_votes.named_vote_info;
            const partyYesCount = new Map<string, number>();
            voteResult.named_votes.named_votes.forEach((namedVote) => {
                const delegate = findDelegateById(namedVote.delegate_id);
                if (delegate != null && namedVote.infavor) {
                    partyYesCount.set(delegate.party, (partyYesCount.get(delegate.party) ?? 0) +1);
                }
            });
            const votes = [...partyYesCount.entries()];
            votes.sort((a, b) => b[1] - a[1]);
			
            let conicStops: ConicStop[] = [];
            
		    let currentStart = 0;

            votes.forEach(vote => {
			    const share = (vote[1] / voteInfo.given_vote_sum) * 360;
			    conicStops.push({ color: partyToColor(vote[0]), start: currentStart, end: currentStart + share });
                currentStart += share;
                
            })
    
    
            if (currentStart < 360) {
			    conicStops.push({ color: NOT_REACHED_COLOR, start: currentStart, end: 360 - currentStart });
		    }

            // simpler method:
			// const voteInfo = voteResult.named_votes.named_vote_info;
			// const share = (voteInfo.pro_count / voteInfo.given_vote_sum) * 360;
			// conicStops.push({ color: REACHED_COLOR, start: 0, end: share });
			// conicStops.push({ color: NOT_REACHED_COLOR, start: share, end: 360 - share });
			return conicStops;
		}
		let fractionSum = 0;
		voteResult.votes.forEach((vote) => {
			fractionSum += vote.fraction;
		});

        return generateConicStopsWithVoteForAchiedVotesWithVoteSum(voteResult.votes.slice(), fractionSum)
		
	}
	const conicsStopsAchievedVotes = generateConicStopsForAchievedVotes();
</script>

<div class="flex flex-wrap {isCenter ? 'justify-center' : ''} info-item gap-3">
	<Square class="accepted-item">
		{#if voteResult.legislative_initiative.accepted == 'a'}
			{@html checkmarkIcon}
			<div>Angenommen</div>
		{:else}
			{@html crossmarkIcon}
			<div>Abgelehnt</div>
			{#if voteResult.legislative_initiative.accepted == 'p'}
				<div>(frühzeitig)</div>
			{/if}
		{/if}
	</Square>
	<Square class="majority-item">
		<SimpleDonut
			stops={voteResult.legislative_initiative.requires_simple_majority
				? conicStopsSimpleMajority
				: conicStopsOtherMajority}
		/>
		<div>Notwendige</div>
		<div>Mehrheit</div>
	</Square>
	<Square>
		<SimpleDonut stops={conicsStopsAchievedVotes} />
		<div>Erreichte</div>
		<div>Mehrheit</div>
		<!-- {voteResult.legislative_initiative.requires_simple_majority ? "1/2" : "2/3" } -->
	</Square>
	<Square>
		<div class="font-bold text-lg">
			{dashDateToDotDate(voteResult.legislative_initiative.created_at.toString())}
		</div>
		<div>Abgestimmt am</div>
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
		overflow: hidden; /* NEW */
		min-width: 0; /* NEW; needed for Firefox */
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
