<script lang="ts">
	import { cachedAllLegisPeriods } from '$lib/caching/legis_periods';
	import { legisDurationString } from '$lib/date';
	import type { LegisPeriod } from '$lib/types';
	import { onMount } from 'svelte';

	export let periods: LegisPeriod[] = [];
	onMount(async () => {
		const fetchedPeriods = await cachedAllLegisPeriods();
		if (fetchedPeriods != undefined) {
			periods = fetchedPeriods.reverse();
		}
	});

	export let showAllButton = true;
	export let selectedPeriod = 'all';
</script>

<!-- (add: Sort asc and desc) -->
<div class="flex flex-wrap flex-row text-black">
	{#each periods as period, i}
		<button
			class:bg-tertiary-500={period.gp == selectedPeriod}
			on:click={() => (selectedPeriod = period.gp)}
			title={legisDurationString(period, periods[i + 1])}
			class="btn bg-primary-300"
			style="margin: 3px;"
		>
			{period.gp}
		</button>
	{/each}
	{#if showAllButton}
		<button
			class:bg-tertiary-500={selectedPeriod == 'all'}
			on:click={() => (selectedPeriod = 'all')}
			class="btn bg-primary-300"
			style="margin: 3px;"
		>
			Alle
		</button>
	{/if}
</div>

<style>
</style>
