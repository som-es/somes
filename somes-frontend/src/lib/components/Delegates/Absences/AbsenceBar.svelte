<script lang="ts">
	import type { Absence } from '$lib/types';
	import AbsenceBarExpanded from './AbsenceBarExpanded.svelte';
	import { slide } from 'svelte/transition';
	
	interface Props {
		absence: Absence;
		page: number;
	}

	let { absence, page }: Props = $props();
	// absence.

	let open: boolean = $state(false);
	let duration = 0.35;

	$effect(() => {
		if (page) open = false;
	})
</script>

<div class="gap-3 mt-5">
	<div
		onclick={() => (open = !open)}
		onkeypress={() => (open = !open)}
		role="button"
		tabindex="0"
		class="entry dark:bg-primary-300 bg-primary-400 text-black"
	>
		<div class="flex justify-between items-center">
			<div class="flex gap-4">
				<!-- <div id={open ? 'open' : 'closed'}>
					{@html rightArrowIcon}
				</div> -->

			</div>

			<div>
				{absence.inr}. Nationalratssitzung | {absence.gp}
			</div>
			<div>
				{absence.missed_legis_init_ids.length} verpasste
				{#if absence.missed_legis_init_ids.length == 1}
					Abstimmung
				{:else}
					Abstimmungen
				{/if}
			</div>
		</div>
		<div class="flex justify-between mt-1">
			<div></div>
			<span class="badge bg-tertiary-400">Abwesenheit</span>
		</div>
	</div>
	{#if open}
		<div transition:slide={{ duration: 240 }}>
			<AbsenceBarExpanded {absence} bind:open />
		</div>
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
