<script lang="ts">
	import { setDelOnBubble, type Bubble, setupParliament } from "$lib/parliament";
	import type { Delegate, InterestShare } from "$lib/types";
	import { get, type Readable } from "svelte/store";
	import { localStorageStore } from "@skeletonlabs/skeleton";
	import { partyToColorFn } from "$lib/getPartyToColor";
	import DelegateCard from "./DelegateCard.svelte";
	import { topicColors } from "$lib/interest-colors";
	import { delegate_interests } from "$lib/api";

	export let seats: number[];
	export let dels: Delegate[];

	const width = 830;
	const height = 900;


	let selected: Bubble;
	let circles2d: Bubble[][] = setupParliament(seats, width, height, 6.9);

	async function fromLocalStorage() {
		const currentDelegateStorage: Readable<Bubble | null> = localStorageStore(
			"currentDelegate",
			null,
		);
		const bubble = get(currentDelegateStorage);

		if (
			bubble == null ||
			bubble.del == null ||
			bubble.del.seat_row == null ||
			bubble.del.seat_col == null
		)
			return;

		await select(circles2d[bubble.del.seat_row - 1][bubble.del.seat_col - 1], null);
	}

	let interests: InterestShare[];

	async function select(bubble: Bubble, event: MouseEvent | KeyboardEvent | null) {
		if (bubble.del == null) return;

		if (selected != null) {
			selected.r = 6;
		}
		bubble.r = +10.9;
		circles2d = circles2d; // TODO: remove this evil pointer hack without breaking everything

		if (event != null) {
			event.stopPropagation();
		}

		interests = await delegate_interests(bubble.del.id);
		interests.sort((a, b) => b.self_share - a.self_share)
		selected = bubble;
	}

	dels.forEach((del) => {
		setDelOnBubble(del, circles2d, partyToColorFn);
	});

	// $: ; 

	fromLocalStorage();
</script>
<div class="grid-container">
	<div class="parliament-item grid-tile bg-white">
		<div class="grid-tile-content">
			<svg viewBox="0 0 {width} {height * 0.5 + 60}">
				{#each circles2d.flat(1) as circle, i}
					<circle
						type="button"
						cx={circle.x}
						cy={circle.y}
						r={circle.r}
						fill={circle.color}
						fill-opacity={circle.color == "rgb(196, 180, 189)" && circle.del == null ? 0.2 : 1}
						on:click={(event) => select(circle, event)}
						on:keypress={(event) => select(circle, event)}
						role="button"
						tabindex={100 + i}
					/>
				{/each}
			</svg>
		</div>
	</div>
	<div class="delegate-item grid-tile bg-slate-600">
		<div class="self-center m-auto">
			{#if selected && selected.del}
				<DelegateCard delegate={selected.del} />
			{/if}

		</div>
	</div>
	{#if interests}

		{#if interests.length >= 1}
		<div class="i1 interest-tile grid-tile bg-slate-600" style = "border: 1px solid {topicColors.get(interests[0].topic)};">
			<div class = "grid-tile-content">
				{interests[0].topic}
			</div>
		</div>
		{/if}
		{#if interests.length >= 2}
		<div class="i2 interest-tile grid-tile bg-slate-600" style = "border: 1px solid {topicColors.get(interests[1].topic)};">
			{interests[1].topic}
		</div>
		{/if}
		{#if interests.length >= 3}
		<div class="i3 interest-tile grid-tile bg-slate-600" style = "border: 1px solid {topicColors.get(interests[2].topic)};">
			{interests[2].topic}
		</div>
		{/if}
	{/if}
	<div class="activity-item interest-tile grid-tile bg-slate-600">
		<div class="">
			aktivität
		</div>
	</div>
	
	<!--<section class="grid-tile">
		<div class="grid-tile-content">
		</div>
	</section> -->
</div>


<div class="flex">
	<div class="self-center m-auto">
		{#if selected && selected.del}
			<DelegateCard delegate={selected.del} />
		{/if}
	</div>
	<div class="self-center m-auto w-8/12">
		<svg viewBox="0 0 {width} {height * 0.5 + 60}">
			{#each circles2d.flat(1) as circle, i}
				<circle
					type="button"
					cx={circle.x}
					cy={circle.y}
					r={circle.r}
					fill={circle.color}
					fill-opacity={circle.color == "rgb(196, 180, 189)" && circle.del == null ? 0.2 : 1}
					on:click={(event) => select(circle, event)}
					on:keypress={(event) => select(circle, event)}
					role="button"
					tabindex={100 + i}
				/>
			{/each}
		</svg>
		<div>
			<h2>Interessen</h2>
		</div>
	</div>
</div>

<style>
	* {
		box-sizing: border-box;
	}

.grid-container {
  display: grid;
  gap: 20px;
  grid-template-areas: 
  	"p p p p p d d"
	"p p p p p d d"
	"p p p p p d d"
	"i1 i2 i3 activity . . .";
	/* "i i i a"; */
  padding: 10px;
}

.grid-container > div {
	text-align: center;
	padding: 20px 0;
	font-size: 30px;
}

.parliament-item {
	grid-area: p;
}

.delegate-item {
	grid-area: d;
}

.activity-item {
	grid-area: activity;
}

.interest-tile {
	height: 300px;
	min-width: 100%;
}

.i1 {
	grid-area: i1;
}
.i2 {
	grid-area: i2;
}
.i3 {
	grid-area: i3;
}

.grid-item {
    border: 1px solid rgba(0, 0, 0, 0.8);
    padding: 20px;
    font-size: 30px;
    text-align: center;
}
.grid-tile-2-col-2-row {
    grid-column: auto/span 2;
    grid-row: auto/span 2;
}
.grid-tile {
    box-sizing: border-box;
    padding: 0;
    border-radius: 25px;
    z-index: 1;
    background: rgb(var(--color-primary-500));
}
.section-sizes,
.grid-tile-content {
    display: flex;
    justify-content: center;
    flex-direction: column;
    align-items: center;
}

</style>
