<script lang="ts">
	import { t } from '$lib/i18n/i18n.svelte';
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
	});
</script>

<div class="mt-5 gap-3">
	<div
		onclick={() => (open = !open)}
		onkeypress={() => (open = !open)}
		role="button"
		tabindex="0"
		class="entry bg-primary-400 text-black dark:bg-primary-300"
	>
		<div class="flex items-center justify-between">
			<!-- <div class="flex gap-4">
				<div id={open ? 'open' : 'closed'}>
					{@html rightArrowIcon}
				</div>
			</div> -->

			<div>
				 {t('absences.sessionLabel', { inr: absence.inr, gp: absence.gp })} 
			</div>
			{#if showDetails}
				<div>
					{absence.missed_legis_init_ids.length} {t('absences.missed')}
					{#if absence.missed_legis_init_ids.length == 1}
						{t('absences.voteSingular')}
					{:else}
						{t('absences.votePlural')}
					{/if}
				</div>
			{/if}
		</div>
		<div class="mt-1 flex justify-between">
			<div></div>
			<span class="badge bg-tertiary-400">{dashDateToDotDate(absence.date.split('T')[0])}</span>
		</div>
	</div>
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
