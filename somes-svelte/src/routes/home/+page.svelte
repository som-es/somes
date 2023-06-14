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

    {#if voteResults}
    <div class="grid-container2">
        <section class="grid-tile-2-col-2-row grid-tile">
            <div class="grid-tile-content">
                
                    <div class="mx-1 w-[360px]"
                        on:click={() => {
                            currentLegisInitStorage.set(voteResults[0]);
                            goto('/vote');
                        }}
                    >
                    <VoteParliament
                        dels={dels}
                        seats={[20, 27, 37, 43, 48, 54]}
                        voteResult={voteResults[0]}
                    />
                    </div>
                
                Das neue Fortnite Phone
            </div>
        </section>
        <section class="grid-tile">
            <div class="grid-tile-content">
                <span class="mx-3 text-center">{voteResults[0].legislative_initiative.description}</span>
                <!-- Das neue Fortnite Phone -->
            </div>
        </section>
        <section class="grid-tile">
            <div class="grid-tile-content">
                Das neue Fortnite Phone
            </div>
        </section>
    </div>
    {/if}
    <!--<div class="grid-container gap-5">
        <div class="grid-item item2 rounded">1</div>
        <div class="grid-item rounded">2</div>
        <div class="grid-item rounded">2</div>
    </div>-->

    <div class="flex flex-wrap mt-4">
        {#if voteResults}
            {#if voteResults.length == 0}
                <p class="no-news">Keine Neuigkeiten verfügbar</p>
            {/if}
            {#each voteResults as voteResult}
                <span class="label legis-init-label">{voteResult.legislative_initiative.description}</span>

                <!-- svelte-ignore a11y-click-events-have-key-events -->
                <!-- class="max-w-[20.2rem] -->
                <div class="mx-1 w-[360px]"
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
            {/each}
        {:else}
            <p class="loading">loading...</p>
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

.legis-init-label {
    width: 5%;
}

@media (max-width: 600px) {
    .legis-init-label {
        width: 100%; /* Adjust the width as per your requirement */
    }
}

.grid-container {
  display: grid;
  grid-template-columns: auto auto auto;
  padding: 10px;
}

.grid-container2 {
  display: grid;
  gap: 20px;
  /* grid-template-columns: auto auto auto; */
  grid-template-columns: 170px 270px 270px;
  grid-auto-rows: 120px;
  padding: 10px;
}

.grid-item {
    border: 1px solid rgba(0, 0, 0, 0.8);
    padding: 20px;
    font-size: 30px;
    text-align: center;
}
.grid-tile-2-col-2-row {
    grid-column: auto/span 2;
    grid-row: auto/span 2;
}

.grid-tile {
    box-sizing: border-box;
    padding: 0;
    border-radius: 25px;
    position: relative;
    z-index: 1;
    overflow: hidden;
    background: #f5f5f7;
}

.section-sizes,
.grid-tile-content {
    display: flex;
    justify-content: center;
    flex-direction: column;
    align-items: center;
}

</style>
