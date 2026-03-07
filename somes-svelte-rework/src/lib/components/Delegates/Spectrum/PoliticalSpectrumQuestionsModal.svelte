<script lang="ts">
	import type { StanceTopicInfluences } from '$lib/types';
	import { getModalStore } from '@skeletonlabs/skeleton';
	import ExpandablePlaceholder from '$lib/components/VoteResults/Expandable/Placeholders/ExpandablePlaceholder.svelte';
	import QaDelegateStanceInfluences from './Stance/QADelegateStanceInfluences.svelte';

	export let parent;
	const modalStore = getModalStore();

	let stanceTopicInfluences: StanceTopicInfluences[] = [];

	if ($modalStore.length > 0) {
		stanceTopicInfluences = $modalStore[0].meta.stanceTopicInfluences;
	}
</script>

<div class="card p-8 w-full min-w-7xl max-w-7xl">
	<button
		on:click={() => {
			modalStore.close();
		}}
		style="font-size: 34px"
		class="w-5 unselectable float-right">&#x2715</button
	>

	{#if stanceTopicInfluences.length > 0}
		{#each stanceTopicInfluences as qa}
			<QaDelegateStanceInfluences class="mt-3" stanceTopicInfluences={qa} />
		{/each}
	{:else}
		<!-- IMPORTANT THIS THING IS SOMEHOW REQUIRED TO MAKE THE MODAL SCROLLABLE. SOMEHOW IT IS ONLY CHECK AT MOUNTING WHETHER THIS NEEDS TO BE SCROLLABLE!!!! -->
		{#each { length: 30 } as _}
			<ExpandablePlaceholder class="min-w-7xl w-full" />
		{/each}
	{/if}
</div>

<style>
</style>
