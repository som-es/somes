<script lang="ts">
	import { address, url } from '$lib/api/api';
	import type { Decree, DecreeDelegate } from './types';
	import { dashDateToDotDate } from '$lib/date';
	import { gotoHistory } from '$lib/goto';
	import { createDecreePath } from './api';
	import { aiViewEnabledStore } from '$lib/stores/stores';

	interface Props {
		decree: DecreeDelegate;
		showDelegate?: boolean;
		coloring?: string;
	}

	let {
		decree,
		showDelegate = false,
		coloring = 'dark:bg-primary-300 bg-primary-400 text-black'
	}: Props = $props();

	let open: boolean = false;

	function onShowDetails() {
		// currentDecreeStore.value = { decree, delegate };
		gotoHistory(createDecreePath(decree.decree.ris_id), true);
	}
</script>

<div class="mt-5 gap-3">
	<a
		href={createDecreePath(decree.decree.ris_id)}
		tabindex="0"
		class="entry {coloring} flex items-center justify-between"
	>
		<div class="flex flex-col gap-1">
			{#if aiViewEnabledStore.value && decree.decree.ai_summary}
				<span class="text-md font-semibold sm:text-lg">
					{decree.decree.ai_summary.short_title}
				</span>
				<span class="sm:text-md text-sm">
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
				<span class="badge bg-tertiary-400 text-wrap text-black"
					>{decree.decree.ministrial_issuer}</span
				>
				<span class="badge bg-tertiary-400 text-black"
					>{dashDateToDotDate(decree.decree.publication_date)}</span
				>
				<span class="badge bg-tertiary-400 text-black">{decree.decree.gp}</span>
			</div>
		</div>

		{#if showDelegate && decree.delegate}
			<div class="hidden flex-col gap-0 sm:flex">
				<img
					class="mx-1 max-h-[80px] min-w-[80px] rounded-full"
					src={`${url}assets/${decree.delegate.id}.jpg`}
					alt="Image of delegate {decree.delegate.name}"
				/>
				<span class="bottom-0 rounded text-[8px]">
					{#if decree.delegate.image_copyright}
						&copy {decree.delegate.image_copyright}
					{:else}
						&copy Parlamentsdirektion
					{/if}
				</span>
			</div>
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
