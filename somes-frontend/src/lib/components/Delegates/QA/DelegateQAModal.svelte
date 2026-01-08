<script lang="ts">
	import type { DelegateQA } from '$lib/types';
	import { Dialog, Popover } from 'bits-ui';
	import DelegateQaEntry from './DelegateQAEntry.svelte';

	interface Props {
		questions: DelegateQA[];
	}

	let { questions }: Props = $props();

</script>

<div class="p-8 max-w-7xl ">
	<div class="flex justify-between">

		<Popover.Root>
			<Popover.Trigger openOnHover openDelay={100}>
				<span class="text-4xl">⚠</span>
			</Popover.Trigger>
			<Popover.Portal>
				<Popover.Content class="z-51 p-4 w-72 bg-primary-300 dark:bg-primary-800 rounded-lg shadow-lg">
					Die Fragen und Antworten wurden aus dem Portraitvideo des Abgeordneten extrahiert.
				</Popover.Content>
			</Popover.Portal>
		</Popover.Root>

		<Dialog.Close>
			<button class="w-5 unselectable float-right text-5xl">✕</button>
		</Dialog.Close>

	</div>
	{#if questions.length > 0}
		{#each questions as qa}
			<DelegateQaEntry class="mt-3" delegateQa={qa} />
		{/each}
	{:else}
		<p class="text-center">Keine Fragen und Antworten verfügbar.</p>
	{/if}
</div>
