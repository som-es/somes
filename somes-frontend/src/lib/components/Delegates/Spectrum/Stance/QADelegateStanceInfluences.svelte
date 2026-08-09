<script lang="ts">
	import Topic from '$lib/components/Topics/Topic.svelte';
	import type { StanceTopicInfluences } from '$lib/types';

	export let stanceTopicInfluences: StanceTopicInfluences;
	let clazz = '';
	export { clazz as class };
</script>

<div class={clazz}>
	<span class="mb-1 flex items-center gap-3 text-xl font-bold sm:text-3xl">
		{stanceTopicInfluences.question}
		<span>
			{#if stanceTopicInfluences.stance_llm.toLowerCase().includes('positive')}
				<span class="badge bg-success-600 text-white">Dafür</span>
			{:else if stanceTopicInfluences.stance_llm.toLowerCase().includes('negative')}
				<span class="badge bg-red-600 text-white">Dagegen</span>
			{:else}
				<span class="badge bg-primary-500 text-white">Neutral</span>
			{/if}
		</span>
	</span>
	<hr />
	<div class="mt-1 flex gap-1 sm:text-xl">
		{stanceTopicInfluences.answer}
	</div>
	<div class="flex flex-row flex-wrap gap-1">
		{#each stanceTopicInfluences.topic_influences as topic_influence}
			{#if Math.abs(topic_influence.score) > 0.005}
				<div>
					<Topic>{topic_influence.topic}</Topic>
					<!-- <div class="badge text-white" style="background-color: {topicColors.get(topic_influence.topic)};">{topic_influence.topic}</div> -->
					{#if topic_influence.score > 0}
						<div class="badge bg-success-600 text-white">
							{topic_influence.score.toPrecision(2)}
						</div>
					{:else}
						<div class="badge bg-red-600 text-white">{topic_influence.score.toPrecision(2)}</div>
					{/if}
				</div>
			{/if}
		{/each}
	</div>
</div>
