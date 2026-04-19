<script lang="ts">
	import type { Speech, SpeechesWithMaxPage } from '$lib/types';
	import { Dialog } from 'bits-ui';
	import AllSpeechesModal from './AllSpeechesModal.svelte';
	import SpeechBar from './SpeechBar.svelte';

	export let speechesPage0: SpeechesWithMaxPage;
	export let delegateId: number;

	$: previewSpeeches = speechesPage0.speeches.slice(0, 2);
</script>

<div>
	<div class="flex items-start justify-between">
		<h1 class="text-lg font-semibold text-black xl:text-xl dark:text-white">Letzte Reden</h1>
		<div class="flex items-center gap-4 text-sm text-black dark:text-white">
			<div class="flex items-center gap-2"><span class="h-2 w-2 rounded-full bg-green-600"></span>Pro</div>
			<div class="flex items-center gap-2"><span class="h-2 w-2 rounded-full bg-red-500"></span>Contra</div>
		</div>
	</div>
</div>
<div>
	{#each previewSpeeches as speech}
		<SpeechBar {speech}></SpeechBar>
	{/each}
</div>
<Dialog.Root>
	<Dialog.Trigger class="mt-3 flex items-center gap-1 text-sm text-gray-700 dark:text-gray-300">
		Alle Reden
		<span aria-hidden="true">→</span>
	</Dialog.Trigger>
	<Dialog.Portal>
		<Dialog.Overlay
			class="data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 fixed inset-0 z-70 bg-black/80"
		/>
		<Dialog.Content
			class="data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 outline-hidden fixed left-[50%] top-[50%] z-70 w-7xl 2xl:max-w-7xl max-w-[90%] h-[90vh] -translate-x-1/2 -translate-y-1/2 overflow-hidden rounded-lg bg-primary-100 shadow-lg dark:bg-gray-800"
		>
			<AllSpeechesModal {delegateId} {speechesPage0} />
		</Dialog.Content>
	</Dialog.Portal>
</Dialog.Root>
