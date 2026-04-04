<script lang="ts">
	import type { Absence } from '$lib/types';
	import { onMount } from 'svelte';
	import ExtendInfoDialog from '../ExtendInfoDialog.svelte';
	import AbsencesModal from './AbsencesModal.svelte';

	interface Props {
		absences: Absence[];
		delegateId: number;
	}

	let { absences = [], delegateId }: Props = $props();

	// Sort absences by date descending
	let sortedAbsences = $derived(
		[...absences].sort((a, b) => new Date(b.date).getTime() - new Date(a.date).getTime())
	);
	let recentAbsences = $derived(sortedAbsences.slice(0, 3));

	// Group by year for some stats
	let absencesByYear = $derived(
		absences.reduce(
			(acc, curr) => {
				const year = new Date(curr.date).getFullYear();
				acc[year] = (acc[year] || 0) + 1;
				return acc;
			},
			{} as Record<number, number>
		)
	);

	let currentYear = new Date().getFullYear();
	let absencesThisYear = $derived(absencesByYear[currentYear] || 0);

	function formatDate(dateString: Date | string) {
		return new Intl.DateTimeFormat('de-AT', {
			day: '2-digit',
			month: 'short',
			year: 'numeric'
		}).format(new Date(dateString));
	}
</script>

<div
	class="title-item flex h-full w-full flex-col rounded-xl bg-primary-300 p-5 dark:bg-primary-500"
>
	<div class="flex-1">
		<div class="flex items-center justify-between">
			<div class="flex flex-col">
				<span class="text-lg font-bold text-black xl:text-xl dark:text-white"> Abwesenheiten </span>
				<p class="text-sm text-primary-600 dark:text-primary-300">
					Verpasste Plenarsitzungen ({currentYear}):
				</p>
				<span class="mt-1 text-4xl font-black text-primary-800 dark:text-primary-100"
					>{absencesThisYear}</span
				>
			</div>
		</div>

		<div class="mt-4">
			<h3 class="mb-2 text-sm font-semibold tracking-wider text-primary-800 dark:text-primary-200">
				Zuletzt abwesend
			</h3>
			<div class="flex flex-col gap-2">
				{#if recentAbsences.length > 0}
					{#each recentAbsences as absence}
						<div
							class="flex items-center justify-between rounded-lg bg-primary-200 p-3 text-sm dark:bg-primary-800/40 dark:hover:bg-primary-800/60"
						>
							<div class="flex items-center gap-3">
								<div class="h-2 w-2 rounded-full bg-red-500/80"></div>
								<span class="font-medium text-primary-900 dark:text-primary-100"
									>{absence.inr}. Nationalratssitzung</span
								>
							</div>
							<div class="text-xs text-primary-600 dark:text-primary-400">
								{formatDate(absence.date)} ({absence.gp})
							</div>
						</div>
					{/each}
				{:else}
					<div class="flex flex-col gap-2">
						<div
							class="flex items-center justify-center rounded-lg bg-primary-200 p-3 text-sm dark:bg-primary-800/40"
						>
							<span class="text-primary-600 dark:text-primary-400">Keine Abwesenheiten</span>
						</div>
					</div>
				{/if}
			</div>
		</div>
	</div>

	{#if recentAbsences.length > 0}
		<div class="mt-auto flex justify-end pt-4">
			<ExtendInfoDialog title="Alle anzeigen">
				<AbsencesModal {absences} />
			</ExtendInfoDialog>
		</div>
	{/if}
</div>
