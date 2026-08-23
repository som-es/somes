<script lang="ts">
	import { cachedUserTopics } from '$lib/caching/user_topics_cache.svelte';
	import type { UniqueTopic } from '$lib/types';
	import { onMount } from 'svelte';
	import MultiValuesFilter from './MultiValuesFilter.svelte';
	import { t } from '$lib/i18n/i18n.svelte';
	import { SvelteSet } from 'svelte/reactivity';

	interface Props {
		selectedTopics: SvelteSet<string>;
		topics: string[];
	}

	let { selectedTopics = $bindable(), topics }: Props = $props();

	let userTopics: UniqueTopic[] | null = $state(null);
	onMount(async () => {
		userTopics = await cachedUserTopics();
	});
</script>

<MultiValuesFilter title={t('filter.topics')} bind:selectedValues={selectedTopics} values={topics}>
	{#snippet prefillSnippet()}
		{#if userTopics !== null}
			<button
				onclick={() => (selectedTopics = new SvelteSet(userTopics?.map((topic) => topic.topic)))}
				class="badge bg-secondary-500 text-white">{t('filter.topics.prefillInterests')}</button
			>
		{:else}
			<span></span>
		{/if}
	{/snippet}
</MultiValuesFilter>
