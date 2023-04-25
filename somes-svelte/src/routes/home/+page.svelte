<script lang="ts">

	import AutoSelectParliament from "../AutoSelectParliament.svelte";
	import type { Delegate } from "$lib/types";
	import { delegates, latest_legis_inits_and_votes } from "$lib/api";
	import Parliament2 from "../Parliament2.svelte";
    import { onMount } from "svelte";

    let dels: Delegate[];
    
    onMount(async function () {
        const austrianDelegates = await delegates();
        dels = austrianDelegates.filter(delegate => delegate.council === "nr");

        const legisInits = await latest_legis_inits_and_votes();
        console.log(legisInits);
    });

</script>

<div class="container mx-auto">
    <h1 class="text-primary-400">Welcome back!</h1>
    <h2 class="mt-5">Nationalrat</h2>
    Current news from the Austrian parliament

    <h2 class="mt-5">Representatives</h2>

    {#if dels}
        <AutoSelectParliament dels={dels} seats={[20, 27, 37, 43, 48, 54]}/>
    {:else}
        <p class="loading">loading...</p>
    {/if}
</div>

<style>

</style>
