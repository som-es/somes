<script lang="ts">
	import Topic from '$lib/components/Topics/Topic.svelte';
import { topicColors } from '$lib/interestColors';
	import type { StanceTopicInfluences } from '$lib/types';

	export let stanceTopicInfluences: StanceTopicInfluences;
	let clazz = '';
	export { clazz as class };
</script>

<div class={clazz}>
	<span class="font-bold text-xl sm:text-3xl">
		{stanceTopicInfluences.question}
	</span>
	<hr />
	<div class="mt-1 sm:text-xl">{stanceTopicInfluences.answer}</div>
	<div class="flex gap-1 flex-row flex-wrap">
		{#each stanceTopicInfluences.topic_influences as topic_influence}	

			{#if Math.abs(topic_influence.score) > 0.005}
				<div>	
					<Topic>{topic_influence.topic}</Topic>
					<!-- <div class="badge text-white" style="background-color: {topicColors.get(topic_influence.topic)};">{topic_influence.topic}</div> -->
					{#if topic_influence.score > 0}
						<div class="badge text-white bg-success-600">{topic_influence.score.toPrecision(2)}</div>
					{:else}
						<div class="badge text-white bg-red-600">{topic_influence.score.toPrecision(2)}</div>
					{/if}
				</div>
			{/if}
		{/each}
	</div>
</div>
