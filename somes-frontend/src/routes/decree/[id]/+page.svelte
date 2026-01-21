<script lang="ts">
	import type { DecreeDelegate } from '$lib/components/Delegates/Decrees/types';
	import Container from '$lib/components/Layout/Container.svelte';
	import MinisterialView from '$lib/components/MinisterialView/MinisterialView.svelte';
	import type { MinisterialViewData } from '$lib/components/MinisterialView/types';
	import SButton from '$lib/components/UI/SButton.svelte';
	import ExpandablePlaceholder from '$lib/components/VoteResults/Expandable/Placeholders/ExpandablePlaceholder.svelte';
	import { hasGoBackStore } from '$lib/stores/stores';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();

	let decreeDelegate: DecreeDelegate | null = $derived(data.decreeDelegate);
	let date = $derived.by(() => {
		if (decreeDelegate == null) return null;
		return new Date(decreeDelegate.decree.publication_date).toLocaleDateString();
	});

	let ministerialData: MinisterialViewData | null = $derived.by(() => {
		if (decreeDelegate == null) return null;
		return {
			aiSummary: null,
			alternativeTitle: decreeDelegate.decree.short_title,
			date: decreeDelegate.decree.publication_date,
			originalDocumentUrl: decreeDelegate.decree.document_url,
			documents: decreeDelegate.decree.documents,
			topics: [],
			otherKeywordTopics: [],
			eurovocTopics: [],
			delegate: decreeDelegate.delegate,
			ressort: decreeDelegate.decree.ministrial_issuer,
			ressortShortform: null,
			ministerialIssuers: [decreeDelegate.delegate.id],
			type: "decree",
			infoBadges: [decreeDelegate.decree.ministrial_issuer, date, decreeDelegate.decree.gp].filter((x) => x !== null) as string[],
			
		};
	});

	const goBack = () => {
		history.back();
	};
</script>

<svelte:head>
	<title>Verordnung</title>
	<meta name="description" content="Spezifische Verordnung" />
</svelte:head>

<Container>
	{#if hasGoBackStore.value}
		<SButton class="bg-primary-500" on:click={goBack}>Zurück</SButton>
	{/if}

	{#if ministerialData}
		<MinisterialView ministerialData={ministerialData}></MinisterialView>
	{:else}
		{#each { length: 10 } as _}
			<ExpandablePlaceholder />
		{/each}
	{/if}
</Container>
