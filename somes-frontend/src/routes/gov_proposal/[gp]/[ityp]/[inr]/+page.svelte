<script lang="ts">
	import { errorToNull } from '$lib/api/api';
	import Container from '$lib/components/Layout/Container.svelte';
	import MinisterialView from '$lib/components/MinisterialView/MinisterialView.svelte';
	import type { MinisterialViewData } from '$lib/components/MinisterialView/types';
	import VoteParliament2 from '$lib/components/Parliaments/VoteParliament2.svelte';
	import ExpandablePlaceholder from '$lib/components/VoteResults/Expandable/Placeholders/ExpandablePlaceholder.svelte';
	import VoteResultExpandableBar from '$lib/components/VoteResults/Expandable/VoteResultExpandableBar.svelte';
	import type { GovProposalDelegate } from '$lib/types';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();

	let govProposalDelegate: GovProposalDelegate | null = $derived(errorToNull(data.govProposal));

	let ministerialData: MinisterialViewData | null = $derived.by(() => {
		if (govProposalDelegate == null) return null;
		return {
			aiSummary: govProposalDelegate.gov_proposal.ai_summary,
			alternativeTitle: govProposalDelegate.gov_proposal.ministrial_proposal.description,
			date: govProposalDelegate.gov_proposal.ministrial_proposal.raw_data_created_at,
			originalDocumentUrl: `https://parlament.gv.at/gegenstand/${govProposalDelegate.gov_proposal.ministrial_proposal.gp}/ME/${govProposalDelegate.gov_proposal.ministrial_proposal.inr}`,
			documents: govProposalDelegate.gov_proposal.documents,
			topics: govProposalDelegate.gov_proposal.topics,
			otherKeywordTopics: govProposalDelegate.gov_proposal.other_keyword_topics,
			eurovocTopics: govProposalDelegate.gov_proposal.eurovoc_topics,
			delegates: govProposalDelegate.delegates,
			ressort: govProposalDelegate.gov_proposal.ministrial_proposal.ressort,
			ressortShortform: govProposalDelegate.gov_proposal.ministrial_proposal.ressort_shortform,
			ministerialIssuers: govProposalDelegate.gov_proposal.ministerial_issuers,
			type: 'gov_proposal',
			infoBadges: [
				govProposalDelegate.gov_proposal.ministrial_proposal.ressort,
				new Date(
					govProposalDelegate.gov_proposal.ministrial_proposal.raw_data_created_at
				).toLocaleDateString(),
				govProposalDelegate.gov_proposal.ministrial_proposal.gp
			].filter((x) => x !== null) as string[]
		};
	});
</script>

<svelte:head>
	<title>Ministerialentwurf</title>
	<meta name="description" content="Spezifischer Ministerialentwurf" />
</svelte:head>

<Container>
	{#if ministerialData && govProposalDelegate}
		<MinisterialView {ministerialData}>
			{#if govProposalDelegate.gov_proposal.vote_result}
				<div class="entry block bg-primary-300 p-4 dark:bg-primary-500">
					<span class="text-lg font-semibold md:text-xl"> Regierungsvorlage </span>
					<VoteResultExpandableBar voteResult={govProposalDelegate.gov_proposal.vote_result} />
				</div>
			{/if}
			<!-- <div class="">
					<VoteParliament2 voteResult={govProposalDelegate.gov_proposal.vote_result} />
			</div> -->
		</MinisterialView>
	{:else}
		{#each { length: 10 } as _}
			<ExpandablePlaceholder />
		{/each}
	{/if}
</Container>

<style>
	.entry {
		border-radius: 0.9rem;
		box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
		gap: 10px;
	}
</style>
