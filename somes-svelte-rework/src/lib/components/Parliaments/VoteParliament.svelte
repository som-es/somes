<!-- TODO: merge this and the Parliament component in to one -->
<script lang="ts">
	import { cachedAllLegisPeriods } from '$lib/caching/legis_periods';
	import { setupParliament, type Bubble, setDelOnBubble } from '$lib/parliament';
	import { getPartyColors, partyToColor } from '$lib/partyColor';
	import type { Delegate, LegisPeriod, VoteResult } from '$lib/types';
	import { onMount } from 'svelte';
	import BaseParliament from './BaseParliament.svelte';

	export let seats: number[] = [20, 27, 37, 43, 48, 54];
	export let dels: Delegate[];
	export let preview: boolean = false;
	export let voteResult: VoteResult;
	let clazz = "";
	export { clazz as class };

	const width = 830;
	const height = 900;

	function isPartyInFavor(party: string): boolean {
		return voteResult.votes.find((vote) => vote.party === party)?.infavor ?? false;
	}

	function findDelegateFromId(id: number): Delegate | undefined {
		return dels.find((del) => del.id === id);
	}

	let circles2d: Bubble[][] = setupParliament(seats, width, height, 6.9);
	let selected: Bubble;

	function select(bubble: Bubble, event: MouseEvent | KeyboardEvent | null) {
		if (event != null) {
			event.stopPropagation();
		}

		selected = bubble;
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

	voteResult.speeches.forEach((speech) => {
		let del = findDelegateFromId(speech.delegate_id);
		if (del == null || del.seat_col == null || del.seat_row == null) return;

		circles2d[del.seat_row - 1][del.seat_col - 1].title = speech.infavor ? `${del.name}: Dafür gesprochen` : `${del.name}: Dagegen gesprochen`;
		circles2d[del.seat_row - 1][del.seat_col - 1].opacity = speech.infavor ? 1.0 : 0.2;
		circles2d[del.seat_row - 1][del.seat_col - 1].r = +10.9;
	});
	
	if (voteResult.named_votes) {
		voteResult.named_votes.named_votes.forEach((namedVote) => {
			let del = findDelegateFromId(namedVote.delegate_id);
			if (del == null || del.seat_col == null || del.seat_row == null) return;
			if (namedVote.was_absent) {
				circles2d[del.seat_row - 1][del.seat_col - 1].r = +5.9;
				circles2d[del.seat_row - 1][del.seat_col - 1].title = `${del.name}: abwesend/keine Stimme abgegeben`;
				return
			}


			circles2d[del.seat_row - 1][del.seat_col - 1].title = namedVote.infavor ? `${del.name}: Ja` : `${del.name}: Nein`;
			circles2d[del.seat_row - 1][del.seat_col - 1].opacity = namedVote.infavor ? 1.0 : 0.2;
			circles2d[del.seat_row - 1][del.seat_col - 1].r = +9.9;
		});
	}

	// for (let r = 0; r < circles2d.length; r++) {
	// 	for (let c = 0; c < circles2d[r].length; c++) {
	// 		if (circles2d[r][c].del == null) {
	// 			circles2d[r][c].opacity = 0.0;
	// 		}
	// 	}
	// }
	

	let currentLegisInit = "XXVII";
	onMount(async () => {
		const allLegisPeriods = await cachedAllLegisPeriods();
		if (allLegisPeriods !== null && allLegisPeriods.length > 0) {
			currentLegisInit = allLegisPeriods[0].gp
		}
	});

</script>
{#if voteResult.legislative_initiative.gp === currentLegisInit}
	<BaseParliament class={clazz} {circles2d} {selected} {preview} {select} {width} {height} />
{:else}	
	Sitzplan nicht verfügbar
{/if}
