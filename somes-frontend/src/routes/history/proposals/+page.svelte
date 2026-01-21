<script lang="ts">
	import Container from '$lib/components/Layout/Container.svelte';
	import PaginationMinistrialProposals from '$lib/components/Proposals/PaginationMinistrialProposals.svelte';
</script>

<svelte:head>
    <title>Ministerialentwürfe</title>
    <meta name="description" content="Filterbare Liste an Ministerialentwürfe" />
</svelte:head>

<!-- <div class="mx-auto px-5"> -->
<Container>
	<h1 class="text-2xl sm:text-4xl font-bold">Ministerialentwürfe</h1>
	<PaginationMinistrialProposals />
</Container>
<!-- </div> -->
