<script lang="ts">

	import AutoSelectParliament from "../AutoSelectParliament.svelte";
	import type { Delegate, VoteResult } from "$lib/types";
	import { delegates, latest_vote_results } from "$lib/api";
    import { onMount } from "svelte";
	import type { Writable } from "svelte/store";
	import { localStorageStore } from "@skeletonlabs/skeleton";
	import VoteParliament from "../VoteParliament.svelte";
	import { goto } from "$app/navigation";

    let dels: Delegate[];

    const currentLegisInitStorage: Writable<VoteResult | null> = localStorageStore('selectedVoteResult', null);
    
    let voteResults: VoteResult[];

    onMount(async function () {
        const austrianDelegates = await delegates();
        dels = austrianDelegates.filter(delegate => delegate.council === "nr");

        voteResults = (await latest_vote_results()).slice(0, 3);
    });

</script>

<div class="container mx-auto px-4">
    <h1 class="text-primary-400">Welcome back!</h1>
    <h2 class="mt-5">Nationalrat</h2>
    Current news from the Austrian parliament

    <div class="flex mt-4">
        {#if voteResults}
            {#each voteResults as voteResult}
                <!-- svelte-ignore a11y-click-events-have-key-events -->
                <div class="max-w-[20.2rem] ml-5"
                    on:click={() => {
                        currentLegisInitStorage.set(voteResult);
                        goto('/vote');
                    }}
                >
                    <VoteParliament
                        dels={dels}
                        seats={[20, 27, 37, 43, 48, 54]}
                        voteResult={voteResult}
                    />
                </div>
                <span style="font-size: 0.4rem;">{voteResult.legislative_initiative.description}</span>
            {/each}
        {/if}
    </div>

    <h2 class="mt-5">Representatives</h2>

    {#if dels}
        <AutoSelectParliament dels={dels} seats={[20, 27, 37, 43, 48, 54]}/>
    {:else}
        <p class="loading">loading...</p>
    {/if}
</div>

<style>

</style>
