<script lang="ts">
	import { getModalStore, popup, type PopupSettings } from '@skeletonlabs/skeleton';
	import DelegateQaEntry from './DelegateQAEntry.svelte';

	const modalStore = getModalStore();
	export let parent;

	const popupFeatured: PopupSettings = {
		event: 'hover',
		target: 'extractedFromIntroductionVideo',
		placement: 'bottom'
	};
</script>

<div class="card p-8 max-w-7xl">
	<div class="z-50! card p-4 w-72 shadow-xl" data-popup="extractedFromIntroductionVideo">
		<div class="z-50 font-bold text-xl">
			Die Fragen und Antworten wurden aus dem Portraitvideo des Abgeordneten extrahiert.
		</div>
	</div>

	<!-- <div class="flex justify-between">
        <div></div> -->
	<button class="text-4xl" use:popup={popupFeatured}>⚠</button>

	<button
		on:click={() => {
			modalStore.close();
		}}
		style="font-size: 34px"
		class="w-5 unselectable float-right">&#x2715</button
	>
	<!-- </div> -->
	{#if $modalStore.length > 0}
		{#each $modalStore[0].meta.questions as qa}
			<DelegateQaEntry class="mt-3" delegateQa={qa} />
		{/each}
	{/if}
</div>
