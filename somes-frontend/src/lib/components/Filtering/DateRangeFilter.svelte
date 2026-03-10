<script lang="ts">
	import { Popover } from "bits-ui";
	import FilterDropdown from "./FilterDropdown.svelte";

	interface Props {
		dateFrom: string;
		dateTo: string;
	}

	let { dateFrom = $bindable(), dateTo = $bindable() }: Props = $props();

	let isDateFilterOpen = $state(false);

	let activeDateFiltersCount = $derived(
		(dateFrom ? 1 : 0) + (dateTo ? 1 : 0)
	);

	function clearDates() {
		dateFrom = '';
		dateTo = '';
	}
</script>

<Popover.Root bind:open={isDateFilterOpen}>
	<Popover.Trigger class="touch-manipulation flex h-full grow items-center justify-center gap-1 rounded-xl bg-secondary-500 px-2 md:grow-0">
		<FilterDropdown title="Datum" activefilterCount={activeDateFiltersCount} isOpen={isDateFilterOpen} />
	</Popover.Trigger>
	<Popover.Content sideOffset={8}>
		<div
			class="z-10 touch-manipulation text-black w-auto rounded-xl border border-gray-300 bg-surface-50 dark:bg-surface-600 dark:text-white px-5 pt-4 pb-5 shadow-lg md:px-6"
		>
			<div class="flex flex-col gap-3">
				<div>
					<label for="date-from" class="text-sm font-semibold text-gray-800 dark:text-gray-50">Von</label>
					<input
						id="date-from"
						type="date"
						class="mt-1 w-full rounded-lg border border-primary-300 dark:border-primary-400 bg-transparent px-2 py-1 text-sm focus:outline-none focus:ring-1 focus:ring-primary-400"
						bind:value={dateFrom}
					/>
				</div>
				<div>
					<label for="date-to" class="text-sm font-semibold text-gray-800 dark:text-gray-50">Bis</label>
					<input
						id="date-to"
						type="date"
						class="mt-1 w-full rounded-lg border border-primary-300 dark:border-primary-400 bg-transparent px-2 py-1 text-sm focus:outline-none focus:ring-1 focus:ring-primary-400"
						bind:value={dateTo}
					/>
				</div>
				{#if dateFrom || dateTo}
					<button
						class="cursor-pointer rounded-lg border border-primary-300 dark:border-primary-400 px-2 py-1 text-xs hover:bg-primary-300 dark:hover:bg-primary-400 md:text-sm"
						onclick={clearDates}
					>
						Zurücksetzen
					</button>
				{/if}
			</div>
			<Popover.Arrow class="rounded-sm fill-current stroke-gray-300 text-gray-300" />
		</div>
	</Popover.Content>
</Popover.Root>
