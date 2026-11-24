<script lang="ts">
	import { SITE_URL } from '$lib/seo/sitemap';
	import { breadcrumb_json_ld } from '$lib/seo/breadcrumbs';
	import { json_ld_script } from '$lib/seo/delegate';
	import { ministerialInfoBadges } from '$lib/components/MinisterialView/infoBadges';
	import type { DecreeDelegate } from '$lib/components/Delegates/Decrees/types';
	import { t } from '$lib/i18n/i18n.svelte';
	import Container from '$lib/components/Layout/Container.svelte';
	import MinisterialView from '$lib/components/MinisterialView/MinisterialView.svelte';
	import type { MinisterialViewData } from '$lib/components/MinisterialView/types';
	import ExpandablePlaceholder from '$lib/components/VoteResults/Expandable/Placeholders/ExpandablePlaceholder.svelte';
	import type { PageProps } from './$types';
	import { decree_seo } from '$lib/seo/decree';

	let { data }: PageProps = $props();

	let decreeDelegate: DecreeDelegate | null = $derived(data.decreeDelegate);

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
			infoBadges: ministerialInfoBadges(
				decreeDelegate.decree.ministrial_issuer,
				decreeDelegate.decree.publication_date,
				decreeDelegate.decree.gp
			),
			gp: decreeDelegate.decree.gp!
		};
	});

	let seo = $derived(decreeDelegate ? decree_seo(decreeDelegate.decree) : null);
</script>

<svelte:head>
	{#if seo}
		<title>{seo.title}</title>
		<meta name="description" content={seo.description} />
	{/if}
	{#if decreeDelegate}
		{@html json_ld_script(
			breadcrumb_json_ld([
				{ name: t('nav.decrees'), path: `/${data.parliament}/history/decrees` },
				{
					name:
						decreeDelegate.decree.ai_summary?.short_title ||
						decreeDelegate.decree.short_title ||
						decreeDelegate.decree.title
				}
			])
		)}
	{/if}
	{#if decreeDelegate}
		<meta
			property="og:image"
			content="{SITE_URL}/og/{data.parliament}/decree/{decreeDelegate.decree.ris_id}.png"
		/>
	{/if}
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
