<script lang="ts">
	import type { Glossary } from "$lib/ai_summary_types";
    export let glossary: Glossary
    export let text: string;
    export let className: string = "";

    $: processedText = (() => {
        const sortedTerms = [...glossary.difficult_terms].sort(
            (a, b) => b.term.length - a.term.length
        );

        const definitionMap = new Map(
            sortedTerms.map(item => [item.term.toLowerCase(), item.simple_definition])
        );

        const termReplacements = [];
        for (const [term, definition] of definitionMap) {
            const termRegex = new RegExp(`\\b${term}`, 'gi');

            const match = termRegex.exec(text);
            if (match) {
                const start = match.index 
                const end = match.index + match[0].length
                if (termReplacements.findIndex(term => {
                    return start <= term.end && end >= term.start
                }) == -1) {
                    termReplacements.push({start, end, definition, match: match[0]})
                }
            }
        }

        termReplacements.sort((a, b) => a.start - b.start)

        const textParts = [];
        let start = 0;
        for (const termReplacement of termReplacements) {
            const newStart = termReplacement.start
            const outsiteTextPart = text.slice(start, newStart);
            if (outsiteTextPart && outsiteTextPart.length > 0) {
                textParts.push({text: outsiteTextPart, definition: null as string | null})
            }
            textParts.push({text: text.slice(newStart, termReplacement.end), definition: termReplacement.definition})
            start = termReplacement.end;
        }
        const outsiteTextPart = text.slice(start)
        if (outsiteTextPart && outsiteTextPart.length > 0) {
            textParts.push({text: outsiteTextPart, definition: null as string | null})
        }
        return textParts
    })();

</script>
<span class={className} >
    {#each processedText as item}
        {#if item.definition}
            <span class="glossary-term" data-definition={item.definition}>{item.text}</span>
        {:else}
            {item.text}
        {/if}
    {/each}
</span>

<style>
   :global(.glossary-term) {
        text-decoration: underline dotted ;
        text-underline-offset: 4px;
        cursor: help;
        position: relative;
        display: inline-block;
    }

    :global(.glossary-term:hover::after) {
        content: attr(data-definition);
        position: absolute;
        bottom: 125%;
        left: 50%;
        transform: translateX(-50%);
        background-color: #333;
        color: #fff;
        padding: 8px 12px;
        border-radius: 4px;
        font-size: 0.85rem;
        line-height: 1.4;
        z-index: 50;
        width: max-content;
        max-width: 250px;
        white-space: normal;
        box-shadow: 0 4px 6px rgba(0,0,0,0.1);
    }

    :global(.glossary-term:hover::before) {
        content: "";
        position: absolute;
        bottom: 105%;
        left: 50%;
        transform: translateX(-50%);
        border: 8px solid transparent;
        border-top-color: #333;
        z-index: 50;
    } 
</style>

