<script lang="ts">
	import PlenarCalendar from './PlenarCalendar.svelte';
	import { dashDateToDotDate } from '$lib/date';
	import { onMount } from 'svelte';
	import { next_plenar_date } from './api';
	import { errorToNull } from '$lib/api/api';
	import { Popover, Portal } from 'bits-ui';

	// const plenarCalendar: PopupSettings = {
	// 	event: 'click',
	// 	target: 'plenarCalendar',
	// 	placement: 'bottom',
	// 	closeQuery: 'none'
	// };

	let days: number | null = null;

	let nextPlenarySessionDateStr: string | undefined = undefined;
	let hours: number | null = null;
	onMount(async () => {
		nextPlenarySessionDateStr = errorToNull(await next_plenar_date())?.date_and_time?.toString();
		if (nextPlenarySessionDateStr) {
			const today = new Date();
			const nextDate = new Date(nextPlenarySessionDateStr);
			days = (nextDate.getTime() - today.getTime()) / (1000 * 60 * 60 * 24);
			if (days < 1) {
				hours = Math.round(days * 24);
			}
		}
	});
</script>

<div
	class="mt-5 flex items-center gap-4 bg-primary-300 dark:bg-primary-500 p-3 rounded-[0.9rem] justify-between w-full flex-wrap"
>
	<div>
		<div class="mt-2 text-xl sm:text-3xl font-bold">Nächste Nationalratssitzung</div>
		<span class="text-xl sm:text-2xl"
			>am
			{#if nextPlenarySessionDateStr}
				{dashDateToDotDate(nextPlenarySessionDateStr.toString().split('T')[0])}
				<span class="sm:text-lg">
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
		<Popover.Trigger
			class="rounded-input bg-dark
			text-background shadow-mini hover:bg-dark/95 inline-flex h-10 select-none items-center justify-center whitespace-nowrap px-[21px] text-[15px] font-medium transition-all hover:cursor-pointer active:scale-[0.98]"
		>
			<button class="btn sm:btn-lg bg-primary-500 text-white preset-filled mt-1" on:click={() => {}}>
				Sitzungskalender
			</button>
		</Popover.Trigger>
		<Popover.Portal>
			<Popover.Content
				class="z-50 max-w-sm:min-w-136 mt-3 sm: md:min-w-3xl bg-primary-400"
			>
				<PlenarCalendar />
			</Popover.Content>
		</Popover.Portal>
  </Popover.Root>	
	<!-- <Popover placement="bottom" trigger="click"  transitionParams={{ duration: 200 }} class="z-40 max-w-sm:min-w-136 sm: md:min-w-3xl"> -->
	<!-- </Popover> -->

<!-- 
	<div class="z-40 max-w-sm:min-w-[34rem] sm: md:min-w-3xl" data-popup="plenarCalendar">
	</div> -->
</div>
