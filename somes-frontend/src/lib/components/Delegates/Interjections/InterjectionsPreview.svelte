<script lang="ts">
	import { t } from '$lib/i18n/i18n.svelte';
	import type { Delegate, HasError, Interjection, InterjectionsWithMaxPage } from '$lib/types';
	import { Popover } from 'bits-ui';
	import ExtendInfoDialog from '../ExtendInfoDialog.svelte';
	import { delegate_by_id, isHasError } from '$lib/api/api';
	import InterjectionsModal from './InterjectionsModal.svelte';
	import { currentDelegateStore } from '$lib/stores/stores';
	import { gotoHistory } from '$lib/goto';
	import { getParliament, plink, type Parliament } from '$lib/api/parliament';
	import DelegateListItem from '../DelegateListItem.svelte';

	interface Props {
		issuerDelegate: Delegate;
		issuedInterjectionsPage0: InterjectionsWithMaxPage;
		receivedInterjectionsPage0: InterjectionsWithMaxPage;
		parliament?: Parliament;
	}

	let {
		issuerDelegate,
		issuedInterjectionsPage0,
		receivedInterjectionsPage0,
		parliament = getParliament()
	}: Props = $props();

	// interjections = interjections.sort(
	// 	(a, b) => (a.interjection_text?.length ?? 0) - (b.interjection_text?.length ?? 0)
	// );
	let activeTab: 'issued' | 'received' = $state('issued');

	function collectTillLengthExceeds(entries: Interjection[]) {
		let currentLen = 0;
		return entries.filter((entry) => {
			currentLen += entry.interjection_text?.length ?? 0;
			return currentLen <= 400;
		});
	}

	let interjections = $derived(
		activeTab === 'issued'
			? collectTillLengthExceeds(issuedInterjectionsPage0.interjections)
			: collectTillLengthExceeds(receivedInterjectionsPage0.interjections)
	);

	const fetchDelegate = async (id: number): Promise<Delegate | HasError> => {
		if (id === issuerDelegate.id) {
			return issuerDelegate;
		}
		return delegate_by_id(id);
	};
	const onShowDetails = (delegate: Delegate) => {
		currentDelegateStore.value = delegate;
		gotoHistory(plink('/delegates'), true);
	};
</script>

<div
	class="title-item flex h-full w-full flex-col rounded-xl bg-primary-300 p-5 dark:bg-primary-500"
>
	<div class="flex-1">
		<div class="flex items-center justify-between">
			<div class="flex min-w-full flex-col">
				<div class="flex flex-row justify-between">
					<span class="text-lg font-bold text-black xl:text-xl dark:text-white">
						{t('interjections.title')}
					</span>
					{#if interjections.length !== 0}
						<div>
							<ExtendInfoDialog title={t('interjections.showAll')}>
								<InterjectionsModal
									delegateId={issuerDelegate.id}
									ty={activeTab}
									interjectionsPage0={activeTab === 'issued'
										? issuedInterjectionsPage0
										: receivedInterjectionsPage0}
								/>
							</ExtendInfoDialog>
						</div>
					{/if}
				</div>
				<div
					class="mt-1 mb-2 flex max-w-fit gap-1 rounded-xl bg-primary-400 p-0.5 dark:bg-surface-500"
				>
					<button
						class="flex-1 rounded-lg px-4 py-1 text-sm font-medium {activeTab === 'issued'
							? 'bg-primary-600 text-white dark:bg-primary-700'
							: 'text-gray-700 hover:bg-primary-400 dark:text-gray-300 dark:hover:bg-primary-600'}"
						onclick={() => (activeTab = 'issued')}
					>
						{t('interjections.issued')}
					</button>
					<button
						class="flex-1 rounded-lg px-4 py-1 text-sm font-medium {activeTab === 'received'
							? 'bg-primary-600 text-white dark:bg-primary-700'
							: 'text-gray-700 hover:bg-primary-400 dark:text-gray-400 dark:hover:bg-primary-600'}"
						onclick={() => (activeTab = 'received')}
					>
						{t('interjections.received')}
					</button>
				</div>
			</div>
		</div>

		<div class="mt-4 flex flex-wrap">
			{#if interjections.length === 0}
				<div class="w-full rounded-lg bg-surface-100-900 p-20 text-center">
					{t('interjections.none')}
				</div>
			{/if}
			{#each interjections as interjection (interjection)}
				<Popover.Root>
					<Popover.Trigger>
						<div
							class="mr-4 mb-4 badge bg-primary-400 px-3 py-0.5 text-sm transition-colors hover:bg-primary-500 dark:bg-primary-600 dark:hover:bg-primary-700"
						>
							<div class="mt-1 max-h-24 overflow-hidden text-wrap">
								{interjection.interjection_text}
							</div>
						</div>
					</Popover.Trigger>
					<Popover.Portal>
						<Popover.Content side="top" sideOffset={2}>
							{#await fetchDelegate(activeTab === 'issued' ? interjection.speaker_delegate_id : interjection.interjector_delegate_id)}
								<div class="rounded-2xl bg-primary-200 px-3 py-2 shadow-lg dark:bg-primary-400">
									{t('interjections.loadingSpeaker')}
								</div>
							{:then delegate}
								{#if !isHasError(delegate)}
									<DelegateListItem
										{delegate}
										{parliament}
										size="md"
										class="shadow-lg"
										onclick={() => {
											onShowDetails(delegate);
										}}
									/>
								{/if}
							{/await}
							<Popover.Arrow class="fill-current text-primary-200 dark:text-primary-400" />
						</Popover.Content>
					</Popover.Portal>
				</Popover.Root>
			{/each}
		</div>

		<div class="mt-4">
			<h3 class="mb-2 text-sm font-semibold tracking-wider text-primary-800 dark:text-primary-200">
				<!-- {lastEntriesText} -->
			</h3>
		</div>
	</div>

	<!-- {#if recentAbsences.length > 0 &&  absences.length > recentAbsences.length}
		<div class="mt-auto flex justify-end pt-4">
			<ExtendInfoDialog title="Alle anzeigen">
				<AbsencesModal absences={sortedAbsences} title={title} showDetails={showDetails} />
			</ExtendInfoDialog>
		</div>
	{/if} -->
</div>
