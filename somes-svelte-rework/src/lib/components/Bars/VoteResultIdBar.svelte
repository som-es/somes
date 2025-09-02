<script lang="ts">
	import { errorToNull, vote_result_by_id, vote_result_by_path } from '$lib/api/api';
	import VoteParliament2 from '$lib/components/Parliaments/VoteParliament2.svelte';
	import { createVoteResultPath, type NamedVote, type Reference, type VoteResult } from '$lib/types';
	import { currentVoteResultStore } from '$lib/stores/stores';
	import rightArrowIcon from '$lib/assets/misc_icons/right-arrow.svg?raw';
	import { gotoHistory } from '$lib/goto';
	import { getModalStore } from '@skeletonlabs/skeleton';
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
		console.log(countObj);
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

	const modalStore = getModalStore();

	function onShowDetails(voteResult: VoteResult | null) {
		if (!voteResult) return;
		dispatch("dataUpdated", voteResult);
		currentVoteResultStore.set(voteResult);
		modalStore.close();
		gotoHistory(createVoteResultPath(voteResult), true);
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
		<button class="w-full entry flex {arrowBackground} justify-between items-center text-black" on:click={() => onShowDetails(voteResult)}>
			<div
				class="border-radius-left spacing-for-left flex dark:bg-primary-300 bg-primary-400 justify-between items-center flex-basis-left"
			>
				<div class="flex flex-col gap-1">
					<div class="sm:text-xl">{voteResult.legislative_initiative.description}</div>
					<slot />
				</div>

				{#if hasVotes}
					<button
						class="max-sm:hidden z-20 w-[7.5rem] bg-primary-100 dark:bg-primary-300 rounded-md"
						on:click={() => onShowDetails(voteResult)}
					>
						<VoteParliament2 {voteResult} preview={true} />
					</button>
				{/if}
			</div>

			{#if hasVotes}
				<button class="spacing-for-right p-36" on:click={() => onShowDetails(voteResult)}>
					{@html rightArrowIcon}
				</button>
			{/if}
		
		</button>
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
