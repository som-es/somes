<script lang="ts">
	import SimpleDonut from '$lib/components/UI/SimpleDonut.svelte';
	import crossmarkIcon from '$lib/assets/misc_icons/crossmark.svg?raw';
	import checkmarkIcon from '$lib/assets/misc_icons/checkmark.svg?raw';
	import { getModeUserPrefers, modeCurrent, type ConicStop } from '@skeletonlabs/skeleton';
	import { partyToColor } from '$lib/partyColor';
	import type { Delegate, Vote, VoteResult } from '$lib/types';
	import Square from '$lib/components/UI/Square.svelte';
	import { dashDateToDotDate } from '$lib/date';
	import { get } from 'svelte/store';

	export let voteResult: VoteResult;
	export let dels: Delegate[];
	export let isCenter: boolean = false;

	let NOT_REACHED_COLOR = getModeUserPrefers()
		? 'rgb(var(--color-primary-600))'
		: 'rgb(var(--color-primary-800))';

	const REACHED_COLOR = 'rgb(var(--color-secondary-300))';

	let conicStopsSimpleMajority: ConicStop[] = [
		{ color: 'rgb(var(--color-secondary-400))', start: 0, end: 180 },
		{ color: NOT_REACHED_COLOR, start: 180, end: 360 }
	];

	let conicStopsOtherMajority: ConicStop[] = [
		{ color: 'rgb(var(--color-secondary-400))', start: 0, end: 240 },
		{ color: NOT_REACHED_COLOR, start: 240, end: 360 }
	];

	let conicsStopsAchievedVotes = generateConicStopsForAchievedVotes();

	let isLightMode = true;

	$: {
		isLightMode = $modeCurrent;
		NOT_REACHED_COLOR = $modeCurrent
			? 'rgb(var(--color-primary-600))'
			: 'rgb(var(--color-primary-800))';
		conicStopsOtherMajority = [
			{ color: 'rgb(var(--color-secondary-400))', start: 0, end: 240 },
			{ color: NOT_REACHED_COLOR, start: 240, end: 360 }
		];
		conicStopsSimpleMajority = [
			{ color: 'rgb(var(--color-secondary-400))', start: 0, end: 180 },
			{ color: NOT_REACHED_COLOR, start: 180, end: 360 }
		];
		conicsStopsAchievedVotes = generateConicStopsForAchievedVotes();
	}
	function generateConicStopsWithVoteForAchiedVotesWithVoteSum(
		votes: Vote[],
		voteSum: number
	): ConicStop[] {
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

		if (currentStart < 360) {
			conicStops.push({ color: NOT_REACHED_COLOR, start: currentStart, end: 360 - currentStart });
		}

		return conicStops;
	}

	function generateConicStopsForAchievedVotes(): ConicStop[] {
		let voteSum = 0;
		let votes = voteResult.votes.slice();
		if (voteResult.named_votes !== null) {
			voteSum = voteResult.named_votes.named_vote_info.given_vote_sum;
			// page extraction approach is currently not implemented, thats why many do not contain the pro-contra information
			if (voteResult.named_votes.named_votes.length == 0) {
				votes = [
					{
						party: '',
						fraction: voteResult.named_votes.named_vote_info.pro_count,
						infavor: true,
						legislative_initiatives_id: 0
					}
				];
			}
		} else {
			voteResult.votes.forEach((vote) => {
				voteSum += vote.fraction;
			});
		}

		return generateConicStopsWithVoteForAchiedVotesWithVoteSum(votes, voteSum);
	}
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
			{isLightMode}
			stops={voteResult.legislative_initiative.requires_simple_majority
				? conicStopsSimpleMajority
				: conicStopsOtherMajority}
		/>
		<div>Notwendige</div>
		<div>Mehrheit</div>
	</Square>
	<Square>
		<SimpleDonut stops={conicsStopsAchievedVotes} {isLightMode} />
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
