<script lang="ts">
	import { delegate_by_id, errorToNull } from '$lib/api/api';
	import type { Delegate, Interjection } from '$lib/types';

	interface Props {
		interjection: Interjection;
		ty: 'issued' | 'received';
		coloring?: string;
	}

	let { interjection, ty, coloring = 'bg-primary-300 dark:bg-primary-500' }: Props = $props();

	let delegate = $state<Delegate | null>(null);
	let loading = $state(true);

	$effect(() => {
		delegate = null;
		loading = true;
		delegate_by_id(
			ty === 'issued' ? interjection.speaker_delegate_id : interjection.interjector_delegate_id
		).then((res) => {
			delegate = errorToNull(res);
			loading = false;
		});
	});

	// Helper to format the date
	let formattedDate = $derived(
		new Date(interjection.date).toLocaleDateString(undefined, {
			year: 'numeric',
			month: 'short',
			day: 'numeric'
		})
	);
</script>

{#if !loading && delegate}
	<div
		class="entry {coloring} flex w-full flex-col items-start justify-between text-gray-900 transition-colors md:flex-row md:items-center dark:text-gray-100"
	>
		<!-- Left Side: Delegate Identity -->
		<div class="flex min-w-[220px] items-center gap-3">
			<!-- Avatar placeholder (Uses initials) -->
			<div
				class="flex h-12 w-12 flex-shrink-0 items-center justify-center rounded-full bg-black/10 text-lg font-bold dark:bg-white/10"
			>
				{delegate.first_name?.[0] || ''}{delegate.last_name?.[0] || ''}
			</div>

			<div class="flex flex-col">
				<span class="text-xs font-semibold tracking-wider uppercase opacity-70">
					{ty === 'issued' ? 'To Speaker' : 'From Interjector'}
				</span>
				<span class="text-lg leading-tight font-bold">
					{delegate.first_name}
					{delegate.last_name}
				</span>
				{#if delegate.party}
					<span class="text-sm opacity-80">{delegate.party}</span>
				{/if}
			</div>
		</div>

		<!-- Middle: Interjection Text -->
		<div
			class="mx-0 my-4 w-full flex-1 border-y border-black/10 px-0 py-4 text-left md:mx-6 md:my-0 md:border-x md:border-y-0 md:px-8 md:py-0 md:text-center dark:border-white/10"
		>
			{#if interjection.interjection_text}
				<span class="text-lg font-medium italic">"{interjection.interjection_text}"</span>
			{:else}
				<span class="text-sm italic opacity-60">(Inaudible / No transcript provided)</span>
			{/if}
		</div>

		<!-- Right Side: Metadata -->
		<div class="flex min-w-[150px] flex-col items-start gap-1 text-sm opacity-90 md:items-end">
			<div class="flex items-center gap-2">
				<svg
					class="h-4 w-4 opacity-70"
					fill="none"
					stroke="currentColor"
					viewBox="0 0 24 24"
					xmlns="http://www.w3.org/2000/svg"
					><path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"
					></path></svg
				>
				<span>{formattedDate}</span>
			</div>

			<div class="flex items-center gap-2">
				<svg
					class="h-4 w-4 opacity-70"
					fill="none"
					stroke="currentColor"
					viewBox="0 0 24 24"
					xmlns="http://www.w3.org/2000/svg"
					><path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M19 11a7 7 0 01-7 7m0 0a7 7 0 01-7-7m7 7v4m0 0H8m4 0h4m-4-8a3 3 0 01-3-3V5a3 3 0 116 0v6a3 3 0 01-3 3z"
					></path></svg
				>
				<!-- Assuming you have a route to view the speech -->
				<a href="/speech/{interjection.plenar_speech_id}" class="font-medium hover:underline">
					Speech #{interjection.plenar_speech_id}
				</a>
			</div>
		</div>
	</div>
{/if}

<style>
	.entry {
		border-radius: 0.9rem;
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
		padding: 20px 24px;
		/* gap is handled by Tailwind flex utilities for better mobile responsiveness */
	}
</style>
