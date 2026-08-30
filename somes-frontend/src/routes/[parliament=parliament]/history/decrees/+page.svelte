<script lang="ts">
	import { errorToNull } from '$lib/api/api';
	import PaginationDecrees from '$lib/components/Decrees/PaginationDecrees.svelte';
	import { t } from '$lib/i18n/i18n.svelte';
	import type { DecreesWithMaxPage } from '$lib/components/Delegates/Decrees/types';
	import Container from '$lib/components/Layout/Container.svelte';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();

	let decrees: DecreesWithMaxPage | null = $derived(errorToNull(data.decrees));
	let departmentsPerGp: Record<string, string[]> | null = $derived(
		errorToNull(data.departmentsPerGp)
	);
</script>

<svelte:head>
	<title>{t('history.decrees.title')}</title>
	<meta name="description" content={t('history.decrees.meta')} />
</svelte:head>

<!-- <div class="mx-auto px-5"> -->
<Container>
	<h1 class="mt-2 px-1 pt-2 text-3xl font-bold sm:mt-0 sm:p-0 sm:text-4xl">
		{t('history.decrees.title')}
	</h1>
	{#if decrees && departmentsPerGp}
		<PaginationDecrees {decrees} selectedGp={data.selectedGp} {departmentsPerGp} />
	{/if}
</Container>
<!-- </div> -->
