<script lang="ts">
	import { topicColors } from "$lib/interestColors";
	import type { StanceTopicScore } from "$lib/types";
	import StanceDiagram from "./StanceDiagram.svelte";
	import collapse from 'svelte-collapse';

	export let leftLabel = 'Links'; // Left label
	export let rightLabel = 'Rechts'; // Right label

    export let stances: StanceTopicScore[];

	let open = false;
</script>

<div class="sm:hidden flex flex-wrap">
    {#each stances.slice(0, 3) as stance}
        <StanceDiagram
            zeroLabel={stance.topic}
            value={stance.score * 2}
            knobColor={topicColors.get(stance.topic)}
            {rightLabel}
            {leftLabel}
        />
    {/each}
    
    <div class="flex flex-wrap " use:collapse={{ open }}>
        {#each stances.slice(3) as stance}
            <StanceDiagram
                zeroLabel={stance.topic}
                value={stance.score * 2}
                knobColor={topicColors.get(stance.topic)}
                {rightLabel}
                {leftLabel}
            />
        {/each}
    </div>
    
    {#if stances.length > 3}
		<button class=" font-bold text-xl" on:click={() => (open = !open)}>
			<span>{open ? 'Weniger' : 'Mehr'} anzeigen</span>
		</button>
	{/if}
</div>

<div class="max-sm:hidden flex flex-wrap gap-3">
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