<script lang="ts">
	import { cachedAllLegisPeriods } from "$lib/caching/legis_periods";
	import { onMount } from "svelte";

	// export let updateFn: Function;
	let periods = ["XX", "XXI", "XXII", "XXIII", "XXIV", "XXV", "XXVI", "XXVII"];
    onMount(async () => {
        const fetchedPeriods = (await cachedAllLegisPeriods())?.map((gp) => gp.gp);
        if (fetchedPeriods != undefined) {
            periods = fetchedPeriods.reverse();
        }
    });

	export let selectedPeriod = "all";
</script>

(add: Sort asc and desc)
<div class="flex flex-wrap flex-row">
	{#each periods as period}
		<button
			class:bg-tertiary-300={period == selectedPeriod}
			on:click={() => (selectedPeriod = period)}
			class="btn bg-primary-300"
			style="margin: 3px;"
        >
            {period}
        </button>
	{/each}
	<button
		class:bg-tertiary-300={selectedPeriod == "all"}
		on:click={() => (selectedPeriod = "all")}
		class="btn bg-primary-300"
		style="margin: 3px;"
    >
        Alle
    </button>
</div>

<style>
</style>
