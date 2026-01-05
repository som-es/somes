<script lang="ts" generics="T">
		import type { FilterInfo } from './types';

	export let filter: FilterInfo<T>;
</script>

{#if !filter.hidden}
	<div>
		<h1 class="text-lg sm:text-2xl font-bold">{filter.title}</h1>
		<button
			class="btn btn-sm sm:btn-md preset-filled-secondary-500 w-40 sm:w-48 justify-between"
			use:popup={filter.popup}
		>
			<span class="capitalize">{filter.translationFn(filter)}</span>
			<span>↓</span>
		</button>
	</div>
{/if}
