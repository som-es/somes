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
	import { currentVoteResultStore } from '$lib/stores/stores';

	interface Props {
		voteResult: VoteResult;
		class: any;
	}

	let { voteResult, class: clazz }: Props = $props();

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

<div class="gap-3 mt-5 {clazz}">
	<a
		href="{createVoteResultPath(voteResult)}"
		onclick={toggleOpen}
		onkeypress={toggleOpen}
		role="button"
		tabindex="0"
		class="entry block bg-primary-300 dark:bg-primary-500"
	>
		<div class="flex">
			<div class="flex w-full flex-wrap items-start justify-between gap-2 lg:flex-nowrap">
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

				<div class="shrink-0 max-lg:order-2">
					{#if voteResult.legislative_initiative.accepted !== null}
					{#if voteResult.legislative_initiative.accepted == 'a'}
						<span
							class="inline-block shrink-0 align-middle stroke-green-600 dark:stroke-green-500"
							style="width:25px; height:25px">{@html checkmarkIcon}</span
						>
					{:else}
						<span class="inline-block shrink-0 align-middle" style="width:22px; height:22px"
							>{@html crossmarkIcon}</span
						>
					{/if}
				{:else}
					<div></div>
					<InfoBadges {voteResult} />
					{/if}
				</div>
			</div>
		</div>

		<!-- REWORK - checks if vote was cast and checks for normal or roll call vote-->
		<div>
			<div class="block sm:flex justify-between mt-4">
				{#if voteResult.legislative_initiative.accepted}
					{#if voteResult.named_votes == null}
						<!-- Normal votes -->
						<div class="flex justify-between md:items-center mx-1 mb-3 sm:mb-0">
							{#each voteResult.votes.slice().sort((a, b) => b.fraction - a.fraction) as vote}
								<div class="flex items-center">
									<h4 class="text-sm">{vote.party}</h4>
									{#if vote.infavor}
										<span
											class="mr-1 md:mr-2 stroke-green-600 dark:stroke-green-500 inline-block align-middle"
											style="width:18px; height:18px;">{@html checkmarkIcon}</span
										>
									{:else}
										<span
											class="mr-1 md:mr-2 inline-block align-middle"
											style="width:18px; height:18px;">{@html crossmarkIcon}</span
										>
									{/if}
								</div>
							{/each}
						</div>

						<div>
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
						</div>
					{:else}
						<!-- Roll call votes -->

						<div class="block sm:flex w-full mb-3">
							<div class="flex items-center mb-1 sm:mb-0">
								<span
									class="mr-1 stroke-green-600 dark:stroke-green-500 inline-block align-middle"
									style="width:20px; height:20px;">{@html checkmarkIcon}</span
								>

								{#if voteResult.votes.length > 0}
									{#each voteResult.votes.slice().sort((a, b) => b.fraction - a.fraction) as vote}
										{#if vote.infavor}
											<div class="flex items-center">
												<h4 class="text-sm mr-1">{vote.party}</h4>
												<h4 class="text-sm mr-2 text-gray-800">{vote.fraction}</h4>
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
									class="mr-1 ml-0 sm:ml-3 inline-block align-middle"
									style="width:20px; height:20px;">{@html crossmarkIcon}</span
								>
								{#if voteResult.votes.length > 0}
									{#each voteResult.votes.slice().sort((a, b) => b.fraction - a.fraction) as vote}
										{#if !vote.infavor}
											<div class="flex items-center">
												<h4 class="text-sm mr-1">{vote.party}</h4>
												<h4 class="text-sm mr-2 text-gray-800">{vote.fraction}</h4>
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
						<div class="max-lg:hidden flex max-h-6 gap-1">
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
						</div>
					{/if}
				{:else}{/if}
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
	{#if open}
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
