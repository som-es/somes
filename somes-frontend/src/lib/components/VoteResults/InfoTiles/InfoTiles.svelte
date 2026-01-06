<script lang="ts">
	import SimpleDonut from '$lib/components/UI/SimpleDonut.svelte';
	import crossmarkIcon from '$lib/assets/misc_icons/crossmark.svg?raw';
	import checkmarkIcon from '$lib/assets/misc_icons/checkmark.svg?raw';
	import { partyToColor } from '$lib/partyColor';
	import type { Delegate, Vote, VoteResult } from '$lib/types';
	import Square from '$lib/components/UI/Square.svelte';
	import { dashDateToDotDate } from '$lib/date';
	import { get } from 'svelte/store';

	export let voteResult: VoteResult;
	export let dels: Delegate[];
	export let squareSize = '140px';
	export let squareClasses = '';
	export let isCenter: boolean = false;
	export let showRequiredMajority = true;
	export let showDate = true;
	export let showAchievedVotes = true;
	export let showAccepted = true;
	export let showText = true;

	showDate = false
	showRequiredMajority = false
	showDate = false
	showAccepted = false

	interface ConicStop {
		color: string;
		start: number;
		end: number;
	}

	$: NOT_REACHED_COLOR = isLightMode
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

	$: conicsStopsAchievedVotes = generateConicStopsForAchievedVotes();

	let isLightMode = true;

	$: {
		isLightMode = true;
		NOT_REACHED_COLOR =isLightMode 
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
		voteResult;
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
						code: '',
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
	<div class="flex gap-3 flex-wrap">
		{#if showAccepted && voteResult.legislative_initiative.accepted}
			<Square {squareSize} class="accepted-item {squareClasses}">
				{#if voteResult.legislative_initiative.accepted == 'a'}
					<span class="fill-green-600 dark:fill-green-500">
						{@html checkmarkIcon}
					</span>
					{#if showText}
						<div>Angenommen</div>
					{/if}
				{:else}
					{@html crossmarkIcon}
					{#if showText}
						<div>Abgelehnt</div>
						{#if voteResult.legislative_initiative.accepted == 'p'}
							<div>(frühzeitig)</div>
						{/if}
					{/if}
				{/if}
			</Square>
		{/if}
		{#if showRequiredMajority}
			<Square {squareSize} class="majority-item {squareClasses}">
				<SimpleDonut
					{isLightMode}
					stops={voteResult.legislative_initiative.requires_simple_majority
						? conicStopsSimpleMajority
						: conicStopsOtherMajority}
				/>

				{#if showText}
					<div>Notwendige</div>
					<div>Mehrheit</div>
				{/if}
			</Square>
		{/if}
	</div>
	<div class="flex gap-3 flex-wrap">
		{#if showAchievedVotes && voteResult.legislative_initiative.accepted}
			<Square {squareSize} class={squareClasses}>
				<SimpleDonut stops={conicsStopsAchievedVotes} {isLightMode} 
					mark50={voteResult.legislative_initiative.requires_simple_majority ?? false}
					mark66={!(voteResult.legislative_initiative.requires_simple_majority ?? true)}
				/>

				{#if showText}
					<div>Erreichte</div>
					<div>Stimmen</div>
				{/if}
				<!-- {voteResult.legislative_initiative.requires_simple_majority ? "1/2" : "2/3" } -->
			</Square>
		{/if}
		{#if showDate}
			<Square {squareSize} class={squareClasses}>
				<div class="font-bold text-lg">
					{dashDateToDotDate(voteResult.legislative_initiative.nr_plenary_activity_date.toString())}
				</div>
				{#if showText}
					{#if voteResult.votes.length > 0}
						<div>Abgestimmt am</div>
					{:else}
						<div>Letzte</div>
						<div>Plenaraktivität</div>
					{/if}
				{/if}
			</Square>
		{/if}
	</div>
</div>

<style>
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
