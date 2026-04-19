<script lang="ts">
	import type { FullMandate, LegisPeriod } from '$lib/types';
	import { partyToColor } from '$lib/partyColor';
	import { Tooltip } from 'bits-ui';

	interface Props {
		mandates: FullMandate[];
		periods: LegisPeriod[];
		gender?: string | null;
	}

	let { mandates, periods, gender = null }: Props = $props();

	const now = new Date();

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

	// Sort periods oldest-first regardless of input order
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

	// Ordinal scale: each GP period gets an equal horizontal slot.
	// Dates within a slot are interpolated linearly inside that slot.
	function ordinalPercent(date: Date): number {
		const ticks = visibleTicks;
		const n = ticks.length;
		if (n === 0) return 0;
		if (n === 1) {
			const slotStart = new Date(ticks[0].start_date);
			const slotEnd = new Date(Math.max(now.getTime(), timeRange.max.getTime()));
			if (date <= slotStart) return 0;
			if (date >= slotEnd) return 100;
			const duration = slotEnd.getTime() - slotStart.getTime();
			return duration > 0 ? ((date.getTime() - slotStart.getTime()) / duration) * 100 : 0;
		}

		const slotWidth = 100 / n;

		for (let i = 0; i < n; i++) {
			const slotStart = new Date(ticks[i].start_date);
			const slotEnd = i + 1 < n ? new Date(ticks[i + 1].start_date) : new Date(Math.max(now.getTime(), timeRange.max.getTime()));

			if (date <= slotStart && i === 0) return 0;

			if (date <= slotEnd) {
				const slotDuration = slotEnd.getTime() - slotStart.getTime();
				const fraction = slotDuration > 0 ? (date.getTime() - slotStart.getTime()) / slotDuration : 0;
				const clampedFraction = Math.max(0, Math.min(1, fraction));
				return (i + clampedFraction) * slotWidth;
			}
		}

		return 100;
	}

	function barStyle(mandate: FullMandate): string {
		const start = mandate.start_date ? new Date(mandate.start_date) : timeRange.min;
		const end = mandate.end_date ? new Date(mandate.end_date) : now;
		const left = ordinalPercent(start);
		const width = Math.max(ordinalPercent(end) - left, 1);
		const color = partyToColor(mandate.party);
		return `left: ${left}%; width: ${width}%; background-color: ${color};`;
	}

	let todayPercent = $derived(ordinalPercent(now));
	let hasActiveMandate = $derived(mandates.some((m) => !m.end_date));

	let hoverStates = $state<boolean[]>([]);
	let hoverTimeouts = new Map<number, ReturnType<typeof setTimeout>>();

	function handleRowEnter(index: number, e?: Event) {
		// Ignore pointer events from touch screens so swiping doesn't trigger tooltips.
		if (e && 'pointerType' in e && (e as PointerEvent).pointerType === 'touch') return;

		clearTimeout(hoverTimeouts.get(index));
		const timeout = setTimeout(() => {
			hoverStates[index] = true;
		}, 150);
		hoverTimeouts.set(index, timeout);
	}

	function handleRowLeave(index: number) {
		clearTimeout(hoverTimeouts.get(index));
		hoverStates[index] = false;
	}
</script>

<div>
	<h1 class="text-lg font-bold text-black xl:text-xl dark:text-white">Mandate</h1>
	<h2 class="text-sm text-primary-600 dark:text-primary-300">
		{mandates.length}
		{mandates.length === 1 ? 'Eintrag' : 'Einträge'}
	</h2>
</div>

<div class="mt-4 w-full overflow-x-auto md:overflow-visible pb-4">
	<div class="min-w-[500px] md:min-w-0">
		<!-- GP tick labels row -->
		<div class="flex gap-3">
			<div class="w-36 shrink-0"></div>
			<div class="relative mb-1 h-5 flex-1">
				{#each visibleTicks as period}
					{@const left = ordinalPercent(new Date(period.start_date))}
					<span
						class="absolute -translate-x-1/2 whitespace-nowrap text-xs text-gray-400 dark:text-gray-500"
						style="left: {left}%"
					>
						{period.gp}
					</span>
				{/each}
			</div>
		</div>

		<!-- Main layout: Unified rows with background ticks -->
		<div class="relative mt-2">
			<!-- Background layer for ticks & today marker -->
			<div class="absolute inset-0 left-[calc(9rem+0.75rem+0.5rem)] pointer-events-none">
				<!-- Vertical tick lines -->
				{#each visibleTicks as period}
					{@const left = ordinalPercent(new Date(period.start_date))}
					<div class="absolute top-0 h-full w-px bg-gray-200 dark:bg-gray-700" style="left: {left}%"></div>
				{/each}

				<!-- Today marker -->
				{#if hasActiveMandate}
					<div class="absolute top-0 z-10 h-full w-0.5 bg-primary-500 dark:bg-primary-400" style="left: {todayPercent}%"></div>
				{/if}
			</div>

			<!-- Foreground layer: Rows -->
			<div class="flex flex-col gap-1 relative z-10">
				<Tooltip.Provider delayDuration={150} disableCloseOnTriggerClick={true}>
					{#each mandates as mandate, i}
						<Tooltip.Root 
							open={hoverStates[i] ?? false} 
						>
							<div 
								class="group relative flex items-center gap-3 h-10 rounded-lg hover:bg-primary-200 dark:hover:bg-primary-200 transition-colors px-2"
								onpointerenter={(e) => handleRowEnter(i, e)}
								onpointerleave={(e) => handleRowLeave(i)}
								onfocus={() => handleRowEnter(i)}
								onblur={() => handleRowLeave(i)}
								tabindex="0"
								role="button"
							>
								<!-- Label -->
								<div class="flex w-36 shrink-0 flex-col justify-center pointer-events-none">
									<span class="truncate text-sm font-semibold text-gray-800 dark:text-gray-200">
										{getMandateType(mandate)}
									</span>
									<span class="text-xs text-gray-400 dark:text-gray-500">
										{formatYear(mandate.start_date)} - {mandate.end_date ? formatYear(mandate.end_date) : 'dato'}
									</span>
								</div>

								<!-- Bar Container -->
								<div class="relative flex-1 h-7">
									<Tooltip.Trigger>
										{#snippet child({ props })}
											<div
												{...props}
												class="absolute h-full rounded-md transition-opacity opacity-85 group-hover:opacity-100 shadow-sm"
												style={barStyle(mandate)}
											></div>
										{/snippet}
									</Tooltip.Trigger>
								</div>
							</div>
							
							<Tooltip.Content 
								sideOffset={6} 
								class="z-[100] max-w-[300px] rounded-md bg-gray-900 px-3 py-2 text-xs text-white shadow-xl dark:bg-gray-100 dark:text-gray-900 pointer-events-none"
							>
								<Tooltip.Arrow class="text-gray-900 dark:text-gray-100" />
								<div class="font-bold text-sm mb-0.5 whitespace-normal">{mandate.name || getMandateType(mandate)}</div>
								<div class="text-gray-300 dark:text-gray-600">
									{mandate.start_date ? new Date(mandate.start_date).toLocaleDateString('de-AT') : '?'} – {mandate.end_date ? new Date(mandate.end_date).toLocaleDateString('de-AT') : 'dato'}
								</div>
							</Tooltip.Content>
						</Tooltip.Root>
					{/each}
				</Tooltip.Provider>
			</div>
		</div>
	</div>
</div>