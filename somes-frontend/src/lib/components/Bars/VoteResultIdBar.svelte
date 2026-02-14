<script lang="ts">
	import { errorToNull, vote_result_by_id, vote_result_by_path } from '$lib/api/api';
	import VoteParliament2 from '$lib/components/Parliaments/VoteParliament2.svelte';
	import { createVoteResultPath, type NamedVote, type Reference, type VoteResult } from '$lib/types';
	import { currentVoteResultStore } from '$lib/stores/stores';
	import rightArrowIcon from '$lib/assets/misc_icons/right-arrow.svg?raw';
	import { gotoHistory } from '$lib/goto';
	import ExpandablePlaceholder from '$lib/components/VoteResults/Expandable/Placeholders/ExpandablePlaceholder.svelte';
	import { removeUserTopic } from '$lib/api/authed';
	import { createEventDispatcher } from 'svelte';
	import { invalidate } from '$app/navigation';

	export let legis_init_id: number | null = null;
	export let legis_init_ref: Reference | null = null;
	export let requiringVotes: boolean = false;
	export let countObj: { count: number } | null = null;
	export const acceptedCondition: (voteResult: VoteResult) => boolean = (voteResult) => (voteResult.votes ?? []).length > 0;

	let voteResult: VoteResult | null = null;

	let loadingVoteResult = false;
  	
	const dispatch = createEventDispatcher();

	const updateCount = () => {
		if (countObj === null) {
			countObj = {count: 0}
		}
		if (voteResult && acceptedCondition(voteResult) && countObj) {
			countObj.count++;
		}
	};

	$: if (legis_init_id || legis_init_ref) {
		voteResult = null;
		loadingVoteResult = true;
		if (legis_init_id) {
			vote_result_by_id(legis_init_id.toString()).then((res) => {
				voteResult = errorToNull(res);
				loadingVoteResult = false;
				updateCount();
			});
		}
		if (legis_init_ref) {
			vote_result_by_path(legis_init_ref.gp, legis_init_ref.ityp, legis_init_ref.inr.toString()).then((res) => {
				voteResult = errorToNull(res);
				loadingVoteResult = false;
				updateCount();
			});
		}	
	}

	function onShowDetails(voteResult: VoteResult | null, e: Event) {
		// e.preventDefault();
		if (!voteResult) return;
		// dispatch("dataUpdated", voteResult);
		currentVoteResultStore.value = voteResult;
		// modalStore.close();
		// gotoHistory(createVoteResultPath(voteResult), true);
	}

	$: arrowBackground =
		voteResult != null && voteResult.votes.length > 0
			? 'bg-secondary-400'
			: 'dark:bg-primary-300 bg-primary-400';
	$: hasVotes = (voteResult?.votes ?? []).length > 0;
</script>

{#if (requiringVotes && hasVotes) || !requiringVotes || loadingVoteResult}
<div class="gap-3 mt-5">
	{#if voteResult}
		<a class="w-full entry flex {arrowBackground} justify-between items-center text-black" href="{createVoteResultPath(voteResult)}" on:click={(e) => onShowDetails(voteResult, e)}>
			<div
				class="border-radius-left spacing-for-left flex dark:bg-primary-300 bg-primary-400 justify-between items-center flex-basis-left"
			>
				<div class="flex flex-col gap-1">
					{#if voteResult.ai_summary}
						<div class="flex flex-1 flex-col flex-wrap min-w-0 max-lg:contents">
							<span
								class="text-xl font-semibold max-lg:order-1 max-lg:flex-1 max-lg:min-w-0"
								style="hyphens: auto; word-break: normal; overflow-wrap: break-word;"
							>
								{voteResult.ai_summary.short_title}
							</span>
							<span class="text-sm sm:text-md max-lg:order-3 max-lg:w-full">
								{voteResult.ai_summary.short_summary}
							</span>
						</div>
					{:else}
						<span class="text-md flex-1 font-semibold min-w-0">
							{voteResult.legislative_initiative.description}
						</span>
					{/if}


					<slot />
				</div>

				{#if hasVotes}
					<div
						class="max-sm:hidden z-20 w-30 bg-primary-100 dark:bg-primary-300 rounded-md"
					>
						<VoteParliament2 {voteResult} preview={true} />
					</div>
				{/if}
			</div>

			{#if hasVotes}
				<div class="spacing-for-right p-36">
					{@html rightArrowIcon}
				</div>
			{/if}
		
		</a>
	{:else if loadingVoteResult}
		<ExpandablePlaceholder class="min-w-7xl w-7xl" />
		<!-- {:else if named_vote.about}
		<div class="rounded-[0.9rem] spacing-for-left flex dark:bg-primary-300 bg-primary-400 justify-between items-center flex-basis-left">
			<div class="flex flex-col">
				<div class="text-lg font-bold">{opinion}</div>
				<div>{named_vote.about}</div>
			</div>
		</div>  -->
	{/if}
</div>
{/if}

<style>
	.entry {
		border-radius: 0.9rem;
		box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
	}

	.border-radius-left {
		border-top-left-radius: 0.9rem;
		border-bottom-left-radius: 0.9rem;
	}

	.spacing-for-left {
		padding: 20px;
		gap: 10px;
	}

	.spacing-for-right {
		padding: 20px;
		gap: 10px;
	}

	.flex-basis-left {
		flex-basis: 96%;
	}
</style>
