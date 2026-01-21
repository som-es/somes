<script lang="ts">
	import FilterListBox from './FilterListBox.svelte';
	import type { FilterInfo } from './types';

	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	export let filter: FilterInfo<any>;
</script>

<FilterListBox bind:filter></FilterListBox>
