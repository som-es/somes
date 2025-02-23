<script lang="ts">
	import { isHasError, vote_result_by_id } from "$lib/api/api";
	import ExpandablePlaceholder from "$lib/components/VoteResults/Expandable/Placeholders/ExpandablePlaceholder.svelte";
	import type { Absence } from "$lib/types";
	import { onMount } from "svelte";
	import VoteResult from '$lib/components/VoteResults/VoteResult.svelte';


    export let absence: Absence;
    export let open: boolean;

</script>

<div class="entry bg-primary-200 dark:bg-primary-400 mt-3 flex flex-wrap">
    {#each absence.missed_legis_init_ids as id, i}
        {#await vote_result_by_id(id.toString())}
			<ExpandablePlaceholder class="!w-80" />
        {:then voteResult}
			{#if !isHasError(voteResult) }
				<VoteResult dels={[]} voteResult={voteResult} tabindex={i} />
            {/if}
        {/await}

    {/each}
</div>
<style>
    .entry {
		border-radius: 0.9rem;
		box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
		padding: 20px;
		gap: 10px;
	}
</style>