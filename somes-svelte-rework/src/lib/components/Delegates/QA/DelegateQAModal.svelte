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

<div class="card p-4 sm:p-8 max-w-7xl w-full mx-auto">
	<div class="!z-50 card p-4 w-72 max-w-full shadow-xl" data-popup="extractedFromIntroductionVideo">
		<div class="z-50 font-bold text-lg sm:text-xl">
			Die Fragen und Antworten wurden aus dem Portraitvideo des Abgeordneten extrahiert.
		</div>
	</div>

	<div class="flex justify-between items-start mb-4">
		<button class="text-2xl sm:text-4xl p-2 hover:bg-surface-200-700-token rounded-lg transition-colors" use:popup={popupFeatured}>⚠</button>
		<button
			on:click={() => {
				modalStore.close();
			}}
			class="text-2xl sm:text-3xl p-2 hover:bg-surface-200-700-token rounded-lg transition-colors min-w-[44px] min-h-[44px] flex items-center justify-center">&#x2715</button
		>
	</div>
	{#if $modalStore.length > 0}
		{#each $modalStore[0].meta.questions as qa}
			<DelegateQaEntry class="mt-3" delegateQa={qa} />
		{/each}
	{/if}
</div>
