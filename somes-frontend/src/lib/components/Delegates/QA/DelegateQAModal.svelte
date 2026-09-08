<script lang="ts">
	import { t } from '$lib/i18n/i18n.svelte';
	import type { DelegateQA } from '$lib/types';
	import { Dialog, Popover } from 'bits-ui';
	import DelegateQaEntry from './DelegateQAEntry.svelte';
	import ModalCloseButton from '$lib/components/UI/ModalCloseButton.svelte';

	interface Props {
		questions: DelegateQA[];
	}

	let { questions }: Props = $props();
</script>

<div class="max-w-7xl p-8">
	<div class="flex justify-between">
		<Popover.Root>
			<Popover.Trigger openOnHover openDelay={100}>
				<span class="text-4xl">⚠</span>
			</Popover.Trigger>
			<Popover.Portal>
				<Popover.Content
					class="z-51 w-72 rounded-lg bg-primary-300 p-4 shadow-lg dark:bg-primary-800"
				>
					{t('qa.extractedFromVideo')}
				</Popover.Content>
			</Popover.Portal>
		</Popover.Root>

		<Dialog.Close>
			<ModalCloseButton />
		</Dialog.Close>
	</div>
	{#if questions.length > 0}
		{#each questions as qa}
			<DelegateQaEntry class="mt-3" delegateQa={qa} />
		{/each}
	{:else}
		<p class="text-center">{t('spectrum.questions.empty')}</p>
	{/if}
</div>
