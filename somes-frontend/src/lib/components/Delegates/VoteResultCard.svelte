<script lang="ts">
	import { type VoteResult, createVoteResultPath } from '$lib/types';
	import { aiViewEnabledStore, currentVoteResultStore } from '$lib/stores/stores';
	import ExpandablePlaceholder from '$lib/components/VoteResults/Expandable/Placeholders/ExpandablePlaceholder.svelte';
	import type { Snippet } from 'svelte';

	interface Props {
		voteResult: VoteResult | null;
		loading: boolean;
		children?: Snippet;
		class?: string;
	}

	let { voteResult, loading, children, class: extraClass = '' }: Props = $props();
</script>

<a
	class="mt-2 flex w-full flex-col rounded-xl bg-primary-200 p-3 transition-colors hover:bg-primary-400 lg:flex-row lg:items-center lg:justify-between dark:bg-primary-300 dark:text-black dark:hover:bg-primary-400 {extraClass}"
	href={voteResult ? createVoteResultPath(voteResult) : undefined}
	onclick={() => {
		if (voteResult) currentVoteResultStore.value = voteResult;
	}}
>
	<div class="flex flex-col">
		{#if loading}
			<ExpandablePlaceholder />
		{:else if voteResult}
			{#if aiViewEnabledStore.value && voteResult.ai_summary}
				<span
					class="text-xl font-semibold"
					style="hyphens: auto; word-break: normal; overflow-wrap: break-word;"
				>
					{voteResult.ai_summary.short_title}
				</span>
				<span class="text-sm sm:text-base">
					{voteResult.ai_summary.short_summary}
				</span>
			{:else}
				<span class="text-md font-semibold">
					{voteResult.legislative_initiative.description}
				</span>
			{/if}
		{/if}
	</div>
	{@render children?.()}
</a>
