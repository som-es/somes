<script lang="ts">
	import type { DbAiSummary } from "$lib/ai_summary_types";
	import { Popover } from "bits-ui";

    interface Props {
        aiSummary: DbAiSummary;
        useTitleHover?: boolean;
        aiGenText?: string;
    }

    let { aiSummary, useTitleHover = false, aiGenText = "Titel, Zusammenfassungen, Schwerpunkte, Gültigkeitszeiträume, Themen und Glossar wurden mittels KI aus den jeweiligen Dokumenten zusammengefasst." }: Props = $props();

    let titleHover = $derived(useTitleHover ? aiGenText : '');

    let generatedAtDate = $derived(new Date(aiSummary.generated_at));
</script>

<Popover.Root>
    <Popover.Trigger class="text-3xl" title={titleHover}>⚠</Popover.Trigger>
    <Popover.Content class="z-50! card p-4 w-72 shadow-xl" data-popup="emphasisAi">
        <div class="z-50 font-bold text-base">{aiGenText}</div>
        <div class="flex flex-col flex-wrap text-sm! font-thin!">
            <span>Generiert am: {generatedAtDate.toLocaleDateString()} {generatedAtDate.toLocaleTimeString()}</span>
            <span>verwendetes Modell: {aiSummary.model_used}</span>
            <span>Version: {aiSummary.version}</span>
        </div>
    </Popover.Content>
</Popover.Root>
