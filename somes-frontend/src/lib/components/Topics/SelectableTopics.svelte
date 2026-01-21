<script lang="ts">
	import type { HasError, UniqueTopic } from '$lib/types';
	import { onMount } from 'svelte';
	import ExpandablePlaceholder from '../VoteResults/Expandable/Placeholders/ExpandablePlaceholder.svelte';
	import { getUserTopics } from '$lib/api/authed';
	import { isHasError } from '$lib/api/api';
	import { translateTopicToParent } from '$lib/interestColors';
	import AccordionTopics from './AccordionTopics.svelte';

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

	function createGroupTopics(topics: UniqueTopic[]): Map<string, UniqueTopic[]> {
		const groupedTopics = new Map<string, UniqueTopic[]>();
		topics.forEach((topic) => {
			const parentTopic = translateTopicToParent(topic.topic);
			if (groupedTopics.has(parentTopic)) {
				groupedTopics.get(parentTopic)?.push(topic);
			} else {
				groupedTopics.set(parentTopic, [topic]);
			}
		});
		return groupedTopics;
	}

	function createCombinedGroupings(topics: UniqueTopic[]): {
		others: UniqueTopic[];
		groupTopics: Map<string, UniqueTopic[]>;
		combinedGroups: { parentTopics: string[]; topics: UniqueTopic[] };
	} {
		const groupTopics = createGroupTopics(topics);
		const combinedGroups: { parentTopics: string[]; topics: UniqueTopic[] } = {
			parentTopics: [],
			topics: []
		};
		groupTopics.forEach((topics, parent) => {
			if (topics.length <= 4) {
				combinedGroups.parentTopics.push(parent);
				combinedGroups.topics.push(...topics);
				groupTopics.delete(parent);
			}
		});
		const others = groupTopics.get('Sonstige') ?? [];
		groupTopics.delete('Sonstige');

		return { others, groupTopics, combinedGroups };
	}

	$: groupedTopics = createCombinedGroupings(topics);
</script>

<div class="flex flex-wrap flex-col gap-2 px-1">
	<!-- {#if done && selectedTopics}
		<Accordion>
			{#each groupedTopics.groupTopics as [parentTopic, topics]}
				<AccordionTopics parentTopics={[parentTopic]} subTopics={topics} bind:selectedTopics />
			{/each}
			<AccordionTopics
				parentTopics={groupedTopics.combinedGroups.parentTopics}
				subTopics={groupedTopics.combinedGroups.topics}
				bind:selectedTopics
			/>
			<AccordionTopics
				parentTopics={['Sonstige']}
				subTopics={groupedTopics.others}
				bind:selectedTopics
			/>
		</Accordion>
	{:else}
		{#each { length: 15 } as _}
			<ExpandablePlaceholder />
		{/each}
	{/if} -->
</div>
