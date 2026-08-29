<script lang="ts">
	import { onMount } from 'svelte';
	import {
		approveDelegateQuestion,
		getUser,
		pendingDelegateQuestions,
		rejectDelegateQuestion
	} from '$lib/api/authed';
	import { isHasError } from '$lib/api/api';
	import { formatDateTime } from '$lib/date';
	import type { AdminDelegateQuestion } from '$lib/types';

	let questions = $state<AdminDelegateQuestion[]>([]);
	let isLoading = $state(true);
	let isAdmin = $state(false);
	let errorMessage = $state<string | null>(null);
	let activeQuestionId = $state<number | null>(null);

	onMount(async () => {
		const user = await getUser();
		if (isHasError(user)) {
			errorMessage = 'Bitte melde dich als Admin an.';
			isLoading = false;
			return;
		}

		isAdmin = user.is_admin;
		if (!user.is_admin) {
			errorMessage = 'Du hast keine Berechtigung für diese Seite.';
			isLoading = false;
			return;
		}

		await loadQuestions();
	});

	async function loadQuestions() {
		isLoading = true;
		errorMessage = null;

		const result = await pendingDelegateQuestions();
		isLoading = false;

		if (isHasError(result)) {
			errorMessage = result.error;
			return;
		}

		questions = result;
	}

	async function reviewQuestion(questionId: number, action: 'approve' | 'reject') {
		activeQuestionId = questionId;
		errorMessage = null;

		const result =
			action === 'approve'
				? await approveDelegateQuestion(questionId)
				: await rejectDelegateQuestion(questionId);

		activeQuestionId = null;

		if (isHasError(result)) {
			errorMessage = result.error;
			return;
		}

		questions = questions.filter((question) => question.id !== questionId);
	}
</script>

<svelte:head>
	<title>Fragen prüfen | somes</title>
</svelte:head>

<section class="mx-auto w-full max-w-6xl px-4 py-8">
	<div class="mb-6 flex flex-col gap-3 sm:flex-row sm:items-end sm:justify-between">
		<div>
			<h1 class="text-2xl font-bold text-black dark:text-white">Fragen prüfen</h1>
			<p class="mt-1 text-sm text-gray-600 dark:text-gray-300">
				Offene Fragen werden erst nach Freigabe per E-Mail versendet.
			</p>
		</div>

		{#if isAdmin}
			<button
				class="rounded-md border border-gray-300 px-3 py-2 text-sm font-semibold text-black hover:bg-gray-100 dark:border-gray-600 dark:text-white dark:hover:bg-gray-800"
				onclick={loadQuestions}
			>
				Aktualisieren
			</button>
		{/if}
	</div>

	{#if errorMessage}
		<p
			class="mb-4 rounded-md border border-red-600/40 bg-red-100 px-4 py-3 text-sm text-red-900 dark:bg-red-950 dark:text-red-100"
		>
			{errorMessage}
		</p>
	{/if}

	{#if isLoading}
		<p class="text-sm text-gray-600 dark:text-gray-300">Fragen werden geladen.</p>
	{:else if isAdmin && questions.length === 0 && errorMessage === null}
		<p class="text-sm text-gray-600 dark:text-gray-300">
			Derzeit warten keine Fragen auf Freigabe.
		</p>
	{:else if isAdmin}
		<div class="space-y-4">
			{#each questions as question (question.id)}
				<article
					class="rounded-md border border-gray-200 bg-white p-4 shadow-sm dark:border-gray-700 dark:bg-gray-900"
				>
					<div class="flex flex-col gap-3 md:flex-row md:items-start md:justify-between">
						<div class="min-w-0">
							<div
								class="flex flex-wrap items-center gap-2 text-xs text-gray-600 dark:text-gray-300"
							>
								<span>{formatDateTime(question.created_at)}</span>
								<span>Delegate #{question.delegate_id}</span>
								<span>User #{question.user_id}</span>
								<span>{question.recipient_kind === 'party' ? 'Parteiklub' : 'Abgeordneter'}</span>
								<span>
									{question.status === 'failed'
										? 'Versand fehlgeschlagen'
										: question.status === 'sending'
											? 'Wird versendet'
											: 'Offen'}
								</span>
							</div>
							<h2 class="mt-2 text-lg font-bold text-black dark:text-white">{question.subject}</h2>
							<p class="mt-1 text-sm text-gray-700 dark:text-gray-200">
								{question.delegate_name} -> {question.recipient_name} ({question.recipient_email})
							</p>
						</div>

						<div class="flex shrink-0 gap-2">
							<button
								class="rounded-md bg-emerald-700 px-3 py-2 text-sm font-semibold text-white disabled:cursor-not-allowed disabled:opacity-60"
								disabled={activeQuestionId === question.id}
								onclick={() => reviewQuestion(question.id, 'approve')}
							>
								Freigeben
							</button>
							<button
								class="rounded-md bg-red-700 px-3 py-2 text-sm font-semibold text-white disabled:cursor-not-allowed disabled:opacity-60"
								disabled={activeQuestionId === question.id}
								onclick={() => reviewQuestion(question.id, 'reject')}
							>
								Ablehnen
							</button>
						</div>
					</div>

					<p class="mt-4 text-sm leading-6 whitespace-pre-wrap text-black dark:text-white">
						{question.body}
					</p>
				</article>
			{/each}
		</div>
	{/if}
</section>
