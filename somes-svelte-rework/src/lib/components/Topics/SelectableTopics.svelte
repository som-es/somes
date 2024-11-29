<script lang="ts">
	import type { UniqueTopic } from '$lib/types';
	import { onMount } from 'svelte';
	import ClickableSpan from '../Utils/ClickableSpan.svelte';
	import DisplayTopic from './Topic.svelte';
	import ExpandablePlaceholder from '../VoteResults/Expandable/Placeholders/ExpandablePlaceholder.svelte';
	
	
	export let selectedTopics = new Set<UniqueTopic>();
	let done = false;
	
	onMount(async () => {

		
		done = true;
	})

	export let topics: UniqueTopic[] = [];
	let addToSelection = async (topic: UniqueTopic) => {
		selectedTopics.add(topic);
		selectedTopics = new Set(selectedTopics);

		
	};
	
	let removeFromSelection = async (topic: UniqueTopic) => {
		selectedTopics.delete(topic)
		selectedTopics = new Set(selectedTopics);
	};

</script>

<div class="flex flex-wrap gap-2 px-1">
	{#if done}
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
	{:else}
		{#each { length: 9 } as _}
			<ExpandablePlaceholder class="my-4" />
		{/each}
	{/if}
</div>
