<script lang="ts">
	import Container from '$lib/components/Layout/Container.svelte';
	import { askDelegateQuestion, getUser } from '$lib/api/authed';
	import { isHasError } from '$lib/api/api';
	import { plink } from '$lib/api/parliament';
	import { partyToColor } from '$lib/partyColor';
	import { t } from '$lib/i18n/i18n.svelte';
	import { jwtStore, loginDrawerOpenStore } from '$lib/caching/stores/stores.svelte';
	import type { ExtendedUserInfo } from '$lib/types';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();

	const delegate = $derived(data.delegate);
	const recipient = $derived(data.recipient);

	const steps = [
		{ label: () => t('qa.ask.step.data') },
		{ label: () => t('qa.ask.step.question') },
		{ label: () => t('qa.ask.step.review') }
	];
	let activeStep = $state(0);

	let subject = $state('');
	let body = $state('');
	let errorMessage = $state<string | null>(null);

	let user = $state<ExtendedUserInfo | null>(null);
	let userChecked = $state(false);

	let isSending = $state(false);
	let wasSubmitted = $state(false);

	$effect(() => {
		const jwt = jwtStore.value;
		let cancelled = false;
		if (jwt === null) {
			user = null;
			userChecked = true;
			return;
		}
		userChecked = false;
		getUser().then((result) => {
			if (cancelled) return;
			user = isHasError(result) ? null : result;
			userChecked = true;
		});
		return () => {
			cancelled = true;
		};
	});

	function goToStep(step: number) {
		errorMessage = null;
		activeStep = step;
	}

	function nextFromQuestion() {
		if (!subject.trim() || !body.trim()) {
			errorMessage = t('qa.ask.missingFields');
			return;
		}
		goToStep(2);
	}

	async function submitQuestion() {
		if (delegate === null) return;

		isSending = true;
		errorMessage = null;

		const result = await askDelegateQuestion(delegate.id, {
			subject: subject.trim(),
			body: body.trim()
		});

		isSending = false;
		if (isHasError(result)) {
			if (result.error === 'No access token' || result.error_type === 'AuthError') {
				errorMessage = t('qa.ask.loginRequired');
				loginDrawerOpenStore.value = true;
			} else {
				errorMessage = result.error;
			}
			return;
		}

		wasSubmitted = true;
	}
</script>

<svelte:head>
	<title>{delegate ? t('qa.ask.title', { name: delegate.name }) : t('qa.title')}</title>
	<meta name="description" content={t('qa.meta.description')} />
</svelte:head>

<Container>
	{#if delegate === null}
		<div class="mt-2 rounded-xl bg-primary-300 p-5 text-center sm:mt-0 dark:bg-primary-500">
			{t('qa.ask.delegateNotFound')}
		</div>
	{:else}
		<h1 class="px-1 pt-2 text-3xl font-bold sm:p-0 sm:text-4xl">
			{t('qa.ask.title', { name: delegate.name })}
		</h1>

		{#if wasSubmitted}
			<div class="mx-auto mt-8 max-w-2xl">
				<p
					class="rounded-md border border-emerald-600/40 bg-emerald-100 px-4 py-3 text-emerald-900 dark:bg-emerald-950 dark:text-emerald-100"
				>
					{t('qa.ask.success')}
				</p>
				<div class="mt-5 flex justify-end">
					<a
						href={plink('/questions')}
						class="rounded-xl bg-secondary-500 px-4 py-2 text-white transition-colors hover:bg-secondary-600"
					>
						{t('qa.ask.toOverview')}
					</a>
				</div>
			</div>
		{:else}
			<!-- Step line -->
			<div class="mx-auto mt-8 flex max-w-md items-start">
				{#each steps as step, index (index)}
					{#if index > 0}
						<div class="mt-4 h-px min-w-6 flex-1 bg-gray-400 dark:bg-gray-500"></div>
					{/if}
					<button
						class="flex flex-col items-center gap-1.5 disabled:opacity-100 {index < activeStep
							? 'hover:cursor-pointer'
							: ''}"
						disabled={index >= activeStep}
						onclick={() => goToStep(index)}
					>
						<span
							class="flex h-8 w-8 items-center justify-center rounded-full text-sm font-semibold {index <=
							activeStep
								? 'bg-surface-500 text-white'
								: 'bg-primary-300 text-gray-700 dark:bg-primary-500 dark:text-gray-300'}"
						>
							{index + 1}
						</span>
						<span class="text-xs text-gray-700 dark:text-gray-300">{step.label()}</span>
					</button>
				{/each}
			</div>

			<div class="mx-auto mt-6 max-w-2xl rounded-xl bg-primary-300 p-4 sm:p-6 dark:bg-primary-500">
				{#if activeStep === 0}
					<!-- Step 1: account & transparency info -->
					{#if !userChecked}
						<p class="text-sm text-gray-700 dark:text-gray-300">...</p>
					{:else if user === null}
						<p
							class="rounded-md border border-red-600/40 bg-red-100 px-3 py-2 text-sm text-red-900 dark:bg-red-950 dark:text-red-100"
						>
							{t('qa.ask.loginRequired')}
						</p>
						<button
							type="button"
							onclick={() => (loginDrawerOpenStore.value = true)}
							class="mt-3 inline-block text-sm font-semibold hover:underline"
						>
							{t('qa.ask.loginLink')} &rarr;
						</button>
					{:else}
						<p class="text-sm">
							<span class="font-semibold">{t('qa.ask.loggedInAs')}:</span>
							{user.is_email_hashed ? t('qa.ask.emailAnonymized') : user.email}
						</p>
					{/if}

					<ul class="mt-4 list-disc space-y-2 pl-5 text-sm text-gray-800 dark:text-gray-200">
						<li>{t('qa.ask.dataHint.review')}</li>
						<li>{t('qa.ask.dataHint.mail')}</li>
						<li>{t('qa.ask.dataHint.anonymous')}</li>
					</ul>
				{:else if activeStep === 1}
					<!-- Step 2: the question itself -->
					<label class="block">
						<span class="mb-1 block text-sm font-semibold">{t('qa.ask.subject')}</span>
						<input
							bind:value={subject}
							maxlength="255"
							placeholder={t('qa.ask.subjectPlaceholder')}
							class="w-full rounded-md border border-gray-300 bg-white px-3 py-2 text-black outline-none focus:border-primary-600 focus:ring-2 focus:ring-primary-300 dark:border-gray-600 dark:bg-gray-900 dark:text-white"
						/>
					</label>

					<label class="mt-4 block">
						<span class="mb-1 block text-sm font-semibold">{t('qa.ask.body')}</span>
						<textarea
							bind:value={body}
							maxlength="10000"
							rows="8"
							placeholder={t('qa.ask.bodyPlaceholder', { name: delegate.name })}
							class="w-full resize-y rounded-md border border-gray-300 bg-white px-3 py-2 text-black outline-none focus:border-primary-600 focus:ring-2 focus:ring-primary-300 dark:border-gray-600 dark:bg-gray-900 dark:text-white"
						></textarea>
						<span class="mt-1 block text-right text-xs text-gray-700 dark:text-gray-300">
							{body.length}/10000
						</span>
					</label>
				{:else}
					<!-- Step 3: review & submit -->
					<div class="flex items-center gap-2.5">
						<div
							class="flex h-9 w-9 shrink-0 items-center justify-center rounded-full text-sm font-semibold text-white"
							style="background-color: {partyToColor(delegate.party)};"
						>
							{delegate.name
								.split(' ')
								.map((part) => part[0])
								.slice(0, 2)
								.join('')}
						</div>
						<div class="flex min-w-0 flex-col">
							<span class="truncate text-sm leading-tight font-medium">
								{t('qa.ask.recipient')}: {recipient?.recipient_name ?? delegate.name}
							</span>
							<span class="text-xs text-gray-700 dark:text-gray-300">
								{#if recipient === null}
									{t('qa.ask.recipientMissing')}
								{:else if recipient.delivery === 'party'}
									{t('qa.ask.recipientParty', { name: recipient.recipient_name })}
								{:else}
									{t('qa.ask.recipientDelegate')}
								{/if}
							</span>
						</div>
					</div>

					<div class="mt-4 rounded-xl bg-surface-50 p-3 sm:p-4 dark:bg-surface-600">
						<span class="block font-bold">{subject}</span>
						<p class="mt-2 text-sm whitespace-pre-line sm:text-base">{body}</p>
					</div>
				{/if}

				{#if errorMessage}
					<p
						class="mt-4 rounded-md border border-red-600/40 bg-red-100 px-3 py-2 text-sm text-red-900 dark:bg-red-950 dark:text-red-100"
					>
						{errorMessage}
					</p>
				{/if}

				<!-- Step navigation -->
				<div class="mt-6 flex justify-between gap-3">
					{#if activeStep > 0}
						<button
							class="rounded-xl px-4 py-2 text-sm font-semibold hover:cursor-pointer"
							onclick={() => goToStep(activeStep - 1)}
						>
							&larr; {t('qa.ask.back')}
						</button>
					{:else}
						<div></div>
					{/if}

					{#if activeStep === 0}
						<button
							class="rounded-xl bg-secondary-500 px-4 py-2 text-sm font-semibold text-white transition-colors hover:cursor-pointer hover:bg-secondary-600 disabled:cursor-not-allowed disabled:opacity-60"
							disabled={user === null}
							onclick={() => goToStep(1)}
						>
							{t('qa.ask.next')} &rarr;
						</button>
					{:else if activeStep === 1}
						<button
							class="rounded-xl bg-secondary-500 px-4 py-2 text-sm font-semibold text-white transition-colors hover:cursor-pointer hover:bg-secondary-600"
							onclick={nextFromQuestion}
						>
							{t('qa.ask.next')} &rarr;
						</button>
					{:else}
						<button
							class="rounded-xl bg-secondary-500 px-4 py-2 text-sm font-semibold text-white transition-colors hover:cursor-pointer hover:bg-secondary-600 disabled:cursor-not-allowed disabled:opacity-60"
							disabled={isSending || recipient === null}
							onclick={submitQuestion}
						>
							{isSending ? t('qa.ask.submitting') : t('qa.ask.submit')}
						</button>
					{/if}
				</div>
			</div>
		{/if}
	{/if}
</Container>
