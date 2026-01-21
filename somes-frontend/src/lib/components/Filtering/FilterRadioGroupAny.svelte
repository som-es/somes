<script lang="ts">
	import FilterRadioGroup from './FilterRadioGroup.svelte';
	import type { FilterInfo } from './types';

	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	export let filter: FilterInfo<any>;
</script>

<FilterRadioGroup bind:filter />
