<script lang="ts">
	import type { FullMandate, LegisPeriod } from '$lib/types';
	import { partyToColor } from '$lib/partyColor';
	import { formatDate } from '$lib/date';
	import { Tooltip } from 'bits-ui';

	interface Props {
		mandates: FullMandate[];
		periods: LegisPeriod[];
		gender?: string | null;
	}

	let { mandates, periods, gender = null }: Props = $props();

	const now = new Date();

	const ROW_GRID = 'grid grid-cols-[9rem_1fr] gap-x-3 px-2';

	function formatYear(dateStr: string | null): string {
		if (!dateStr) return '?';
		return new Date(dateStr).getFullYear().toString();
	}

	function getMandateType(mandate: FullMandate): string {
		const isFemale = gender === 'f';

		switch (mandate.function) {
			case 'EP':
				return 'EU Parlament';
			case 'BM':
				return isFemale ? 'Bundesministerin' : 'Bundesminister';
			case 'STS':
				return isFemale ? 'Staatssekretärin' : 'Staatssekretär';
			case 'NR':
				return 'Nationalrat';
			case 'VK':
				return isFemale ? 'Vizekanzlerin' : 'Vizekanzler';
			case 'L':
				return isFemale ? 'Betraute' : 'Betrauter';
			default:
				return 'Mandat';
		}
	}

	function mandateYears(mandate: FullMandate): string {
		const end = mandate.end_date ? formatYear(mandate.end_date) : 'dato';
		return `${formatYear(mandate.start_date)} - ${end}`;
	}

	let sortedMandates = $derived(
		[...mandates].sort((a, b) => (a.start_date ?? '').localeCompare(b.start_date ?? ''))
	);

	let sortedPeriods = $derived(
		[...periods].sort((a, b) => new Date(a.start_date).getTime() - new Date(b.start_date).getTime())
	);

	// Overall time range across all mandates
	let timeRange = $derived.by(() => {
		if (mandates.length === 0) return { min: now, max: now };
		const allDates = mandates.flatMap((m) => [
			m.start_date ? new Date(m.start_date) : now,
			m.end_date ? new Date(m.end_date) : now
		]);
		const min = new Date(Math.min(...allDates.map((d) => d.getTime())));
		const max = new Date(Math.max(...allDates.map((d) => d.getTime())));
		return { min, max };
	});

	// GP tick marks that fall within (or right before) the time range
	let visibleTicks = $derived.by(() => {
		if (sortedPeriods.length === 0) return [];

		let startIndex = 0;
		// Find the first period that started before or exactly at our minimum time
		for (let i = sortedPeriods.length - 1; i >= 0; i--) {
			if (new Date(sortedPeriods[i].start_date) <= timeRange.min) {
				startIndex = i;
				break;
			}
		}

		let endIndex = sortedPeriods.length - 1;
		// Find the first period that starts after or exactly at our maximum time
		for (let i = 0; i < sortedPeriods.length; i++) {
			if (new Date(sortedPeriods[i].start_date) >= timeRange.max) {
				endIndex = i;
				break;
			}
		}

		return sortedPeriods.slice(startIndex, endIndex + 1);
	});

	let ticks = $derived(
		visibleTicks.map((period, i) => ({
			gp: period.gp,
			percent: (i * 100) / visibleTicks.length
		}))
	);

	// Ordinal scale: each GP period gets an equal horizontal slot.
	// Dates within a slot are interpolated linearly inside that slot.
	function ordinalPercent(date: Date): number {
		const n = visibleTicks.length;
		if (n === 0) return 0;

		const slotWidth = 100 / n;
		// last GP runs till now
		const lastSlotEnd = new Date(Math.max(now.getTime(), timeRange.max.getTime()));

		for (let i = 0; i < n; i++) {
			const slotStart = new Date(visibleTicks[i].start_date);
			const slotEnd = i + 1 < n ? new Date(visibleTicks[i + 1].start_date) : lastSlotEnd;

			if (date <= slotStart) return i * slotWidth;

			if (date <= slotEnd) {
				const slotDuration = slotEnd.getTime() - slotStart.getTime();
				const fraction =
					slotDuration > 0 ? (date.getTime() - slotStart.getTime()) / slotDuration : 0;
				return (i + fraction) * slotWidth;
			}
		}

		return 100;
	}

	function barStyle(mandate: FullMandate): string {
		const start = mandate.start_date ? new Date(mandate.start_date) : timeRange.min;
		const end = mandate.end_date ? new Date(mandate.end_date) : now;
		const left = ordinalPercent(start);
		const width = Math.max(Math.min(ordinalPercent(end) - left, 100 - left), 1);
		const color = partyToColor(mandate.party);
		return `left: ${left}%; width: ${width}%; background-color: ${color};`;
	}

	let todayPercent = $derived(ordinalPercent(now));
	let hasActiveMandate = $derived(mandates.some((m) => !m.end_date));
</script>

<div>
	<h1 class="text-lg font-bold text-black xl:text-xl dark:text-white">Mandate</h1>
	<h2 class="text-sm text-gray-800 dark:text-gray-300">
		{mandates.length}
		{mandates.length === 1 ? 'Eintrag' : 'Einträge'}
	</h2>
</div>

{#if sortedMandates.length === 0}
	<p class="mt-4 text-sm text-gray-800 dark:text-gray-300">Keine Mandate vorhanden.</p>
{:else}
	<div class="mt-4 w-full overflow-x-auto pb-4 md:overflow-visible">
		<div class="min-w-[500px] md:min-w-0">
			<!-- title ticks -->
			<div class="{ROW_GRID} mb-1">
				<div></div>
				<div class="relative h-5">
					{#each ticks as tick (tick.gp)}
						<span
							class="absolute -translate-x-1/2 text-xs whitespace-nowrap text-gray-800 dark:text-gray-300"
							style="left: {tick.percent}%"
						>
							{tick.gp}
						</span>
					{/each}

					{#if hasActiveMandate}
						<span
							class="absolute -translate-x-full text-xs font-semibold whitespace-nowrap text-gray-800 dark:text-gray-300"
							style="left: {todayPercent}%"
						>
							dato
						</span>
					{/if}
				</div>
			</div>

			<!-- Hover Information -->
			<Tooltip.Provider delayDuration={0} disableCloseOnTriggerClick={true}>
				{#each sortedMandates as mandate, i (i)}
					<Tooltip.Root>
						<Tooltip.Trigger
							class="{ROW_GRID} group h-11 w-full cursor-default rounded-lg text-left transition-colors hover:bg-primary-200 dark:hover:bg-primary-200"
							aria-label="{getMandateType(mandate)}, {mandateYears(mandate)}"
						>
							<span class="flex min-w-0 flex-col justify-center">
								<span class="truncate text-sm font-semibold text-gray-800 dark:text-gray-200">
									{getMandateType(mandate)}
								</span>
								<span class="text-xs text-gray-800 dark:text-gray-300">
									{mandateYears(mandate)}
								</span>
							</span>

							<span class="relative block">
								{#each ticks as tick (tick.gp)}
									<span
										class="absolute top-0 h-full w-px bg-gray-600 dark:bg-gray-300"
										style="left: {tick.percent}%"
									></span>
								{/each}

								{#if hasActiveMandate}
									<span
										class="absolute top-0 h-full w-0.5 bg-primary-500 dark:bg-primary-400"
										style="left: {todayPercent}%"
									></span>
								{/if}

								<span
									class="absolute top-1/2 h-7 -translate-y-1/2 rounded-md opacity-85 shadow-sm transition-opacity group-hover:opacity-100"
									style={barStyle(mandate)}
								></span>
							</span>
						</Tooltip.Trigger>

						<Tooltip.Content
							sideOffset={6}
							class="pointer-events-none z-[100] max-w-[300px] rounded-md bg-gray-900 px-3 py-2 text-xs text-white shadow-xl dark:bg-gray-100 dark:text-gray-900"
						>
							<Tooltip.Arrow class="text-gray-900 dark:text-gray-100" />
							<div class="mb-0.5 text-sm font-bold whitespace-normal">
								{mandate.name || getMandateType(mandate)}
							</div>
							{#if mandate.party}
								<div class="mb-0.5 flex items-center gap-1.5">
									<span
										class="h-2 w-2 shrink-0 rounded-full"
										style="background-color: {partyToColor(mandate.party)}"
									></span>
									{mandate.party}
								</div>
							{/if}
							<div class="text-gray-300 dark:text-gray-600">
								{mandate.start_date ? formatDate(mandate.start_date) : '?'} – {mandate.end_date
									? formatDate(mandate.end_date)
									: 'dato'}
							</div>
						</Tooltip.Content>
					</Tooltip.Root>
				{/each}
			</Tooltip.Provider>
		</div>
	</div>
{/if}
