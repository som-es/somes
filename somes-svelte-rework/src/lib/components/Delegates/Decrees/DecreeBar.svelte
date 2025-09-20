<script lang="ts">
	import collapse from 'svelte-collapse';
	import type {  Delegate } from '$lib/types';
	import DecreeBarExpanded from './DecreeBarExpanded.svelte';
	import { address } from '$lib/api/api';
	import type { Decree } from './types';
	import { dashDateToDotDate } from '$lib/date';

	export let decree: Decree;
	export let delegate: Delegate | null = null;
	export let page: number;
	export let coloring: string = "dark:bg-primary-300 bg-primary-400"
	// absence.

	let open: boolean = false;
	let duration = 0.35;

	$: if (page) {
		open = false;
	}
</script>

<div class="gap-3 mt-5">
	<div
		on:click={() => (open = !open)}
		on:keypress={() => (open = !open)}
		role="button"
		tabindex="0"
		class="entry {coloring} text-black"
	>
		<div class="flex justify-between items-center">
			<div class="flex flex-col gap-1">
				{decree.short_title}
				<div class="flex flex-wrap gap-1">
					<span class="badge bg-secondary-400">{decree.ministrial_issuer}</span>
					<span class="badge bg-secondary-400">{dashDateToDotDate(decree.publication_date)}</span>
					<span class="badge bg-secondary-400">{decree.gp}</span>
				</div>
			</div>

			{#if delegate}
				<img
					class="min-w-[80px] max-h-[80px] rounded-full mx-1"
					src={`${address}/assets/${delegate.id}.jpg`}
					alt="Image of delegate {delegate.name}"
				/>
			{/if}
			
		</div>

	</div>
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

	#open :global(.right-arrow) {
		transform: rotate(90deg);
		transition: transform 0.35s;
	}

	#closed :global(.right-arrow) {
		transform: rotate(0deg);
		transition: transform 0.35s;
	}
</style>
