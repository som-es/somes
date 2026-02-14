<script lang="ts">
	import Topics from '$lib/components/Topics/Topics.svelte';
	import SButton from '$lib/components/UI/SButton.svelte';
	import { createVoteResultPath, type Delegate, type VoteResult } from '$lib/types';
	import Emphasis from '../Emphasis/Emphasis.svelte';
	import InfoTiles from '../InfoTiles/InfoTiles.svelte';
	import { currentDelegatesAtDateStore, currentVoteResultStore } from '$lib/stores/stores';
	import { gotoHistory } from '$lib/goto';
	import VoteTypeBadge from '../VoteTypeBadge.svelte';
	import MovingArrowButton from '$lib/components/UI/MovingArrowButton.svelte';

	export let voteResult: VoteResult;
	export let open: boolean = true;

	let delsAtDate: Delegate[] = [];

	function onShowDetails() {
		currentVoteResultStore.value = voteResult;
		currentDelegatesAtDateStore.value = [
			voteResult.legislative_initiative.nr_plenary_activity_date.toString(),
			delsAtDate
		];
		gotoHistory(createVoteResultPath(voteResult), true);
	}

	$: topics = (
		voteResult.eurovoc_topics.length > 0
			? voteResult.eurovoc_topics
			: (voteResult.ai_summary?.full_summary?.topics ?? []).map((topic) => {
					return { topic };
				})
	).sort((a, b) => {
		return a.topic.length - b.topic.length;
	});
</script>

<div class="entry rounded-xl mt-3 hidden bg-primary-200 p-2 lg:block dark:bg-primary-400">
	<div class="flex gap-2">
		<div class="grow basis-3/4">
			{#if voteResult.ai_summary}
				<Emphasis
					emphasis={voteResult.ai_summary.full_summary.key_points}
					glossary={voteResult.ai_summary.full_summary.glossary}
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
				<MovingArrowButton  
					onclick={onShowDetails}
				/>
			</div>
		</div>
	</div>
</div>
