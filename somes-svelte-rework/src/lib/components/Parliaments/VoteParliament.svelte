<!-- TODO: merge this and the Parliament component in to one -->
<script lang="ts">
	import { cachedAllLegisPeriods } from '$lib/caching/legis_periods';
	import {
		setupParliament,
		type Bubble,
		setDelOnBubble,
		enrichCirclesWithNamedVoteInfoOnSeat,
		enrichCirclesWithSpeechInfoOnSeat
	} from '$lib/parliament';
	import { getPartyColors, partyToColor } from '$lib/partyColor';
	import type { Delegate, LegisPeriod, VoteResult } from '$lib/types';
	import { onMount } from 'svelte';
	import BaseParliament from './BaseParliament.svelte';
	import { delegates_at } from '$lib/api';

	export let seats: number[] = [20, 27, 37, 43, 48, 54];
	export let dels: Delegate[];
	export let delsAtDate: Delegate[] = [];
	export let preview: boolean = false;
	export let voteResult: VoteResult;
	export let delegate: Delegate | null = null;

	let clazz = '';
	export { clazz as class };

	const width = 830;
	const height = 900;

	function isPartyInFavor(party: string): boolean {
		const votes = voteResult.votes.slice();
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
			bubble.opacity = partyInfavorMap.get(bubble.del.party) ? 1 : 0.16;
			return;
		}

		bubble.opacity = 1;
	}

	dels.forEach((del) => {
		setDelOnBubble(del, circles2d, partyToColor);

		if (del.seat_col != null && del.seat_row != null) {
			setOpacity(circles2d[del.seat_row - 1][del.seat_col - 1]);
		}
	});

	enrichCirclesWithSpeechInfoOnSeat(voteResult.speeches, circles2d, dels);
	if (voteResult.named_votes) {
		enrichCirclesWithNamedVoteInfoOnSeat(voteResult.named_votes.named_votes, circles2d, dels);
	}

	// for (let r = 0; r < circles2d.length; r++) {
	// 	for (let c = 0; c < circles2d[r].length; c++) {
	// 		if (circles2d[r][c].del == null) {
	// 			circles2d[r][c].opacity = 0.0;
	// 		}
	// 	}
	// }

	let currentLegisInit = 'XXVII';
	onMount(async () => {
		const fetchedDelsAtDate = await delegates_at(voteResult.legislative_initiative.created_at);
		if (fetchedDelsAtDate) delsAtDate = fetchedDelsAtDate;
		// circles2d = setupParliament(seats, width, height, 7.9);
		const allLegisPeriods = await cachedAllLegisPeriods();
		if (allLegisPeriods !== null && allLegisPeriods.length > 0) {
			currentLegisInit = allLegisPeriods[0].gp;
		}
	});

	const defaultSeats = [18, 25, 29, 33, 37, 41];
	let circlesPerParty2: Bubble[][] = setupParliament(defaultSeats, width, height, 7.9, false);
	$: {
		let partyToDelegates = new Map<string, Delegate[]>();
		delsAtDate.forEach((del) => {
			del.seat_row = null;
			del.seat_col = null;
			if (del.party == null || del.council != 'nr') {
				return;
			}

			if (!partyToDelegates.has(del.party)) {
				partyToDelegates.set(del.party, []);
			}
			const currentDels = partyToDelegates.get(del.party);
			currentDels?.push(del);
		});
		let all = 0;
		partyToDelegates.forEach((dels, _party) => {
			all += dels.length;
		});

		const startIdxs = [0, 0, 0, 0, 0, 0];

		const partyToDelegatesArray = Array.from(partyToDelegates.entries());
		partyToDelegatesArray.sort((a, b) => b[1].length - a[1].length);

		partyToDelegatesArray.forEach(([party, dels], g) => {
			const fraction = dels.length;
			const share = fraction / all;
			let restSeats = 0;
			let startDelegateIdx = 0;
			defaultSeats.forEach((seats, r) => {
				let realSeats = Math.round(seats * share);
				console.log(realSeats);
				restSeats += seats * share - realSeats;

				if (realSeats + startIdxs[r] >= seats) {
					restSeats += realSeats + startIdxs[r] - seats;
					realSeats = seats - startIdxs[r];
				}
				const useDels = dels.slice(startDelegateIdx, startDelegateIdx + realSeats);

				useDels.forEach((del, c) => {
					del.seat_row = r + 1;
					del.seat_col = c + startIdxs[r] + 1;
				});

				startDelegateIdx += realSeats;
				startIdxs[r] += realSeats;
			});

			restSeats = Math.round(restSeats);
			let row = defaultSeats.length - 1;
			while (true) {
				const seats = defaultSeats[row];

				if (restSeats <= 0) {
					break;
				}
				if (startIdxs[row] + 1 > seats) {
					row -= 1;
					continue;
				}

				const del = dels[startDelegateIdx];
				if (del == null) break;
				del.seat_row = row + 1;
				del.seat_col = startIdxs[row] + 1;

				startDelegateIdx += 1;
				restSeats -= 1;
				startIdxs[row] += 1;
				row -= 1;
				// if (row >= defaultSeats.length) row = 0;
				if (row <= 0) row = defaultSeats.length - 1;
			}
			console.log(` ${party} ${restSeats}`);
		});

		if (circlesPerParty2.length > 0)
			delsAtDate.forEach((del) => {
				setDelOnBubble(del, circlesPerParty2, partyToColor);

				if (del.seat_col != null && del.seat_row != null) {
					setOpacity(circlesPerParty2[del.seat_row - 1][del.seat_col - 1]);
				}
			});
		enrichCirclesWithSpeechInfoOnSeat(voteResult.speeches, circlesPerParty2, delsAtDate);
		if (voteResult.named_votes) {
			enrichCirclesWithNamedVoteInfoOnSeat(
				voteResult.named_votes.named_votes,
				circlesPerParty2,
				delsAtDate
			);
		}
		circlesPerParty2 = circlesPerParty2;
	}

	$: if (delegate && delegate.seat_row != null) {
		const circleArray =
			voteResult.legislative_initiative.gp === currentLegisInit ? circles2d : circlesPerParty2;
		select(circleArray[delegate.seat_row - 1][delegate.seat_col! - 1], null);
	}
</script>

{#if voteResult.legislative_initiative.gp === currentLegisInit}
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
