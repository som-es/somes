<script lang="ts">
	import { popup, type PopupSettings } from '@skeletonlabs/skeleton';
	import type { Day } from './types';

	export let headers: string[] = [];
	export let days: Day[] = [];
	export let title: string;
	export let month: number;
	export let year: number;

	const plenarSessionInfo: PopupSettings = {
		event: 'click',
		target: 'plenarSessionInfo',
		placement: 'top',
		closeQuery: 'none'
	};

	let days2D: Day[][] = [];

	$: {
		days2D = [];
		let acc: Day[] = [];
		days.forEach((day, i) => {
			if (i > 0 && i % 5 === 0) {
				days2D.push([...acc]);
				acc = [];
			}
			acc.push(day);
		});
		if (acc.length > 0) {
			for (let i = 0; i < 4 - acc.length; i++) {
				days2D.push();
			}
			days2D.push(acc);
		}
	}
	function next() {
		month++;
		if (month == 12) {
			year++;
			month = 0;
		}
	}
	function prev() {
		if (month == 0) {
			month = 11;
			year--;
		} else {
			month--;
		}
	}
</script>

<div class="calendar max-w-[50rem] bg-primary-100 dark:bg-primary-600 rounded-xl">
	<div class="flex flex-col">
		<div class="flex items-center justify-around bg-surface-300 dark:bg-surface-600 py-1">
			<button class="px-5" on:click={() => year--}>{'<<'} </button>
			<button class="px-5" on:click={prev}>{'<'}</button>
			<span class="text-3xl text-pretty font-bold text-center">
				{title}
			</span>
			<button class="px-5" on:click={next}>{'>'}</button>
			<button class="px-5" on:click={() => year++}>{'>>'}</button>
		</div>

		<div class="flex flex-row">
			{#each headers as header}
				<span class="day-name text-secondary-500">
					{header}
				</span>
			{/each}
		</div>

		{#each days2D as week}
			<div class="flex flex-row">
				{#each Array(5) as _, i}
					{#if week[i]}
						{#if week[i].enabled}
							<div class="sm:hidden flex-1 flex items-center">
								{#if week[i].item !== null}
									<button class="day bg-tertiary-400" use:popup={plenarSessionInfo}>
										{week[i].name}
									</button>
								{:else}
									<div class="day">{week[i].name}</div>
								{/if}
							</div>
							<div class="max-sm:hidden flex-1 flex items-center">
								{#if week[i].item !== null}
									<div class={week[i].item.classes}>{week[i].item.title}</div>
								{/if}
								<div class="day">{week[i].name}</div>
							</div>
						{:else}
							<div class="day day-disabled">{week[i].name}</div>
						{/if}
					{:else}
						<div class="flex-1">
							<div class="day empty-day h-full"></div>
						</div>
					{/if}
				{/each}
			</div>
		{/each}
	</div>
	<div class="z-50" data-popup="plenarSessionInfo">
		<div class="badge text-xs bg-tertiary-500 text-black">Plenarsitzung</div>
	</div>
</div>

<style>
	.calendar {
		overflow: auto;
		display: flex;
		flex-direction: column;
	}

	.flex-row {
		display: flex;
		flex-direction: row;
	}

	.day {
		border-bottom: 1px solid rgba(166, 168, 179, 0.12);
		border-right: 1px solid rgba(166, 168, 179, 0.12);
		text-align: right;
		padding: 14px 20px;
		font-size: 14px;
		color: #98a0a6;
		flex: 1; /* evenly distribute 5 days per row */
		box-sizing: border-box;
	}

	.day-name {
		font-size: 12px;
		text-transform: uppercase;
		text-align: center;
		border-bottom: 1px solid rgba(166, 168, 179, 0.12);
		line-height: 50px;
		font-weight: 500;
		flex: 1; /* evenly distribute headers */
	}

	.day-disabled {
		color: rgba(152, 160, 166, 0.5);
		background-color: #fff;
		cursor: not-allowed;
	}

	.task {
		border-left-width: 3px;
		padding: 8px 12px;
		margin: 10px;
		border-left-style: solid;
		font-size: 14px;
		position: relative;
		align-self: center;
		z-index: 2;
		border-radius: 15px;
	}
</style>
