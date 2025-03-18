<script lang="ts">
	import type { Topic, UniqueTopic } from '$lib/types';
	import { onMount } from 'svelte';
	import DisplayTopic from './Topic.svelte';
	import { cachedUserTopics } from '$lib/caching/user_topics_cache';

	export let topics: Topic[] = [];

	let userTopics: Set<string> = new Set();
	onMount(async () => {
		userTopics = new Set(((await cachedUserTopics()) ?? []).map((x) => x.topic));
	});
</script>

<div class="flex flex-wrap gap-2 px-1">
	{#each topics as topic}
		{#if topic.topic == 'namentliche Abstimmung' || userTopics.has(topic.topic)}
			<DisplayTopic class={'!bg-secondary-400'}>{topic.topic}</DisplayTopic>
		{:else}
			<DisplayTopic>{topic.topic}</DisplayTopic>
		{/if}
	{/each}
</div>
