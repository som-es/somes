<script lang="ts">
	import { Popover } from "bits-ui";
	import type { GenericFilterGroup } from "./types";
	import FilterDropdown from "./FilterDropdown.svelte";
	import FilterGroup from "./FilterGroup.svelte";

    interface Props {
        genericFilters: (GenericFilterGroup<string> | GenericFilterGroup<boolean>)[];
        legisPeriodFilter?: GenericFilterGroup<string>;
    }

    let { genericFilters = $bindable(), legisPeriodFilter = $bindable() }: Props = $props();

    let activeGenericFiltersCount = $derived(
		genericFilters.filter((f) => f.activeValue !== undefined && f.activeValue !== 'all').length + (legisPeriodFilter ? +(legisPeriodFilter?.activeValue !== "all") : 0)
	);
	let isGenericFilterOpen = $state(false);
</script>
<Popover.Root bind:open={isGenericFilterOpen}>
    <Popover.Trigger class="touch-manipulation flex h-full grow items-center justify-center gap-1 rounded-xl bg-secondary-500 px-2 md:grow-0">
        <FilterDropdown title="Filter" activefilterCount={activeGenericFiltersCount} isOpen={isGenericFilterOpen} />
    </Popover.Trigger>
    <Popover.Content sideOffset={8}>
        <div
            class="z-10 touch-manipulation text-black w-auto rounded-xl border border-gray-300 bg-surface-50 dark:bg-surface-600 dark:text-white px-5 pt-4 pb-5 shadow-lg md:px-6"
        >
            {#each genericFilters as group, i }
                <FilterGroup bind:group={genericFilters[i]} /> 
            {/each}
            {#if legisPeriodFilter}
                <div class="mt-4 first:mt-0">
                    <span class="text-base font-semibold text-gray-800 dark:text-gray-50">{legisPeriodFilter.title}</span>
                    <div class="flex w-72 flex-wrap gap-1 text-sm">
                        {#each legisPeriodFilter.options as option}
                            <button
                                class="cursor-pointer rounded-lg border border-primary-300 {legisPeriodFilter.activeValue === option.value ? "bg-primary-300 dark:bg-primary-400" : ""} px-2 py-1 text-xs md:text-sm"
                                onclick={() => {
                                    legisPeriodFilter.activeValue = option.value;
                                }}
                            >
                                <span class="text-nowrap">{option.title}</span>
                            </button>
                        {/each}
                    </div>
                </div>
            {/if}
            <Popover.Arrow class="rounded-sm fill-current stroke-gray-300 text-gray-300" />
        </div>
    </Popover.Content>
</Popover.Root>