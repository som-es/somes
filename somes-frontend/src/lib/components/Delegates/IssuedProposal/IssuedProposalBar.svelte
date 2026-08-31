<script lang="ts">
	import { t } from '$lib/i18n/i18n.svelte';
	import { type VoteResult } from '$lib/types';
	import { errorToNull, vote_result_by_id } from '$lib/api/api';
	import type { IssuedProposal } from '$lib/types';
	import VoteResultCard from '../VoteResultCard.svelte';
	import checkmarkIcon from '$lib/assets/misc_icons/checkmark_small.svg?raw';
	import crossmarkIcon from '$lib/assets/misc_icons/crossmark_small.svg?raw';
	import VoteResultExpandableBar from '$lib/components/VoteResults/Expandable/VoteResultExpandableBar.svelte';

	interface Props {
		issuedProposal: IssuedProposal;
	}

	let { issuedProposal }: Props = $props();

	let voteResult = $state<VoteResult | null>(null);
	let loading = $state(true);

	$effect(() => {
		voteResult = null;
		loading = true;
		vote_result_by_id(issuedProposal.legis_init_id.toString()).then((res) => {
			voteResult = errorToNull(res);
			loading = false;
		});
	});
</script>

{#if !loading && voteResult}
	<VoteResultExpandableBar
		{voteResult}
		coloring="bg-primary-200 hover:bg-primary-400 dark:bg-primary-300 dark:hover:bg-primary-400 text-black! "
	/>
{/if}
<!--
<VoteResultCard {voteResult} {loading}>
	{#if !loading && voteResult}
		{#if voteResult.legislative_initiative.accepted === null}
			<div class="badge bg-primary-500 mt-3 max-w-fit text-sm font-bold text-white lg:ml-5 lg:mt-0">
				{t('proposals.pendingVote')}
			</div>
		{:else if voteResult.legislative_initiative.accepted === 'a'}
			<span
				class="mt-3 inline-block shrink-0 stroke-green-600 align-middle dark:stroke-green-500 lg:ml-5 lg:mt-0"
				style="width:25px; height:25px"
			>{@html checkmarkIcon}</span>
		{:else}
			<span
				class="mt-3 inline-block shrink-0 align-middle lg:ml-5 lg:mt-0"
				style="width:22px; height:22px"
			>{@html crossmarkIcon}</span>
		{/if}
	{/if}
</VoteResultCard> -->
