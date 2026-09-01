<script lang="ts">
	import { Dialog } from 'bits-ui';
	import Container from '$lib/components/Layout/Container.svelte';
	import DelegateQuestionModal from '$lib/components/Delegates/Questions/DelegateQuestionModal.svelte';
	import { t } from '$lib/i18n/i18n.svelte';
	import { formatDate } from '$lib/date';
	import { partyToColor } from '$lib/partyColor';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();

	const question = $derived(data.entry?.question ?? null);
	const delegate = $derived(data.entry?.delegate ?? null);

	const initials = $derived(
		delegate?.name
			.split(' ')
			.map((part) => part[0])
			.slice(0, 2)
			.join('') ?? ''
	);
</script>

<svelte:head>
	<title>{question ? question.subject : t('qa.title')}</title>
	<meta name="description" content={t('qa.meta.description')} />
</svelte:head>

<Container>
	{#if question}
		<div class="flex flex-col gap-4 lg:flex-row lg:items-start">
			<div class="flex min-w-0 flex-1 flex-col gap-4">
				<!-- Question -->
				<div class="rounded-xl bg-primary-300 p-4 sm:p-6 dark:bg-primary-500">
					<div
						class="flex flex-wrap items-baseline justify-between gap-2 text-sm text-gray-700 dark:text-gray-300"
					>
						<span>{t('qa.questionTo', { name: delegate?.name ?? '' })}</span>
						<span class="shrink-0">{formatDate(question.created_at)}</span>
					</div>
					<h1
						class="mt-2 text-xl leading-tight font-bold lg:text-2xl"
						style="hyphens: auto; overflow-wrap: break-word;"
					>
						{question.subject}
					</h1>
					<p class="mt-8 whitespace-pre-line">{question.body}</p>
				</div>

				<!-- Answers (a question can receive several reply mails) -->
				{#each question.answers as answer (answer.received_at)}
					<div class="rounded-xl bg-primary-300 p-4 sm:p-6 dark:bg-primary-500">
						<div class="flex justify-between text-sm text-gray-700 dark:text-gray-300">
							<div class="flex items-center gap-3">
								<div
									class="flex h-11 w-11 shrink-0 items-center justify-center rounded-full font-semibold text-white"
									style="background-color: {partyToColor(delegate?.party ?? null)};"
								>
									{initials}
								</div>
								<div class="flex min-w-0 flex-col">
									<span class="truncate leading-tight font-bold">{delegate?.name ?? ''}</span>
									{#if delegate?.party}
										<div class="mt-0.5 flex items-center gap-1.5">
											<div
												class="h-2 w-2 shrink-0 rounded-full"
												style="background-color: {partyToColor(delegate.party)};"
											></div>
											<span class="truncate text-xs text-gray-700 dark:text-gray-300">
												{delegate.party}
											</span>
										</div>
									{/if}
								</div>
							</div>
							<span class="shrink-0">{formatDate(answer.received_at)}</span>
						</div>

						<p class="mt-4 whitespace-pre-line">{answer.body}</p>
					</div>
				{:else}
					<div
						class="rounded-xl bg-primary-300 p-5 text-center text-gray-700 italic dark:bg-primary-500 dark:text-gray-300"
					>
						{t('qa.unansweredHint')}
					</div>
				{/each}
			</div>

			<!-- Call to action -->
			<div
				class="shrink-0 rounded-xl bg-primary-300 p-5 lg:sticky lg:top-4 lg:w-72 dark:bg-primary-500"
			>
				{#if delegate}
					<span class="block text-lg font-bold">{t('qa.askFromTitle')}</span>
					<div class="mt-3 flex items-center gap-2.5">
						<div
							class="flex h-9 w-9 shrink-0 items-center justify-center rounded-full text-sm font-semibold text-white"
							style="background-color: {partyToColor(delegate.party)};"
						>
							{initials}
						</div>
						<div class="flex min-w-0 flex-col">
							<span class="truncate leading-tight font-medium">{delegate.name}</span>
							{#if delegate.party}
								<div class="mt-0.5 flex items-center gap-1.5">
									<div
										class="h-2 w-2 shrink-0 rounded-full"
										style="background-color: {partyToColor(delegate.party)};"
									></div>
									<span class="truncate text-xs text-gray-700 dark:text-gray-300">
										{delegate.party}
									</span>
								</div>
							{/if}
						</div>
					</div>
					<Dialog.Root>
						<Dialog.Trigger
							class="mt-4 w-full rounded-xl bg-secondary-500 px-3 py-2 text-white hover:cursor-pointer hover:bg-secondary-600"
						>
							{t('qa.askButton')}
						</Dialog.Trigger>
						<Dialog.Portal>
							<Dialog.Overlay
								class="fixed inset-0 z-50 bg-black/80 data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:animate-in data-[state=open]:fade-in-0"
							/>
							<Dialog.Content
								class="fixed top-[50%] left-[50%] z-50 w-full max-w-xl translate-x-[-50%] translate-y-[-50%] overflow-hidden rounded-lg bg-primary-100 shadow-lg outline-hidden data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95 data-[state=open]:animate-in data-[state=open]:fade-in-0 data-[state=open]:zoom-in-95 dark:bg-gray-800"
							>
								<DelegateQuestionModal {delegate} />
							</Dialog.Content>
						</Dialog.Portal>
					</Dialog.Root>
				{/if}
			</div>
		</div>
	{:else}
		<div class="rounded-xl bg-primary-300 p-5 text-center dark:bg-primary-500">
			{t('qa.notFound')}
		</div>
	{/if}
</Container>
