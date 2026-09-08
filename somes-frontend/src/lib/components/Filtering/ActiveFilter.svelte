<script>
	export let filter;
</script>

{#if filter.filterObj !== undefined && filter.filterObj !== ''}
	<button
		class="badge cursor-pointer bg-secondary-400 p-3 text-black"
		on:click={() => (filter.filterObj = undefined)}
	>
		{filter.translationFn(filter)}
		<span class="ml-1" style="font-size: 18px;">✕</span>
	</button>
{/if}
