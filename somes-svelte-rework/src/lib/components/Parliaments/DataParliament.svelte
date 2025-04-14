<!-- TODO: merge this and the Parliament component in to one -->
<script lang="ts">
	import { type Bubble, enrichParliamentBubbles, setupParliament } from '$lib/parliament';
	import { getPartyColors } from '$lib/partyColor';
	import type { Delegate, VoteResult } from '$lib/types';
	import { onMount } from 'svelte';
	import BaseParliament from './BaseParliament.svelte';
	import { createPartyInfavorMap } from '$lib/partyInfavor';
	import SwitchBox from '../UI/SwitchBox.svelte';
	import App3D from './3D/App3D.svelte';

	export let width = 830;
	export let height = 900;

	let clazz = '';
	export { clazz as class };

	export let preview: boolean = false;
	export let againstOpacity: number = 0.16;
	export let useOffset: boolean = true;

	export let delegate: Delegate | null = null;
	export let selected: Bubble | null = null;

	export let delegates: Delegate[];

	export let seats: number[];
	export let gp: string = 'XXVIII';
	export let voteResult: VoteResult | null;
	export let supplyDate: Date | null = null;
	export let show3D = false;

	if (voteResult) gp = voteResult.legislative_initiative.gp;
	let date = new Date();
	if (supplyDate) date = supplyDate;
	if (voteResult) date = voteResult.legislative_initiative.created_at;

	export let circles2d: Bubble[][];
	circles2d = setupParliament(seats, width, height, 7.9, useOffset);

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

	$: partyInfavorMap = createPartyInfavorMap(voteResult);

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

	onMount(async () => {});

	const updateLayout = async () => {
		circles2d = setupParliament(seats, width, height, 7.9, useOffset);
		enrichParliamentBubbles(circles2d, delegates, voteResult, setOpacity);
		circles2d = circles2d;
	};

	$: if (delegate && delegate.seat_row != null && circles2d.length >= 1) {
		select(circles2d[delegate.seat_row - 1][delegate.seat_col! - 1], null);
	}

	$: if (delegates || voteResult) {
		updateLayout();
	}

	$: checked = false;
</script>

{#if show3D}
	<div class="flex justify-between">
		<div></div>
		<SwitchBox bind:checked />
	</div>
{/if}

{#if checked}
	<App3D {circles2d} {selected} {preview} {select} />
{:else}
	<BaseParliament class={clazz} {circles2d} {selected} {preview} {select} {width} {height} />
{/if}
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
