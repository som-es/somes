<script lang="ts">
	import { page } from "$app/stores";
	import { delegate_by_id, errorToNull } from "$lib/api/api";
	import { decree_by_ris_id } from "$lib/components/Delegates/Decrees/api";
	import DecreeView from "$lib/components/Delegates/Decrees/DecreeView.svelte";
	import type { Decree, DecreeDelegate } from "$lib/components/Delegates/Decrees/types";
	import Container from "$lib/components/Layout/Container.svelte";
	import SButton from "$lib/components/UI/SButton.svelte";
	import ExpandablePlaceholder from "$lib/components/VoteResults/Expandable/Placeholders/ExpandablePlaceholder.svelte";
	import { currentDecreeStore, hasGoBackStore } from "$lib/stores/stores";
	import { onMount } from "svelte";
	import { get } from "svelte/store";

	let decree_delegate: DecreeDelegate = {decree: null, delegate: null};
	let currentlyUpdating = false;

	$: ris_id = $page.params.id;
	
	const loadVoteResult = async () => {
		currentlyUpdating = true;
		decree_delegate.decree = errorToNull(await decree_by_ris_id(ris_id));
		if (decree_delegate.decree) {
			decree_delegate.delegate = errorToNull(await delegate_by_id(decree_delegate.decree.gov_official_id))
		}
		currentDecreeStore.set(decree_delegate);
		currentlyUpdating = false;
	};

	const runDecreeUpdate = () => {
		decree_delegate = get(currentDecreeStore) ?? {decree: null, delegate: null};
		const stored_ris_id = decree_delegate?.decree?.ris_id;
		if (ris_id != stored_ris_id) {
			loadVoteResult();
		}
	};

	onMount(runDecreeUpdate)
	
	const goBack = () => {
		history.back();
	};
</script>
<Container>
	{#if currentlyUpdating}
				<!-- <CenterPrograssRadial /> -->
	{:else}
		{#if get(hasGoBackStore)}
			<SButton class="bg-primary-500" on:click={goBack}>Zurück</SButton>
		{/if}

		{#if decree_delegate.decree}
			<DecreeView decree={decree_delegate.decree} delegate={decree_delegate.delegate} />
		{:else}
			{#each { length: 10 } as _}
				<ExpandablePlaceholder />
			{/each}
		{/if}
	{/if}
</Container>