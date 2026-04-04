<script lang="ts">
	import { createVoteResultPath, type VoteResult } from '$lib/types';
	import { aiViewEnabledStore } from '$lib/stores/stores';
	import checkmarkIcon from '$lib/assets/misc_icons/checkmark_small.svg?raw';
	import crossmarkIcon from '$lib/assets/misc_icons/crossmark_small.svg?raw';
	import InfoBadges from '../VoteResults/InfoTiles/InfoBadges.svelte';

	interface Props {
		ref: VoteResult;
		showRequiredMajority?: boolean;
	}

	let { ref, showRequiredMajority = false }: Props = $props();
</script>

<a
	href={createVoteResultPath(ref)}
	class="mb-1 flex rounded-xl bg-primary-200/50 px-3 py-2 transition-colors hover:bg-primary-200 dark:bg-primary-400/50 dark:hover:bg-primary-400"
>
	<div class="flex-1">
		<span class="text-base">
			{#if aiViewEnabledStore.value && ref.ai_summary}
				{ref.ai_summary.short_title}
			{:else}
				{ref.legislative_initiative.description}
			{/if}
		</span>
		<InfoBadges voteResult={ref} showGp={false} {showRequiredMajority} />
	</div>
	<div class="flex items-center">
		{#if ref.legislative_initiative.accepted === 'a'}
			<span class="shrink-0 stroke-green-600 dark:stroke-green-500" style="width:20px; height:20px;"
				>{@html checkmarkIcon}</span
			>
		{:else if ref.legislative_initiative.accepted}
			<span class="shrink-0" style="width:20px; height:20px;">{@html crossmarkIcon}</span>
		{/if}
	</div>
</a>
