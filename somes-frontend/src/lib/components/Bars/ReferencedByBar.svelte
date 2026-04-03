<script lang="ts">
	import { createVoteResultPath, type VoteResult } from '$lib/types';
	import { aiViewEnabledStore } from '$lib/stores/stores';
	import { dashDateToDotDate } from '$lib/date';
	import VoteTypeBadge from '$lib/components/VoteResults/VoteTypeBadge.svelte';
	import checkmarkIcon from '$lib/assets/misc_icons/checkmark_small.svg?raw';
	import crossmarkIcon from '$lib/assets/misc_icons/crossmark_small.svg?raw';

	interface Props {
		ref: VoteResult;
	}

	let { ref }: Props = $props();
</script>

<a
	href={createVoteResultPath(ref)}
	class="flex px-3 py-2 rounded-xl mb-1 bg-primary-200/50 hover:bg-primary-200 dark:bg-primary-400/50 dark:hover:bg-primary-400 transition-colors"
>
	<div class="flex-1">
		<span class="text-base">
			{#if aiViewEnabledStore.value && ref.ai_summary}
				{ref.ai_summary.short_title}
			{:else}
				{ref.legislative_initiative.description}
			{/if}
		</span>
		<div class="flex items-center gap-1 mt-1 flex-wrap">
			<span class="badge bg-tertiary-400 text-black">{dashDateToDotDate(ref.legislative_initiative.nr_plenary_activity_date.toString())}</span>
			<VoteTypeBadge voteResult={ref} />
		</div>
	</div>
	<div class="flex items-center">
		{#if ref.legislative_initiative.accepted === 'a'}
			<span class="shrink-0 stroke-green-600 dark:stroke-green-500" style="width:20px; height:20px;">{@html checkmarkIcon}</span>
		{:else if ref.legislative_initiative.accepted}
			<span class="shrink-0" style="width:20px; height:20px;">{@html crossmarkIcon}</span>
		{/if}
	</div>
</a>
