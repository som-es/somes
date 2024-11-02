<!-- TODO: merge this and the Parliament component in to one -->
<script lang="ts">
	import {
		setupParliament,
		type Bubble	} from '$lib/parliament';
	import { getPartyColors } from '$lib/partyColor';
	import type { Delegate, VoteResult } from '$lib/types';
	import { onMount } from 'svelte';
	import { delegates_at, delegates_with_seats_near_date } from '$lib/api';
	import { groupPartyDelegates, setSeatsOfDels } from '$lib/parliaments/defaultParliament';
	import { cachedAllSeats, getSeats } from '$lib/caching/seats';
	import DataParliament from './DataParliament.svelte';
	import { createPartyInfavorMap, isPartyInFavor } from '$lib/partyInfavor';

	const width = 830;
	const height = 900;

	let clazz = '';
	export { clazz as class };
	
	export let orderingFactor: number = 1;
	export let preview: boolean = false;
	export let againstOpacity: number = 0.16;

	export let delegate: Delegate | null = null;
	export let selected: Bubble | null = null;

	export let gp: string = 'XXVIII';
	export let voteResult: VoteResult | null;
	export let supplyDate: Date | null = null;
	if (voteResult) gp = voteResult.legislative_initiative.gp;
	let date = new Date();
	if (supplyDate) date = supplyDate;
	if (voteResult) date = voteResult.legislative_initiative.created_at;
	

	let seats = [18, 25, 29, 33, 37, 41];
	export let delegates: Delegate[] = [];
	let noSeats = false;

	onMount(async () => {
		
	});

	const updateLayout = async () => {
		const allSeats = await cachedAllSeats();
		if (allSeats) {
			seats = getSeats(allSeats, gp)
		}

		const fetchedDelegates = await delegates_with_seats_near_date(date, gp)
		if (fetchedDelegates) delegates = fetchedDelegates;

		// we do not have seat information, therefore we fetch them in a base format
		if (delegates.length == 0) {
			const fetchedDelegates = await delegates_at(date);
			if (fetchedDelegates) delegates = fetchedDelegates;
			noSeats = true;
		}

		if (noSeats) {
			let partyToDelegates = groupPartyDelegates(delegates);
			let all = 0;
			partyToDelegates.forEach((dels, _party) => {
				all += dels.length;
			});

			const partyInfavorMap = createPartyInfavorMap(voteResult);

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
				// return b[1].length - a[1].length
			});

			setSeatsOfDels(partyToDelegatesArray, all, seats.slice());
		}
	}
	

	$: if (gp || date) {
		updateLayout();
	}

</script>

<DataParliament 
	bind:delegate 
	bind:selected 
	{againstOpacity} 
	class={clazz} 
	{delegates} 
	{preview} 
	{width} 
	{height} 
	{voteResult} 
	{seats}
/>
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
