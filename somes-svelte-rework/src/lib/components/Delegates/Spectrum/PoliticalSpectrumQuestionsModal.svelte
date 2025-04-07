<script lang="ts">
	import { delegate_political_questions, errorToNull } from '$lib/api/api';
	import type { DelegateQA } from '$lib/types';
	import { getModalStore } from '@skeletonlabs/skeleton';
	import DelegateQaEntry from '../QA/DelegateQAEntry.svelte';
	import ExpandablePlaceholder from '$lib/components/VoteResults/Expandable/Placeholders/ExpandablePlaceholder.svelte';

	export let parent;
	const modalStore = getModalStore();

	let politicalQuestions: DelegateQA[] = [];

	if ($modalStore.length > 0) {
		const delegate = $modalStore[0].meta.delegate;
		delegate_political_questions(delegate.id).then((res) => {
			politicalQuestions = errorToNull(res) ?? [];
		});
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

	{#if politicalQuestions.length > 0}
		{#each politicalQuestions as qa}
			<DelegateQaEntry class="mt-3" delegateQa={qa} />
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
