<script lang="ts">
	import { dashDateToDotDate } from '$lib/date';
	export let texts: (string | null)[];
</script>

<div class="mt-1 flex gap-1 max-md:flex-wrap">
	{#each texts as text}
		{#if text}
			<span
				class="badge bg-tertiary-400 text-black"
				style="white-space: normal !important; hyphens: auto;">{text}</span
			>
		{/if}
	{/each}
</div>
