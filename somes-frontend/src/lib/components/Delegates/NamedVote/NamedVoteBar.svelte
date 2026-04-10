<script lang="ts">
	import { type NamedVote, type VoteResult } from '$lib/types';
	import { errorToNull, vote_result_by_id } from '$lib/api/api';
	import VoteResultCard from '../VoteResultCard.svelte';

	interface Props {
		namedVote: NamedVote;
	}

	let { namedVote }: Props = $props();

	let voteResult = $state<VoteResult | null>(null);
	let loading = $state(true);

	$effect(() => {
		voteResult = null;
		loading = true;
		vote_result_by_id(namedVote.legis_init_id.toString()).then((res) => {
			voteResult = errorToNull(res);
			loading = false;
		});
	});

	let opinion = $derived(
		namedVote.infavor != null
			? namedVote.infavor
				? 'Pro'
				: 'Contra'
			: 'Abwesend/keine Stimme abgegeben'
	);
	let opinionColor = $derived(
		namedVote.infavor != null
			? namedVote.infavor
				? 'bg-success-600'
				: 'bg-red-600'
			: 'bg-primary-500'
	);
</script>

<VoteResultCard {voteResult} {loading}>
	<div class="badge text-sm font-bold {opinionColor} mt-3 max-w-fit text-white lg:ml-5 lg:mt-0">
		{opinion}
	</div>
</VoteResultCard>
