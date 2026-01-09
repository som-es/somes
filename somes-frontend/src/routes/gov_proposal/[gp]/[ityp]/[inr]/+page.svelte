<script lang="ts">
	import { errorToNull } from '$lib/api/api';
	import Container from '$lib/components/Layout/Container.svelte';
	import MinisterialView from '$lib/components/MinisterialView/MinisterialView.svelte';
	import type { MinisterialViewData } from '$lib/components/MinisterialView/types';
	import VoteParliament2 from '$lib/components/Parliaments/VoteParliament2.svelte';
	import SButton from '$lib/components/UI/SButton.svelte';
	import ExpandablePlaceholder from '$lib/components/VoteResults/Expandable/Placeholders/ExpandablePlaceholder.svelte';
	import { hasGoBackStore } from '$lib/stores/stores';
	import type { GovProposalDelegate } from '$lib/types';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();

	let govProposalDelegate: GovProposalDelegate | null = $derived(errorToNull(data.govProposal));

	const goBack = () => {
		history.back();
	};

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
			delegate: govProposalDelegate.delegate,
			ressort: govProposalDelegate.gov_proposal.ministrial_proposal.ressort,
			ressortShortform: govProposalDelegate.gov_proposal.ministrial_proposal.ressort_shortform,
			ministerialIssuers: govProposalDelegate.gov_proposal.ministerial_issuers,
			type: "gov_proposal",
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
	{#if hasGoBackStore.value}
		<SButton class="bg-primary-500" on:click={goBack}>Zurück</SButton>
	{/if}

	{#if ministerialData && govProposalDelegate}
		<MinisterialView ministerialData={ministerialData}>
			<!-- <div class="">
				{#if govProposalDelegate.gov_proposal.vote_result}
					<VoteParliament2 voteResult={govProposalDelegate.gov_proposal.vote_result} />
				{/if}
			</div> -->
		</MinisterialView>
	{:else}
		{#each { length: 10 } as _}
			<ExpandablePlaceholder />
		{/each}
	{/if}
</Container>
