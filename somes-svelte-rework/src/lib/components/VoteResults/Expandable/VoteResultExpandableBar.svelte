<script lang="ts">
	import type { Delegate, VoteResult } from "$lib/types";
	import VoteParliament from "../../Parliaments/VoteParliament.svelte";
    import collapse from 'svelte-collapse'
	import upArrowIcon from "$lib/assets/misc_icons/up-arrow.svg?raw";
	import downArrowIcon from "$lib/assets/misc_icons/down-arrow.svg?raw";
	import { CollapsibleCard } from "svelte-collapsible";

    export let voteResult: VoteResult;
    export let dels: Delegate[];
    let clazz;
	export { clazz as class }; 
    let open = false;

</script>
<div class="gap-3 mt-5">
    <div 
        on:click={() => open = !open}
        on:keypress={() => open = !open}
        role="button"
        tabindex="0"
        class="entry  bg-primary-300 dark:bg-primary-500 flex justify-between items-center"
    >
        <div>
            {#if open}
                {@html upArrowIcon}
            {:else}
                {@html downArrowIcon}
            {/if}
        </div>
        <div>{voteResult.legislative_initiative.title}</div>
        <div class="w-20 bg-primary-400 dark:bg-primary-600 rounded-md"><VoteParliament {dels} {voteResult} preview={true}/></div>
    </div>

    <div use:collapse={{open}}>
        <div class="entry bg-primary-200 flex flex-wrap dark:bg-primary-400 mt-3 justify-between ">
            <div>Abstimmung </div>
            <div class=" rounded-md w-96"><VoteParliament {dels} {voteResult} preview={true}/></div>
        </div>
    </div>
</div>

<style>
	
    .entry {
        border-radius: 0.9rem;     
        box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1); 
        padding: 20px;         
        gap: 10px;
    }
</style>