<script lang="ts">

	import type { Delegate, VoteResult } from "$lib/types";
	import type { Writable } from "svelte/store";
	import { localStorageStore } from "@skeletonlabs/skeleton";
	import VoteParliament from "./VoteParliament.svelte";
	import { goto } from "$app/navigation";

    const currentLegisInitStorage: Writable<VoteResult | null> = localStorageStore('selectedVoteResult', null);
    export let voteResult: VoteResult;
    export let dels: Delegate[];
</script>

<div class="grid-container2">
    <section class="grid-tile-2-col-2-row grid-tile">
        <div class="grid-tile-content">
            
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
            
            <span class="mx-3 text-center">{voteResult.legislative_initiative.description}</span>
        </div>
    </section>
    <section class="grid-tile">
        <div class="grid-tile-content">
            <span class="mx-3 text-center">emphasis?</span>
            <!-- Das neue Fortnite Phone -->
        </div>
    </section>
    <section class="grid-tile">
        <div class="grid-tile-content">
            Das neue Fortnite Phone
        </div>
    </section>
</div>

<style>
    .grid-container {
  display: grid;
  grid-template-columns: auto auto auto;
  padding: 10px;
}

.grid-container2 {
  display: grid;
  gap: 1rem;
  /* grid-template-columns: auto auto auto; */
  grid-template-columns: 100% 0px 0px;
  /* grid-template-columns: 10rem 15rem 15rem; */
  /* grid-auto-rows: 120px; */
  /* grid-template-columns: repeat(auto-fit, minmax(350px, 2fr)); */

  /* grid-auto-rows: 8rem; */
  padding: 10px;
}

@media (min-width: 600px) {
    .grid-container2 { 
        grid-template-columns: repeat(3, 2fr); 
        max-width: 40rem;
    }
}
/*
@media (min-width: 900px) {
  .grid-container2 { 
    grid-template-columns: repeat(3, 2fr); 
    max-width: 40rem;
}
}*/

.grid-item {
    border: 1px solid rgba(0, 0, 0, 0.8);
    padding: 20px;
    font-size: 30px;
    text-align: center;
}
.grid-tile-2-col-2-row {
    grid-column: auto/span 2;
    grid-row: auto/span 2;
    /* width: 25rem; */
    min-width: 100%;
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