<script lang="ts">
	import { Popover } from 'bits-ui';
	import type { Day } from './types';

	export let headers: string[] = [];
	export let days: Day[] = [];
	export let title: string;
	export let leftTitle: string;
	export let rightTitle: string;
	export let month: number;
	export let year: number;

	// const plenarSessionInfo: PopupSettings = {
	// 	event: 'click',
	// 	target: 'plenarSessionInfo',
	// 	placement: 'top',
	// 	closeQuery: 'none'
	// };

	let days2D: (Day | null)[][] = [];

	$: {
		days2D = [];
		let acc: (Day | null)[] = [null, null, null, null, null];
		days.forEach((day, i) => {
			acc[day.date.getDay() - 1] = day;
			if (day.date.getDay() == 5) {
				days2D.push(acc);
				acc = [null, null, null, null, null];
			}
		});
		if (acc[0] != null) {
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

<div class="calendar w-full max-w-200 rounded-xl bg-primary-200 dark:bg-primary-600">
	<div class="flex flex-col">
		<div
			class="flex flex-wrap items-center justify-around gap-1 bg-primary-400 py-1 dark:bg-surface-600"
		>
			<button class="px-3 sm:px-5" on:click={() => year--}>{year - 1}</button>
			<button class="px-3 sm:px-5" on:click={prev}>{leftTitle}</button>
			<span class="flex-1 text-center text-xl font-bold text-pretty sm:text-3xl">
				{title}
			</span>
			<button class="px-3 sm:px-5" on:click={next}>{rightTitle}</button>
			<button class="px-3 sm:px-5" on:click={() => year++}>{year + 1}</button>
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
							<Popover.Root>
								<div class="flex flex-1 items-center md:hidden">
									{#if week[i].item !== null}
										<Popover.Trigger class="day bg-tertiary-400">
											{week[i].name}
										</Popover.Trigger>
										<Popover.Portal>
											<Popover.Content
												class="z-90 mt-3 flex min-w-40 items-center justify-center sm:min-w-56"
											>
												<div class={week[i].item.classes}>{week[i].item.title}</div>
											</Popover.Content>
										</Popover.Portal>
									{:else}
										<div class="day">{week[i].name}</div>
									{/if}
								</div>
							</Popover.Root>
							<div class="flex min-w-37 flex-1 items-center max-md:hidden">
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
	<!-- <div class="z-50" data-popup="plenarSessionInfo">
		<div class="badge text-xs bg-tertiary-500 text-black">Plenarsitzung</div>
	</div> -->
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

	:global(.day) {
		border-bottom: 1px solid rgba(166, 168, 179, 0.12);
		border-right: 1px solid rgba(166, 168, 179, 0.12);
		text-align: right;
		padding: 10px 12px;
		font-size: 12px;
		color: #98a0a6;
		flex: 1;
		min-height: 48px;
	}

	@media (min-width: 640px) {
		:global(.day) {
			padding: 14px 20px;
			font-size: 14px;
			min-height: 60px;
		}
	}

	.day-name {
		font-size: 10px;
		text-transform: uppercase;
		text-align: center;
		border-bottom: 1px solid rgba(166, 168, 179, 0.12);
		line-height: 40px;
		font-weight: 500;
		flex: 1;
	}

	@media (min-width: 640px) {
		.day-name {
			font-size: 12px;
			line-height: 50px;
		}
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
