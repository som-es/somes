<script lang="ts">
	import type { NamedVote } from '$lib/types';
	import VoteResultBar from '$lib/components/Bars/VoteResultIdBar.svelte';

	interface Props {
		namedVote: NamedVote;
	}

	let { namedVote }: Props = $props();

	let opinion =
		$derived(namedVote.infavor != null
			? namedVote.infavor
				? 'Ja'
				: 'Nein'
			: 'Abwesend/keine Stimme abgegeben');
	let opinionColor =
		$derived(namedVote.infavor != null
			? namedVote.infavor
				? 'bg-success-600'
				: 'bg-red-600'
			: 'bg-primary-500');
</script>

<VoteResultBar legis_init_id={namedVote.legis_init_id}> 
	<div class=" text-sm sm:text-lg font-bold badge {opinionColor} text-white max-w-fit">{opinion}</div>
</VoteResultBar>