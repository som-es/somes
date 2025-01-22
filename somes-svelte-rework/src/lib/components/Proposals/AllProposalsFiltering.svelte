<script lang="ts">
	import { isHasError, isThere } from "$lib/api/api";
	import type { GovProposal } from "$lib/types";
	import { RadioGroup, RadioItem } from "@skeletonlabs/skeleton";

    export let filteredGovProposals: GovProposal[];
    export let allGovProposals: GovProposal[] | undefined;

    let alreadyVotedOnFilter: boolean | undefined = undefined;
    // let alreadyVotedOnFilter: boolean | undefined = undefined;

    $: if (allGovProposals) {
        filteredGovProposals = allGovProposals.filter((prop) => {
            let keep = true;
            if (alreadyVotedOnFilter != undefined) {
                keep = keep && ((isThere(prop.vote_result)) == alreadyVotedOnFilter)
            }
            return keep;
        });
        // filteredGovProposals = filteredGovProposals;
    }

</script>

<div class="max-lg:hidden flex gap-4 flex-wrap">
	<div class="mt-5">
		<h1 class="text-2xl font-bold">Mit Abstimmung</h1>
		<RadioGroup
			rounded="rounded-container-token sm:!rounded-token"
			active="variant-filled-secondary"
			hover="hover:variant-soft-secondary"
			flexDirection="flex-col sm:flex-row"
		>
			<RadioItem bind:group={alreadyVotedOnFilter} name="alreadyVotedOn" value={undefined}
				>egal</RadioItem
			>
			<RadioItem bind:group={alreadyVotedOnFilter} name="alreadyVotedOn" value={false}
				>keine Abstimmung</RadioItem
			>
			<RadioItem bind:group={alreadyVotedOnFilter} name="alreadyVotedOn" value={true}
				>mit Abstimmung</RadioItem
			>
		</RadioGroup>
	</div>
	<div class="mt-5">
		<h1 class="text-2xl font-bold">Angenommen</h1>
		<RadioGroup
			rounded="rounded-container-token sm:!rounded-token"
			active="variant-filled-secondary"
			hover="hover:variant-soft-secondary"
			flexDirection="flex-col sm:flex-row"
		>
			<!-- <RadioItem bind:group={acceptedFilter} name="accepted" value={undefined}>egal</RadioItem>
			<RadioItem bind:group={acceptedFilter} name="accepted" value={'a'}>angenommen</RadioItem>
			<RadioItem bind:group={acceptedFilter} name="accepted" value={'d'}>abgelehnt</RadioItem>
			<RadioItem
				bind:group={acceptedFilter}
				name="accepted"
				value={'p'}
				title="frühzeitig abgelehnt - vor der 3. Lesung">frühzeitig abgelehnt</RadioItem
			> -->
		</RadioGroup>
	</div>
	<div class="mt-5">
		<h1 class="text-2xl font-bold">Abstimmung</h1>
		<!-- <RadioGroup active="variant-filled-secondary" hover="hover:variant-soft-secondary">
			<RadioItem bind:group={namedVoteFilter} name="namedVote" value={undefined}>egal</RadioItem>
			<RadioItem bind:group={namedVoteFilter} name="namedVote" value={true}
				>namentliche Abstimmung</RadioItem
			>
		</RadioGroup> -->
	</div>
</div>

