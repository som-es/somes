<script lang="ts">
	import type { Document } from '$lib/types';
	// import SButton from "../UI/SButton.svelte";
	import pdfIcon from '$lib/assets/misc_icons/pdf-icon-middle.svg?raw';

	export let documents: Document[];
</script>

<span class="font-bold text-lg md:text-xl">Dokumente (PDFs)</span>
<div class="gap-3 flex flex-wrap">
	{#each documents.sort((a, b) => (b.title ?? '').length - (a.title ?? '').length) as document}
		{#if document.document_type.includes('PDF')}
			<a
				class="underline flex items-center gap-1 text-sm md:text-base"
				href={document.document_url}
				target="_blank"
				>{document.title}
				<span class="min-w-6">
					{@html pdfIcon}
				</span>
			</a>
			<!-- <SButton
                class="bg-secondary-500 text-black"
                on:click={() =>
                    window.open(
                        `https://www.parlament.gv.at${document.document_url}`,
                        '_blank'
                    )}>{document.title}</SButton
            > -->
		{/if}
	{/each}
</div>
