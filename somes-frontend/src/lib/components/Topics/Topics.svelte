<script lang="ts">
	import type { Topic, UniqueTopic } from '$lib/types';
	import { onMount } from 'svelte';
	import DisplayTopic from './Topic.svelte';
	import { cachedUserTopics } from '$lib/caching/user_topics_cache.svelte';

	export let topics: Topic[] = [];

	let userTopics: Set<string> = new Set();
	onMount(async () => {
		userTopics = new Set(((await cachedUserTopics()) ?? []).map((x) => x.topic));
	});
</script>

<div class="flex flex-wrap gap-1 px-1">
	{#each topics as topic}
		{#if userTopics.has(topic.topic)}
			<DisplayTopic class={'!bg-secondary-400'}>{topic.topic}</DisplayTopic>
		{:else if topic.topic == 'namentliche Abstimmung'}
			<DisplayTopic class={'!bg-tertiary-500 text-black!'}>{topic.topic}</DisplayTopic>
		{:else}
			<DisplayTopic>{topic.topic}</DisplayTopic>
		{/if}
	{/each}
</div>
