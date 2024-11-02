<!-- TODO: merge this and the Parliament component in to one -->
<script lang="ts">
	import {
		type Bubble,
		enrichParliamentBubbles
	} from '$lib/parliament';
	import { getPartyColors } from '$lib/partyColor';
	import type { Delegate, VoteResult } from '$lib/types';
	import { onMount } from 'svelte';
	import BaseParliament from './BaseParliament.svelte';
	
	export let width = 830;
	export let height = 900;

	let clazz = '';
	export { clazz as class };
	
	export let preview: boolean = false;
	export let againstOpacity: number = 0.16;

	export let delegate: Delegate | null = null;
	export let selected: Bubble | null = null;
	
	export let delegates: Delegate[];

	export let gp: string = 'XXVIII';
	export let voteResult: VoteResult | null;
	export let supplyDate: Date | null = null;
	if (voteResult) gp = voteResult.legislative_initiative.gp;
	let date = new Date();
	if (supplyDate) date = supplyDate;
	if (voteResult) date = voteResult.legislative_initiative.created_at;
	
	let circles2d: Bubble[][] = [];

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
		enrichParliamentBubbles(circles2d, delegates, voteResult, setOpacity);
		circles2d = circles2d;
	}
	
	$: if (delegate && delegate.seat_row != null && circles2d.length >= 1) {
		select(circles2d[delegate.seat_row - 1][delegate.seat_col! - 1], null);
	}

	$: if (gp || date || delegates) {
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
