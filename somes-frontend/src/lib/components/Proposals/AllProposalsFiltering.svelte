<script lang="ts">
	import { t } from '$lib/i18n/i18n.svelte';
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
			title: t('filter.votingWith'),
			activeValue: undefined,
			hidden: false,
			options: [
				{ title: t('filter.egal'), value: undefined },
				{ title: t('filter.yes'), value: true },
				{ title: t('filter.no'), value: false }
			]
		}
	]);

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

<FilterGroup bind:group={filters[0]} />

{#if filteredGovProposals.length === 0}
	<p class="text-center">Keine Ministerialentwürfe gefunden.</p>
{/if}
