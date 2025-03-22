<script lang="ts">
	import { errorToNull, isHasError, vote_result_by_id } from "$lib/api/api";
	import ExpandablePlaceholder from "$lib/components/VoteResults/Expandable/Placeholders/ExpandablePlaceholder.svelte";
	import type { Absence, VoteResult as VoteResultType } from "$lib/types";
	import { onMount } from "svelte";
	import VoteResult from '$lib/components/VoteResults/VoteResult.svelte';
	import VoteResults from "$lib/components/VoteResults/VoteResults.svelte";


    export let absence: Absence;
    export let open: boolean;
    let voteResults: VoteResultType[] = [];

    // onMount(async () => {
    //     absence.missed_legis_init_ids.forEach(async (id) => {
    //         const res = errorToNull(await vote_result_by_id(id.toString()));
    //         if (res) {
    //             voteResults.push(res)
    //             console.log(res);
    //             voteResults = voteResults;
    //         }
    //     }) 
    //     voteResults = voteResults;
    // })

    const updateVoteResults = () => {
        absence.missed_legis_init_ids.forEach(async (id) => {
            const res = errorToNull(await vote_result_by_id(id.toString()));
            if (res) {
                voteResults.push(res)
                voteResults = voteResults;
            }
        }) 
    }

    $: if(absence) updateVoteResults()

    $: if (open && voteResults.length == 0) {
        updateVoteResults();
        voteResults = voteResults;
    }
    

</script>

<div class="entry bg-primary-200 dark:bg-primary-400 mt-3 flex flex-wrap">
    <VoteResults {voteResults} dels={[]} />
</div>
<style>
    .entry {
		border-radius: 0.9rem;
		box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
		padding: 20px;
		gap: 10px;
	}
</style>