<script lang="ts">
	import type { SpeechesWithMaxPage } from '$lib/types';
	import { Dialog } from 'bits-ui';
	import AllSpeechesModal from './AllSpeechesModal.svelte';
	import SpeechBar from './SpeechBar.svelte';
	import ExtendInfoDialog from '../ExtendInfoDialog.svelte';

	export let speechesPage0: SpeechesWithMaxPage;
	export let delegateId: number;

	$: previewSpeeches = speechesPage0.speeches.slice(0, 2);
</script>

<div>
	<div class="flex items-start justify-between">
		<div>
			<h1 class="text-lg font-semibold text-black xl:text-xl dark:text-white">Letzte Reden</h1>
			<h2 class="text-sm text-gray-800 dark:text-gray-300">
				{speechesPage0.entry_count}
				{speechesPage0.entry_count == 1 ? 'Rede' : 'Reden'} insgesamt
			</h2>
		</div>

		<ExtendInfoDialog title="Alle anzeigen">
			<AllSpeechesModal {delegateId} {speechesPage0} />
		</ExtendInfoDialog>
	</div>
</div>
<div>
	{#each previewSpeeches as speech}
		<SpeechBar {speech}></SpeechBar>
	{/each}
</div>
<div class="float-right mt-3 flex items-center gap-4 text-sm text-black dark:text-white">
	<div class="flex items-center gap-2">
		<span class="h-2 w-2 rounded-full bg-green-600"></span>Pro
	</div>
	<div class="flex items-center gap-2">
		<span class="h-2 w-2 rounded-full bg-red-500"></span>Contra
	</div>
</div>
