<script lang="ts">
	import type { DecreeDelegate } from '$lib/components/Delegates/Decrees/types';
	import { t } from '$lib/i18n/i18n.svelte';
	import Container from '$lib/components/Layout/Container.svelte';
	import MinisterialView from '$lib/components/MinisterialView/MinisterialView.svelte';
	import type { MinisterialViewData } from '$lib/components/MinisterialView/types';
	import ExpandablePlaceholder from '$lib/components/VoteResults/Expandable/Placeholders/ExpandablePlaceholder.svelte';
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
			aiSummary: decreeDelegate.decree.ai_summary,
			alternativeTitle: decreeDelegate.decree.short_title,
			date: decreeDelegate.decree.publication_date,
			originalDocumentUrl: decreeDelegate.decree.document_url,
			documents: decreeDelegate.decree.documents,
			topics: [],
			otherKeywordTopics: [],
			eurovocTopics: [],
			delegates: [decreeDelegate.delegate],
			ressort: decreeDelegate.decree.ministrial_issuer,
			ressortShortform: null,
			ministerialIssuers: [decreeDelegate.delegate.id],
			type: 'decree',
			infoBadges: [decreeDelegate.decree.ministrial_issuer, date, decreeDelegate.decree.gp].filter(
				(x) => x !== null
			) as string[],
			gp: decreeDelegate.decree.gp!
		};
	});

	const title = $derived(ministerialData?.aiSummary !== null ? ministerialData?.aiSummary?.short_title : ministerialData.alternativeTitle);
	const content = $derived(ministerialData?.aiSummary !== null ? ministerialData?.aiSummary?.very_detailed_summary : ministerialData.alternativeTitle);
</script>

<svelte:head>
	<title>{title}</title>
	<meta name="description" {content} />
</svelte:head>

<Container>
	{#if ministerialData}
		<MinisterialView {ministerialData}></MinisterialView>
	{:else}
		{#each { length: 10 } as _}
			<ExpandablePlaceholder />
		{/each}
	{/if}
</Container>
