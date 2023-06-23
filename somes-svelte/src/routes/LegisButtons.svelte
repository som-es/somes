<script lang="ts">
    export let updateFn: Function;
    const periods = ["XX", "XXI", "XXII", "XXIII", "XXIV", "XXV", "XXVI", "XXVII"];

    let selectedPeriod = "all";

</script>
(add: Sort asc and desc)
<div class="flex flex-wrap flex-row">
    {#each periods as period}
        <button class:bg-primary-200={period==selectedPeriod} on:click={() => selectedPeriod = period} class="btn bg-primary-300" style="margin: 3px;" on:click={() => updateFn(period)}>{period}</button>
    {/each}
    <button class:bg-primary-200={selectedPeriod=="all"} on:click={() => selectedPeriod = "all"} class="btn bg-primary-300" style="margin: 3px;" on:click={() => updateFn("all")}>All time</button>
</div>

<style>

</style>