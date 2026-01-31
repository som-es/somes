<script lang="ts">
	import { Popover } from "bits-ui";
	import FilterDropdown from "./FilterDropdown.svelte";
	import type { SvelteSet } from "svelte/reactivity";
	import searchIcon from '$lib/assets/misc_icons/search-glass.svg?raw';

    interface Props {
        title: string;
        selectedValues: SvelteSet<string>
        values: string[]
    }

    let { selectedValues: selectedTopics = $bindable(), title, values }: Props = $props();

	let isTopicFilterOpen = $state(false);

	let topicSearchValue = $state('');
	let activeTopicFiltersCount = $derived(selectedTopics.size);
</script>

<Popover.Root bind:open={isTopicFilterOpen}>
    <Popover.Trigger class="touch-manipulation flex h-full grow items-center justify-center gap-1 rounded-xl bg-secondary-500 px-2 md:grow-0">
        <FilterDropdown {title} activefilterCount={activeTopicFiltersCount} isOpen={isTopicFilterOpen} />
    </Popover.Trigger>
    <Popover.Content sideOffset={8}>
        <div
            class="z-10 w-72 touch-manipulation rounded-xl border border-gray-300 bg-surface-50 shadow-lg"
            data-popup="popupTopics"
        >
            <!-- Search bar -->
            <div class="flex items-center gap-2 border-b border-gray-400 px-2 py-1">
                <div class="flex h-9 w-10 items-center justify-center text-gray-600">
                    {@html searchIcon}
                </div>
                <input
                    type="search"
                    class="block w-full bg-transparent py-2 placeholder:text-gray-600 focus:outline-none"
                    placeholder="Suche nach {title}..."
                    bind:value={topicSearchValue}
                />
            </div>
            <div class="flex max-h-72 flex-col gap-1 overflow-y-auto px-3 py-2">
                <!-- Selected topics first -->
                {#each values.filter((t) => selectedTopics.has(t) && t
                            .toLowerCase()
                            .includes(topicSearchValue.toLowerCase())) as topic}
                    <button
                        class="flex cursor-pointer items-center gap-2"
                        onclick={() => {
                            selectedTopics.delete(topic);
                        }}
                    >
                        <div class="h-4 w-4 rounded-md bg-primary-500"></div>
                        <span class="text-left text-sm font-semibold text-gray-800">{topic}</span>
                    </button>
                {/each}
                <!-- Unselected topics -->
                {#each values.filter((t) => !selectedTopics.has(t) && t
                            .toLowerCase()
                            .includes(topicSearchValue.toLowerCase())) as topic}
                    <button
                        class="flex cursor-pointer items-center gap-2"
                        onclick={() => {
                            selectedTopics.add(topic);
                        }}
                    >
                        <div class="h-4 w-4 rounded-md border-[2px] border-primary-500"></div>
                        <span class="text-left text-sm text-gray-800">{topic}</span>
                    </button>
                {/each}
            </div>
            <Popover.Arrow class="rounded-sm fill-current stroke-gray-300 text-gray-300" />
        </div>
    </Popover.Content>
</Popover.Root>