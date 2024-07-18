<script lang="ts">
	import { cachedDelegates } from "$lib/caching/delegates";
	import DelegateCard from "$lib/components/Delegates/DelegateCard.svelte";
	import DelegatesParliament from "$lib/components/Parliaments/DelegatesParliament.svelte";
	import type { Delegate } from "$lib/types";
	import { onMount } from "svelte";


    let delegates: Delegate[] | null;

    onMount(async () => {
        delegates = await cachedDelegates();
    });

    let delegate: Delegate | null;
</script>

<div>
    delegates
    {#if delegates}

        {#if delegate}
            <DelegateCard delegate={delegate}/>
        {/if}
        <DelegatesParliament bind:delegate={delegate} dels={delegates}/>
    {/if}
</div>