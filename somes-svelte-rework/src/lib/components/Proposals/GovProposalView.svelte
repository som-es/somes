<script lang="ts">
	import Documents from '$lib/components/Documents/Documents.svelte';
	import Emphasis from '$lib/components/VoteResults/Emphasis/Emphasis.svelte';
	import { dashDateToDotDate } from '$lib/date';
	import type { GovProposalDelegate } from '$lib/types';
	import { split } from 'postcss/lib/list';
	import AiSummaryHintPopup from '../AiHint/AiSummaryHintPopup.svelte';
	import DelegateCard from '../Delegates/DelegateCard.svelte';
	import Topics from '../Topics/Topics.svelte';
	import GlossaryText from '../UI/GlossaryText.svelte';
	import InfoBadgesCustom from '../VoteResults/InfoTiles/InfoBadgesCustom.svelte';

	export let govProposal: GovProposalDelegate;

	$: aiSummary = govProposal.gov_proposal.ai_summary;

	$: documentUrl = `https://parlament.gv.at/gegenstand/${govProposal.gov_proposal.ministrial_proposal.gp}/ME/${govProposal.gov_proposal.ministrial_proposal.inr}`;
	$: date = new Date(govProposal.gov_proposal.ministrial_proposal.raw_data_created_at).toLocaleDateString();
</script>

<title>
	{#if govProposal.gov_proposal.ai_summary}
		{govProposal.gov_proposal.ai_summary.short_title}
	{:else}
		{govProposal.gov_proposal.ministrial_proposal.description}
	{/if}
</title>

<div class="entry bg-primary-200 dark:bg-primary-400 mt-3 flex max-lg:flex-wrap gap-3">
	<div class="flex flex-col gap-2 w-full">
		<div class="rounded-xl bg-primary-300 dark:bg-primary-500 px-3 py-3">
			<div class="flex jusify-between items-start">
				<div class="flex items-center gap-4">
					<div class="flex flex-col">
						<span class="leading-tight">
							{#if aiSummary}
								<AiSummaryHintPopup {aiSummary} />
								<span class="text-3xl font-bold">
									{aiSummary.short_title}
								</span>
							{:else}
								<span class="text-3xl font-bold">
									{govProposal.gov_proposal.ministrial_proposal.description}
								</span>
							{/if}
						</span>
						<span class="text-sm opacity-90"> Ministerialentwurf vom </span>
					</div>
				</div>
				<a href={documentUrl} target="_blank">
					<img
						class="w-28"
						alt="parlament.gv.at favicon"
						src="https://www.parlament.gv.at/static/img/favicon/favicon.svg"
					/>
				</a>
			</div>
			<div
				class="flex flex-wrap justify-between items-center gap-3 w-full border-t border-black/5 dark:border-white/5 pt-1"
			>
				<div class="flex-shrink-0">
					<InfoBadgesCustom texts={[
						govProposal.gov_proposal.ministrial_proposal.ressort, 
						date, 
						govProposal.gov_proposal.ministrial_proposal.gp
					]}/>
				</div>

				<div class="flex-1 flex justify-end">
					{#if aiSummary && govProposal.gov_proposal.eurovoc_topics.length == 0}
						<Topics
							topics={aiSummary.full_summary.topics
								.sort((a, b) => {
									return a.length - b.length;
								})
								.map((topic) => {
									return { topic };
								})}
						/>
					{:else}
						<Topics
							topics={govProposal.gov_proposal.eurovoc_topics.sort((a, b) => {
								return a.topic.length - b.topic.length;
							})}
						/>
					{/if}
				</div>
			</div>
		</div>

		{#if govProposal.gov_proposal.ai_summary}
			<div class="emphasis-item rounded-xl bg-primary-300 dark:bg-primary-500 px-3 pt-3 pb-3">
				<h1 class="font-bold text-lg md:text-xl">Zusammenfassung</h1>
				<span class="text-sm lg:text-base">
					<GlossaryText
						text={govProposal.gov_proposal.ai_summary.short_summary}
						glossary={govProposal.gov_proposal.ai_summary.full_summary.glossary}
					/>
				</span>
			</div>
			<Emphasis
				emphasis={govProposal.gov_proposal.ai_summary.full_summary.key_points}
				glossary={govProposal.gov_proposal.ai_summary.full_summary.glossary}
			/>
		{/if}
		<div class="flex flex-wrap gap-2 w-full">
			{#if govProposal.gov_proposal.documents.length > 0}
				<div class="rounded-xl bg-primary-300 dark:bg-primary-500 p-3">
					<Documents documents={govProposal.gov_proposal.documents} />
				</div>
			{/if}
		</div>
	</div>

	<div class="rounded-xl bg-primary-300 dark:bg-primary-500 px-3 py-3">
		<DelegateCard delegate={govProposal.delegate} />
	</div>
</div>

<style>
	.entry {
		border-radius: 0.9rem;
		box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
		padding: 11px;
		gap: 10px;
	}
</style>
