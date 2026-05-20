<script lang="ts">
	import { dashDateToDotDate } from '$lib/date';
	import type { Absence } from '$lib/types';
	import AbsenceBarExpanded from './AbsenceBarExpanded.svelte';
	import { slide } from 'svelte/transition';
	
	interface Props {
		absence: Absence;
		showDetails?: boolean;
		page: number;
	}

	let { absence, page, showDetails = true }: Props = $props();
	// absence.

	let open: boolean = $state(false);
	let duration = 0.35;

	$effect(() => {
		if (page) open = false;
	})
</script>

<div class="gap-3 mt-5">
	<button
		type="button"
		onclick={() => (open = !open)}
		aria-expanded={open}
		class="entry dark:bg-primary-300 bg-primary-400 text-black w-full text-left"
	>
		<div class="flex justify-between items-center">
			<!-- <div class="flex gap-4">
				<div id={open ? 'open' : 'closed'}>
					{@html rightArrowIcon}
				</div>
			</div> -->

			<div>
				{absence.inr}. Nationalratssitzung | {absence.gp}
			</div>
			{#if showDetails}
				<div>
					{absence.missed_legis_init_ids.length} verpasste
					{#if absence.missed_legis_init_ids.length == 1}
						Abstimmung
					{:else}
						Abstimmungen
					{/if}
				</div>
			{/if}
		</div>
		<div class="flex justify-between mt-1">
			<div></div>
			<span class="badge bg-tertiary-400">{dashDateToDotDate(absence.date.split("T")[0])}</span>
		</div>
	</button>
	{#if open}
		<!-- <div transition:slide={{ duration: 240 }}>
			<AbsenceBarExpanded {absence} bind:open />
		</div> -->
	{/if}
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
