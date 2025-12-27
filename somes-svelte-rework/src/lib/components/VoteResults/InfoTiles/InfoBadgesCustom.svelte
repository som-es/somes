<script lang="ts">
	import { dashDateToDotDate } from "$lib/date";
    export let texts: (string | null)[];
</script>
<div class="flex max-sm:flex-wrap gap-1 mt-1">
    {#each texts as text}
        {#if text}
            <span class="badge bg-tertiary-400 text-black">{text}</span>
        {/if}
    {/each}
</div>
