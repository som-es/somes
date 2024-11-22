<script lang="ts">
	import type { Topic } from '$lib/types';
	import ClickableSpan from '../Utils/ClickableSpan.svelte';
	import DisplayTopic from './Topic.svelte';
	
	
	export let selectedTopics = new Set<Topic>();

	export let topics: Topic[] = [];
	let addToSelection = (topic: Topic) => {
		selectedTopics.add(topic);
		selectedTopics = new Set(selectedTopics);
	};
	
	let removeFromSelection = (topic: Topic) => {
		selectedTopics.delete(topic)
		selectedTopics = new Set(selectedTopics);
	};

</script>

<div class="flex flex-wrap gap-2 px-1">
	{#each topics as topic}
		{#if selectedTopics.has(topic)}
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
