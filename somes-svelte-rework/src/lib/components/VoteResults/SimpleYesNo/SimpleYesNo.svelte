<script lang="ts">
	import { partyToColor } from "$lib/partyColor";
	import type { Vote } from "$lib/types";
	import BadgeList from "./BadgeList.svelte";

    export let votes: Vote[];

    votes.sort((a, b) => b.fraction - a.fraction);

</script>
<div>
    <span class="font-bold text-lg">Dafür</span>
    <div class="flex mt-2 gap-3 py-4">
        <BadgeList votes={votes} infavor/>
    </div>
</div>
<div>
    <span class="font-bold text-lg">Dagegen</span>
    <div class="flex mt-2 gap-3 py-4">
        <BadgeList votes={votes} />
    </div>

</div>

