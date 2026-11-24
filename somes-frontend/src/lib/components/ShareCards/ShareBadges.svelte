<script lang="ts">
	let { texts, wrapText = false }: { texts: string[]; wrapText?: boolean } = $props();
</script>

<div style="display: flex; flex-wrap: wrap; gap: 8px; margin-top: 14px;">
	{#each texts as text (text)}
		<div
			style="display: flex; flex-shrink: {wrapText ? 1 : 0}; white-space: {wrapText
				? 'normal'
				: 'nowrap'}; padding: 7px 21px; border-radius: {wrapText
				? 22
				: 999}px; font-size: 21px; color: rgb(0 0 0); background: rgb(250 233 218);"
		>
			{text}
		</div>
	{/each}
</div>
