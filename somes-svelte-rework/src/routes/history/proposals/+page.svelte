<script lang="ts">
	import Container from '$lib/components/Layout/Container.svelte';
	import PaginationMinistrialProposals from '$lib/components/Proposals/PaginationMinistrialProposals.svelte';
</script>

<!-- <div class="mx-auto px-5"> -->
<Container>
	<!-- alles, was noch nicht abgestimmt wurde / was schon wurde, aber halt wovon kam ok?? -> Ministerialentwürfe, Anträge -->
	<h1 class="text-2xl sm:text-4xl font-bold">Ministerialentwürfe</h1>
	<PaginationMinistrialProposals />
</Container>
<!-- </div> -->
