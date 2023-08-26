<!-- TODO: merge this and the Parliament component in to one -->
<script lang="ts">
	import { getPartyToColor } from "$lib/getPartyToColor";
	import { setupParliament, type Bubble, setDelOnBubble } from "$lib/parliament";
	import type { Delegate, VoteResult } from "$lib/types";

	export let seats: number[];
	export let dels: Delegate[];
	export let voteResult: VoteResult;
	export let preview: boolean = false;

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
		console.log(selected);
	}

	const partyToColorMap = getPartyToColor();

	function partyToColor(party: string): string {
		const color = partyToColorMap.get(party);

		if (color == null) {
			return "#B8B8B8";
		}

		return color;
	}

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

		circles2d[del.seat_row - 1][del.seat_col - 1].opacity = speech.infavor ? 1.0 : 0.2;
		circles2d[del.seat_row - 1][del.seat_col - 1].r = +10.9;
	});
</script>

<div style="pointer-events: {preview ? 'none' : 'auto'}">
	<svg viewBox="0 0 {width} {height * 0.5 + 60}" style="max-width: 100%;">
		{#each circles2d.flat(1) as circle, i}
			<circle
				class="translated-circle"
				type="button"
				cx={circle.x}
				cy={circle.y}
				r={circle.r}
				role="button"
				on:click={(event) => select(circle, event)}
				on:keypress={(event) => select(circle, event)}
				fill={circle.color}
				fill-opacity={circle.opacity}
				tabindex={100 + i}
				stroke={circle == selected ? "orange" : ""}
				stroke-width={circle == selected ? "4" : ""}
			/>
		{/each}
	</svg>
</div>
