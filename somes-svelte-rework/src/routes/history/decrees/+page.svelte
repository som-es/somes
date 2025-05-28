<script lang="ts">
	import PaginationDecrees from '$lib/components/Decrees/PaginationDecrees.svelte';
	import Container from '$lib/components/Layout/Container.svelte';
	import PaginationMinistrialProposals from '$lib/components/Proposals/PaginationMinistrialProposals.svelte';
</script>

<!-- <div class="mx-auto px-5"> -->
<Container>
	<!-- alles, was noch nicht abgestimmt wurde / was schon wurde, aber halt wovon kam ok?? -> Ministerialentwürfe, Anträge -->
	<h1 class="text-2xl sm:text-4xl font-bold">Verordnungen</h1>
	<PaginationDecrees />
</Container>
<!-- </div> -->
