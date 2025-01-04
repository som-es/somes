<script lang="ts">
	import { errorToNull, vote_result_by_id } from "$lib/api";
	import VoteParliament2 from "$lib/components/Parliaments/VoteParliament2.svelte";
	import type { Speech, VoteResult } from "$lib/types";
	import { onMount } from "svelte";

    export let speech: Speech;

    let voteResult: VoteResult | null = null;

    onMount(async () => {
        voteResult = errorToNull(await vote_result_by_id(speech.legislative_initiatives_id.toString()));
        console.log(voteResult)
    }) 

</script>

<div class="gap-3 rounded variant-filled my-1">
    {#if voteResult}
        {voteResult.legislative_initiative.description}
        {speech.legislative_initiatives_id} {speech.opinion} 
        {#if voteResult.votes.length > 0}
            <div>
                <VoteParliament2 {voteResult}></VoteParliament2>
            </div>
        {/if}
    {/if}
</div>
