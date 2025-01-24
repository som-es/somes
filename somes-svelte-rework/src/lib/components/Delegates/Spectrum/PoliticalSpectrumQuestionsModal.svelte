<script lang="ts">
	import { delegate_political_questions, errorToNull } from '$lib/api/api';
	import type { DelegateQA } from '$lib/types';
	import { getModalStore } from '@skeletonlabs/skeleton';
	import DelegateQaEntry from '../QA/DelegateQAEntry.svelte';
	

	export let parent;
    const modalStore = getModalStore();

	let politicalQuestions: DelegateQA[] = []

	if ($modalStore.length > 0) { 
		const delegate = $modalStore[0].meta.delegate;
		delegate_political_questions(delegate.id).then(res => {
			politicalQuestions = errorToNull(res) ?? []
		});
	}

</script>

{#if $modalStore.length > 0 && $modalStore[0].meta }
<div class="card p-8 max-w-7xl">
	<button
		on:click={() => {
			modalStore.close();
		}}
		style="font-size: 34px"
		class="w-5 unselectable float-right">&#x2715</button
	>

	<div class="flex flex-wrap flex-col">
		{#each politicalQuestions as qa}
			<DelegateQaEntry class="mt-3" delegateQa={qa} />
		{/each}
	</div>

</div>
{/if}

<style>

</style>
