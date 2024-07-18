
<script lang="ts">
	import { setupParliament, type Bubble, setDelOnBubble } from "$lib/parliament";
	import { getPartyColors, partyToColor } from "$lib/partyColor";
	import type { Delegate, VoteResult } from "$lib/types";
	import BaseParliament from "./BaseParliament.svelte";

	export let seats: number[] = [20, 27, 37, 43, 48, 54];
	export let dels: Delegate[];
	export let preview: boolean = false;
    export let delegate: Delegate | null;

	const width = 830;
	const height = 900;

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
        delegate = bubble.del;
		console.log(delegate);
	}

	function setOpacity(bubble: Bubble) {
		if (bubble.del == null) {
			bubble.opacity = 0.2;
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
</script>

<BaseParliament {circles2d} {selected} {preview} {select} {width} {height} />
