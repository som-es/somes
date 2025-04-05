<script lang="ts">
	import { topicColors } from "$lib/interestColors";
	import type { StanceTopicScore } from "$lib/types";
	import StanceDiagram from "./StanceDiagram.svelte";

	export let leftLabel = 'Links'; // Left label
	export let rightLabel = 'Rechts'; // Right label

    export let stances: StanceTopicScore[];
</script>
<div class="flex flex-wrap gap-5">
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