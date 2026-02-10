<script lang="ts">
	import { Dialog } from 'bits-ui';
	import { createEvent, deleteEvent, updateEvent, type DialogEvent, type SomesEvent } from '../../../routes/types';
	import { isHasError } from '$lib/api/api';
	import { browser } from '$app/environment';

	interface Props {
		event?: SomesEvent;
		events?: DialogEvent[];
		dialogOpen: boolean;
	}

	let {
		dialogOpen = $bindable(false),
		event = $bindable({
			id: null,
			title: '',
			location: '',
			event_date: '',
			start_time: '',
			description: '',
			requires_membership: false,
			requires_registration: false,
			image: null,
		}),
		events = $bindable(),
	}: Props = $props();

	let formData: SomesEvent = $state($state.snapshot(event));

	let isEditing = $derived(formData.id !== null);

	async function handleSubmit(e: Event) {
		e.preventDefault();
		const formDataSnapshot = $state.snapshot(formData);
		if (isEditing) {
			const status = await updateEvent(formData);
			if (isHasError(status)) {
				console.error(status)
			}
			event = formDataSnapshot;
		} else {
			const status = await createEvent(formData);
			if (isHasError(status)) {
				console.error(status);
			} else {
				if (events) {
					formDataSnapshot.id = status.id;
					events.push({event: formDataSnapshot, dialogOpen: false})
				}
			}

		}
		dialogOpen = false;
	}

	async function handleDelete() {
		if (isEditing && formData.id) {
			const status = await deleteEvent(formData.id);
			if (isHasError(status)) {
				console.error(status);
			} else if (events) {
				const index = events.findIndex(e => e.event.id === formData.id);
				if (index !== -1) {
					events.splice(index, 1);
				}
			}
			dialogOpen = false;
		}
	}
</script>

<Dialog.Portal>
	<Dialog.Overlay
		class="fixed inset-0 z-50 bg-black/60 backdrop-blur-sm data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:animate-in data-[state=open]:fade-in-0"
	/>

	<Dialog.Content
		class="fixed top-[50%] left-[50%] z-50 flex max-h-[90vh] w-full max-w-2xl translate-x-[-50%] translate-y-[-50%] flex-col overflow-hidden rounded-2xl bg-white shadow-2xl outline-none data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95 data-[state=open]:animate-in data-[state=open]:fade-in-0 data-[state=open]:zoom-in-95 dark:bg-surface-900"
	>
		<!-- HEADER -->
		<div
			class="flex items-center justify-between border-b border-surface-100 bg-primary-50/50 px-6 py-4 dark:border-surface-800 dark:bg-surface-800/50"
		>
			<div>
				<Dialog.Title class="text-xl font-bold text-primary-800 dark:text-primary-100">
					{isEditing ? 'Event bearbeiten' : 'Neues Event erstellen'}
				</Dialog.Title>
				<Dialog.Description class="text-sm text-surface-500 dark:text-surface-400">
					{isEditing
						? 'Aktualisiere die Details für dieses Event.'
						: 'Füge ein neues Event zur Timeline hinzu.'}
				</Dialog.Description>
			</div>
			<Dialog.Close
				class="rounded-full p-2 text-surface-500 transition-colors hover:bg-surface-200 hover:text-surface-900 dark:hover:bg-surface-700 dark:hover:text-surface-100"
			>
				<svg
					xmlns="http://www.w3.org/2000/svg"
					width="20"
					height="20"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
					stroke-linecap="round"
					stroke-linejoin="round"
				>
					<path d="M18 6 6 18" />
					<path d="m6 6 12 12" />
				</svg>
			</Dialog.Close>
		</div>

		<!-- BODY (Scrollable Form) -->
		<div class="flex-1 overflow-y-auto p-6">
			<form id="event-form" onsubmit={handleSubmit} class="space-y-5">
				<!-- Title Input -->
				<div class="space-y-1.5">
					<label
						for="title"
						class="text-sm font-bold tracking-wide text-surface-700 uppercase dark:text-surface-300"
						>Titel</label
					>
					<input
						type="text"
						id="title"
						bind:value={formData.title}
						placeholder="z.B. Podiumsdiskussion"
						class="w-full rounded-lg border border-surface-300 bg-surface-50 px-4 py-2.5 text-surface-900 placeholder:text-surface-400 focus:border-secondary-500 focus:ring-2 focus:ring-secondary-500/20 focus:outline-none dark:border-surface-700 dark:bg-surface-800 dark:text-surface-100"
						required
					/>
				</div>

				<!-- Location Input -->
				<div class="space-y-1.5">
					<label
						for="location"
						class="text-sm font-bold tracking-wide text-surface-700 uppercase dark:text-surface-300"
						>Ort</label
					>
					<div class="relative">
						<span class="absolute top-1/2 left-3 -translate-y-1/2 text-surface-400">
							<svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z"
								/>
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M15 11a3 3 0 11-6 0 3 3 0 016 0z"
								/>
							</svg>
						</span>
						<input
							type="text"
							id="location"
							bind:value={formData.location}
							placeholder="Ort eingeben..."
							class="w-full rounded-lg border border-surface-300 bg-surface-50 py-2.5 pr-4 pl-10 text-surface-900 placeholder:text-surface-400 focus:border-secondary-500 focus:ring-2 focus:ring-secondary-500/20 focus:outline-none dark:border-surface-700 dark:bg-surface-800 dark:text-surface-100"
							required
						/>
					</div>
				</div>

				<!-- Date & Time Row -->
				<div class="grid grid-cols-2 gap-4">
					<div class="space-y-1.5">
						<label
							for="date"
							class="text-sm font-bold tracking-wide text-surface-700 uppercase dark:text-surface-300"
							>Datum</label
						>
						<input
							type="date"
							id="date"
							bind:value={formData.event_date}
							class="w-full rounded-lg border border-surface-300 bg-surface-50 px-4 py-2.5 text-surface-900 focus:border-secondary-500 focus:ring-2 focus:ring-secondary-500/20 focus:outline-none dark:border-surface-700 dark:bg-surface-800 dark:text-surface-100"
							required
						/>
					</div>
					<div class="space-y-1.5">
						<label
							for="time"
							class="text-sm font-bold tracking-wide text-surface-700 uppercase dark:text-surface-300"
							>Uhrzeit</label
						>
						<input
							type="time"
							id="time"
							bind:value={formData.start_time}
							class="w-full rounded-lg border border-surface-300 bg-surface-50 px-4 py-2.5 text-surface-900 focus:border-secondary-500 focus:ring-2 focus:ring-secondary-500/20 focus:outline-none dark:border-surface-700 dark:bg-surface-800 dark:text-surface-100"
							required
						/>
					</div>
				</div>

				<!-- Description -->
				<div class="space-y-1.5">
					<label
						for="description"
						class="text-sm font-bold tracking-wide text-surface-700 uppercase dark:text-surface-300"
						>Beschreibung</label
					>
					<textarea
						id="description"
						rows="4"
						bind:value={formData.description}
						class="w-full resize-none rounded-lg border border-surface-300 bg-surface-50 px-4 py-2.5 text-surface-900 placeholder:text-surface-400 focus:border-secondary-500 focus:ring-2 focus:ring-secondary-500/20 focus:outline-none dark:border-surface-700 dark:bg-surface-800 dark:text-surface-100"
						required
					></textarea>
				</div>

				<!-- Checkboxes -->
				<div class="grid gap-4 sm:grid-cols-2">
					<label
						class="group relative flex cursor-pointer items-start gap-3 rounded-lg border border-surface-200 bg-white p-4 transition-all hover:border-primary-300 dark:border-surface-700 dark:bg-surface-800 dark:hover:border-primary-700"
					>
						<div class="flex h-5 items-center">
							<input
								type="checkbox"
								bind:checked={formData.requires_membership}
								class="h-4 w-4 rounded border-surface-300 text-secondary-600 focus:ring-secondary-500 dark:border-surface-600"
							/>
						</div>
						<div class="text-sm">
							<span class="font-bold text-surface-900 dark:text-surface-100"> Mitgliedschaft </span>
							<p class="text-xs text-surface-500 dark:text-surface-400">Nur für Mitglieder.</p>
						</div>
					</label>

					<label
						class="group relative flex cursor-pointer items-start gap-3 rounded-lg border border-surface-200 bg-white p-4 transition-all hover:border-primary-300 dark:border-surface-700 dark:bg-surface-800 dark:hover:border-primary-700"
					>
						<div class="flex h-5 items-center">
							<input
								type="checkbox"
								bind:checked={formData.requires_registration}
								class="h-4 w-4 rounded border-surface-300 text-secondary-600 focus:ring-secondary-500 dark:border-surface-600"
							/>
						</div>
						<div class="text-sm">
							<span class="font-bold text-surface-900 dark:text-surface-100"> Anmeldung </span>
							<p class="text-xs text-surface-500 dark:text-surface-400">Voranmeldung nötig.</p>
						</div>
					</label>
				</div>
			</form>
		</div>

		<!-- FOOTER -->
		<div
			class="flex items-center justify-between border-t border-surface-100 bg-surface-50 px-6 py-4 dark:border-surface-800 dark:bg-surface-800"
		>
			<!-- Delete Button (Only visible in Edit Mode) -->
			<div>
				{#if isEditing}
					<button
						type="button"
						onclick={handleDelete}
						class="text-sm font-medium text-red-600 hover:text-red-700 hover:underline dark:text-red-400 dark:hover:text-red-300"
					>
						Löschen
					</button>
				{/if}
			</div>

			<div class="flex gap-3">
				<Dialog.Close
					class="rounded-lg border border-surface-300 px-5 py-2.5 text-sm font-bold text-surface-700 transition-colors hover:bg-surface-100 focus:ring-2 focus:ring-surface-400 focus:ring-offset-2 focus:outline-none dark:border-surface-600 dark:text-surface-200 dark:hover:bg-surface-700"
				>
					Abbrechen
				</Dialog.Close>
				<button
					type="submit"
					form="event-form"
					class="rounded-lg bg-secondary-600 px-5 py-2.5 text-sm font-bold text-white shadow-sm transition-all hover:-translate-y-0.5 hover:bg-secondary-500 focus:ring-2 focus:ring-secondary-500 focus:ring-offset-2 focus:outline-none"
				>
					{isEditing ? 'Speichern' : 'Erstellen'}
				</button>
			</div>
		</div>
	</Dialog.Content>
</Dialog.Portal>
