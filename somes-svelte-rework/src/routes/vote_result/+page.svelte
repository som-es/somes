<script lang="ts">
	import { pushState } from '$app/navigation';
	import { vote_result_by_id } from '$lib/api';
	import { currentVoteResultStore } from '$lib/stores/stores';
	import { onMount } from 'svelte';
	import { get } from 'svelte/store';
	import CenterPrograssRadial from '$lib/components/ProgressInfos/CenterPrograssRadial.svelte';
	import SButton from '$lib/components/UI/SButton.svelte';

	let voteResult = get(currentVoteResultStore);
	let voteResultId: string | null = null;
	let oldVoteResultId: string | null = voteResultId;

	onMount(async () => {
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
</script>

{#if voteResult}
	{#if currentlyUpdating}
		<CenterPrograssRadial />
	{:else}
		<SButton class="bg-primary-500" on:click={goBack}>Zurück</SButton>
		{voteResult.legislative_initiative.description}
	{/if}
{:else}
	<CenterPrograssRadial />
{/if}
