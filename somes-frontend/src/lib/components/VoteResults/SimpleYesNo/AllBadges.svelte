<script lang="ts">
	import { groupPartyDelegates } from '$lib/parliaments/defaultParliament';
	import type { Delegate, Vote } from '$lib/types';
	import BadgeList from './BadgeList.svelte';
	import BadgePlaceholder from './BadgePlaceholder.svelte';

	export let delsAtDate: Delegate[];
	let votes: Vote[] = [];

	$: {
		const newVotes: Vote[] = [];
		let partyToDelegates: Map<string, Delegate[]> = groupPartyDelegates(delsAtDate);
		partyToDelegates.forEach((dels, party) => {
			newVotes.push({
				party,
				fraction: dels.length,
				code: null,
				infavor: true,
				legislative_initiatives_id: 0
			});
		});
		votes = newVotes.sort((a, b) => b.fraction - a.fraction);
	}
</script>

{#if votes.length > 0}
	<div class="flex flex-wrap mt-0 gap-3 py-2">
		<BadgeList {votes} infavor />
	</div>
{:else}
	<BadgePlaceholder />
{/if}
