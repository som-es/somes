<script lang="ts">
	import { Popover } from 'bits-ui';
	import FilterDropdown from './FilterDropdown.svelte';
	import type { SvelteSet } from 'svelte/reactivity';
	import searchIcon from '$lib/assets/misc_icons/search-glass.svg?raw';
	import { cachedUserTopics } from '$lib/caching/user_topics_cache.svelte';
	import { onMount, type Snippet } from 'svelte';
	import type { UniqueTopic } from '$lib/types';

	interface Props {
		title: string;
		selectedValues: SvelteSet<string>;
		values: string[];
		prefillSnippet?: Snippet<[SvelteSet<string>]>;
	}

	let {
		selectedValues: selectedTopics = $bindable(),
		title,
		values,
		prefillSnippet
	}: Props = $props();

	let isTopicFilterOpen = $state(false);

	let topicSearchValue = $state('');
	let activeTopicFiltersCount = $derived(selectedTopics.size);
</script>

<Popover.Root bind:open={isTopicFilterOpen}>
	<Popover.Trigger
		class="flex h-full grow touch-manipulation items-center justify-center gap-1 rounded-xl bg-secondary-500 px-2 md:grow-0"
	>
		<FilterDropdown
			{title}
			activefilterCount={activeTopicFiltersCount}
			isOpen={isTopicFilterOpen}
		/>
	</Popover.Trigger>
	<Popover.Content sideOffset={8}>
		<div
			class="z-10 w-72 touch-manipulation rounded-xl border border-gray-300 bg-surface-50 shadow-lg dark:bg-surface-600"
			data-popup="popupTopics"
		>
			<!-- Search bar -->
			<div class="flex items-center gap-2 border-b border-gray-400 px-2 py-1">
				<div class="flex h-9 w-10 items-center justify-center text-gray-600 dark:text-gray-200">
					{@html searchIcon}
				</div>
				<input
					type="search"
					class="block w-full bg-transparent py-2 placeholder:text-gray-600 focus:outline-none dark:placeholder:text-gray-300"
					placeholder="Suche nach {title}..."
					bind:value={topicSearchValue}
				/>
			</div>
			<div class="flex justify-between gap-1 px-1 pt-1">
				{#if prefillSnippet}
					{@render prefillSnippet()}
				{:else}
					<span></span>
				{/if}
				<button onclick={() => selectedTopics.clear()} class="badge bg-primary-500 text-white">
					Zurücksetzen
				</button>
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
						<div class="min-h-4 min-w-4 rounded-md bg-primary-500"></div>
						<span class="text-left text-sm font-semibold text-gray-800 dark:text-gray-300"
							>{topic}</span
						>
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
						<div class="min-h-4 min-w-4 rounded-md border-[2px] border-primary-500"></div>
						<span class="text-left text-sm text-gray-800 dark:text-gray-300">{topic}</span>
					</button>
				{/each}
			</div>
			<Popover.Arrow
				class="rounded-sm fill-current stroke-gray-300 text-gray-300 dark:stroke-gray-600 dark:text-gray-600"
			/>
		</div>
	</Popover.Content>
</Popover.Root>
