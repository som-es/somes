<script lang="ts">
	import type { Delegate, PoliticalPosition } from '$lib/types';
	import PoliticalSpectrum from './PoliticalSpectrum.svelte';
	export let delegate: Delegate;
	export let politicalPosition: PoliticalPosition;
</script>

<div class="bg-primary-300 dark:bg-primary-500 p-4 rounded-xl">
	<div class="bg-primary-100 dark:bg-primary-400">
		<PoliticalSpectrum {delegate} {politicalPosition} />
	</div>
</div>
