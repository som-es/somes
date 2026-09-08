<script lang="ts">
	import type { UniqueTopic } from '$lib/types';
	import { translateTopicToParent } from '$lib/interestColors';
	import AccordionTopics from './AccordionTopics.svelte';

	interface Props {
		selectedTopics: Set<string>;
		topics?: UniqueTopic[];
	}

	let { selectedTopics = $bindable(), topics = [] }: Props = $props();

	function createGroupTopics(topics: UniqueTopic[]): Map<string, UniqueTopic[]> {
		const groupedTopics = new Map<string, UniqueTopic[]>();
		topics.forEach((topic) => {
			const parentTopic = translateTopicToParent(topic.id);
			if (groupedTopics.has(parentTopic)) {
				groupedTopics.get(parentTopic)?.push(topic);
			} else {
				groupedTopics.set(parentTopic, [topic]);
			}
		});
		return groupedTopics;
	}

	function createCombinedGroupings(topics: UniqueTopic[]): {
		others: UniqueTopic[];
		groupTopicsEntries: [string, UniqueTopic[]][];
		combinedGroups: { parentTopics: string[]; topics: UniqueTopic[] };
	} {
		const groupTopics = createGroupTopics(topics);
		const combinedGroups: { parentTopics: string[]; topics: UniqueTopic[] } = {
			parentTopics: [],
			topics: []
		};
		groupTopics.forEach((topics, parent) => {
			if (topics.length <= 4) {
				combinedGroups.parentTopics.push(parent);
				combinedGroups.topics.push(...topics);
				groupTopics.delete(parent);
			}
		});
		const others = groupTopics.get('Sonstige') ?? [];
		groupTopics.delete('Sonstige');

		return { others, groupTopicsEntries: Array.from(groupTopics.entries()).sort(), combinedGroups };
	}

	let groupedTopics = $derived(createCombinedGroupings(topics));
</script>

<div class="flex flex-col gap-2 px-1">
	{#if topics.length > 0}
		{#each groupedTopics.groupTopicsEntries as [parentTopic, subTopics]}
			<AccordionTopics parentTopics={[parentTopic]} {subTopics} bind:selectedTopics />
		{/each}
		{#if groupedTopics.combinedGroups.topics.length > 0}
			<AccordionTopics
				parentTopics={groupedTopics.combinedGroups.parentTopics}
				subTopics={groupedTopics.combinedGroups.topics}
				bind:selectedTopics
			/>
		{/if}
		{#if groupedTopics.others.length > 0}
			<AccordionTopics
				parentTopics={['Sonstige']}
				subTopics={groupedTopics.others}
				bind:selectedTopics
			/>
		{/if}
	{/if}
</div>
