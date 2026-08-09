<script lang="ts">
	import { getParliament, type Parliament } from '$lib/api/parliament';
	import { cachedPlenarySessions } from '$lib/caching/plenarySessions';
	import { formatDate } from '$lib/date';
	import type { Absence, Delegate, PlenarySession } from '$lib/types';
	import { onMount } from 'svelte';
	import ExtendInfoDialog from '../ExtendInfoDialog.svelte';
	import AbsencesModal from './AbsencesModal.svelte';

	interface Props {
		absences: Absence[];
		title?: string;
		explanation?: string;
		lastEntriesText?: string;
		noEntriesText?: string;
		showTotal?: boolean;
		showDetails?: boolean;
		delegate: Delegate;
		parliament: Parliament;
	}

	let currentYear = new Date().getFullYear();
	let {
		absences = [],
		delegate,
		title = 'Abwesenheiten',
		explanation = `Verpasste Plenarsitzungen (${currentYear})`,
		lastEntriesText = 'Zuletzt abwesend',
		noEntriesText = 'Keine Abwesenheiten',
		showTotal = false,
		showDetails = true,
		parliament
	}: Props = $props();

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

	let allPlenarySessions: Record<string, PlenarySession[]> | null = $state(null);
	onMount(async () => {
		if (getParliament() == 'eu') {
			allPlenarySessions = await cachedPlenarySessions();
		}
	});

	let absencesThisYear = $derived(absencesByYear[currentYear] || 0);

	const entryCount = $derived.by(() => {
		if (showTotal) {
			return absences.length;
		} else {
			return absencesThisYear;
		}
	});

	const sessionText = (council) => {
		switch (parliament) {
			case 'at': {
				return council === 'NR' ? 'Nationalratssitzung' : 'Bundesratssitzung';
			}
			case 'eu':
				return 'Plenarsitzung';
		}
	};

	const translateInr = $derived((allPlenarySessions, inr, gp) => {
		switch (parliament) {
			case 'at': {
				return inr;
			}
			case 'eu': {
				if (allPlenarySessions) {
					const sessionsInGp = allPlenarySessions[gp];
					const idx = sessionsInGp.findIndex((session) => session.inr == inr);
					return idx + 1;
				}
				return null;
			}
		}
	});
</script>

<div
	class="title-item flex h-full w-full flex-col rounded-xl bg-primary-300 p-5 dark:bg-primary-500"
>
	<div class="flex-1">
		<div class="flex items-center justify-between">
			<div class="flex flex-col">
				<span class="text-lg font-bold text-black xl:text-xl dark:text-white"> {title} </span>
				<p class="text-sm text-primary-600 dark:text-primary-300">
					{explanation}
				</p>
				<span class="mt-1 text-4xl font-black text-primary-800 dark:text-primary-100"
					>{entryCount}</span
				>
			</div>
		</div>

		<div class="mt-4">
			<h3 class="mb-2 text-sm font-semibold tracking-wider text-primary-800 dark:text-primary-200">
				{lastEntriesText}
			</h3>
			<div class="flex flex-col gap-2">
				{#if recentAbsences.length > 0}
					{#each recentAbsences as absence ((absence.gp, absence.inr, absence))}
						{@const inr = translateInr(allPlenarySessions, absence.inr, absence.gp)}
						<svelte:element
							this={absence.source_url ? 'a' : 'div'}
							href={absence.source_url || undefined}
							target={absence.source_url ? '_blank' : undefined}
							rel={absence.source_url ? 'noopener noreferrer' : undefined}
							class="flex items-center justify-between rounded-lg bg-primary-200 p-3 text-sm dark:bg-primary-800/40 {absence.source_url
								? 'transition-colors hover:bg-primary-300 dark:hover:bg-primary-800/60'
								: ''}"
						>
							<div class="flex items-center gap-3">
								<div class="h-2 w-2 rounded-full bg-red-500/80"></div>
								{#if inr}
									<span class="font-medium text-primary-900 dark:text-primary-100">
										{inr}. {sessionText(absence.council)}
									</span>
								{/if}
							</div>
							<div class="text-xs text-primary-600 dark:text-primary-400">
								{formatDate(absence.date)} ({absence.gp})
							</div>
						</svelte:element>
					{/each}
				{:else}
					<div class="flex flex-col gap-2">
						<div
							class="flex items-center justify-center rounded-lg bg-primary-200 p-3 text-sm dark:bg-primary-800/40"
						>
							<span class="text-primary-600 dark:text-primary-400">{noEntriesText}</span>
						</div>
					</div>
				{/if}
			</div>
		</div>
	</div>

	{#if recentAbsences.length > 0}
		<div class="mt-auto flex justify-end pt-4">
			<ExtendInfoDialog title="Alle anzeigen">
				<AbsencesModal absences={sortedAbsences} {title} {showDetails} {delegate} />
			</ExtendInfoDialog>
		</div>
	{/if}
</div>
