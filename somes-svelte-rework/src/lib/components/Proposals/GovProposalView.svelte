<script lang="ts">
	import Documents from '$lib/components/Documents/Documents.svelte';
	import Emphasis from '$lib/components/VoteResults/Emphasis/Emphasis.svelte';
	import { dashDateToDotDate } from '$lib/date';
	import type { GovProposalDelegate } from '$lib/types';
	import DelegateCard from '../Delegates/DelegateCard.svelte';

	export let govProposal: GovProposalDelegate;


	$: documentUrl = `https://parlament.gv.at/gegenstand/${govProposal.gov_proposal.ministrial_proposal.gp}/ME/${govProposal.gov_proposal.ministrial_proposal.inr}`;
</script>

<title>
	{govProposal.gov_proposal.ministrial_proposal.description}
</title>

<div class="entry bg-primary-200 dark:bg-primary-400 mt-3 flex max-lg:flex-wrap gap-3">
	<div class="flex flex-col gap-3 w-full">
		<div class="rounded-xl bg-primary-300 dark:bg-primary-500 px-3 py-3">
			<div class="flex justify-between">
				<div>
					<h1 class="font-bold text-xl md:text-3xl">Ministerialentwurf</h1>
					<span class="md:text-xl">{govProposal.gov_proposal.ministrial_proposal.description}</span>
				</div>
				<a href={documentUrl} target="_blank">
					<img
						class="w-28"
						alt="parlament.gv.at favicon"
						src="https://www.parlament.gv.at/static/img/favicon/favicon.svg"
					/>
				</a>
			</div>
		</div>
		{#if govProposal.gov_proposal.ai_summary}
			<Emphasis 
				emphasis={govProposal.gov_proposal.ai_summary.full_summary.key_points} useTitleHover glossary={govProposal.gov_proposal.ai_summary.full_summary.glossary} />
		{/if}
		<div class="flex flex-wrap gap-2 w-full">
			<div class="flex flex-wrap gap-1 rounded-xl bg-primary-300 dark:bg-primary-500 px-3 py-3">
				<span class="badge bg-tertiary-400 text-wrap text-lg text-black"
					>{govProposal.gov_proposal.ministrial_proposal.ressort}</span
				>
				<span class="badge bg-tertiary-400 text-lg text-black"
					>{dashDateToDotDate(govProposal.gov_proposal.ministrial_proposal.created_at)}</span
				>
				<span class="badge bg-tertiary-400 text-lg text-black"
					>{govProposal.gov_proposal.ministrial_proposal.gp}</span
				>
			</div>
			<!-- <DecreeInfoTiles {decree} /> -->
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
