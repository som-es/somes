<script lang="ts">
	import { errorToNull, vote_result_by_id } from '$lib/api/api';
	import VoteParliament2 from '$lib/components/Parliaments/VoteParliament2.svelte';
	import { createVoteResultPath, type Speech, type VoteResult } from '$lib/types';
	import { currentVoteResultStore } from '$lib/stores/stores';
	import rightArrowIcon from '$lib/assets/misc_icons/right-arrow.svg?raw';
	import clockIcon from '$lib/assets/misc_icons/clock-two.svg?raw';
	import { gotoHistory } from '$lib/goto';
	import ExpandablePlaceholder from '$lib/components/VoteResults/Expandable/Placeholders/ExpandablePlaceholder.svelte';

	export let speech: Speech;

	let voteResult: VoteResult | null = null;

	let loadingVoteResult = false;

	$: if (speech.legis_init_id) {
		voteResult = null;
		loadingVoteResult = true;
		vote_result_by_id(speech.legis_init_id.toString()).then((res) => {
			voteResult = errorToNull(res);
			loadingVoteResult = false;
		});
	}

	function onShowDetails(voteResult: VoteResult | null) {
		if (!voteResult) return;
		currentVoteResultStore.value = voteResult;
		// modalStore.close();
		gotoHistory(createVoteResultPath(voteResult), true);
	}

	$: opinion =
		speech.infavor != null
			? speech.infavor
				? 'Pro'
				: 'Contra'
			: speech.opinion;
	$: arrowBackground =
		voteResult != null && voteResult.votes.length > 0
			? 'bg-secondary-400'
			: 'dark:bg-primary-300 bg-primary-400';
	$: barColor =
		speech.infavor === true
			? 'bg-green-600'
			: speech.infavor === false
				? 'bg-red-500'
				: 'bg-gray-400';
	$: hasVotes = (voteResult?.votes ?? []).length > 0;

	let speechDuration: { mins: number; seconds: number } | null = null;
	$: if (speech.duration_in_seconds !== null) {
		const mins = Math.floor(speech.duration_in_seconds / 60);
		speechDuration = { mins, seconds: speech.duration_in_seconds - mins * 60 };
	}
</script>

<div class="mt-5">
	<div class="entry flex bg-primary-100 dark:bg-primary-300 items-stretch text-black overflow-hidden">
		<div class="w-1.5 shrink-0 {barColor}"></div>
		{#if voteResult}
			<div
				class="flex-1 flex justify-between items-start gap-3 p-3 lg:p-5 min-w-0 cursor-pointer"
				role="button"
				tabindex="0"
				on:click={() => onShowDetails(voteResult)}
				on:keypress={(e) => (e.key === 'Enter' || e.key === ' ') && onShowDetails(voteResult)}
			>
				<div class="flex flex-col w-full min-w-0">
					<div class="flex items-start justify-between gap-2">
						<div class="flex flex-row flex-wrap gap-3 items-center min-w-0">
							<div class="text-sm lg:text-lg font-semibold leading-snug">
								{voteResult.legislative_initiative.title}
							</div>
							<div class="hidden lg:flex gap-2 items-center text-gray-700 dark:text-gray-300">
								{#if speech.document_url}
									<a href={speech.document_url} target="_blank" aria-label="Dokument" on:click|stopPropagation>
										<svg class="h-5 w-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M2 4h7a3 3 0 0 1 3 3v13a2 2 0 0 0-2-2H2zM22 4h-7a3 3 0 0 0-3 3v13a2 2 0 0 1 2-2h8z"/></svg>
									</a>
								{/if}
								<button on:click={() => onShowDetails(voteResult)} aria-label="Abspielen">
									<svg class="h-4 w-4" viewBox="0 0 24 24" fill="currentColor"><path d="M5 3l14 9-14 9z"/></svg>
								</button>
							</div>
						</div>
						<svg class="lg:hidden h-4 w-4 text-green-600 shrink-0 mt-1" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"><polyline points="20 6 9 17 4 12"></polyline></svg>
					</div>
					<div class="mt-1 text-[10px] lg:text-xs text-gray-600 dark:text-gray-300">
						{voteResult.legislative_initiative.vote_date
							? new Date(voteResult.legislative_initiative.vote_date).toLocaleDateString('de-AT')
							: ''}
					</div>
					<div class="mt-2 text-[10px] lg:text-base font-normal line-clamp-3 lg:line-clamp-none">
						Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.
					</div>
					<div class="mt-3 flex items-center justify-between lg:hidden">
						<button
							class="flex items-center gap-1 text-xs text-gray-600 dark:text-gray-300"
							on:click={() => onShowDetails(voteResult)}
						>
							Mehr lesen ↓
						</button>
						<div class="flex items-center gap-3 text-gray-700 dark:text-gray-300">
							{#if speech.document_url}
								<a href={speech.document_url} target="_blank" aria-label="Dokument" on:click|stopPropagation>
									<svg class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M2 4h7a3 3 0 0 1 3 3v13a2 2 0 0 0-2-2H2zM22 4h-7a3 3 0 0 0-3 3v13a2 2 0 0 1 2-2h8z"/></svg>
								</a>
							{/if}
							<button on:click={() => onShowDetails(voteResult)} aria-label="Abspielen">
								<svg class="h-3 w-3" viewBox="0 0 24 24" fill="currentColor"><path d="M5 3l14 9-14 9z"/></svg>
							</button>
						</div>
					</div>
				</div>
				<svg class="hidden lg:block self-start h-7 w-7 text-green-600 shrink-0" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"><polyline points="20 6 9 17 4 12"></polyline></svg>
				{#if hasVotes}
					<button class="hidden" on:click={() => onShowDetails(voteResult)}>
						<VoteParliament2 {voteResult} preview={true} />
					</button>
					<button class="hidden" on:click={() => onShowDetails(voteResult)}>
						{@html rightArrowIcon}
					</button>
				{/if}
				{#if speechDuration}
					<div class="hidden">
						{@html clockIcon}
						{speechDuration.mins}min {speechDuration.seconds}s
					</div>
				{/if}
			</div>
		{:else if loadingVoteResult}
			<ExpandablePlaceholder class="flex-1" />
		{:else if speech.about}
			<div class="flex-1 p-5 flex flex-col gap-1">
				<div class="text-sm lg:text-lg font-semibold">{opinion}</div>
				<div class="text-[10px] lg:text-base">{speech.about}</div>
			</div>
		{/if}
		<!--
	<div use:collapse={{ open, duration }}>
		<GovProposalExpanded {govProposal} bind:open />
	</div> -->
	</div>
</div>

<!--
<div class="gap-3 rounded-sm variant-filled my-1">
    {#if voteResult}
        {voteResult.legislative_initiative.description}
        {speech.legislative_initiatives_id} {speech.opinion}
        {#if voteResult.votes.length > 0}
            <div>
                <VoteParliament2 {voteResult}></VoteParliament2>
            </div>
        {/if}
    {/if}
</div> -->

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
