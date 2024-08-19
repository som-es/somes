<!-- TODO: merge this and the Parliament component in to one -->
<script lang="ts">
	import { cachedAllLegisPeriods } from '$lib/caching/legis_periods';
	import {
		setupParliament,
		type Bubble,
		setDelOnBubble,
		enrichCirclesWithNamedVoteInfoOnSeat,
		enrichCirclesWithSpeechInfoOnSeat,
		enrichParliamentBubbles
	} from '$lib/parliament';
	import { getPartyColors, partyToColor } from '$lib/partyColor';
	import type { Delegate, LegisPeriod, VoteResult } from '$lib/types';
	import { onMount } from 'svelte';
	import BaseParliament from './BaseParliament.svelte';
	import { delegates_at } from '$lib/api';
	import { groupPartyDelegates, setSeatsOfDels } from '$lib/parliaments/defaultParliament';
	import { get } from 'svelte/store';
	import { currentDelegatesAtDateStore } from '$lib/stores/stores';

	export let seats: number[] = [20, 27, 37, 43, 48, 54];
	export let dels: Delegate[];
	export let delsAtDate: Delegate[] = [];
	export let preview: boolean = false;
	export let voteResult: VoteResult | null;
	export let delegate: Delegate | null = null;
	export let againstOpacity: number = 0.16;
	export let gp: string = "";
	if (voteResult) gp = voteResult.legislative_initiative.gp;

	let clazz = '';
	export { clazz as class };

	const width = 830;
	const height = 900;

	function isPartyInFavor(party: string): boolean {
		const votes = voteResult?.votes.slice();
		if (!votes) {
			return false
		}
		// this sort is there because of named votes -> it should only look at the one with the higher count (pro, contra)
		// otherwise, it could happen that (absent, or new) delegates are marked as e.g. contra delegates even though the majority of the party voted for the change
		votes.sort((a, b) => b.fraction - a.fraction);
		return votes.find((vote) => vote.party === party)?.infavor ?? false;
	}

	function findDelegateById(id: number): Delegate | undefined {
		return dels.find((del) => del.id === id);
	}

	export let circles2d: Bubble[][] = setupParliament(seats, width, height, 7.9);
	export let circles: Bubble[] = [];
	export let selected: Bubble | null = null;

	function select(bubble: Bubble, event: MouseEvent | KeyboardEvent | null) {
		if (event != null) {
			event.stopPropagation();
		}

		if (bubble == null || bubble.del == null) {
			return;
		}

		selected = bubble;
		delegate = bubble.del;
	}

	const partyToColorMap = getPartyColors();

	let partyInfavorMap = new Map<string, boolean>();
	partyToColorMap.forEach((_v, party) => {
		partyInfavorMap.set(party, isPartyInFavor(party));
	});

	function setOpacity(bubble: Bubble) {
		if (bubble == null || bubble.del == null) {
			bubble.opacity = 0.2;
			return;
		}

		if (partyInfavorMap.has(bubble.del.party)) {
			bubble.opacity = partyInfavorMap.get(bubble.del.party) ? 1 : againstOpacity;
			return;
		}

		bubble.opacity = 1;
	}

	// enrichParliamentBubbles(circles2d, dels, voteResult, setOpacity);

	dels.forEach((del) => {
		setDelOnBubble(del, circles2d, partyToColor);

		if (del.seat_col != null && del.seat_row != null) {
			setOpacity(circles2d[del.seat_row - 1][del.seat_col - 1]);
		}
	});

	if (voteResult) {
		enrichCirclesWithSpeechInfoOnSeat(voteResult.speeches, circles2d, dels);
		if (voteResult.named_votes) {
			enrichCirclesWithNamedVoteInfoOnSeat(voteResult.named_votes.named_votes, circles2d, dels);
		}
	}

	// for (let r = 0; r < circles2d.length; r++) {
	// 	for (let c = 0; c < circles2d[r].length; c++) {
	// 		if (circles2d[r][c].del == null) {
	// 			circles2d[r][c].opacity = 0.0;
	// 		}
	// 	}
	// }
	const defaultSeats = [18, 25, 29, 33, 37, 41];
	let circlesPerParty2: Bubble[][] = setupParliament(defaultSeats, width, height, 7.9, false);

	let currentLegisInit = 'XXVII';
	onMount(async () => {
		let fetchedDelsAtDate;
		if (delsAtDate.length == 0) {
			const cachedDelsAtDate = get(currentDelegatesAtDateStore);
			if (voteResult) {
				if (cachedDelsAtDate && cachedDelsAtDate[0] == voteResult.legislative_initiative.created_at.toString()) {
					fetchedDelsAtDate = cachedDelsAtDate[1]
				} else {
					fetchedDelsAtDate = await delegates_at(voteResult.legislative_initiative.created_at);
				}
			}
		} else {
			fetchedDelsAtDate = delsAtDate;
		}
		if (fetchedDelsAtDate) {
			delsAtDate = fetchedDelsAtDate;
		}

		// circles2d = setupParliament(seats, width, height, 7.9);
		const allLegisPeriods = await cachedAllLegisPeriods();
		if (allLegisPeriods !== null && allLegisPeriods.length > 0) {
			currentLegisInit = allLegisPeriods[0].gp;
		}
	});

	$:  {

		let partyToDelegates = groupPartyDelegates(delsAtDate);
		// console.log(partyToDelegates);
		let all = 0;
		partyToDelegates.forEach((dels, _party) => {
			all += dels.length;
		});

		const partyToDelegatesArray = Array.from(partyToDelegates.entries());
		partyToDelegatesArray.sort((a, b) => {
			const aInfavor = partyInfavorMap.get(a[0]);
			const bInfavor = partyInfavorMap.get(b[0]);
			if (aInfavor == bInfavor) {
				return b[1].length - a[1].length;
			} else if (aInfavor == true && bInfavor == false) {
				return -1;
			} else {
				return 1;
			}
			// return b[1].length - a[1].length
		});

		setSeatsOfDels(partyToDelegatesArray, all, defaultSeats.slice());

		enrichParliamentBubbles(circlesPerParty2, delsAtDate, voteResult, setOpacity);
		circlesPerParty2 = circlesPerParty2;
	}

	// $: {

	// 		let partyToDelegates = groupPartyDelegates(delsAtDate);
	// 		console.log(partyToDelegates);
	// 		let all = 0;
	// 		partyToDelegates.forEach((dels, _party) => {
	// 			all += dels.length;
	// 		});

	// 		const partyToDelegatesArray = Array.from(partyToDelegates.entries());
	// 		partyToDelegatesArray.sort((a, b) => b[1].length - a[1].length);

	// 		setSeatsOfDels(partyToDelegatesArray, all, defaultSeats.slice());

	// 		enrichParliamentBubbles(circlesPerParty2, delsAtDate, voteResult, setOpacity);

	// 		circlesPerParty2 = circlesPerParty2;
	// }

	$: if (delegate && delegate.seat_row != null) {
		const circleArray =
			gp === currentLegisInit ? circles2d : circlesPerParty2;
		select(circleArray[delegate.seat_row - 1][delegate.seat_col! - 1], null);
	}
</script>

{#if gp === currentLegisInit}
	<BaseParliament class={clazz} {circles2d} {selected} {preview} {select} {width} {height} />
{:else if circlesPerParty2.length > 0}
	<BaseParliament
		class={clazz}
		bind:circles2d={circlesPerParty2}
		{selected}
		{preview}
		{select}
		{width}
		{height}
	/>
{:else}
	Sitzplan nicht verfügbar
{/if}
