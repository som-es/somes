<script lang="ts">
	// import collapse from 'svelte-collapse';
	import type { Delegate } from '$lib/types';
	import DecreeBarExpanded from './DecreeBarExpanded.svelte';
	import { address } from '$lib/api/api';
	import type { Decree } from './types';
	import { dashDateToDotDate } from '$lib/date';
	import { gotoHistory } from '$lib/goto';
	import { currentDecreeStore } from '$lib/stores/stores';

	export let decree: Decree;
	export let delegate: Delegate | null = null;
	export let page: number;
	export let showDelegate: boolean = false;
	export let coloring: string = 'dark:bg-primary-300 bg-primary-400 text-black';
	// absence.

	let open: boolean = false;
	let duration = 0.35;

	$: if (page) {
		open = false;
	}
	function onShowDetails() {
		currentDecreeStore.value = { decree, delegate };
		gotoHistory(`/decree/${decree.ris_id}`, true);
	}
</script>

<div class="gap-3 mt-5">
	<div
		on:click={onShowDetails}
		on:keypress={onShowDetails}
		role="button"
		tabindex="0"
		class="entry {coloring} flex justify-between items-center"
	>
		<div class="flex flex-col gap-1">
			<!-- <span class="dark:text-white"> -->
			{decree.short_title}
			<!-- </span> -->
			<div class="flex flex-wrap gap-1">
				<span class="badge bg-tertiary-400 text-wrap text-black">{decree.ministrial_issuer}</span>
				<span class="badge bg-tertiary-400 text-black"
					>{dashDateToDotDate(decree.publication_date)}</span
				>
				<span class="badge bg-tertiary-400 text-black">{decree.gp}</span>
			</div>
		</div>

		{#if showDelegate && delegate}
			<img
				class="min-w-[80px] max-h-[80px] rounded-full mx-1"
				src={`${address}/assets/${delegate.id}.jpg`}
				alt="Image of delegate {delegate.name}"
			/>
		{/if}
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
</style>
