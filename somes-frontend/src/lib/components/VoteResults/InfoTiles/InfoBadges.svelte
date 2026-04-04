<script lang="ts">
	import { dashDateToDotDate } from '$lib/date';
	import { type VoteResult } from '$lib/types';
	import VoteTypeBadge from '../VoteTypeBadge.svelte';
	export let voteResult: VoteResult;
	export let showRequiredMajority: boolean = true;
	export let showGp: boolean = true;
	export let showDate: boolean = true;
	export let showVoteType: boolean = true;
</script>

<div class="mt-1 flex gap-1 max-sm:flex-wrap">
	{#if showRequiredMajority}
		{#if voteResult.legislative_initiative.requires_simple_majority}
			<span class="badge bg-tertiary-400 text-black">einfache Mehrheit</span>
		{:else}
			<span class="badge bg-tertiary-400 text-black">2/3 Mehrheit</span>
		{/if}
	{/if}
	{#if showGp}
		<span class="badge bg-tertiary-400 text-black">{voteResult.legislative_initiative.gp}</span>
	{/if}
	{#if showDate}
		<span class="badge bg-tertiary-400 text-black"
			>{dashDateToDotDate(
				voteResult.legislative_initiative.nr_plenary_activity_date.toString()
			)}</span
		>
	{/if}
	{#if showVoteType}
		<VoteTypeBadge {voteResult} />
	{/if}
</div>

<style>
</style>
