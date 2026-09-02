<script lang="ts">
	import type { VolksbgEintragungswoche } from '../../../routes/types';
	import { t } from '$lib/i18n/i18n.svelte';
	import { dashDateToDate, dashDateToDotDate } from '$lib/date';

	interface Props {
		week: VolksbgEintragungswoche | null;
	}

	let { week }: Props = $props();

	const DAY_MS = 1000 * 60 * 60 * 24;

	const DEFAULT_COUNTDOWN_DAYS = 35;
	const RADIUS = 42;
	const CIRCUMFERENCE = 2 * Math.PI * RADIUS;

	type Phase = 'upcoming' | 'running' | 'ended';

	function startOfDay(date: Date): Date {
		return new Date(date.getFullYear(), date.getMonth(), date.getDate());
	}

	function daysBetween(a: Date, b: Date): number {
		return Math.round((startOfDay(b).getTime() - startOfDay(a).getTime()) / DAY_MS);
	}

	function clamp01(value: number): number {
		return Math.min(1, Math.max(0, value));
	}

	/** Last moment to sign: the online deadline if known, otherwise midnight after the last day. */
	function deadlineOf(week: VolksbgEintragungswoche): Date | null {
		if (week.online_deadline_utc) {
			const deadline = new Date(week.online_deadline_utc);
			if (!isNaN(deadline.getTime())) return deadline;
		}
		const end = dashDateToDate(week.end_date);
		return end ? new Date(end.getTime() + DAY_MS) : null;
	}

	function countdown(week: VolksbgEintragungswoche): {
		phase: Phase;
		days: number;
		progress: number;
	} {
		const now = new Date();
		const today = startOfDay(now);
		const start = dashDateToDate(week.start_date);
		const end = dashDateToDate(week.end_date);
		const deadline = deadlineOf(week);

		if (!start || !end || !deadline || now.getTime() >= deadline.getTime()) {
			return { phase: 'ended', days: 0, progress: 1 };
		}

		if (today.getTime() < startOfDay(start).getTime()) {
			const reference =
				dashDateToDate(week.cut_off_date) ??
				new Date(start.getTime() - DEFAULT_COUNTDOWN_DAYS * DAY_MS);
			const span = start.getTime() - reference.getTime();
			const progress = span > 0 ? clamp01((now.getTime() - reference.getTime()) / span) : 0;
			return { phase: 'upcoming', days: daysBetween(today, start), progress };
		}

		const progress = clamp01(
			(now.getTime() - start.getTime()) / (deadline.getTime() - start.getTime())
		);
		return { phase: 'running', days: Math.max(1, daysBetween(today, end) + 1), progress };
	}

	const status = $derived(week ? countdown(week) : null);
	const ringLabel = $derived(status?.days === 1 ? t('volksbg.ringDay') : t('volksbg.ringDays'));
	const ringAriaLabel = $derived(
		status?.phase === 'ended' ? t('volksbg.ended') : `${status?.days ?? 0} ${ringLabel}`
	);
</script>

{#if week && status && week.volksbgs && week.volksbgs.length > 0}
	<section
		class="mt-3 w-full rounded-xl bg-secondary-500 p-4 text-black shadow-md dark:bg-secondary-600 dark:text-white"
		aria-labelledby="volksbg-reminder-title"
	>
		<div class="flex flex-wrap items-center gap-x-5 gap-y-3">
			<!-- Countdown ring -->
			<div class="relative h-20 w-20 shrink-0" role="img" aria-label={ringAriaLabel}>
				<svg viewBox="0 0 100 100" class="h-full w-full -rotate-90">
					<circle
						cx="50"
						cy="50"
						r={RADIUS}
						class="fill-none stroke-black/10 dark:stroke-black/20"
						stroke-width="9"
					/>
					<circle
						cx="50"
						cy="50"
						r={RADIUS}
						class="fill-none transition-[stroke-dashoffset] duration-1000 ease-out {status.phase ===
						'ended'
							? 'stroke-white/60'
							: 'stroke-white'}"
						stroke-width="9"
						stroke-linecap="round"
						stroke-dasharray={CIRCUMFERENCE}
						stroke-dashoffset={CIRCUMFERENCE * (1 - status.progress)}
					/>
				</svg>
				<div class="absolute inset-0 flex flex-col items-center justify-center leading-none">
					{#if status.phase === 'ended'}
						<svg viewBox="0 0 24 24" class="h-8 w-8 stroke-current" fill="none" aria-hidden="true">
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2.5"
								d="M5 13l4 4L19 7"
							/>
						</svg>
					{:else}
						<span class="text-2xl font-bold">{status.days}</span>
						<span class="mt-1 text-[0.65rem] font-semibold tracking-wider uppercase"
							>{ringLabel}</span
						>
					{/if}
				</div>
			</div>

			<div class="min-w-48 flex-1">
				<h2 id="volksbg-reminder-title" class="text-xl font-bold">{t('volksbg.title')}</h2>
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
						class="group flex h-full flex-col rounded-lg bg-white/70 p-3 transition-colors hover:bg-white dark:bg-white/10 dark:hover:bg-white/20"
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

		<p class="mt-3 text-xs text-gray-800 dark:text-gray-200">
			{t('volksbg.disclaimer')}
		</p>
	</section>
{/if}
