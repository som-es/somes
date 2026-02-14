<script lang="ts">

	import { isThere } from '$lib/api/api';
	import type { GovProposal } from '$lib/types';
	import FilterGroup from '../Filtering/FilterGroup.svelte';
	
	interface Props {
		filteredGovProposals: GovProposal[];
		allGovProposals: GovProposal[];
	}

	let { filteredGovProposals = $bindable(), allGovProposals }: Props = $props();

	let filters = $state([
		{
			title: 'mit Abstimmung',
			activeValue: undefined,
			hidden: false,
			options: [
				{ title: 'egal', value: undefined },
				{ title: 'Ja', value: true },
				{ title: 'Nein', value: false }
			]
		}]);

	$effect(() => {
		filteredGovProposals = allGovProposals.filter((prop) => {
			let keep = true;
			if (filters[0].activeValue != undefined) {
				keep = keep && isThere(prop.vote_result) == filters[0].activeValue;
			}
			return keep;
		});
	});

</script>

<FilterGroup group={filters[0]} />

{#if filteredGovProposals.length === 0}
	<p class="text-center">Keine Ministerialentwürfe gefunden.</p>
{/if}