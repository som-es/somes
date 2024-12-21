<script lang="ts">
	// import { topicColors } from '$lib/interestColors';
	import type { InterestShare } from '$lib/types';
	import Square from '../UI/Square.svelte';

	export let interests: InterestShare[];

	function calcSizeForText(text: string): string {
		if (text.length < 12) return 'text-xl';
		if (text.length < 15) return 'text-lg';
		if (text.length < 24) return 'text-md';
		return 'text-sm';
	}
</script>

<div
	class="grid-tile bg-primary-200 dark:bg-primary-400 flex flex-row flex-wrap interests-item gap-3"
>
	{#each interests as interest}
		<Square class=" break-words">
			<div
				class="text-center font-bold {calcSizeForText(interest.topic)}"
				style="max-width: 140px;"
			>
				{interest.topic}
			</div>
			<div class="text-center">
				{(interest.self_share * 100).toFixed(2)}%
			</div>
		</Square>
	{/each}
</div>

<style>
	.grid-tile {
		/* box-sizing: border-box; */
		border-radius: 25px;
	}

	.interest-tile {
		align-content: center;
		text-align: center;
		/* padding: 60px; */
		word-break: break-all;
	}
</style>
