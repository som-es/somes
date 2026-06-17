<script lang="ts">
	import PlenarCalendar from './PlenarCalendar.svelte';
	import { dashDateToDotDate } from '$lib/date';
	import { Popover } from 'bits-ui';
	import calendarIcon from '$lib/assets/icons/calendar.svg?raw';

	interface Props {
		// };
		nextPlenarySessionDateStr?: string | null;
	}

	let { nextPlenarySessionDateStr = undefined }: Props = $props();

	let days: number | null = $derived.by(() => {
		if (nextPlenarySessionDateStr == null) return null;
		const today = new Date();
		const nextDate = new Date(nextPlenarySessionDateStr);
		return (nextDate.getTime() - today.getTime()) / (1000 * 60 * 60 * 24);
	});

	let hours: number | null = $derived(
		days == null ? null : days < 1 ? Math.round(days * 24) : null
	);
</script>

<div
	class="mt-3 flex w-full flex-wrap items-center justify-between gap-4 rounded-xl bg-primary-300 p-3 shadow-md dark:bg-primary-500"
>
	<div class="flex">
		<div class="mr-2 flex w-12 items-center justify-center">
			<div class="h-8 w-8">
				{@html calendarIcon}
			</div>
		</div>

		<div class="items-center gap-2">
			<div class="text-xl font-bold">Nächste Nationalratssitzung</div>
			<span class="text-base text-gray-800 dark:text-gray-200"
				>am
				{#if nextPlenarySessionDateStr}
					{dashDateToDotDate(nextPlenarySessionDateStr.toString().split('T')[0])}
					<span>
						{#if hours}
							(in {hours} Stunden)
						{:else if days}
							(in {#if days == 1}
								1 Tag)
							{:else}
								{Math.round(days)} Tagen)
							{/if}
						{/if}
					</span>
				{/if}
			</span>
		</div>
	</div>
	<Popover.Root>
		<!-- ToDo: Does currently not show Calendar on mobile as it is not responsive -->
		<Popover.Trigger
			class="rounded-input bg-dark text-background
			shadow-mini hover:bg-dark/95 inline-flex h-10 items-center justify-center text-[15px] font-medium whitespace-nowrap transition-all select-none hover:cursor-pointer active:scale-[0.98] sm:px-[21px]"
		>
			<span class="preset-filled mt-1 btn bg-primary-500 text-white dark:bg-surface-500">
				Sitzungskalender
			</span>
		</Popover.Trigger>
		<Popover.Portal>
			<Popover.Content
				class="
				z-50 mt-3 data-[state=closed]:animate-out
				data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95 data-[state=open]:animate-in
				data-[state=open]:fade-in-0  data-[state=open]:zoom-in-95 "
			>
				<PlenarCalendar />
			</Popover.Content>
		</Popover.Portal>
	</Popover.Root>

	<!--
	<div class="z-40 max-w-sm:min-w-[34rem] sm: md:min-w-3xl" data-popup="plenarCalendar">
	</div> -->
</div>
