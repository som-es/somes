<script lang="ts">
	import PlenarCalendar from './PlenarCalendar.svelte';
	import { dashDateToDotDate } from '$lib/date';
	import { onMount } from 'svelte';
	import { Popover, Portal } from 'bits-ui';

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

	let hours: number | null = $derived(days == null ? null : days < 1 ? Math.round(days * 24) : null);
</script>

<div
	class="mt-3 flex items-center gap-4 bg-primary-300 dark:bg-primary-500 p-3 rounded-xl justify-between w-full flex-wrap shadow-md"
>
	<div>
		<div class="text-xl font-bold">Nächste Nationalratssitzung</div>
		<span class="text-base text-gray-800"
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
	<Popover.Root>
		<!-- ToDo: Does currently not show Calendar on mobile as it is not responsive -->
		<Popover.Trigger
			class="rounded-input bg-dark hidden md:inline-flex
			text-background shadow-mini hover:bg-dark/95 h-10 select-none items-center justify-center whitespace-nowrap px-[21px] text-[15px] font-medium transition-all hover:cursor-pointer active:scale-[0.98]"
		>
			<span class="btn bg-primary-500 dark:bg-primary-300 text-white preset-filled mt-1">
				Sitzungskalender
			</span>
		</Popover.Trigger>
		<Popover.Portal>
			<Popover.Content
				class="
				data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 
				data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 
				z-50 max-w-sm:min-w-136 mt-3 sm: md:min-w-3xl bg-primary-400"
			>
				<PlenarCalendar />
			</Popover.Content>
		</Popover.Portal>
  </Popover.Root>	

<!-- 
	<div class="z-40 max-w-sm:min-w-[34rem] sm: md:min-w-3xl" data-popup="plenarCalendar">
	</div> -->
</div>
