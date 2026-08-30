<script lang="ts">
	import { dashDateToDotDate } from '$lib/date';
	import { type VoteResult } from '$lib/types';
	import VoteTypeBadge from '../VoteTypeBadge.svelte';
	import { t } from '$lib/i18n/i18n.svelte';
	import { getParliament } from '$lib/api/parliament';
	export let voteResult: VoteResult;
	export let showRequiredMajority: boolean = true;
	export let showGp: boolean = true;
	export let showDate: boolean = true;
	export let showVoteType: boolean = true;
</script>

{#if showRequiredMajority && getParliament() !== 'eu'}
	{#if voteResult.legislative_initiative.requires_simple_majority}
		<span class="badge bg-tertiary-400 text-black">{t('filterOption.simpleMajority')}</span>
	{:else}
		<span class="badge bg-tertiary-400 text-black">{t('filterOption.twoThirdsMajority')}</span>
	{/if}
{/if}
{#if showGp}
	<span class="badge bg-tertiary-400 text-black">{voteResult.legislative_initiative.gp}</span>
{/if}
{#if showDate}
	<span class="badge bg-tertiary-400 text-black"
		>{dashDateToDotDate(
			voteResult.legislative_initiative.vote_date !== null
				? voteResult.legislative_initiative.vote_date
				: voteResult.legislative_initiative.nr_plenary_activity_date.toString()
		)}</span
	>
{/if}
{#if showVoteType}
	<VoteTypeBadge {voteResult} />
{/if}

<style>
</style>
