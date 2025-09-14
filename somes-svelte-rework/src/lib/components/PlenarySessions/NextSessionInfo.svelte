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

	let nextPlenarySessionDate: string | undefined = undefined;
	onMount(async () => {
		nextPlenarySessionDate = errorToNull(await next_plenar_date())?.date?.toString();
		if (nextPlenarySessionDate) {
			const today = new Date();
			const nextDate = new Date(nextPlenarySessionDate);
			days = Math.round((nextDate.getTime() - today.getTime()) / (1000 * 60 * 60 * 24));
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
			{#if nextPlenarySessionDate}
				{dashDateToDotDate(nextPlenarySessionDate)}
				<span class="sm:text-lg"
					>({#if days == 0}
						heute
					{:else}
						in {#if days == 1}
							1 Tag)
						{:else}
							{days} Tagen)
						{/if}
					{/if}</span
				>
			{/if}
		</span>
	</div>
	<button class="btn sm:btn-lg variant-filled mt-1" use:popup={plenarCalendar} on:click={() => {}}
		>Sitzungskalender</button
	>

	<div class="z-40 max-w-sm:min-w-[34rem] sm: md:min-w-[48rem]" data-popup="plenarCalendar">
		<PlenarCalendar />
	</div>
</div>
