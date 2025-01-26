
<script lang="ts">
	import type { Delegate, PoliticalPosition } from "$lib/types";
	import PoliticalSpectrum from "./PoliticalSpectrum.svelte";
	import { getModalStore, popup, type ModalSettings, type PopupSettings } from '@skeletonlabs/skeleton';

    export let delegate: Delegate;
    export let politicalPosition: PoliticalPosition;

    $: questionDetails = {
		type: 'component',
		component: 'politicalSpectrumQuestions',
		meta: { delegate }
	} as ModalSettings;

	const popupFeatured: PopupSettings = {
		event: 'hover',
		target: 'politicalPositionHint',
		placement: 'bottom'
	};
	
    const modalStore = getModalStore();
</script>

<div class="bg-primary-300 dark:bg-primary-500 p-4 rounded-xl">
    <div class="flex self-center items-center">
        <button class="mr-auto btn btn-lg variant-filled m-2" on:click={() => modalStore.trigger(questionDetails)}
            >Details</button
        >
        <button class="text-5xl" use:popup={popupFeatured}>⚠</button>
    </div>
    <div class="bg-primary-100 dark:bg-primary-700">
        <PoliticalSpectrum {delegate} {politicalPosition} />
    </div>
</div>

<div class="!z-50 card p-4 w-72 shadow-xl" data-popup="politicalPositionHint">
	<div class="z-50 font-bold text-xl">
        Die Einordnung der politschen Position ist eine grobe Schätzung und muss nicht der Realität entsprechen. 
        Verwendet wurden AI generierte Antworten zu Fragen, die nicht immer vollständig korrekt sind und unter 'Details' abrufbar sind.
	</div>
</div>