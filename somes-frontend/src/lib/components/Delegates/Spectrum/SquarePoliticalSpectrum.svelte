<script lang="ts">
	import type { Delegate, PoliticalPosition } from '$lib/types';
	import PoliticalSpectrum from './PoliticalSpectrum.svelte';

	let { delegate, politicalPosition }: { delegate: Delegate; politicalPosition: PoliticalPosition } = $props();

</script>

<div class="w-full lg:w-auto lg:shrink-0 bg-primary-300 dark:bg-primary-500 p-4 rounded-xl flex items-center justify-center lg:block">
	<div class="w-fit bg-primary-100 dark:bg-primary-400">
		<PoliticalSpectrum {delegate} {politicalPosition} />
	</div>
</div>
