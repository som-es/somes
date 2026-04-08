<script lang="ts">
	import type { UniqueTopic } from '$lib/types';
	import { Accordion } from 'bits-ui';
	import ClickableSpan from '../Utils/ClickableSpan.svelte';
	import DisplayTopic from './Topic.svelte';
	import { addUserTopic, removeUserTopic } from '$lib/api/authed';
	import { cachedUserTopics } from '$lib/caching/user_topics_cache.svelte';
	import arrowDown from '$lib/assets/misc_icons/down-arrow.svg?raw';
	import checkmark from '$lib/assets/misc_icons/checkmark_small.svg?raw';

	interface Props {
		parentTopics: string[];
		subTopics: UniqueTopic[];
		selectedTopics: Set<number>;
	}

	let { parentTopics, subTopics, selectedTopics = $bindable() }: Props = $props();

	let addToSelection = async (topic: UniqueTopic) => {
		selectedTopics.add(topic.id);
		selectedTopics = new Set(selectedTopics);

		await addUserTopic(topic);
		await cachedUserTopics(true);
	};

	let removeFromSelection = async (topic: UniqueTopic) => {
		selectedTopics.delete(topic.id);
		selectedTopics = new Set(selectedTopics);

		await removeUserTopic(topic);
		await cachedUserTopics(true);
	};

	let allSelected = $derived(subTopics.every((t) => selectedTopics.has(t.id)));
	let someSelected = $derived(subTopics.some((t) => selectedTopics.has(t.id)));

	let toggleAll = async () => {
		if (allSelected) {
			await Promise.all(subTopics.map(removeFromSelection));
		} else {
			const unselected = subTopics.filter((t) => !selectedTopics.has(t.id));
			await Promise.all(unselected.map(addToSelection));
		}
	};
</script>

<Accordion.Root type="multiple">
	<Accordion.Item value={parentTopics.join('-')}>
		<Accordion.Trigger
			class="group flex w-full items-center justify-between gap-2 rounded-xl bg-primary-100 px-4 py-3
           text-left hover:bg-primary-200 data-[state=open]:rounded-b-none
           data-[state=open]:bg-primary-200 dark:bg-primary-700
           dark:hover:bg-primary-600 dark:data-[state=open]:bg-primary-600"
		>
			<div class="flex min-w-0 flex-1 items-center gap-2">
				<button
					onclick={(e) => { e.stopPropagation(); toggleAll(); }}
					class="flex h-4 w-4 shrink-0 items-center justify-center rounded border-2
					       transition-colors
					       {allSelected
							? 'border-secondary-500 bg-secondary-400 text-white'
							: someSelected
								? 'border-secondary-400 bg-secondary-400 hover:border-secondary-500 dark:border-secondary-400'
								: 'border-primary-400 bg-transparent hover:border-secondary-400 dark:border-primary-400'}"
				>
					{#if allSelected}
						<span class="w-3 mt-0.5 [&_path]:stroke-white">{@html checkmark}</span>
					{/if}
				</button>
				<span class="truncate text-sm font-semibold text-primary-900 dark:text-primary-100">
					{parentTopics.join('  ·  ')}
				</span>
			</div>
			<span class="shrink-0 text-xs text-primary-500 dark:text-primary-400">{subTopics.length} topics</span>
			<span class="w-4">{@html arrowDown}</span>
		</Accordion.Trigger>

		<Accordion.Content>
			<div
				class="flex flex-wrap gap-2 rounded-b-xl border-t border-primary-300 bg-primary-200
                px-3 py-3
                dark:border-primary-500 dark:bg-primary-600"
			>
				{#each subTopics as topic (topic.id)}
					{#if selectedTopics.has(topic.id)}
						<ClickableSpan action={() => removeFromSelection(topic)}>
							<DisplayTopic class="bg-secondary-400!">
								{topic.topic}
							</DisplayTopic>
						</ClickableSpan>
					{:else}
						<ClickableSpan action={() => addToSelection(topic)}>
							<DisplayTopic>
								{topic.topic}
							</DisplayTopic>
						</ClickableSpan>
					{/if}
				{/each}
			</div>
		</Accordion.Content>
	</Accordion.Item>
</Accordion.Root>
