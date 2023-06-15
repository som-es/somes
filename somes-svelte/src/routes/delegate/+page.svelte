<script lang="ts">
	import Halfcircle from "../Halfcircle.svelte";
    import Parliament from "../Parliament.svelte"
    import Parliament2 from "../Parliament2.svelte"
    import { delegates } from '$lib/api';
	import { onMount } from "svelte";
	import type { Delegate } from "$lib/types";

    let dels: Delegate[];
    
    onMount(async function () {
        const austrianDelegates = await delegates();
        // use local storage to cache the delegates
        dels = austrianDelegates.filter(delegate => delegate.council === "nr");        
    });

</script>
<!-- <div> -->
    <!-- <Halfcircle n={17} r={70 * 1.5} /> -->
    <!-- <Halfcircle n={26} r={100 * 1.5} /> -->
    <!-- <Halfcircle n={32} r={130 * 1.5} /> -->
    <!-- <Halfcircle n={37} r={150. * 1.5} /> -->
    <!-- <Halfcircle n={40} r={170. * 1.5} /> -->
    <!-- <Halfcircle n={32} r={190. * 1.5} /> -->

    {#if dels}
        <!-- <Parliament seats={[17, 26, 32, 37, 40, 32]} /> -->
        <Parliament2 dels={dels} seats={[20, 27, 37, 43, 48, 54]} />

    {:else}
        <p class="loading">loading...</p>
    {/if}

<!-- </div> -->

<style>

</style>
