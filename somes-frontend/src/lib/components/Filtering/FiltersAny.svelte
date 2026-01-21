<script lang="ts">
	import Filters from './Filters.svelte';
	import type { FilterInfo } from './types';

	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	export let filters: FilterInfo<any>[];
	export let selectedPeriod: string;
	export let searchValue: string;
	export let update: () => void;
</script>

<Filters bind:filters bind:selectedPeriod bind:searchValue {update} />
