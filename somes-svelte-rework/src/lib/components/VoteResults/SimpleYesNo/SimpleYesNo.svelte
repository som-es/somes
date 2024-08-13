<script lang="ts">
	import { partyToColor } from "$lib/partyColor";
	import type { Vote } from "$lib/types";

    export let votes: Vote[];
    console.log(votes);
</script>
<div>
    Dafür
    <div class="flex mt-2 gap-3 py-4">
        {#each votes as vote}
            {#if vote.infavor}
                <div>
                    <span style="background-color: {partyToColor(vote.party)}" class="px-4 py-3 rounded-lg">
                    </span>
                    <span class="ml-1">
                        {vote.party}
                    </span>
                </div>
            {/if}
        {/each}
    </div>
</div>
<div>
    Dagegen
    <div class="flex mt-2 gap-3 py-4">
       {#each votes as vote}
       
            {#if !vote.infavor}
                <div>
                    <span style="background-color: {partyToColor(vote.party)}" class="px-4 py-3 rounded-lg">
                    </span>
                    <span class="ml-1">
                        {vote.party}
                    </span>
                </div>
            {/if}
        {/each}
    </div>
</div>

