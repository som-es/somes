<script lang="ts">
	import type { NamedVote } from '$lib/types';
	import VoteResultBar from '$lib/components/Bars/VoteResultIdBar.svelte';

	export let namedVote: NamedVote;

	$: opinion =
		namedVote.infavor != null
			? namedVote.infavor
				? 'Ja'
				: 'Nein'
			: 'Abwesend/keine Stimme abgegeben';
	$: opinionColor =
		namedVote.infavor != null
			? namedVote.infavor
				? 'bg-success-600'
				: 'bg-red-600'
			: 'bg-primary-500';
</script>

<VoteResultBar legis_init_id={namedVote.legis_init_id}> 
	<div class="text-lg font-bold badge {opinionColor} text-white max-w-fit">{opinion}</div>
</VoteResultBar>