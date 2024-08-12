<script lang="ts">
	import { pushState } from '$app/navigation';
	import { vote_result_by_id } from '$lib/api';
	import { currentVoteResultStore } from '$lib/stores/stores';
	import { onMount } from 'svelte';
	import { get } from 'svelte/store';
	import CenterPrograssRadial from '$lib/components/ProgressInfos/CenterPrograssRadial.svelte';
	import SButton from '$lib/components/UI/SButton.svelte';
	import Container from '$lib/components/Layout/Container.svelte';
	import Topics from '$lib/components/Topics/Topics.svelte';
	import type { Delegate, VoteResult } from '$lib/types';
	import Emphasis from '$lib/components/VoteResults/Emphasis/Emphasis.svelte';
	import VoteParliament from '$lib/components/Parliaments/VoteParliament.svelte';
	import InfoTiles from '$lib/components/VoteResults/InfoTiles/InfoTiles.svelte';
	import { filteredDelegates } from '$lib/caching/delegates';
	import DelegateCard from '$lib/components/Delegates/DelegateCard.svelte';
	import VoteDelegateCard from '$lib/components/Delegates/VoteDelegateCard.svelte';

	let dels: Delegate[] | null = null;

	let voteResult = get(currentVoteResultStore);
	let voteResultId: string | null = null;
	let oldVoteResultId: string | null = voteResultId;

	let delegate: Delegate | null = null;

	onMount(async () => {
		dels = await filteredDelegates();
		if (dels !== null) {
			delegate = dels[Math.floor(Math.random() * dels.length)];
		}
		const url = new URL(window.location.href);

		voteResultId = url.searchParams.get('id');
		if (voteResultId == null && voteResult !== null) {
			voteResultId = voteResult.legislative_initiative.id;
			oldVoteResultId = voteResultId;
		}
		// if (voteResultId !== null && voteResult?.legislative_initiative.id != voteResultId) {
		//     voteResult = await vote_result_by_id(voteResultId);
		//     if (voteResult !== null) voteResultId = voteResult?.legislative_initiative.id;
		//     currentVoteResultStore.set(voteResult);
		// }
	});

	let currentlyUpdating = false;

	const loadVoteResult = async (voteResultId: string) => {
		if (voteResultId == voteResult?.legislative_initiative.id) {
			return;
		}
		currentlyUpdating = true;
		voteResult = await vote_result_by_id(voteResultId);
		currentVoteResultStore.set(voteResult);
		currentlyUpdating = false;
	};

	let updatedQueryParam = false;

	const update = () => {
		if (voteResultId == null) {
			return;
		}

		loadVoteResult(voteResultId);

		// update query params
		const url = new URL(window.location.href);
		url.searchParams.set('id', voteResultId.toString());
		try {
			updatedQueryParam = true;
			pushState(url.toString(), { replaceState: true });
		} catch (e) {
			voteResultId = oldVoteResultId;
		}

		oldVoteResultId = voteResultId;
	};

	const goBack = () => {
		if (updatedQueryParam) {
			history.go(-2);
		} else {
			history.back();
		}
	};

	$: if (voteResultId) {
		update();
	}

	const emphasis = voteResult?.legislative_initiative.emphasis
		?.split('\n\t')
		.filter((x) => x.length > 0);

	const whichGridContainer =
		emphasis == null ? 'grid-container-without-emphasis' : 'grid-container-with-emphasis';
</script>

{#if voteResult && dels && delegate}
	{#if currentlyUpdating}
		<CenterPrograssRadial />
	{:else}
		<Container>
			<SButton class="bg-primary-500" on:click={goBack}>Zurück</SButton>
			<br />
			<div
				class="max-lg:!hidden entry bg-primary-200 dark:bg-primary-400 mt-3 {whichGridContainer}"
			>
				<div class="title-item rounded-xl bg-primary-300 px-3 py-3">
					<h1 class="font-bold text-3xl">
						{voteResult.legislative_initiative.voted_by_name ? 'namentliche ' : ''}Abstimmung über
					</h1>
					<span class="text-xl">{voteResult.legislative_initiative.description}</span>
				</div>
				{#if emphasis}
					<div class="emphasis-item">
						<Emphasis {emphasis}></Emphasis>
					</div>
				{/if}

				<div class="rounded-xl parliament-item bg-primary-300">
					<VoteParliament {dels} {voteResult} bind:delegate />
				</div>
				<div class="delegate-item">
					<VoteDelegateCard {delegate} />
				</div>
				<div class="info-item">
					<InfoTiles {voteResult} {dels} />
				</div>

				<div
					class="topics-item flex rounded-xl justify-center items-center bg-primary-300 pt-3 pb-3 px-3"
				>
					<Topics
						topics={voteResult.topics.sort((a, b) => {
							return a.topic.length - b.topic.length;
						})}
					/>
				</div>
			</div>
		</Container>
	{/if}
{:else}
	<CenterPrograssRadial />
{/if}

<style>
	.entry {
		border-radius: 0.9rem;
		box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
		padding: 20px;
		gap: 10px;
	}
	/* .grid-container-with-emphasis {
		box-sizing: border-box;
		display: grid;
		min-width: 0;
		min-height: 0;
		grid-template-columns: 3fr 2fr;
		grid-template-rows: auto auto 2fr auto auto;
		grid-template-areas:
            'ti ti'
            'e e'
			'p d'
            'r r'
			'i t';
		padding: 10px;
	} */

	.grid-container-with-emphasis {
		display: flex;
		flex-wrap: wrap;
	}

	.title-item {
		grid-area: ti;
		flex-basis: 100%;
	}

	.parliament-item {
		grid-area: p;
		flex-basis: 66.6%;
	}
	.delegate-item {
		grid-area: d;
		flex-basis: 31%;
	}
	.topics-item {
		grid-area: t;
		flex-basis: 38%;
	}

	.emphasis-item {
		grid-area: e;
		flex-basis: 100%;
	}

	.info-item {
		grid-area: i;
		flex-basis: 60%;
	}

	.grid-container-without-emphasis {
		/* box-sizing: border-box; */
		display: grid;
		min-width: 0;
		min-height: 0;
		grid-template-columns: 3fr 1fr;
		grid-template-rows: auto 2fr auto auto;
		grid-template-areas:
			'ti ti'
			'p d'
			'r r'
			'i t';
		padding: 10px;
	}
</style>
