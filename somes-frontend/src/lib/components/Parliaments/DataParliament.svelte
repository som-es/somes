<script lang="ts">
	import { type Bubble, enrichParliamentBubbles, setupParliament } from '$lib/parliament';
	import { getPartyColors, partyColors } from '$lib/partyColor';
	import type { Delegate, VoteResult } from '$lib/types';
	import { onMount, untrack } from 'svelte';
	import BaseParliament from './BaseParliament.svelte';
	import { createPartyInfavorMap } from '$lib/partyInfavor';
	import SwitchBox from '../UI/SwitchBox.svelte';
	// import App3D from './3D/App3D.svelte';
	import GptCanvasParliament from './GptCanvasParliament.svelte';
	import { cachedPartyColors } from '$lib/caching/party_color';

	interface Props {
		width?: number;
		height?: number;
		class?: string;
		preview?: boolean;
		againstOpacity?: number;
		useOffset?: boolean;
		delegate?: Delegate | null;
		selected?: Bubble | null;
		delegates: Delegate[];
		seats: number[];
		voteResult: VoteResult | null;
		show3D?: boolean;
		enforceSvg?: boolean;
		forceColor?: string | null;
		localPartyColors?: Map<string, string>;
	}

	let {
		width = 830,
		height = 900,
		class: className = '',
		preview = false,
		againstOpacity = 0.16,
		useOffset = true,
		delegate = $bindable(),
		selected = $bindable(),
		delegates,
		seats,
		voteResult,
		show3D = false,
		enforceSvg = false,
		forceColor = null,
		localPartyColors = partyColors
	}: Props = $props();

	let partyInfavorMap = $derived(createPartyInfavorMap(voteResult, localPartyColors));

	let circles2d = $derived.by(() => {
		void delegates; 
		void voteResult;

		return untrack(() => {
			const bubbles = setupParliament(seats, width, height, 7.9, useOffset);
			enrichParliamentBubbles(bubbles, $state.snapshot(delegates), voteResult, setOpacity);
			return bubbles;
		});
	});

	function select(bubble: Bubble, event: MouseEvent | KeyboardEvent | null, updateDelegate: boolean = true) {
		if (event != null) {
			event.stopPropagation();
		}

		if (bubble == null || bubble.del == null) {
			return;
		}

		selected = bubble;
		if (updateDelegate) delegate = bubble.del;
	}

	function setOpacity(bubble: Bubble) {
		if (bubble == null || bubble.del == null) {
			bubble.opacity = 0.2;
			return;
		}

		if (partyInfavorMap.has(bubble.del.party)) {
			if (bubble.del.council == 'nr')
				bubble.opacity = partyInfavorMap.get(bubble.del.party) ? 1 : againstOpacity;
			else {
				bubble.opacity = 1;
			}

			return;
		}

		bubble.opacity = 1;
	}


	$effect(() => {
		if (delegate && delegate.seat_row != null && circles2d.length >= 1) {
			select(circles2d[delegate.seat_row - 1][delegate.seat_col! - 1], null, false);
		}
	});

	let checked = $state(false);
</script>

{#if show3D}
	<div class="flex justify-between">
		<div></div>
		<SwitchBox bind:checked />
	</div>
{/if}

{#if checked}
	<!-- <App3D {circles2d} {selected} {preview} {select} /> -->
{:else if preview && !enforceSvg}
	<GptCanvasParliament class={className} {circles2d} {width} {height} />
	<!-- <BaseParliament class={clazz} {circles2d} {selected} {preview} {select} {width} {height} /> -->
{:else}
	<BaseParliament class={className} {circles2d} {selected} {preview} {select} {width} {height} {forceColor} />
	<!-- <GptBaseParliament class={clazz} {circles2d} {selected} {preview} {select} {width} {height} /> -->
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
