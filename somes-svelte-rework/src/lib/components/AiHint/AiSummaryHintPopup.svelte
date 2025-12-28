<script lang="ts">
	import type { DbAiSummary } from "$lib/ai_summary_types";
	import { popup, type PopupSettings } from "@skeletonlabs/skeleton";

    export let aiSummary: DbAiSummary;
    export let useTitleHover: boolean = false;
    export let aiGenText: string = "Titel, Zusammenfassungen, Schwerpunkte, Gültigkeitszeiträume, Themen und Glossar wurden mittels KI aus den jeweiligen Dokumenten zusammengefasst.";

    $: titleHover = useTitleHover ? aiGenText : '';

    $: generatedAtDate = new Date(aiSummary.generated_at);

	const popupFeatured: PopupSettings = {
		event: 'hover',
		target: 'emphasisAi',
		placement: 'bottom'
	};
</script>


<div class="!z-50 card p-4 w-72 shadow-xl" data-popup="emphasisAi">
    <div class="z-50 font-bold text-base">{aiGenText}</div>
    <div class="flex flex-col flex-wrap !text-sm !font-thin">
        <span>Generiert am: {generatedAtDate.toLocaleDateString()} {generatedAtDate.toLocaleTimeString()}</span>
        <span>verwendetes Modell: {aiSummary.model_used}</span>
        <span>Version: {aiSummary.version}</span>
    </div>
</div>

<button class="text-3xl" title={titleHover} use:popup={popupFeatured}>⚠</button>