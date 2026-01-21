<script lang="ts">
	import SimpleDonut from '$lib/components/UI/SimpleDonut.svelte';
	import crossmarkIcon from '$lib/assets/misc_icons/crossmark.svg?raw';
	import checkmarkIcon from '$lib/assets/misc_icons/checkmark.svg?raw';
	import { type ConicStop } from '@skeletonlabs/skeleton-svelte';
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

	showDate = false;
	showDate = false;
	showAccepted = false;

	$: NOT_REACHED_COLOR = getModeUserPrefers()
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

<div class="bg-primary-300 rounded-xl dark:bg-primary-400 p-4 flex-1 justify-center min-w-[150px]">
	{#if showAchievedVotes && voteResult.legislative_initiative.accepted}
		<div class="flex gap-3">
			<SimpleDonut
				stops={conicsStopsAchievedVotes}
				{isLightMode}
				mark50={voteResult.legislative_initiative.requires_simple_majority ?? false}
				mark66={!(voteResult.legislative_initiative.requires_simple_majority ?? true)}
			/>

			{#if showText}
				<div class="flex flex-col justify-center">
					<div class="flex-col text-sm">
						<div>Erreichte</div>
						<div>Stimmen</div>
					</div>
				</div>
			{/if}
		</div>
	{/if}
	{#if showRequiredMajority}
		<div class="flex gap-3">
			<SimpleDonut
				{isLightMode}
				stops={voteResult.legislative_initiative.requires_simple_majority
					? conicStopsSimpleMajority
					: conicStopsOtherMajority}
			/>

			{#if showText}
				<div class="flex flex-col justify-center">
					<div class="flex-col text-sm">
						<div>Mindest-</div>
						<div>Mehrheit</div>
					</div>
				</div>
			{/if}
		</div>
	{/if}
</div>
