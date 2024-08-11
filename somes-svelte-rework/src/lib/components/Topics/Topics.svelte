<script lang="ts">
	import type { Topic } from '$lib/types';
	import DisplayTopic from './Topic.svelte';

	export let topics: Topic[] = [];
</script>

<div class="flex flex-wrap gap-2">
	{#each topics as topic}
		{#if topic.topic == 'namentliche Abstimmung'}
			<DisplayTopic class={'!bg-secondary-400'}>{topic.topic}</DisplayTopic>
		{:else}
			<DisplayTopic>{topic.topic}</DisplayTopic>
		{/if}
	{/each}
</div>
