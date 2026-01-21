<script lang="ts">
	import PaginationDecrees from '$lib/components/Decrees/PaginationDecrees.svelte';
	import Container from '$lib/components/Layout/Container.svelte';
</script>

<svelte:head>
    <title>Verordnungen</title>
    <meta name="description" content="Filterbare Liste an Verordnungen" />
</svelte:head>

<!-- <div class="mx-auto px-5"> -->
<Container>
	<h1 class="text-2xl sm:text-4xl font-bold">Verordnungen</h1>
	<PaginationDecrees />
</Container>
<!-- </div> -->
