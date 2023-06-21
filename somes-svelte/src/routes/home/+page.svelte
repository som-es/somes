<script lang="ts">

	import AutoSelectParliament from "../AutoSelectParliament.svelte";
    import LegisInitCard from "../LegisInitCard.svelte";
	import type { Delegate, Party, VoteResult } from "$lib/types";
	import { delegates, latest_vote_results, parties } from "$lib/api";
    import { onMount } from "svelte";
	import type { Writable } from "svelte/store";
	import { localStorageStore } from "@skeletonlabs/skeleton";
	import VoteParliament from "../VoteParliament.svelte";
    import SpeakersByHours from "../SpeakersByHours.svelte";
	import { goto } from "$app/navigation";
	import CallToOrdersPerPartyDelegates from "../CallToOrdersPerPartyDelegates.svelte";
	import CallToOrders from "../CallToOrders.svelte";
	import OtherCallToOrders from "../OtherCallToOrders.svelte";

    let dels: Delegate[];

    const currentLegisInitStorage: Writable<VoteResult | null> = localStorageStore('selectedVoteResult', null);
    const partyColorStorage: Writable<string> = localStorageStore('parties', "");

    let voteResults: VoteResult[];
    
    async function updateColorStorage() {    
        let partyToColor = new Map<string, string>();
        (await parties()).forEach(party => {
            partyToColor.set(party.name, party.color);
        });
        partyColorStorage.set(JSON.stringify(Array.from(partyToColor.entries())));
    }

    onMount(async function () {
        await updateColorStorage();

        const austrianDelegates = await delegates();
        dels = austrianDelegates.filter(delegate => delegate.council === "nr");

        voteResults = (await latest_vote_results()).slice(0, 6);
    });

</script>

<div class="container mx-auto px-4">
    <h1 class="text-primary-400">Welcome back!</h1>
    <h2 class="mt-5">Nationalrat</h2>
    Current news from the Austrian parliament

    {#if voteResults}
        {#if voteResults.length == 0}
            <p class="no-news">Keine Neuigkeiten verfügbar</p>
        {/if}
        <div class="card-container">
            {#each voteResults as voteResult}
            <span class="card tile">
                <div class="tile-content">
                    <!-- svelte-ignore a11y-click-events-have-key-events -->
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
                    <span class="mx-3 text-left">{voteResult.legislative_initiative.description}</span>
                </div>
            </span>
            {/each}
        </div>
    {:else}
        <p class="loading">loading...</p>
    <!--
    <div class="flex flex-wrap">
        <LegisInitCard voteResult={voteResults[0]} dels={dels} />
        <LegisInitCard voteResult={voteResults[1]} dels={dels} />
        <LegisInitCard voteResult={voteResults[2]} dels={dels} />
    </div>
    -->
    {/if}
    
    <!--<div class="grid-container gap-5">
        <div class="grid-item item2 rounded">1</div>
        <div class="grid-item rounded">2</div>
        <div class="grid-item rounded">2</div>
    </div>-->

    <h2 class="mt-5">Representatives</h2>

    {#if dels}
        <AutoSelectParliament dels={dels} seats={[20, 27, 37, 43, 48, 54]}/>
    {:else}
        <p class="loading">loading...</p>
    {/if}

    <h2 class="mt-5">Statistics</h2>
    <h4>Here, you can find statistics about the Austrian parliament</h4>
    <div>
    <p class="mt-3">
        CTOs
        <OtherCallToOrders />
    </p>
    <p class="mt-3">
        The top speakers of the Nationalrat by hours spoken
        <SpeakersByHours />
    </p>
    
    <p class="mt-3">
        Top call to orders receivers (make selectable: ALL TIME, XXVII, XXVI, XXV, .. by party?)
        <CallToOrders />
    </p>

    <p class="mt-3">
        Call to orders of parties relative to the number of delegates in the Nationalrat
        = get issuer of call to order (wie sind die CTOs verteilt in Bezug auf den aktiven Präsidenten? (ÖVP, SPÖ, FPÖ)) 
        "Forschungsfrage": Warum hat die ÖVP so wenige CTOs über die Zeit? 
        <CallToOrdersPerPartyDelegates />
    </p>
    </div>
</div>

<style>

.tile {
    box-sizing: border-box;
    padding: 0;
    border-radius: 25px;
    position: relative;
    z-index: 1;
    overflow: hidden;
    width: 25rem;   
    margin: 0.8rem;
}

.tile-content {
    display: flex;
    justify-content: center;
    flex-direction: column;
    align-items: center;
}

.legis-init-label {
    width: 5%;
}

.card-container {
    margin: auto;
    display: flex;
    flex-direction: row;
    flex-wrap: wrap;
}

@media (max-width: 600px) {
    .legis-init-label {
        width: 100%; /* Adjust the width as per your requirement */
    }
}

</style>
