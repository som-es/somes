<script lang="ts">
	import { Popover } from "bits-ui";
	import type { GenericFilterGroup } from "./types";
	import FilterDropdown from "./FilterDropdown.svelte";

    interface Props {
        genericFilters: (GenericFilterGroup<string> | GenericFilterGroup<boolean>)[];
        legisPeriodFilter?: GenericFilterGroup<string>;
    }

    let { genericFilters = $bindable(), legisPeriodFilter = $bindable() }: Props = $props();

    let activeGenericFiltersCount = $derived(
		genericFilters.filter((f) => f.activeValue !== undefined && f.activeValue !== 'all').length + +(legisPeriodFilter?.activeValue !== "all")
	);
	let isGenericFilterOpen = $state(false);
</script>
<Popover.Root bind:open={isGenericFilterOpen}>
    <Popover.Trigger>
        <FilterDropdown title="Filter" activefilterCount={activeGenericFiltersCount} isOpen={isGenericFilterOpen} />
    </Popover.Trigger>
    <Popover.Content sideOffset={8}>
        <div
            class="z-10 w-auto rounded-xl border border-gray-300 bg-surface-50 px-5 pt-4 pb-5 shadow-lg md:px-6"
        >
            {#each genericFilters as group}
                {#if !group.hidden}
                    <div class="mt-4 first:mt-0">
                        <span class="text-base font-semibold text-gray-800">{group.title}</span>
                        <div class="flex w-fit gap-1 rounded-lg border border-primary-300 text-sm">
                            {#each group.options as option}
                                <button
                                    class="cursor-pointer rounded-lg px-2 py-1 text-xs md:text-sm"
                                    class:bg-primary-300={group.activeValue === option.value}
                                    onclick={() => {
                                        group.activeValue = option.value;
                                    }}
                                >
                                    <span class="text-nowrap">{option.title}</span>
                                </button>
                            {/each}
                        </div>
                    </div>
                {/if}
            {/each}
            {#if legisPeriodFilter}
                <div class="mt-4 first:mt-0">
                    <span class="text-base font-semibold text-gray-800">{legisPeriodFilter.title}</span>
                    <div class="flex w-72 flex-wrap gap-1 text-sm">
                        {#each legisPeriodFilter.options as option}
                            <button
                                class="cursor-pointer rounded-lg border border-primary-300 px-2 py-1 text-xs md:text-sm"
                                class:bg-primary-300={legisPeriodFilter.activeValue === option.value}
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