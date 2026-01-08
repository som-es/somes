<script lang="ts">
	import { type Bubble } from '$lib/parliament';
	import type { Delegate, VoteResult } from '$lib/types';
	import { onMount, untrack } from 'svelte';
	import { groupPartyDelegates, setSeatsOfDels } from '$lib/parliaments/defaultParliament';
	import { cachedAllSeats, getSeats } from '$lib/caching/seats';
	import DataParliament from './DataParliament.svelte';
	import { createPartyInfavorMap } from '$lib/partyInfavor';
	import { partyColors } from '$lib/partyColor';
	import { cachedPartyColors } from '$lib/caching/party_color';
	import { fetchDelegates } from '$lib/api/fetch_delegates';
	import { toActualDateString } from '$lib/api/api';

	const width = 830;
	const height = 900;

	let {
		class: clazz = " ",
		orderingFactor = 1,
		preview = false,
		againstOpacity = 0.16,
		delegate = $bindable(),
		selected = $bindable(),
		gp = $bindable('XXVIII'),
		voteResult = null,
		supplyDate = null,
		circles2d = $bindable([]),
		showGovs = false,
		enforceSvg = false,
		forceColor = null,
		delegates = $bindable([]),
		govOfficials = $bindable([]),
		overrideDelegates = false,
		noSeats = $bindable(false),
		useOffset = $bindable(true),
		show3D = false,
		syncDelegates = $bindable([]),
		allSeats = null 
	}: {
		class?: string,
		orderingFactor?: number,
		preview?: boolean,
		againstOpacity?: number,
		delegate?: Delegate | null,
		selected?: Bubble | null,
		gp?: string,
		voteResult?: VoteResult | null,
		supplyDate?: Date | null,
		circles2d?: Bubble[][],
		showGovs?: boolean,
		enforceSvg?: boolean,
		forceColor?: string | null,
		delegates?: Delegate[],
		govOfficials?: Delegate[],
		overrideDelegates?: boolean,
		noSeats?: boolean,
		useOffset?: boolean,
		show3D?: boolean,
		syncDelegates?: Delegate[],
		allSeats?: Map<string, number[]> | null
	} = $props();

	const effectiveGp = $derived(voteResult?.legislative_initiative.gp ?? gp);

	let seats: number[] = $derived.by(() => {
		if (allSeats) {
			if (noSeats) {
				return getSeats(allSeats, 'XX', true);
			} else {
				return getSeats(allSeats, effectiveGp);
			}
		} else {
			return []
		}
	});

	let localPartyColors: Map<string, string> = $state(partyColors);

	const activeDate = $derived.by(() => {
		if (supplyDate) {
			return supplyDate
		};
		if (voteResult && voteResult.legislative_initiative.nr_plenary_activity_date) {
			return new Date(voteResult.legislative_initiative.nr_plenary_activity_date)
		};
		return new Date();
	});

	onMount(async () => {
		if (allSeats == null) {
			allSeats = await cachedAllSeats();
		}
		if (localPartyColors.size == 0) {
			localPartyColors = await cachedPartyColors();
		}
		await updateDelegates();
	});

	let displayDelegates = $derived.by(() => {
		if (delegates.length == 0) {
			return [];
		}
		if (!noSeats) {
			return delegates
		}

		let newDelegates = structuredClone($state.snapshot(delegates));
		let partyToDelegates = groupPartyDelegates(newDelegates);
		let all = 0;
		partyToDelegates.forEach((dels, _party) => {
			all += dels.length;
		});

		const partyInfavorMap = createPartyInfavorMap(voteResult, partyColors);

		const partyToDelegatesArray = Array.from(partyToDelegates.entries());
		partyToDelegatesArray.sort((a, b) => {
			const aInfavor = partyInfavorMap.get(a[0]);
			const bInfavor = partyInfavorMap.get(b[0]);
			if (aInfavor == bInfavor) {
				return (b[1].length - a[1].length) * orderingFactor;
			} else if (aInfavor == true && bInfavor == false) {
				return -1;
			} else {
				return 1;
			}
		});

		setSeatsOfDels(partyToDelegatesArray, all, seats.slice());

		return newDelegates
	});

	$effect(() => {
		void displayDelegates;
		untrack(() => {
			syncDelegates = displayDelegates;
		});
	});


	const updateDelegates = async () => {
		const dateStr = toActualDateString(activeDate);
		
		if (!overrideDelegates && delegates.length == 0) {
			const { hasSeatInfo, delegates: fetchedDelegates } = await fetchDelegates(dateStr, effectiveGp)
			noSeats = !hasSeatInfo;
			useOffset = hasSeatInfo;

			if (fetchedDelegates) {
				delegates = fetchedDelegates;
			}
		}
	};
	
</script>

{#if displayDelegates.length > 0}
	<DataParliament
		bind:delegate
		bind:selected
		{againstOpacity}
		class={clazz}
		delegates={displayDelegates}
		{preview}
		{width}
		{height}
		{voteResult}
		{seats}
		{useOffset}
		{show3D}
		{enforceSvg}
		{localPartyColors}
		{forceColor}
	/>
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
