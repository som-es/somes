<script lang="ts">
	import { popup, type PopupSettings } from '@skeletonlabs/skeleton';
	import PlenarCalendar from './PlenarCalendar.svelte';
	import { dashDateToDotDate } from '$lib/date';
	import { onMount } from 'svelte';
	import { next_plenar_date } from './api';
	import { errorToNull } from '$lib/api/api';

	const plenarCalendar: PopupSettings = {
		event: 'click',
		target: 'plenarCalendar',
		placement: 'bottom',
		closeQuery: 'none'
	};

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
	<button class="btn sm:btn-lg variant-filled mt-1" use:popup={plenarCalendar} on:click={() => {}}
		>Sitzungskalender</button
	>

	<div class="z-40 max-w-sm:min-w-[34rem] sm: md:min-w-3xl" data-popup="plenarCalendar">
		<PlenarCalendar />
	</div>
</div>
