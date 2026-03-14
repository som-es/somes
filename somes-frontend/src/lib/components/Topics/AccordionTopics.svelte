<script lang="ts">
	import type { UniqueTopic } from '$lib/types';
	import { Accordion } from 'bits-ui';
	import ClickableSpan from '../Utils/ClickableSpan.svelte';
	import DisplayTopic from './Topic.svelte';
	import { addUserTopic, removeUserTopic } from '$lib/api/authed';
	import { cachedUserTopics } from '$lib/caching/user_topics_cache.svelte';

	interface Props {
		parentTopics: string[];
		subTopics: UniqueTopic[];
		selectedTopics: Set<number>;
	}

	let { parentTopics, subTopics, selectedTopics = $bindable() }: Props = $props();

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

<Accordion.Root type="multiple">
	<Accordion.Item value={parentTopics.join('-')}>
		<Accordion.Trigger class="flex w-full items-center gap-1 rounded-lg px-2 py-2 text-left hover:bg-primary-200 dark:hover:bg-primary-600">
			{#each parentTopics as parentTopic}
				<span class="rounded-md bg-primary-400 px-2 py-0.5 text-sm font-bold dark:bg-primary-700">
					{parentTopic}
				</span>
			{/each}
		</Accordion.Trigger>

		<Accordion.Content>
			<div class="flex flex-wrap gap-2 px-2 py-2">
				{#each subTopics as topic}
					{#if selectedTopics.has(topic.id)}
						<ClickableSpan action={() => removeFromSelection(topic)}>
							<DisplayTopic class={'!bg-secondary-400'}>
								{topic.topic}
							</DisplayTopic>
						</ClickableSpan>
					{:else}
						<ClickableSpan action={() => addToSelection(topic)}>
							<DisplayTopic>
								{topic.topic}
							</DisplayTopic>
						</ClickableSpan>
					{/if}
				{/each}
			</div>
		</Accordion.Content>
	</Accordion.Item>
</Accordion.Root>
