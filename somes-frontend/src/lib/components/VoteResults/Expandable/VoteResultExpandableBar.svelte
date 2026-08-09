<script lang="ts">
	import { createVoteResultPath, type Delegate, type VoteResult } from '$lib/types';
	import VoteResultExpanded from './VoteResultExpanded.svelte';
	import { slide } from 'svelte/transition';

	import crossmarkIcon from '$lib/assets/misc_icons/crossmark_small.svg?raw';
	import checkmarkIcon from '$lib/assets/misc_icons/checkmark_small.svg?raw';
	import VoteTypeBadge from '../VoteTypeBadge.svelte';
	import { dashDateToDotDate } from '$lib/date';
	import InfoBadges from '../InfoTiles/InfoBadges.svelte';
	import { gotoHistory } from '$lib/goto';
	import { currentVoteResultStore, aiViewEnabledStore } from '$lib/stores/stores';
	import InfoBadgesCore from '../InfoTiles/InfoBadgesCore.svelte';

	interface Props {
		voteResult: VoteResult;
		class?: any;
		coloring?: string;
		unexpandable?: boolean;
	}

	let {
		voteResult,
		class: clazz,
		coloring = 'bg-primary-300 dark:bg-primary-500',
		unexpandable = false
	}: Props = $props();

	function onShowDetails() {
		currentVoteResultStore.value = voteResult;
		gotoHistory(createVoteResultPath(voteResult), true);
	}

	function toggleOpen(e: Event) {
		e.preventDefault();
		if (typeof window !== 'undefined' && window.innerWidth < 1024) {
			onShowDetails();
		} else {
			open = !open;
		}
	}

	let open = $state(false);
</script>

<div class="gap-3 {clazz}">
	<a
		href={createVoteResultPath(voteResult)}
		onclick={toggleOpen}
		onkeypress={toggleOpen}
		role="button"
		tabindex="0"
		class="entry block {coloring}"
	>
		<div class="flex">
			<div class="flex w-full flex-wrap items-start justify-between gap-2 lg:flex-nowrap">
				{#if aiViewEnabledStore.value && voteResult.ai_summary}
					<div class="flex min-w-0 flex-1 flex-col flex-wrap max-lg:contents">
						<span
							class="text-xl font-semibold max-lg:order-1 max-lg:min-w-0 max-lg:flex-1"
							style="hyphens: auto; word-break: normal; overflow-wrap: break-word;"
						>
							{voteResult.ai_summary.short_title}
						</span>
						<span class="sm:text-md text-sm max-lg:order-3 max-lg:w-full">
							{voteResult.ai_summary.short_summary}
						</span>
					</div>
				{:else}
					<span class="text-md min-w-0 flex-1 font-semibold">
						{voteResult.legislative_initiative.description}
					</span>
				{/if}

				<div class="shrink-0 max-lg:order-2">
					{#if voteResult.legislative_initiative.accepted !== null && voteResult.legislative_initiative.accepted !== 'u'}
						{#if voteResult.legislative_initiative.accepted == 'a'}
							<span
								class="inline-block shrink-0 stroke-green-600 align-middle dark:stroke-green-500"
								style="width:25px; height:25px">{@html checkmarkIcon}</span
							>
						{:else}
							<span class="inline-block shrink-0 align-middle" style="width:22px; height:22px"
								>{@html crossmarkIcon}</span
							>
						{/if}
					{/if}
				</div>
			</div>
		</div>

		<!-- REWORK - checks if vote was cast and checks for normal or roll call vote-->
		<div>
			<div class="mt-4 block justify-between sm:flex">
				{#if voteResult.legislative_initiative.accepted !== null}
					{#if voteResult.named_votes == null}
						<!-- Normal votes -->
						<div class="mx-1 mb-3 flex justify-between sm:mb-0 md:items-center">
							{#each voteResult.votes.slice().sort((a, b) => b.fraction - a.fraction) as vote}
								<div class="flex items-center">
									<h4 class="text-sm">{vote.party}</h4>
									{#if vote.infavor}
										<span
											class="mr-1 inline-block stroke-green-600 align-middle md:mr-2 dark:stroke-green-500"
											style="width:18px; height:18px;">{@html checkmarkIcon}</span
										>
									{:else}
										<span
											class="mr-1 inline-block align-middle md:mr-2"
											style="width:18px; height:18px;">{@html crossmarkIcon}</span
										>
									{/if}
								</div>
							{/each}
						</div>

						<div>
							<InfoBadgesCore {voteResult} />
						</div>
					{:else}
						<!-- Roll call votes -->

						<div class="mb-3 block w-full sm:flex">
							<div class="mb-1 flex items-center sm:mb-0">
								<span
									class="mr-1 inline-block stroke-green-600 align-middle dark:stroke-green-500"
									style="width:20px; height:20px;">{@html checkmarkIcon}</span
								>

								{#if voteResult.votes.length > 0}
									{#each voteResult.votes.slice().sort((a, b) => b.fraction - a.fraction) as vote}
										{#if vote.infavor}
											<div class="flex items-center">
												<h4 class="mr-1 text-sm">{vote.party}</h4>
												<h4 class="mr-2 text-sm text-gray-800">{vote.fraction}</h4>
											</div>
										{/if}
									{/each}
								{:else}
									<h4 class="text-sm text-gray-800">
										{voteResult.named_votes.named_vote_info.pro_count}
									</h4>
								{/if}
							</div>
							<div class="flex flex-wrap items-center">
								<span
									class="mr-1 ml-0 inline-block align-middle sm:ml-3"
									style="width:20px; height:20px;">{@html crossmarkIcon}</span
								>
								{#if voteResult.votes.length > 0}
									{#each voteResult.votes.slice().sort((a, b) => b.fraction - a.fraction) as vote}
										{#if !vote.infavor}
											<div class="flex items-center">
												<h4 class="mr-1 text-sm">{vote.party}</h4>
												<h4 class="mr-2 text-sm text-gray-800">{vote.fraction}</h4>
											</div>
										{/if}
									{/each}
								{:else}
									<h4 class="text-sm text-gray-800">
										{voteResult.named_votes.named_vote_info.contra_count}
									</h4>
								{/if}
							</div>
						</div>
						<div class="flex max-h-6 gap-1 max-lg:hidden">
							<InfoBadgesCore {voteResult} />
						</div>
					{/if}
				{:else}
					<InfoBadges {voteResult} />
				{/if}
			</div>
			<span class="lg:hidden">
				{#if voteResult.named_votes != null}
					{#if voteResult.legislative_initiative.requires_simple_majority}
						<span class="badge bg-tertiary-400 text-black">einfache Mehrheit</span>
					{:else}
						<span class="badge bg-tertiary-400 text-black">2/3 Mehrheit</span>
					{/if}
					<span class="badge bg-tertiary-400 text-black"
						>{voteResult.legislative_initiative.gp}</span
					>
					<span class="badge bg-tertiary-400 text-black"
						>{dashDateToDotDate(
							voteResult.legislative_initiative.nr_plenary_activity_date.toString()
						)}</span
					>
					<VoteTypeBadge {voteResult} />
				{/if}
			</span>
		</div>
	</a>
	{#if open && !unexpandable}
		<div transition:slide={{ duration: 240 }}>
			<VoteResultExpanded {voteResult} bind:open />
		</div>
	{/if}
</div>

<style>
	.entry {
		border-radius: 0.9rem;
		box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
		padding: 20px;
		gap: 10px;
	}
	/* .grid-container {
		display: grid;
		grid-template-columns: 2fr 1fr;
	} */

	#open :global(.right-arrow) {
		transform: rotate(90deg);
		transition: transform 0.35s;
	}

	#closed :global(.right-arrow) {
		transform: rotate(0deg);
		transition: transform 0.35s;
	}
</style>
