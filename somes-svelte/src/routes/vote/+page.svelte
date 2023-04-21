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
        dels = austrianDelegates.filter(delegate => delegate.council === "nr");
        console.log(dels);
        
    });

</script>

<div>
    <!-- <Halfcircle n={17} r={70 * 1.5} /> -->
    <!-- <Halfcircle n={26} r={100 * 1.5} /> -->
    <!-- <Halfcircle n={32} r={130 * 1.5} /> -->
    <!-- <Halfcircle n={37} r={150. * 1.5} /> -->
    <!-- <Halfcircle n={40} r={170. * 1.5} /> -->
    <!-- <Halfcircle n={32} r={190. * 1.5} /> -->

    {#if dels}
        <div style = "margin-top: 200px">
            <!-- <Parliament seats={[17, 26, 32, 37, 40, 32]} /> -->
            <Parliament2 dels={dels} seats={[17, 26, 32, 37, 40, 32]} />
        </div>
    {:else}
        <p class="loading">loading...</p>
    {/if}

</div>