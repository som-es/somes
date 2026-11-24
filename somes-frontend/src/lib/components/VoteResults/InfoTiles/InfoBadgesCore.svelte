<script lang="ts">
	import { majorityLabel, voteDateLabel } from '$lib/voteBadges';
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
	<span class="badge bg-tertiary-400 text-black">{majorityLabel(voteResult, t)}</span>
{/if}
{#if showGp}
	<span class="badge bg-tertiary-400 text-black">{voteResult.legislative_initiative.gp}</span>
{/if}
{#if showDate}
	<span class="badge bg-tertiary-400 text-black">{voteDateLabel(voteResult)}</span>
{/if}
{#if showVoteType}
	<VoteTypeBadge {voteResult} />
{/if}

<style>
</style>
