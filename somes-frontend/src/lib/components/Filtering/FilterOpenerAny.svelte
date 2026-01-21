<script lang="ts">
	import FilterOpener from './FilterOpener.svelte';
	import type { FilterInfo } from './types';

	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	export let filter: FilterInfo<any>;
</script>

<FilterOpener {filter} />
