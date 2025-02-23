<script lang="ts">
	import type { Absence } from "$lib/types";
	import rightArrowIcon from '$lib/assets/misc_icons/right-arrow.svg?raw';
	import AbsenceBarExpanded from "./AbsenceBarExpanded.svelte";
	import collapse from 'svelte-collapse';

    export let absence: Absence;
    export let page: number;
    // absence.

    let open: boolean = false;
	let duration = 0.35;

    $: if (page) {
        open = false
    }

</script>

<div class="gap-3 mt-5">
    <div
		on:click={() => (open = !open)}
		on:keypress={() => (open = !open)}
		role="button"
		tabindex="0"
		class="entry dark:bg-primary-300 bg-primary-400 flex justify-between items-center text-black"
	>
		<div>
			<div id={open ? 'open' : 'closed'}>
				{@html rightArrowIcon}
			</div>
		</div>
        <div>
            {absence.inr}. Nationalratssitzung | {absence.gp}
        </div>
        <div>
            {absence.missed_legis_init_ids.length} verpasste 
            {#if absence.missed_legis_init_ids.length > 1}
                Abstimmungen
            {:else}
                Abstimmung
            {/if}
        </div>
    </div>
    
    <div use:collapse={{ open, duration }}>
		<!-- <AbsenceBarExpanded {absence} bind:open /> -->
	</div>
</div>

<style>
    .entry {
		border-radius: 0.9rem;
		box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
		padding: 20px;
		gap: 10px;
	}
    
    #open :global(.right-arrow) {
		transform: rotate(90deg);
		transition: transform 0.35s;
	}

	#closed :global(.right-arrow) {
		transform: rotate(0deg);
		transition: transform 0.35s;
	}
</style>