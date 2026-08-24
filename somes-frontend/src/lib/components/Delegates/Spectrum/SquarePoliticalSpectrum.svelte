<script lang="ts">
	import type { Delegate, PoliticalScore } from '$lib/types';
	import PoliticalSpectrum from './PoliticalSpectrum.svelte';

	let {
		delegate,
		politicalPosition
	}: { delegate: Delegate; politicalPosition: PoliticalScore } = $props();
</script>

<div
	class="flex w-full items-center justify-center rounded-xl bg-primary-300 p-4 lg:block lg:w-auto lg:shrink-0 dark:bg-primary-500"
>
	<div class="w-fit bg-primary-100 dark:bg-primary-400">
		<PoliticalSpectrum {delegate} {politicalPosition} />
	</div>
</div>
