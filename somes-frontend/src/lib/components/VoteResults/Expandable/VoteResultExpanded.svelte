<script lang="ts">
	import Topics from '$lib/components/Topics/Topics.svelte';
	import SButton from '$lib/components/UI/SButton.svelte';
	import { createVoteResultPath, type Delegate, type VoteResult } from '$lib/types';
	import Emphasis from '../Emphasis/Emphasis.svelte';
	import InfoTiles from '../InfoTiles/InfoTiles.svelte';
	import { currentDelegatesAtDateStore, currentVoteResultStore } from '$lib/stores/stores';
	import { gotoHistory } from '$lib/goto';
	import VoteTypeBadge from '../VoteTypeBadge.svelte';

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
</script>

<div class="entry bg-primary-200 dark:bg-primary-400 mt-3 p-2 lg:block hidden">
	<div class="flex gap-2">
			<div class="flex-grow basis-3/4">
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
			<div class="flex flex-col justify-between flex-grow basis-1/4 min-w-[320px]">
				<!-- <div class="flex gap-2">
					<div class="flex flex-1">
						<InfoTiles {voteResult} showRequiredMajority={false} showAchievedVotes={true} {dels} />
					</div>
					<div class="xl:flex flex-1 hidden">
						<InfoTiles {voteResult} showRequiredMajority={true} showAchievedVotes={false} {dels} />
					</div>
				</div> -->
				<div class="pt-2">
					<Topics
						topics={voteResult.eurovoc_topics.sort((a, b) => {
							return a.topic.length - b.topic.length;
						})}
					/>
				</div>
				<div class="flex items-center justify-end h-8 rounded-xl px-2">
					<button on:click={onShowDetails} class="rounded-lg px-2 py-1 bg-primary-300">
						<span class="text-base font-semibold">Mehr Details &#8594;</span>
					</button>
				</div>
			</div>
	</div>
</div>