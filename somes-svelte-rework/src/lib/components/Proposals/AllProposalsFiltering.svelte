<script lang="ts">
	import { isHasError, isThere } from '$lib/api/api';
	import type { GovProposal } from '$lib/types';
	import { Segment } from '@skeletonlabs/skeleton-svelte';

	export let filteredGovProposals: GovProposal[];
	export let allGovProposals: GovProposal[] | undefined;

	let alreadyVotedOnFilter: boolean | undefined = undefined;
	// let alreadyVotedOnFilter: boolean | undefined = undefined;

	$: if (allGovProposals) {
		filteredGovProposals = allGovProposals.filter((prop) => {
			let keep = true;
			if (alreadyVotedOnFilter != undefined) {
				keep = keep && isThere(prop.vote_result) == alreadyVotedOnFilter;
			}
			return keep;
		});
		// filteredGovProposals = filteredGovProposals;
	}
</script>

<div class="max-lg:hidden flex gap-4 flex-wrap">
	<div class="mt-5">
		<h1 class="text-2xl font-bold">Mit Abstimmung</h1>
		<Segment
			rounded="rounded-container sm:!rounded-base"
			active="preset-filled-secondary-500"
			hover="hover:preset-tonal-secondary"
			flexDirection="flex-col sm:flex-row"
		>
			<Segment.Item bind:group={alreadyVotedOnFilter} name="alreadyVotedOn" value={undefined}
				>egal</Segment.Item
			>
			<Segment.Item bind:group={alreadyVotedOnFilter} name="alreadyVotedOn" value={false}
				>keine Abstimmung</Segment.Item
			>
			<Segment.Item bind:group={alreadyVotedOnFilter} name="alreadyVotedOn" value={true}
				>mit Abstimmung</Segment.Item
			>
		</Segment>
	</div>
	<div class="mt-5">
		<h1 class="text-2xl font-bold">Angenommen</h1>
		<Segment
			rounded="rounded-container sm:!rounded-base"
			active="preset-filled-secondary-500"
			hover="hover:preset-tonal-secondary"
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
		</Segment>
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
