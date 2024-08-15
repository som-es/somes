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

		if (bubble.del == null) {
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
		if (bubble.del == null) {
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
		// circles2d = setupParliament(seats, width, height, 7.9);
		const allLegisPeriods = await cachedAllLegisPeriods();
		if (allLegisPeriods !== null && allLegisPeriods.length > 0) {
			currentLegisInit = allLegisPeriods[0].gp;
		}
	});

	let circlesPerParty: Bubble[][]

	$: {
		let partyToDelegates = new Map<string, Delegate[]>();
		delsAtDate.forEach(del => {
			if (del.party == null) {
				return;
			}
			if (!partyToDelegates.has(del.party)) {
				partyToDelegates.set(del.party, []);
			}
			const currentDels = partyToDelegates.get(del.party);
			currentDels?.push(del)
		});
		let rowIdx = 0;
		partyToDelegates.forEach((dels, _party) => {
			dels.forEach((del, colIdx) => {
				const bubble = {
					r: 7.9,
					x: 0,
					y: 0,
					del: null,
					color: 'rgb(196, 180, 189)',
					opacity: 0.0,
					title: null,
					namedVote: null
				};
				circlesPerParty[rowIdx].push(bubble)

				del.seat_row = rowIdx + 1;
				del.seat_col = colIdx + 1;
				setDelOnBubble(del, circlesPerParty, partyToColor);
				setOpacity(bubble);
			})
			rowIdx += 1;
		});
		enrichCirclesWithSpeechInfoOnSeat(voteResult.speeches, circlesPerParty, delsAtDate);
		if (voteResult.named_votes) {
			enrichCirclesWithNamedVoteInfoOnSeat(voteResult.named_votes.named_votes, circlesPerParty, delsAtDate);
		}
	}

	$: if (delegate && delegate.seat_row != null)
		select(circles2d[delegate.seat_row - 1][delegate.seat_col! - 1], null);
</script>

{#if voteResult.legislative_initiative.gp === currentLegisInit}
	<BaseParliament class={clazz} {circles2d} {selected} {preview} {select} {width} {height} />
{:else}
	Sitzplan nicht verfügbar
{/if}
