<script lang="ts">
	import { delegates } from "$lib/api";
	import VoteResults from "$lib/components/VoteResults/VoteResults.svelte";
	import { updateColorStorage } from "$lib/partyColor";
	import type { Delegate } from "$lib/types";
	import { onMount } from "svelte";

    let dels: Delegate[];
    onMount(async function () {
		await updateColorStorage();

		const austrianDelegates = await delegates();
		dels = austrianDelegates.filter((delegate) => delegate.council === "nr");
	}); 

</script>
<div class="container mx-auto px-0 mt-5">
    home
    <VoteResults {dels} /> 
</div>