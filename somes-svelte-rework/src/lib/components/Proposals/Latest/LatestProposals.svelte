<script lang="ts">
	import type { GovProposalDelegate } from "$lib/types";
	import LatestProposal from "./LatestProposal.svelte";

    export let govProposals: GovProposalDelegate[];

</script>

{#each govProposals as govProposal}
    <LatestProposal {govProposal} />
{/each}