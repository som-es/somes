<script lang="ts">
	import { delegates } from "$lib/api";
	import VoteResults from "$lib/components/VoteResults/VoteResults.svelte";
    import { cachedDelegates } from "$lib/caching/delegates";
	import { updateColorStorage } from "$lib/partyColor";
	import type { Delegate } from "$lib/types";
	import { onMount } from "svelte";

    let dels: Delegate[];
    onMount(async function () {
		await updateColorStorage();

		const austrianDelegates = await cachedDelegates();
		dels = austrianDelegates.filter((delegate) => delegate.council === "nr");
	}); 

</script>
<div class="container mx-auto px-0 mt-5">
    home
    {#if dels}
        <VoteResults {dels} />
    {:else}
        <p>loading...</p>
    {/if}
</div>