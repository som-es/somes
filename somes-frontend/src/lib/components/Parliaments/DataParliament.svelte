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
	import { VOTE_COLORS, type SeatColorMode } from '$lib/voteColors';

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
		searchValue?: string;
		maxAngle?: number;
		yOffset?: number;
		colorMode?: SeatColorMode;
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
		localPartyColors = partyColors,
		searchValue = '',
		maxAngle = 180,
		yOffset = 0,
		colorMode = 'party'
	}: Props = $props();

	let partyInfavorMap = $derived(createPartyInfavorMap(voteResult, localPartyColors));

	let circles2d = $derived.by(() => {
		void delegates;
		void voteResult;
		void searchValue;
		void colorMode;

		function partyToColor(party: string | null): string {
			if (party == null) {
				return '#B8B8B8';
			}

			const color = localPartyColors.get(party);
			if (color == null) {
				return '#B8B8B8';
			}

			return color;
		}

		return untrack(() => {
			const bubbles = setupParliament(seats, width, height, 7.9, useOffset, maxAngle, yOffset);
			enrichParliamentBubbles(
				bubbles,
				$state.snapshot(delegates),
				voteResult,
				setOpacity,
				partyToColor
			);
			if (colorMode === 'vote') {
				bubbles.flat().forEach(applyVoteColor);
			}
			return bubbles;
		});
	});

	/**
	 * Recolors a seat by the delegate's vote instead of their party:
	 * green = in favor, red = against, blue = abstention, gray = absent / no vote.
	 * All seats get the same radius so only the color carries information.
	 * Falls back to the party's voting behaviour when no named vote exists.
	 */
	/** Uniform seat radius in vote mode (like the EP display, where every seat has the same size) */
	const VOTE_MODE_RADIUS = 9.9;

	function applyVoteColor(bubble: Bubble) {
		if (bubble.del == null) return;
		if (bubble.del.council == 'gov') return;
		bubble.r = VOTE_MODE_RADIUS;

		const search = searchValue.trim().toLowerCase();
		const matchesSearch = search.length == 0 || bubble.del.name.toLowerCase().includes(search);

		let voteColor: string | null = null;
		if (bubble.namedVote) {
			if (bubble.namedVote.was_absent) {
				voteColor = null;
			} else if (bubble.namedVote.was_abstention) {
				voteColor = VOTE_COLORS.abstention;
			} else {
				voteColor = bubble.namedVote.infavor ? VOTE_COLORS.infavor : VOTE_COLORS.against;
			}
		} else if (voteResult?.named_votes == null && partyInfavorMap.has(bubble.del.party)) {
			voteColor = partyInfavorMap.get(bubble.del.party) ? VOTE_COLORS.infavor : VOTE_COLORS.against;
		}

		if (voteColor == null) {
			bubble.color = VOTE_COLORS.absent;
			bubble.opacity = matchesSearch ? 0.35 : 0.1;
			return;
		}

		bubble.color = voteColor;
		bubble.opacity = matchesSearch ? 1 : 0.2;
	}

	function select(
		bubble: Bubble,
		event: MouseEvent | KeyboardEvent | null,
		updateDelegate: boolean = true
	) {
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

		if (searchValue.trim().length > 0) {
			if (bubble.del.name.toLowerCase().includes(searchValue.trim().toLowerCase())) {
				bubble.opacity = 1;
			} else {
				bubble.opacity = 0.2;
			}
			return;
		}

		if (bubble.namedVote && !bubble.namedVote.was_absent) {
			bubble.opacity = bubble.namedVote.infavor ? 1.0 : 0.2;
			return;
		}

		if (bubble.speech && bubble.speech.speech.infavor != null) {
			bubble.opacity = bubble.speech.speech.infavor ? 1.0 : 0.2;
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
		const newDel = delegates.find((del) => del.id == delegate?.id) ?? delegate;
		if (newDel && newDel.seat_row != null && circles2d.length >= 1) {
			select(circles2d[newDel.seat_row - 1][newDel.seat_col! - 1], null, false);
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
	<BaseParliament
		class={className}
		{circles2d}
		{selected}
		{preview}
		{select}
		{width}
		{height}
		{forceColor}
	/>
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
