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
	import { delegates_at, delegates_with_seats_near_date } from '$lib/api';
	import { groupPartyDelegates, setSeatsOfDels } from '$lib/parliaments/defaultParliament';
	import { get } from 'svelte/store';
	import { currentDelegatesAtDateStore } from '$lib/stores/stores';
	import Autocomplete from '../Autocompletion/Autocomplete.svelte';
	import { cachedAllSeats, getSeats } from '$lib/caching/seats';

	const width = 830;
	const height = 900;

	let clazz = '';
	export { clazz as class };
	
	export let orderingFactor: number = 1;
	export let preview: boolean = false;
	export let againstOpacity: number = 0.16;

	export let delegate: Delegate | null = null;
	export let selected: Bubble | null = null;

	export let gp: string = 'XXVIII';
	export let voteResult: VoteResult | null;
	export let supplyDate: Date | null = null;
	if (voteResult) gp = voteResult.legislative_initiative.gp;
	let date = new Date();
	if (supplyDate) date = supplyDate;
	if (voteResult) date = voteResult.legislative_initiative.created_at;
	

	let seats = [18, 25, 29, 33, 37, 41];
	let circles2d: Bubble[][] = [];
	export let delegates: Delegate[] = [];
	let noSeats = false;

	function isPartyInFavor(party: string): boolean {
		const votes = voteResult?.votes.slice();
		if (!votes) {
			return false;
		}
		// this sort is there because of named votes -> it should only look at the one with the higher count (pro, contra)
		// otherwise, it could happen that (absent, or new) delegates are marked as e.g. contra delegates even though the majority of the party voted for the change
		votes.sort((a, b) => b.fraction - a.fraction);
		return votes.find((vote) => vote.party === party)?.infavor ?? false;
	}

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

	onMount(async () => {
		
	});

	const updateLayout = async () => {
		const allSeats = await cachedAllSeats();
		if (allSeats) {
			seats = getSeats(allSeats, gp)
		}

		// do not forget offset toggling
		circles2d = setupParliament(seats, width, height, 7.9);

		const fetchedDelegates = await delegates_with_seats_near_date(date, gp)
		if (fetchedDelegates) delegates = fetchedDelegates;

		// we do not have seat information, therefore we fetch them in a base format
		if (delegates.length == 0) {
			const fetchedDelegates = await delegates_at(date);
			if (fetchedDelegates) delegates = fetchedDelegates;
			noSeats = true;
		}

		if (noSeats) {
			let partyToDelegates = groupPartyDelegates(delegates);
			let all = 0;
			partyToDelegates.forEach((dels, _party) => {
				all += dels.length;
			});

			const partyToDelegatesArray = Array.from(partyToDelegates.entries());
			partyToDelegatesArray.sort((a, b) => {
				const aInfavor = partyInfavorMap.get(a[0]);
				const bInfavor = partyInfavorMap.get(b[0]);
				if (aInfavor == bInfavor) {
					return (b[1].length - a[1].length) * orderingFactor;
				} else if (aInfavor == true && bInfavor == false) {
					return -1;
				} else {
					return 1;
				}
				// return b[1].length - a[1].length
			});

			setSeatsOfDels(partyToDelegatesArray, all, seats.slice());
		}
		enrichParliamentBubbles(circles2d, delegates, voteResult, setOpacity);
		circles2d = circles2d;
	}
	
	$: if (delegate && delegate.seat_row != null && circles2d.length >= 1) {
		select(circles2d[delegate.seat_row - 1][delegate.seat_col! - 1], null);
	}

	$: if (gp || date) {
		updateLayout();
	}

</script>

<BaseParliament class={clazz} {circles2d} {selected} {preview} {select} {width} {height} />
<!-- 
{#if gp === currentLegisInit && !enforceBase}
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
{/if} -->
