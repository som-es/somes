<script lang="ts">
	import type { Topic } from "$lib/types";
	import DisplayTopic from "./Topic.svelte";

    export let topics: Topic[] = [];
</script>

<div class="flex flex-wrap gap-2 justify-center mt-3 content-center">
    {#each topics as topic}
        <DisplayTopic>{topic.topic}</DisplayTopic>
    {/each}
</div>
