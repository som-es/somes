<script lang="ts">
	import type { VolksbgEintragungswoche } from '../../../routes/types';
	import { t } from '$lib/i18n/i18n.svelte';
	import { dashDateToDotDate } from '$lib/date';

	interface Props {
		week: VolksbgEintragungswoche | null;
	}

	let { week }: Props = $props();

	function getDeadline(week: VolksbgEintragungswoche): Date {
		if (week.online_deadline_utc) return new Date(week.online_deadline_utc);
		return new Date(`${week.end_date}T23:59:59`);
	}

	function getPhase(week: VolksbgEintragungswoche): 'upcoming' | 'running' | 'ended' {
		const now = new Date();
		if (now >= getDeadline(week)) return 'ended';
		if (now < new Date(`${week.start_date}T00:00`)) return 'upcoming';
		return 'running';
	}

	const phase = $derived(week ? getPhase(week) : 'ended');

	const headlines = {
		upcoming: 'volksbg.upcoming',
		running: 'volksbg.running',
		ended: 'volksbg.ended'
	} as const;
</script>

{#if week && week.volksbgs && week.volksbgs.length > 0}
	<section
		class="mt-3 w-full rounded-xl bg-secondary-200 p-4 text-black shadow-md dark:bg-secondary-900 dark:text-white"
		aria-labelledby="volksbg-reminder-title"
	>
		<div class="flex flex-wrap items-center gap-x-5 gap-y-3">
			<div class="min-w-0 flex-1">
				<span
					class="text-xs font-bold tracking-widest text-secondary-800 uppercase dark:text-secondary-200"
				>
					{t('volksbg.kicker')}
				</span>
				<h2 id="volksbg-reminder-title" class="text-xl font-bold">{t(headlines[phase])}</h2>
				<p class="text-sm text-gray-800 dark:text-gray-200">
					{#if week.start_date && week.end_date}
						{dashDateToDotDate(week.start_date)} – {dashDateToDotDate(week.end_date)}
					{/if}
				</p>
			</div>

			{#if week.polling_stations_url}
				<!-- eslint-disable svelte/no-navigation-without-resolve -- external BMI link -->
				<a
					href={week.polling_stations_url}
					target="_blank"
					rel="noopener noreferrer"
					class="group flex items-center gap-1 rounded-lg bg-white/70 px-3 py-2 text-sm font-semibold hover:bg-white dark:bg-white/10 dark:hover:bg-white/20"
				>
					{t('volksbg.pollingStations')}
					<span class="transition-transform group-hover:translate-x-1">→</span>
				</a>
				<!-- eslint-enable svelte/no-navigation-without-resolve -->
			{/if}
		</div>

		<ul
			class="mt-4 flex snap-x snap-mandatory gap-3 overflow-x-auto pb-1 sm:grid sm:grid-cols-2 sm:overflow-visible lg:grid-cols-3"
		>
			{#each week.volksbgs as volksbg (volksbg.id)}
				<li class="w-64 shrink-0 snap-start sm:w-auto">
					<!-- eslint-disable svelte/no-navigation-without-resolve -- external BMI link -->
					<a
						href={volksbg.overview_url}
						target="_blank"
						rel="noopener noreferrer"
						class="group flex h-full flex-col rounded-lg bg-white/80 p-3 transition-colors hover:bg-white dark:bg-surface-800 dark:hover:bg-surface-700"
					>
						<h3 class="line-clamp-2 font-bold group-hover:underline">{volksbg.title}</h3>
						<p class="mt-1 line-clamp-3 text-sm text-gray-700 dark:text-gray-300">
							{volksbg.description}
						</p>
					</a>
					<!-- eslint-enable svelte/no-navigation-without-resolve -->
				</li>
			{/each}
		</ul>
	</section>
{/if}
