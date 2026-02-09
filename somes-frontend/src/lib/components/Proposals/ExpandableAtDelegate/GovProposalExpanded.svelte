<script lang="ts">
	import Topics from '$lib/components/Topics/Topics.svelte';
	import SButton from '$lib/components/UI/SButton.svelte';
	import { createVoteResultPath, type Delegate, type VoteResult } from '$lib/types';
	import Emphasis from '../../VoteResults/Emphasis/Emphasis.svelte';
	import InfoTiles from '../../VoteResults/InfoTiles/InfoTiles.svelte';
	import { currentDelegatesAtDateStore, currentGovProposalDelegateStore } from '$lib/stores/stores';
	import { gotoHistory } from '$lib/goto';
	import VoteTypeBadge from '../../VoteResults/VoteTypeBadge.svelte';
	import { type GovProposal } from '$lib/types';
	import { createGovProposalPath } from '../types';

	export let govProposal: GovProposal;
	export let delegate: Delegate;
	export let showDelegate: boolean = false;
	export let open: boolean = true;

	let delsAtDate: Delegate[] = [];

	function onShowDetails(govProposal: GovProposal, delegate: Delegate) {
		currentGovProposalDelegateStore.value = { gov_proposal: govProposal, delegate };
		gotoHistory(createGovProposalPath(govProposal.ministrial_proposal), true);
	}

	$: topics = (
		govProposal.eurovoc_topics.length > 0
			? govProposal.eurovoc_topics
			: (govProposal.ai_summary?.full_summary?.topics ?? []).map((topic) => {
					return { topic };
				})
	).sort((a, b) => {
		return a.topic.length - b.topic.length;
	});
</script>

<div class="entry mt-3 hidden bg-primary-200 p-2 lg:block dark:bg-primary-400">
	<div class="flex gap-2">
		<div class="flex-grow basis-3/4">
			{#if govProposal.ai_summary}
				<Emphasis
					emphasis={govProposal.ai_summary.full_summary.key_points}
					glossary={govProposal.ai_summary.full_summary.glossary}
				/>
			{:else}
				<!-- <Emphasis
					emphasis={}
				/> -->
			{/if}
		</div>
		<div class="flex min-w-[320px] flex-grow basis-1/4 flex-col justify-between">
			<!-- <div class="flex gap-2">
					<div class="flex flex-1">
						<InfoTiles {voteResult} showRequiredMajority={false} showAchievedVotes={true} {dels} />
					</div>
					<div class="xl:flex flex-1 hidden">
						<InfoTiles {voteResult} showRequiredMajority={true} showAchievedVotes={false} {dels} />
					</div>
				</div> -->
			<div class="pt-2">
				<Topics {topics} />
			</div>
			<div class="flex h-8 items-center justify-end rounded-xl px-2">
				<button
					on:click={() => onShowDetails(govProposal, delegate)}
					class="rounded-lg bg-primary-300 px-2 py-1"
				>
					<span class="text-base font-semibold">Mehr Details &#8594;</span>
				</button>
			</div>
		</div>
	</div>
</div>
