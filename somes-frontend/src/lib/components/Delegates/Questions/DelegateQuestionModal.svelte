<script lang="ts">
	import { onMount } from 'svelte';
	import { Dialog } from 'bits-ui';
	import { askDelegateQuestion } from '$lib/api/authed';
	import { delegate_question_recipient, isHasError } from '$lib/api/api';
	import ModalCloseButton from '$lib/components/UI/ModalCloseButton.svelte';
	import type { Delegate, DelegateQuestionRecipient } from '$lib/types';

	interface Props {
		delegate: Delegate;
	}

	let { delegate }: Props = $props();

	let subject = $state('');
	let body = $state('');
	let isSending = $state(false);
	let errorMessage = $state<string | null>(null);
	let wasSubmitted = $state(false);
	let recipient = $state<DelegateQuestionRecipient | null>(null);
	let recipientError = $state<string | null>(null);

	onMount(async () => {
		const result = await delegate_question_recipient(delegate.id);
		if (isHasError(result)) {
			recipientError = result.error;
			return;
		}
		recipient = result;
	});

	async function submitQuestion() {
		const trimmedSubject = subject.trim();
		const trimmedBody = body.trim();

		if (!trimmedSubject || !trimmedBody) {
			errorMessage = 'Bitte gib einen Betreff und deine Frage ein.';
			return;
		}

		isSending = true;
		errorMessage = null;

		const result = await askDelegateQuestion(delegate.id, {
			subject: trimmedSubject,
			body: trimmedBody
		});

		isSending = false;
		if (isHasError(result)) {
			errorMessage =
				result.error === 'No access token'
					? 'Bitte melde dich an, um eine Frage zu stellen.'
					: result.error;
			return;
		}

		wasSubmitted = true;
		subject = '';
		body = '';
	}
</script>

<div class="flex max-h-[85vh] flex-col overflow-y-auto bg-primary-100 dark:bg-gray-800">
	<div class="flex items-center justify-between bg-primary-300 px-5 py-4 dark:bg-primary-500">
		<div>
			<h2 class="text-lg font-bold text-black dark:text-white">Frage an {delegate.name}</h2>
			{#if recipient?.delivery === 'party'}
				<p class="mt-1 text-sm text-black/75 dark:text-white/80">
					Die Frage wird an den {recipient.recipient_name} gesendet, weil für diese Person keine direkte
					E-Mail-Adresse hinterlegt ist.
				</p>
			{:else if recipient}
				<p class="mt-1 text-sm text-black/75 dark:text-white/80">
					Deine Frage wird nach Freigabe als E-Mail direkt an den Abgeordneten gesendet.
				</p>
			{:else}
				<p class="mt-1 text-sm text-black/75 dark:text-white/80">Zustelladresse wird geprüft.</p>
			{/if}
		</div>
		<Dialog.Close>
			<ModalCloseButton />
		</Dialog.Close>
	</div>

	{#if wasSubmitted}
		<div class="p-5">
			<p
				class="rounded-md border border-emerald-600/40 bg-emerald-100 px-4 py-3 text-sm text-emerald-900 dark:bg-emerald-950 dark:text-emerald-100"
			>
				Deine Frage wurde erfolgreich zur Prüfung eingereicht. Nach der Freigabe wird sie per
				E-Mail weitergeleitet.
			</p>
			<div class="mt-5 flex justify-end">
				<Dialog.Close>
					<button class="rounded-md bg-primary-600 px-4 py-2 text-sm font-semibold text-white">
						Schliessen
					</button>
				</Dialog.Close>
			</div>
		</div>
	{:else}
		<form
			class="space-y-4 p-5"
			onsubmit={(event) => {
				event.preventDefault();
				submitQuestion();
			}}
		>
			{#if recipientError}
				<p
					class="rounded-md border border-red-600/40 bg-red-100 px-3 py-2 text-sm text-red-900 dark:bg-red-950 dark:text-red-100"
				>
					Für diese Person ist derzeit kein Empfänger für Fragen hinterlegt.
				</p>
			{/if}

			<label class="block">
				<span class="mb-1 block text-sm font-semibold text-black dark:text-white">Betreff</span>
				<input
					bind:value={subject}
					maxlength="255"
					placeholder="Worum geht es?"
					class="w-full rounded-md border border-gray-300 bg-white px-3 py-2 text-black outline-none focus:border-primary-600 focus:ring-2 focus:ring-primary-300 dark:border-gray-600 dark:bg-gray-900 dark:text-white"
				/>
			</label>

			<label class="block">
				<span class="mb-1 block text-sm font-semibold text-black dark:text-white">Deine Frage</span>
				<textarea
					bind:value={body}
					maxlength="10000"
					rows="7"
					placeholder="Formuliere deine Frage an {delegate.name}..."
					class="w-full resize-y rounded-md border border-gray-300 bg-white px-3 py-2 text-black outline-none focus:border-primary-600 focus:ring-2 focus:ring-primary-300 dark:border-gray-600 dark:bg-gray-900 dark:text-white"
				></textarea>
			</label>

			{#if errorMessage}
				<p
					class="rounded-md border border-red-600/40 bg-red-100 px-3 py-2 text-sm text-red-900 dark:bg-red-950 dark:text-red-100"
				>
					{errorMessage}
				</p>
			{/if}

			<div class="flex justify-end gap-3 pt-1">
				<Dialog.Close>
					<button
						type="button"
						class="rounded-md px-4 py-2 text-sm font-semibold text-black dark:text-white"
					>
						Abbrechen
					</button>
				</Dialog.Close>
				<button
					type="submit"
					disabled={isSending || recipient === null || recipientError !== null}
					class="rounded-md bg-primary-600 px-4 py-2 text-sm font-semibold text-white disabled:cursor-not-allowed disabled:opacity-60"
				>
					{isSending ? 'Wird eingereicht...' : 'Frage einreichen'}
				</button>
			</div>
		</form>
	{/if}
</div>
