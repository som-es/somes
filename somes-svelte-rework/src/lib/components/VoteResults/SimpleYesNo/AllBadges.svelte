<script lang="ts">
	import { groupPartyDelegates } from "$lib/parliaments/defaultParliament";
	import type { Delegate, Vote } from "$lib/types";
	import BadgeList from "./BadgeList.svelte";

    export let delsAtDate: Delegate[];
    let votes: Vote[] = [];
    $: {
	    let partyToDelegates: Map<string, Delegate[]> = groupPartyDelegates(delsAtDate);
        partyToDelegates.forEach((dels, party) => {
            votes.push({
                party,
                fraction: dels.length,
                infavor: true,
                legislative_initiatives_id: 0
            })
        })
    }

</script>

<div class="flex flex-wrap mt-0 gap-3 py-2">
    <BadgeList {votes} infavor />
</div>
