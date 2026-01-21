<script lang="ts">
	import type { UniqueTopic } from '$lib/types';
	// import { Accordion } from '@skeletonlabs/skeleton-svelte';
	import ClickableSpan from '../Utils/ClickableSpan.svelte';
	import DisplayTopic from './Topic.svelte';
	import { addUserTopic, removeUserTopic } from '$lib/api/authed';
	import { cachedUserTopics } from '$lib/caching/user_topics_cache';

	export let parentTopics: string[];
	export let subTopics: UniqueTopic[];
	export let selectedTopics: Set<number>;

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

<!-- <Accordion.Item>
	<svelte:fragment slot="summary">
		{#each parentTopics as parentTopic}
			<span class="text-xl font-bold bg-primary-400 badge m-1">
				{parentTopic}
			</span>
		{/each}
	</svelte:fragment>
	<svelte:fragment slot="content">
		<div class="flex flex-wrap gap-2 px-1 mt-1">
			{#each subTopics as topic}
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
		</div>
	</svelte:fragment>
</Accordion.Item> -->
