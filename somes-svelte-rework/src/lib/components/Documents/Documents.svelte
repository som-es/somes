<script lang="ts">
	import type { Document } from "$lib/types";
	import SButton from "../UI/SButton.svelte";

    export let documents: Document[];
</script>
<span class="font-bold text-lg md:text-2xl">Dokumente (PDFs)</span>
<div class="gap-3 flex flex-wrap">
    {#each documents.sort((a, b) => (b.title ?? '').length - (a.title ?? '').length) as document}
        <!-- {#if document.document_type.includes('PDF')} -->
            <SButton
                class="bg-secondary-500 text-black"
                on:click={() =>
                    window.open(
                        `https://www.parlament.gv.at${document.document_url}`,
                        '_blank'
                    )}>{document.title}</SButton
            >
        <!-- {/if} -->
    {/each}
</div>