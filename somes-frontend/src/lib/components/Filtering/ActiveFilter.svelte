<script>
	export let filter;
</script>

{#if filter.filterObj !== undefined}
	<button
		class="badge p-3 bg-secondary-400 text-black cursor-pointer"
		on:click={() => (filter.filterObj = undefined)}
	>
		{filter.translationFn(filter)}
		<span class="ml-1" style="font-size: 18px;">✕</span>
	</button>
{/if}
