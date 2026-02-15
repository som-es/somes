<script lang="ts">
	import { address, url } from '$lib/api/api';
	import type { Decree, DecreeDelegate } from './types';
	import { dashDateToDotDate } from '$lib/date';
	import { gotoHistory } from '$lib/goto';
	import { createDecreePath } from './api';

	interface Props {
		decree: DecreeDelegate;
		showDelegate?: boolean;
		coloring?: string;
	}

	let { decree, showDelegate = false, coloring = 'dark:bg-primary-300 bg-primary-400 text-black' }: Props = $props();

	let open: boolean = false;

	function onShowDetails() {
		// currentDecreeStore.value = { decree, delegate };
		gotoHistory(createDecreePath(decree.decree.ris_id), true);
	}
</script>

<div class="gap-3 mt-5">
	<a
		href="{createDecreePath(decree.decree.ris_id)}"
		tabindex="0"
		class="entry {coloring} flex justify-between items-center"
	>
		<div class="flex flex-col gap-1">
			{#if decree.decree.ai_summary}
				<span class="text-md sm:text-lg font-semibold ">
					{decree.decree.ai_summary.short_title}
				</span>
				<span class="text-sm sm:text-md">
					{decree.decree.ai_summary.short_summary}
				</span>
			{:else}
				<span>
					{decree.decree.short_title}
				</span>
			{/if}
			<!-- <span class="dark:text-white"> -->
			<!-- </span> -->
			<div class="flex flex-wrap gap-1">
				<span class="badge bg-tertiary-400 text-wrap text-black">{decree.decree.ministrial_issuer}</span>
				<span class="badge bg-tertiary-400 text-black"
					>{dashDateToDotDate(decree.decree.publication_date)}</span
				>
				<span class="badge bg-tertiary-400 text-black">{decree.decree.gp}</span>
			</div>
		</div>

		{#if showDelegate && decree.delegate}
			<img
				class="min-w-[80px] max-h-[80px] rounded-full mx-1"
				src={`${url}assets/${decree.delegate.id}.jpg`}
				alt="Image of delegate {decree.delegate.name}"
			/>
		{/if}
	</a>
	<!-- <div use:collapse={{ open, duration }}>
		<DecreeBarExpanded {decree} bind:open />
	</div> -->
</div>

<style>
	.entry {
		border-radius: 0.9rem;
		box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
		padding: 20px;
		gap: 10px;
	}
</style>
