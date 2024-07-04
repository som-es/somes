<script lang="ts">
	import { goto } from "$app/navigation";
	import type { Delegate, VoteResult } from "$lib/types";
	import VoteParliament from "../Parliaments/VoteParliament.svelte";

    export let voteResult: VoteResult;
    export let dels: Delegate[];
    export let tabindex: number; 

    function onClick() {
        // currentLegisInitStorage.set(voteResult);
        // $: if (browser) {
            goto("/")
        // }
    }

</script>
<span class="card tile">
    <div class="tile-content">
        <div
            class="mx-1 w-[360px]"
            on:click={onClick}
            on:keypress={onClick}
            role="link"
            tabindex={10 + tabindex}
        >
            <VoteParliament {dels} {voteResult} preview={true} />
        </div>
        <span class="mx-3 text-left">{voteResult.legislative_initiative.description}</span>
    </div>
</span>
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
</style>