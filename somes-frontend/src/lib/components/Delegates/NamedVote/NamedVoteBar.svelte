<script lang="ts">
	import { type NamedVote, type VoteResult, createVoteResultPath } from '$lib/types';
	import { aiViewEnabledStore, currentVoteResultStore } from '$lib/stores/stores';
	import { errorToNull, vote_result_by_id } from '$lib/api/api';
	import ExpandablePlaceholder from '$lib/components/VoteResults/Expandable/Placeholders/ExpandablePlaceholder.svelte';

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

<a
	class="mt-2 flex w-full flex-col lg:flex-row lg:items-center lg:justify-between rounded-xl bg-primary-400 p-3 dark:bg-primary-300 transition-colors hover:bg-primary-500 dark:hover:bg-primary-400"
	href={voteResult ? createVoteResultPath(voteResult) : undefined}
	onclick={() => { if (voteResult) currentVoteResultStore.value = voteResult; }}
>
	<div>
		{#if loading}
			<ExpandablePlaceholder />
		{:else if voteResult}
			{#if aiViewEnabledStore.value && voteResult.ai_summary}
				<div class="flex min-w-0 flex-1 flex-col">
					<span
						class="text-xl font-semibold"
						style="hyphens: auto; word-break: normal; overflow-wrap: break-word;"
					>
						{voteResult.ai_summary.short_title}
					</span>
					<span class="text-sm sm:text-base">
						{voteResult.ai_summary.short_summary}
					</span>
				</div>
			{:else}
				<span class="text-md min-w-0 flex-1 font-semibold">
					{voteResult.legislative_initiative.description}
				</span>
			{/if}
		{/if}
	</div>
	<div class="badge text-sm font-bold {opinionColor} mt-3 max-w-fit text-white lg:ml-5 lg:mt-0">
		{opinion}
	</div>
</a>
