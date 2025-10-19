<script lang="ts">
	import Documents from '$lib/components/Documents/Documents.svelte';
	import Emphasis from '$lib/components/VoteResults/Emphasis/Emphasis.svelte';
	import DecreeInfoTiles from '$lib/components/VoteResults/InfoTiles/DecreeInfoTiles.svelte';
	import { dashDateToDotDate } from '$lib/date';
	import type { Delegate } from '$lib/types';
	import DelegateCard from '../DelegateCard.svelte';
	import type { Decree } from './types';
	import externalLink from '$lib/assets/misc_icons/external-link.svg?raw';

	export let decree: Decree;
	export let delegate: Delegate | null;

	decree.emphasis = '';
	// 'Hasdfkadsfaadakjfklsafklasödjfklösdfjöklskdfljdskflsdfsdsdfdsaf\nCooler test!\n';
</script>

<title>
	{decree.short_title}
</title>

<div class="entry bg-primary-200 dark:bg-primary-400 mt-3 flex max-lg:flex-wrap gap-3">
	<div class="flex flex-col gap-3 w-full">
		<div class="rounded-xl bg-primary-300 dark:bg-primary-500 px-3 py-3">
			<div class="flex justify-between">
				<div>
					<h1 class="font-bold text-xl md:text-3xl">Verordnung</h1>
					<span class="md:text-xl">{decree.short_title}</span>
				</div>
				{#if decree.document_url}
					<a class="w-10" href={decree.document_url} target="_blank">
						{@html externalLink}
					</a>
				{/if}
			</div>
		</div>
		{#if decree.emphasis}
			<Emphasis rawEmphasis={decree.emphasis} />
		{/if}
		<div class="flex flex-wrap gap-2 w-full">
			<div class="flex flex-wrap gap-1 rounded-xl bg-primary-300 dark:bg-primary-500 px-3 py-3">
				<span class="badge bg-tertiary-400 text-wrap text-lg text-black"
					>{decree.ministrial_issuer}</span
				>
				<span class="badge bg-tertiary-400 text-lg text-black"
					>{dashDateToDotDate(decree.publication_date)}</span
				>
				<span class="badge bg-tertiary-400 text-lg text-black">{decree.gp}</span>
			</div>
			<!-- <DecreeInfoTiles {decree} /> -->
			{#if decree.documents.length > 0}
				<div class="rounded-xl bg-primary-300 dark:bg-primary-500 p-3">
					<Documents documents={decree.documents} />
				</div>
			{/if}
		</div>
	</div>

	<div class="rounded-xl bg-primary-300 dark:bg-primary-500 px-3 py-3">
		{#if delegate}
			<DelegateCard {delegate} />
		{/if}
	</div>
</div>

<style>
	.entry {
		border-radius: 0.9rem;
		box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
		padding: 11px;
		gap: 10px;
	}
</style>
