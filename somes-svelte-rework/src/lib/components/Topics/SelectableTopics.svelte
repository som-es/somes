<script lang="ts">
	import type { HasError, UniqueTopic } from '$lib/types';
	import { onMount } from 'svelte';
	import ClickableSpan from '../Utils/ClickableSpan.svelte';
	import DisplayTopic from './Topic.svelte';
	import ExpandablePlaceholder from '../VoteResults/Expandable/Placeholders/ExpandablePlaceholder.svelte';
	import { addUserTopic, getUserTopics, isHasError, removeUserTopic } from '$lib/api';
	import { cachedUserTopics } from '$lib/caching/user_topics_cache';

	export let selectedTopics: Set<number>;
	export let topics: UniqueTopic[] = [];
	let done = false;

	onMount(async () => {
		const data: UniqueTopic[] | HasError = await getUserTopics();

		if (!isHasError(data)) {
			selectedTopics = new Set<number>(data.map((topic) => topic.id));
			topics = topics;
		}

		done = true;
	});

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
</script>

<div class="flex flex-wrap gap-2 px-1">
	{#if done && selectedTopics}
		{#each topics as topic}
			{#if selectedTopics.has(topic.id)}
				<ClickableSpan action={() => removeFromSelection(topic)}>
					<DisplayTopic class={'!bg-secondary-400'}>{topic.topic}</DisplayTopic>
				</ClickableSpan>
			{:else}
				<ClickableSpan action={() => addToSelection(topic)}>
					<DisplayTopic>{topic.topic}</DisplayTopic>
				</ClickableSpan>
			{/if}
		{/each}
	{:else}
		{#each { length: 15 } as _}
			<ExpandablePlaceholder />
		{/each}
	{/if}
</div>
