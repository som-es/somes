<script lang="ts">
	import { dashDateToDotDate } from '$lib/date';
	import { type VoteResult } from '$lib/types';
	import VoteTypeBadge from '../VoteTypeBadge.svelte';
	export let voteResult: VoteResult;
</script>

<div class="flex max-sm:flex-wrap gap-1 mt-1">
	{#if voteResult.legislative_initiative.requires_simple_majority}
		<span class="badge bg-tertiary-400 text-black">einfache Mehrheit</span>
	{:else}
		<span class="badge bg-tertiary-400 text-black">2/3 Mehrheit</span>
	{/if}
	<span class="badge bg-tertiary-400 text-black">{voteResult.legislative_initiative.gp}</span>
	<span class="badge bg-tertiary-400 text-black"
		>{dashDateToDotDate(voteResult.legislative_initiative.created_at.toString())}</span
	>
	<VoteTypeBadge {voteResult} />
</div>

<style>
</style>
