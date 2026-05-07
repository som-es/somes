<script lang="ts">
	import type { Delegate, HasError, Interjection, InterjectionsWithMaxPage } from '$lib/types';
	import { Popover } from 'bits-ui';
	import ExtendInfoDialog from '../ExtendInfoDialog.svelte';
	import { delegate_by_id, isHasError, url } from '$lib/api/api';
	import DelegateCard from '../DelegateCard.svelte';
	import InterjectionsModal from './InterjectionsModal.svelte';
	import { currentDelegateStore } from '$lib/stores/stores';
	import { gotoHistory } from '$lib/goto';
	import { resolve } from '$app/paths';

	interface Props {
		issuerDelegate: Delegate;
		issuedInterjectionsPage0: InterjectionsWithMaxPage;
		receivedInterjectionsPage0: InterjectionsWithMaxPage;
	}

	let { issuerDelegate, issuedInterjectionsPage0, receivedInterjectionsPage0 }: Props = $props();

	function formatDate(dateString: Date | string) {
		return new Intl.DateTimeFormat('de-AT', {
			day: '2-digit',
			month: '2-digit',
			year: 'numeric'
		}).format(new Date(dateString));
	}

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
		gotoHistory(resolve(`/delegates`), true);
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
						Zwischenrufe
					</span>
					<div>
						<ExtendInfoDialog title="Alle anzeigen">
							<InterjectionsModal
								delegateId={issuerDelegate.id}
								ty={activeTab}
								interjectionsPage0={activeTab === 'issued'
									? issuedInterjectionsPage0
									: receivedInterjectionsPage0}
							/>
						</ExtendInfoDialog>
					</div>
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
						Vergeben
					</button>
					<button
						class="flex-1 rounded-lg px-4 py-1 text-sm font-medium {activeTab === 'received'
							? 'bg-primary-600 text-white dark:bg-primary-700'
							: 'text-gray-700 hover:bg-primary-400 dark:text-gray-400 dark:hover:bg-primary-600'}"
						onclick={() => (activeTab = 'received')}
					>
						Erhalten
					</button>
				</div>
			</div>
		</div>

		<div class="mt-4 flex flex-wrap">
			{#each interjections as interjection}
				<Popover.Root>
					<Popover.Trigger>
						<div class="mr-4 mb-4 badge bg-primary-400 px-3 py-0.5 text-sm dark:bg-primary-600">
							<div class="mt-1 max-h-24 overflow-hidden text-ellipsis">
								{interjection.interjection_text}
							</div>
						</div>
					</Popover.Trigger>
					<Popover.Portal>
						<Popover.Content side="top">
							<div class="rounded-lg bg-primary-200 p-5 dark:bg-primary-400">
								{#await fetchDelegate(activeTab === 'issued' ? interjection.speaker_delegate_id : interjection.interjector_delegate_id)}
									Lädt Redner..
								{:then delegate}
									{#if !isHasError(delegate)}
										<button
											onclick={() => {
												onShowDetails(delegate);
											}}
											class="flex flex-row items-center gap-2"
										>
											<div class="relative flex justify-center pb-6">
												<img
													src={`${url}assets/${delegate.id}.jpg`}
													class="w-20 rounded-full md:w-30"
													alt="Image of politician {delegate.name}"
												/>
												<span class="absolute bottom-0 rounded px-1 text-[10px]">
													{#if delegate.image_copyright}
														&copy {delegate.image_copyright}
													{:else}
														&copy Parlamentsdirektion
													{/if}
												</span>
											</div>
											<span class="font-bold">{delegate.name}</span>
										</button>
									{/if}
								{/await}
							</div>
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
