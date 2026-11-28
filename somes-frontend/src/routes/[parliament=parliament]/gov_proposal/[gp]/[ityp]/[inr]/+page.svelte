<script lang="ts">
	import { SITE_URL } from '$lib/seo/sitemap';
	import { breadcrumb_json_ld } from '$lib/seo/breadcrumbs';
	import { json_ld_script } from '$lib/seo/delegate';
	import { ministerialInfoBadges } from '$lib/components/MinisterialView/infoBadges';
	import { meta_description } from '$lib/seo/text';
	import { errorToNull } from '$lib/api/api';
	import { t } from '$lib/i18n/i18n.svelte';
	import ReferencedByBar from '$lib/components/Bars/ReferencedByBar.svelte';
	import Container from '$lib/components/Layout/Container.svelte';
	import MinisterialView from '$lib/components/MinisterialView/MinisterialView.svelte';
	import type { MinisterialViewData } from '$lib/components/MinisterialView/types';
	import MoodBarometer from '$lib/components/MoodBarometer/MoodBarometer.svelte';
	import ExpandablePlaceholder from '$lib/components/VoteResults/Expandable/Placeholders/ExpandablePlaceholder.svelte';
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
			infoBadges: ministerialInfoBadges(
				govProposalDelegate.gov_proposal.ministrial_proposal.ressort,
				govProposalDelegate.gov_proposal.ministrial_proposal.raw_data_created_at,
				govProposalDelegate.gov_proposal.ministrial_proposal.gp
			),
			gp: govProposalDelegate.gov_proposal.ministrial_proposal.gp
		};
	});

	const title = $derived(
		govProposalDelegate?.gov_proposal?.ai_summary?.short_title ||
			govProposalDelegate?.gov_proposal?.ministrial_proposal?.description
	);
	const content = $derived(
		meta_description(
			govProposalDelegate?.gov_proposal?.ai_summary?.short_summary ||
				govProposalDelegate?.gov_proposal?.ministrial_proposal?.description ||
				''
		)
	);
</script>

<svelte:head>
	<title>{title ? `${title} | Somes` : 'Somes'}</title>
	<meta name="description" {content} />
	{#if govProposalDelegate && title}
		{@html json_ld_script(
			breadcrumb_json_ld([
				{ name: t('nav.ministerialDrafts'), path: `/${data.parliament}/history/proposals` },
				{ name: title }
			])
		)}
	{/if}
	{#if govProposalDelegate}
		<meta
			property="og:image"
			content="{SITE_URL}/og/{data.parliament}/gov_proposal/{govProposalDelegate.gov_proposal
				.ministrial_proposal.gp}/{govProposalDelegate.gov_proposal.ministrial_proposal.inr}.png"
		/>
	{/if}
</svelte:head>

<Container>
	{#if ministerialData && govProposalDelegate}
		<!-- Regierungsvorlage -->
		{#snippet voteable()}
			{#if govProposalDelegate.gov_proposal.vote_result}
				<div class="entry block bg-primary-300 p-4 dark:bg-primary-500">
					<span class="mb-1 text-lg font-semibold md:text-xl"> {t('govProposal.title')} </span>
					<ReferencedByBar
						ref={govProposalDelegate.gov_proposal.vote_result}
						showRequiredMajority
					/>
					<!-- <VoteResultExpandableBar voteResult={govProposalDelegate.gov_proposal.vote_result} /> -->
				</div>
			{/if}
		{/snippet}
		{#snippet mood()}
			<MoodBarometer
				gp={govProposalDelegate.gov_proposal.ministrial_proposal.gp}
				inr={govProposalDelegate.gov_proposal.ministrial_proposal.inr}
			/>
		{/snippet}
		<MinisterialView
			{ministerialData}
			snippets={{
				voteable: govProposalDelegate.gov_proposal.vote_result === null ? undefined : voteable,
				mood
			}}
		/>
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
