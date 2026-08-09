<script lang="ts">
	import { topicColors } from '$lib/interestColors';
	import type { StanceTopicScore } from '$lib/types';
	import StanceDiagram from './StanceDiagram.svelte';
	import { slide } from 'svelte/transition';

	interface Props {
		leftLabel?: string; // Left label
		rightLabel?: string; // Right label
		stances: StanceTopicScore[];
	}

	let { leftLabel = 'Links', rightLabel = 'Rechts', stances }: Props = $props();

	let open = $state(false);
</script>

<div class="flex flex-col flex-wrap sm:hidden sm:flex-row">
	{#each stances.slice(0, 3) as stance}
		<StanceDiagram
			zeroLabel={stance.topic}
			value={stance.score * 2}
			knobColor={topicColors.get(stance.topic)}
			{rightLabel}
			{leftLabel}
		/>
	{/each}

	{#if open}
		<span transition:slide={{ duration: 240 }} class="flex flex-wrap gap-3">
			{#each stances.slice(3) as stance}
				<StanceDiagram
					zeroLabel={stance.topic}
					value={stance.score * 2}
					knobColor={topicColors.get(stance.topic)}
					{rightLabel}
					{leftLabel}
				/>
			{/each}
		</span>
	{/if}

	{#if stances.length > 3}
		<button class=" text-xl font-bold" onclick={() => (open = !open)}>
			<span>{open ? 'Weniger' : 'Mehr'} anzeigen</span>
		</button>
	{/if}
</div>

<div class="flex flex-wrap gap-3 max-sm:hidden">
	{#each stances as stance}
		<StanceDiagram
			zeroLabel={stance.topic}
			value={stance.score * 2}
			knobColor={topicColors.get(stance.topic)}
			{rightLabel}
			{leftLabel}
		/>
	{/each}
</div>
