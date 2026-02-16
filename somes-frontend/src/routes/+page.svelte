<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { fade, fly, slide } from 'svelte/transition';
	import somesWithText from '$lib/assets/somes_with_text2.svg?raw';
	import { resolve } from '$app/paths';
	import type { PlatformItem, PlatformItemType, SomesEvent as Event, DialogEvent } from './types';
	import type { PageProps } from './$types';
	import checkmarkIcon from '$lib/assets/misc_icons/checkmark_small.svg?raw';
	import crossmarkIcon from '$lib/assets/misc_icons/crossmark_small.svg?raw';
	import { errorToNull } from '$lib/api/api';
	import CreateEventModal from '$lib/components/Events/CreateEventModal.svelte';
	import { Dialog } from 'bits-ui';
	import EditButton from '$lib/components/Events/EditButton.svelte';
	import { getUser } from '$lib/api/authed';

	let { data }: PageProps = $props();

	interface Member {
		name: string;
		role: string;
		subRole?: string;
		image?: string;
	}

	let nextPlenaryDate: Date | null = $derived.by(() => {
		const date = errorToNull(data.nextPlenarDate);

		if (!date) return null;

		return new Date(date.date_and_time);
	});

	let developmentProgress = $state(47); // 47% Progress

	const rawPlatformItems: PlatformItem[] = $derived(data.platformItems);

	let eventsData: DialogEvent[] = $state(
		$state.snapshot(data.somesEvents).map((event) => {
			return { event, dialogOpen: false, hidden: false };
		})
	);

	// 4. Board Members
	const boardMembers: Member[] = [
		{
			name: 'Daniel Atzwanger',
			role: 'Obperson',
			subRole: 'Kommunikation, Organisation',
			image: ''
		},
		{
			name: 'Clemens Bauer',
			role: 'Obperson',
			subRole: 'Moderation, Öffentlichkeitsarbeit',
			image: ''
		},
		{
			name: 'Florian Nagy',
			role: 'Obperson',
			subRole: 'Plattformentwicklung, technische Leitung',
			image: ''
		},
		{ name: 'Martin Zambal', role: 'Kassier', subRole: 'Finanzen, Förderanträge', image: '' }
	];

	const devTeamMembers: Member[] = [
		{ name: 'Fabian Cemerka', role: 'Fullstack Developer', image: '' },
		{ name: 'Christoph Handschuh', role: 'Frontend & KI Developer', image: '' },
		{ name: 'Florian Nagy', role: 'Data Scraping, Fullstack & KI Developer', image: '' },
		{ name: 'Jakob Pronebner', role: 'Mobile Developer', image: '' }
	];

	// --- STATE MANAGEMENT (RUNES) ---

	// Platform Widget Logic
	let selectedPlatformType: PlatformItemType = $state('vote');
	let platformTickerIndex = $state(0);
	let tickerInterval: any;

	// Filtered Platform Items
	let currentPlatformItems = $derived(
		rawPlatformItems.filter((i) => i.type === selectedPlatformType)
	);

	let currentTickerItem = $derived(
		currentPlatformItems.length > 0
			? currentPlatformItems[platformTickerIndex % currentPlatformItems.length]
			: null
	);

	// Event Logic (Past vs Upcoming)
	const today = new Date(); // Use current date
	// Mocking 'today' to be before the last event for demonstration if needed,
	// but strictly using system time as requested:

	let upcomingEvents = $derived(
		eventsData
			.filter((e) => new Date(e.event.event_date) >= today)
			.sort(
				(a, b) => new Date(a.event.event_date).getTime() - new Date(b.event.event_date).getTime()
			)
	);
	let pastEvents = $derived(
		eventsData
			.filter((e) => new Date(e.event.event_date) < today)
			.sort(
				(a, b) => new Date(b.event.event_date).getTime() - new Date(a.event.event_date).getTime()
			)
	); // Newest past first

	let dialogOpen = $state(false);

	// --- EFFECTS & FUNCTIONS ---

	function startTicker() {
		stopTicker();
		tickerInterval = setInterval(() => {
			platformTickerIndex++;
		}, 6000); // 6 seconds rotation
	}

	function stopTicker() {
		if (tickerInterval) clearInterval(tickerInterval);
	}

	function setPlatformType(type: PlatformItemType) {
		selectedPlatformType = type;
		platformTickerIndex = 0;
		startTicker(); // Restart timer on manual switch
	}

	function formatDate(dateStr: string) {
		const d = new Date(dateStr);
		return d.toLocaleDateString('de-AT', { day: '2-digit', month: '2-digit', year: 'numeric' });
	}

	let isAdmin = $state(false);

	onMount(async () => {
		startTicker();

		const user = errorToNull(await getUser());
		if (user) {
			isAdmin = user.is_admin;
		}
	});

	onDestroy(() => {
		stopTicker();
	});
</script>

<svelte:head>
	<title>Somes</title>
	<meta name="description" content="Verlinkung zur Plattform und Informationen über den Verein" />
</svelte:head>

<div
	class="text-base-font-color font-base dark:bg-surface-950 min-h-screen pb-20 dark:text-surface-50"
>
	<!-- NAVIGATION / HEADER -->
	<header
		class="sticky top-0 z-10 border-b border-surface-200 bg-surface-50/90 shadow-sm backdrop-blur-md dark:border-surface-700 dark:bg-surface-900/90"
	>
		<div class="mx-auto flex h-16 max-w-7xl items-center justify-between px-4 sm:px-6 lg:px-8">
			<nav class="hidden gap-6 text-sm font-medium text-surface-600 md:flex dark:text-surface-300">
				<a
					href={resolve('/home')}
					class="transition-colors hover:text-primary-600 dark:hover:text-primary-400">Plattform</a
				>
				<a
					href="{resolve('/')}#events"
					class="transition-colors hover:text-primary-600 dark:hover:text-primary-400">Events</a
				>
				<a
					href="{resolve('/')}#team"
					class="transition-colors hover:text-primary-600 dark:hover:text-primary-400">Team</a
				>
				<a
					href="{resolve('/')}#join"
					class="transition-colors hover:text-primary-600 dark:hover:text-primary-400">Mitmachen</a
				>
			</nav>
			<a
				href="https://forms.gle/mVx8K2cm3TPx8CJP9"
				target="_blank"
				class="rounded-container bg-primary-600 px-4 py-2 text-sm font-bold text-white shadow-md transition-all hover:bg-primary-700"
			>
				Mitglied werden
			</a>
		</div>
	</header>

	<!-- HERO SECTION -->
	<section class="relative overflow-hidden pt-20 pb-32">
		<!-- Background Elements -->
		<div
			class="absolute top-[-10%] right-[-5%] -z-10 h-[500px] w-[500px] rounded-full bg-secondary-200/40 blur-3xl dark:bg-secondary-900/20"
		></div>
		<div
			class="absolute bottom-[-10%] left-[-10%] -z-10 h-[600px] w-[600px] rounded-full bg-tertiary-200/60 blur-3xl dark:bg-tertiary-900/20"
		></div>

		<div class="flex flex-wrap-reverse items-center justify-around gap-5">
			<div class="flex flex-col items-center justify-center text-center">
				<div class="w-80 fill-current stroke-current md:w-110 lg:w-150">
					{@html somesWithText}
				</div>
				<div
					class="mt-4 mb-4 inline-block rounded-full border border-tertiary-300 bg-tertiary-200 px-3 py-1 text-sm font-bold tracking-wider text-tertiary-900 uppercase md:text-base dark:border-tertiary-700 dark:bg-tertiary-900/50 dark:text-tertiary-100"
				>
					Verein für Demokratie und Transparenz
				</div>
			</div>
			<div class="max-w-5xl space-y-8 px-4 text-center">
				<h1
					class="font-heading text-5xl leading-tight font-extrabold text-primary-900 md:text-7xl dark:text-primary-50"
				>
					Demokratie <span
						class="bg-linear-to-r from-secondary-700 to-secondary-500 bg-clip-text text-transparent dark:from-secondary-400"
						>verstehen.</span
					><br />
					Zukunft
					<span
						class="bg-linear-to-r from-primary-500 to-tertiary-700 bg-clip-text text-transparent dark:from-tertiary-500"
						>gestalten.</span
					>
				</h1>
				<p
					class="mx-auto max-w-2xl text-lg leading-relaxed text-surface-600 md:text-xl dark:text-surface-300"
				>
					Wir bringen Demokratie direkt zu dir. Mit der somes Plattform und interaktiven Events
					machen wir Politik transparent und einfach greifbar.
				</p>

				<div class="flex flex-wrap justify-center gap-4 pt-4">
					<a
						href={resolve('/home')}
						class="flex items-center gap-2 rounded-full bg-surface-900 px-6 py-3 text-lg font-bold text-white shadow-xl transition-transform hover:scale-105 dark:bg-surface-50 dark:text-surface-900"
					>
						Zur Somes Plattform
						<svg
							xmlns="http://www.w3.org/2000/svg"
							class="h-5 w-5"
							fill="none"
							viewBox="0 0 24 24"
							stroke="currentColor"
							><path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M17 8l4 4m0 0l-4 4m4-4H3"
							/></svg
						>
					</a>
					<a
						href="{resolve('/')}#events"
						class="rounded-full border border-surface-200 bg-white px-6 py-3 text-lg font-bold text-surface-900 shadow-sm transition-colors hover:bg-surface-50 dark:border-surface-600 dark:bg-surface-800 dark:text-surface-50 dark:hover:bg-surface-700"
					>
						Unsere Events
					</a>
				</div>
			</div>
		</div>
	</section>

	<!-- PLATFORM WIDGET SECTION -->
	<section id="platform" class="relative bg-surface-900 py-16 text-white">
		<div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
			<div class="grid grid-cols-1 gap-8 lg:grid-cols-3">
				<!-- Left: Intro & Progress -->
				<div class="space-y-6 lg:col-span-1">
					<h2 class="font-heading text-3xl font-bold text-secondary-300">Somes Plattform</h2>
					<p class="leading-relaxed text-surface-300">
						Unsere Open-Source Web-App macht das Parlament gläsern. Analysiere Abstimmungen, lies
						KI-Zusammenfassungen von Gesetzen und verfolge, was deine Abgeordneten tun.
					</p>

					<!-- Development Progress -->
					<div class="rounded-xl border border-surface-700 bg-surface-800 p-5 shadow-inner">
						<div class="mb-2 flex items-end justify-between">
							<span class="text-sm font-medium text-surface-300">Entwicklung v1.0 - 31.04.2026</span
							>
							<span class="text-2xl font-bold text-primary-300">{developmentProgress}%</span>
						</div>
						<div class="h-2.5 w-full overflow-hidden rounded-full bg-surface-700">
							<div
								class="h-2.5 rounded-full bg-linear-to-r from-primary-400 to-secondary-400 transition-all duration-1000 ease-out"
								style="width: {developmentProgress}%"
							></div>
						</div>
						<p class="mt-2 text-xs text-surface-400"></p>
					</div>

					{#if nextPlenaryDate}
						<div class="rounded-xl border border-primary-700/50 bg-primary-900/50 p-4">
							<span class="mb-1 block text-xs tracking-wide text-primary-300 uppercase"
								>Nächste Nationalratssitzung</span
							>
							<span class="text-xl font-bold text-white">
								{nextPlenaryDate.toLocaleDateString('de-AT', {
									weekday: 'long',
									day: 'numeric',
									month: 'long'
								})}
							</span>
						</div>
					{/if}
				</div>

				<!-- Right: The "Live" Ticker Widget -->
				<div class="lg:col-span-2">
					<div
						class="flex h-full min-h-[400px] flex-col overflow-hidden rounded-2xl border border-surface-700 bg-surface-800 shadow-2xl"
					>
						<!-- Widget Header / Controls -->
						<div
							class="bg-surface-850 flex flex-wrap items-center justify-between gap-2 border-b border-surface-700 p-4"
						>
							<div class="bg-surface-850 border-t border-surface-700 p-3 text-center">
								<a
									href="{resolve('/history')}/votes"
									class="group flex items-center justify-center gap-1 text-xs text-primary-400 hover:text-primary-300"
								>
									Alle Details auf somes.at ansehen
									<span class="transition-transform group-hover:translate-x-1">→</span>
								</a>
							</div>
							<!-- <div class="flex gap-2">
								<div class="w-3 h-3 rounded-full bg-error-500"></div>
								<div class="w-3 h-3 rounded-full bg-warning-500"></div>
								<div class="w-3 h-3 rounded-full bg-success-500"></div>
							</div> -->
							<div class="flex gap-1 rounded-lg bg-surface-900 p-1">
								<button
									class="rounded-md px-3 py-1 text-sm transition-colors {selectedPlatformType ===
									'vote'
										? 'bg-secondary-600 text-white shadow-md'
										: 'text-surface-400 hover:text-white'}"
									onclick={() => setPlatformType('vote')}
								>
									Abstimmungen
								</button>
								<button
									class="rounded-md px-3 py-1 text-sm transition-colors {selectedPlatformType ===
									'proposal'
										? 'bg-primary-600 text-white shadow-md'
										: 'text-surface-400 hover:text-white'}"
									onclick={() => setPlatformType('proposal')}
								>
									Entwürfe
								</button>
								<button
									class="rounded-md px-3 py-1 text-sm transition-colors {selectedPlatformType ===
									'decree'
										? 'bg-tertiary-600 text-white shadow-md'
										: 'text-surface-400 hover:text-white'}"
									onclick={() => setPlatformType('decree')}
								>
									Verordnungen
								</button>
							</div>
						</div>

						<!-- Widget Content Area -->
						<div
							class="relative flex flex-1 justify-center bg-linear-to-br from-surface-800 to-surface-900 p-8"
						>
							{#key currentTickerItem}
								<div
									in:fly={{ y: 20, duration: 400, delay: 100, opacity: 0 }}
									out:fade={{ duration: 200 }}
									class="absolute w-full max-w-lg text-center"
								>
									{#if currentTickerItem}
										<!-- Badge -->
										<span
											class="mb-3 inline-flex items-center rounded-full px-3 py-1 text-xs font-bold tracking-wider uppercase
											{currentTickerItem.type === 'vote'
												? 'border border-secondary-500/30 bg-secondary-500/20 text-secondary-300'
												: currentTickerItem.type === 'proposal'
													? 'border border-primary-500/30 bg-primary-500/20 text-primary-300'
													: 'border border-tertiary-500/30 bg-tertiary-500/20 text-tertiary-300'}"
										>
											{currentTickerItem.type === 'vote'
												? 'Abstimmung'
												: currentTickerItem.type === 'proposal'
													? 'Ministerialentwurf'
													: 'Verordnung'}
										</span>

										<!-- Title -->
										<h3
											class="font-heading mb-4 text-lg leading-tight font-semibold text-white md:text-2xl"
										>
											{currentTickerItem.title}
										</h3>

										<!-- Meta Info -->
										<div class="flex justify-center gap-4 text-sm font-light text-surface-400">
											<span class="">{formatDate(currentTickerItem.date)}</span>
											{#if currentTickerItem.status}
												<span class="mx-2">•</span>
												{#if currentTickerItem.status === 'accepted'}
													<span
														class="inline-block stroke-green-600 align-middle dark:stroke-green-500"
														style="width:40px; height:40px">{@html checkmarkIcon}</span
													>
												{:else if currentTickerItem.status === 'rejected'}
													<span class="inline-block align-middle" style="width:40px; height:40px;"
														>{@html crossmarkIcon}</span
													>
												{:else}
													<span class="text-warning-400">In Bearbeitung</span>
												{/if}
												<!-- <span
													class={currentTickerItem.status === 'accepted'
														? 'text-success-400'
														: currentTickerItem.status === 'rejected'
															? 'text-error-400'
															: 'text-warning-400'}
												>
													{currentTickerItem.status === 'accepted'
														? 'Angenommen'
														: currentTickerItem.status === 'rejected'
															? 'Abgelehnt'
															: 'In Bearbeitung'}
												</span> -->
											{/if}
										</div>

										<div
											class="absolute -bottom-16 left-1/2 h-1 w-full -translate-x-1/2 overflow-hidden rounded-full bg-surface-700"
										>
											<div class="h-full animate-progress-indeterminate bg-white/20"></div>
										</div>
									{:else}
										<p class="text-surface-500">Keine Daten verfügbar.</p>
									{/if}
								</div>
							{/key}
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- EVENTS SECTION -->
	<section id="events" class="mx-auto max-w-6xl px-4 py-20 sm:px-6">
		<div class="mb-16 text-center">
			<h2 class="mb-2 text-sm font-bold tracking-widest text-primary-600 uppercase">
				Engagiere dich
			</h2>
			<h3 class="font-heading text-4xl font-bold text-surface-900 dark:text-surface-50">Events</h3>
		</div>

		<!-- Upcoming Events -->
		{#if upcomingEvents.length > 0 || isAdmin}
			<div class="mb-16">
				<div class="mb-6 flex items-center gap-4">
					<h4 class="text-2xl font-bold text-primary-800 dark:text-primary-200">
						Kommende Termine
					</h4>
					<div class="h-px flex-1 bg-primary-200 dark:bg-primary-800"></div>
				</div>

				<div class="grid gap-6 md:grid-cols-2">
					{#each upcomingEvents as dialogEvent, i (dialogEvent.event.id)}
						<div
							class="{dialogEvent.hidden
								? 'hidden'
								: ''} group relative overflow-hidden rounded-2xl border border-primary-100 bg-white p-6 shadow-lg transition-all hover:-translate-y-1 hover:border-secondary-300 dark:border-surface-700 dark:bg-surface-800"
						>
							<div class="absolute top-0 left-0 h-full w-2 bg-secondary-500"></div>

							<!-- Header Row -->
							<div class="mb-4 flex items-start justify-between pl-4">
								<div
									class="rounded-lg border border-secondary-100 bg-secondary-50 px-3 py-1 text-center font-bold text-secondary-800 dark:border-secondary-700 dark:bg-secondary-900 dark:text-secondary-100"
								>
									<div class="text-xs uppercase">
										{new Date(dialogEvent.event.event_date).toLocaleDateString('de-AT', {
											month: 'short'
										})}
									</div>
									<div class="text-xl">{new Date(dialogEvent.event.event_date).getDate()}</div>
								</div>

								<!-- Location & Edit Button Wrapper -->
								<div class="flex flex-col items-end gap-2">
									<!-- Edit Button -->
									{#if isAdmin}
										<Dialog.Root bind:open={upcomingEvents[i].dialogOpen}>
											<Dialog.Trigger>
												<EditButton />
											</Dialog.Trigger>
											<CreateEventModal
												bind:dialogOpen={upcomingEvents[i].dialogOpen}
												bind:event={upcomingEvents[i].event}
												bind:events={eventsData}
											/>
										</Dialog.Root>
									{/if}
									<span
										class="flex items-center gap-1 text-sm font-medium text-surface-500 dark:text-surface-400"
									>
										<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"
											><path
												stroke-linecap="round"
												stroke-linejoin="round"
												stroke-width="2"
												d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z"
											/><path
												stroke-linecap="round"
												stroke-linejoin="round"
												stroke-width="2"
												d="M15 11a3 3 0 11-6 0 3 3 0 016 0z"
											/></svg
										>
										{dialogEvent.event.location}
									</span>
								</div>
							</div>

							<div class="pl-4">
								<h5
									class="mb-2 text-xl font-bold text-surface-900 transition-colors group-hover:text-secondary-600 dark:text-surface-100 dark:group-hover:text-secondary-400"
								>
									{dialogEvent.event.title}
								</h5>
								<p class="mb-4 line-clamp-3 text-sm text-surface-600 dark:text-surface-300">
									{dialogEvent.event.description}
								</p>

								{#if dialogEvent.event.requires_membership}
									<div class="mb-2 text-xs font-medium text-primary-500">
										🔒 Mitgliedschaft erforderlich
									</div>
								{:else}
									<div class="mb-2 text-xs font-medium text-success-500">
										✅ Offen für alle - einfach kommen!
									</div>
								{/if}
							</div>
						</div>
					{/each}

					{#if isAdmin}
						<Dialog.Root bind:open={dialogOpen}>
							<Dialog.Trigger>
								<button
									class="flex h-full min-h-[300px] w-full items-center justify-center rounded-2xl border-2 border-dashed border-surface-300 bg-surface-50 text-6xl text-surface-300 transition-all hover:cursor-pointer hover:border-primary-400 hover:bg-primary-50 hover:text-primary-500 dark:border-surface-700 dark:bg-surface-800/50 dark:hover:border-primary-700 dark:hover:bg-surface-800"
									aria-label="Create new event"
								>
									+
								</button>
							</Dialog.Trigger>
							<CreateEventModal bind:dialogOpen bind:events={eventsData} />
						</Dialog.Root>
					{/if}
				</div>
			</div>
		{/if}

		<!-- Past Events -->
		<div>
			<div class="mb-6 flex items-center gap-4">
				<h4 class="text-2xl font-bold text-surface-500 dark:text-surface-400">Vergangene Events</h4>
				<div class="h-px flex-1 bg-surface-200 dark:bg-surface-700"></div>
			</div>

			<div class="space-y-4">
				{#each pastEvents as dialogEvent, i (dialogEvent.event.id)}
					<div
						class="{dialogEvent.hidden
							? 'hidden'
							: ''} flex flex-col gap-4 rounded-xl border border-surface-200/50 bg-surface-50 p-5 transition-colors hover:bg-white md:flex-row dark:border-surface-700 dark:bg-surface-800/50 dark:hover:bg-surface-800"
					>
						<div
							class="flex shrink-0 items-center gap-2 text-surface-400 md:w-24 md:flex-col md:justify-center md:gap-0 dark:text-surface-400"
						>
							<span class="font-bold">{formatDate(dialogEvent.event.event_date)}</span>
						</div>
						<div class="flex-1">
							<div class="flex justify-between">
								<h5 class="text-lg font-bold text-surface-700 dark:text-surface-100">
									{dialogEvent.event.title}
								</h5>
								{#if isAdmin}
									<Dialog.Root bind:open={pastEvents[i].dialogOpen}>
										<Dialog.Trigger>
											<EditButton />
										</Dialog.Trigger>
										<CreateEventModal
											bind:dialogOpen={pastEvents[i].dialogOpen}
											bind:event={pastEvents[i].event}
											bind:events={eventsData}
										/>
									</Dialog.Root>
								{/if}
							</div>
							<p class="mt-1 text-sm text-surface-500 dark:text-surface-400">
								{dialogEvent.event.description}
							</p>
							<div class="flex flex-wrap justify-between">
								<div class="mt-2 text-xs font-medium text-primary-500">
									📍 {dialogEvent.event.location}
								</div>
								{#if dialogEvent.event.requires_membership}
									<div class="mt-2 text-xs font-medium text-primary-500">
										🔒 Mitgliedschaft erforderlich
									</div>
								{/if}
								{#if dialogEvent.event.requires_registration}
									<div class="mt-2 text-xs font-medium text-primary-500">
										📝 Anmeldung erforderlich
									</div>
								{/if}
							</div>
						</div>
					</div>
				{/each}
			</div>
		</div>
	</section>

	<!-- MEMBERSHIP CTA -->
	<section id="join" class="px-4 py-4">
		<div
			class="relative mx-auto max-w-4xl overflow-hidden rounded-3xl bg-primary-800 p-8 text-center text-white shadow-2xl md:p-12"
		>
			<!-- Decorative Circles -->
			<div
				class="absolute top-0 right-0 h-64 w-64 translate-x-1/2 -translate-y-1/2 rounded-full bg-primary-600 opacity-50 blur-3xl"
			></div>
			<div
				class="absolute bottom-0 left-0 h-64 w-64 -translate-x-1/2 translate-y-1/2 rounded-full bg-secondary-600 opacity-50 blur-3xl"
			></div>

			<div class="relative z-5">
				<h2 class="font-heading mb-6 text-3xl font-bold md:text-5xl">Werde Teil von Somes</h2>
				<p class="mx-auto mb-8 max-w-2xl text-lg text-primary-100">
					Unterstütze uns dabei, den einfachen Zugang zu politischen Inhalten zu revolutionieren.
					Als Mitglied bei
					<span class="font-bold italic"
						>somes - Verein für Demokratie und politische Transparenz</span
					>
					trägst du aktiv zur Gestaltung unserer Zukunft bei.
				</p>

				<div class="flex flex-col justify-center gap-4 sm:flex-row">
					<a
						href="https://forms.gle/mVx8K2cm3TPx8CJP9"
						target="_blank"
						class="transform rounded-xl bg-secondary-500 px-8 py-4 text-lg font-bold text-white shadow-lg transition-all hover:-translate-y-0.5 hover:bg-secondary-400 hover:shadow-secondary-500/30"
					>
						Mitglied werden
					</a>
					<a
						href="mailto:somes.austria@gmail.com"
						class="rounded-xl border border-primary-500 bg-primary-700 px-8 py-4 text-lg font-bold text-white transition-all hover:bg-primary-600"
					>
						Kontakt aufnehmen
					</a>
				</div>
			</div>
		</div>
	</section>

	<!-- TEAM SECTION -->
	<section id="team" class="mt-24 bg-tertiary-100 py-10 dark:bg-surface-900">
		<div class="mx-auto max-w-7xl px-4">
			<h2
				class="font-heading mb-12 text-center text-3xl font-bold text-primary-900 dark:text-primary-100"
			>
				Vorstand
			</h2>

			<div class="flex flex-wrap justify-center gap-8 md:gap-12">
				{#each boardMembers as member (member.name)}
					<div class="group flex flex-col items-center">
						<div
							class="relative mb-4 h-32 w-32 overflow-hidden rounded-full border-4 border-white bg-linear-to-br from-tertiary-300 to-tertiary-400 shadow-lg dark:border-surface-800"
						>
							{#if member.image}
								<img src={member.image} alt={member.name} class="h-full w-full object-cover" />
							{:else}
								<div
									class="flex h-full w-full items-center justify-center text-2xl font-bold text-tertiary-600 dark:text-tertiary-800"
								>
									{member.name.charAt(0)}
								</div>
							{/if}
						</div>
						<h3 class="text-lg font-bold text-primary-900 dark:text-primary-100">{member.name}</h3>
						{#if member.role}
							<span
								class="text-sm font-medium tracking-wide text-secondary-600 uppercase dark:text-secondary-400"
								>{member.role}</span
							>
						{/if}
						{#if member.subRole}
							<span class="text-xs text-surface-500 dark:text-surface-400">{member.subRole}</span>
						{/if}
					</div>
				{/each}
			</div>
		</div>
	</section>
	<section id="team" class="bg-tertiary-100 py-10 dark:bg-surface-900">
		<div class="mx-auto max-w-7xl px-4">
			<h2
				class="font-heading mb-12 text-center text-3xl font-bold text-primary-900 dark:text-primary-100"
			>
				Netidee Entwicklungsteam
			</h2>

			<div class="flex flex-wrap justify-center gap-8 md:gap-12">
				{#each devTeamMembers as member (member.name)}
					<div class="group flex flex-col items-center">
						<div
							class="relative mb-4 h-32 w-32 overflow-hidden rounded-full border-4 border-white bg-linear-to-br from-tertiary-300 to-tertiary-400 shadow-lg dark:border-surface-800"
						>
							{#if member.image}
								<img src={member.image} alt={member.name} class="h-full w-full object-cover" />
							{:else}
								<div
									class="flex h-full w-full items-center justify-center text-2xl font-bold text-tertiary-600 dark:text-tertiary-800"
								>
									{member.name.charAt(0)}
								</div>
							{/if}
						</div>
						<h3 class="text-lg font-bold text-primary-900 dark:text-primary-100">{member.name}</h3>
						{#if member.role}
							<span
								class="text-sm font-medium tracking-wide text-secondary-600 uppercase dark:text-secondary-400"
								>{member.role}</span
							>
						{/if}
						{#if member.subRole}
							<span class="text-xs text-surface-500 dark:text-surface-400">{member.subRole}</span>
						{/if}
					</div>
				{/each}
			</div>
		</div>
	</section>
</div>

<style>
	/* Custom Progress Bar Animation */
	@keyframes progress-indeterminate {
		0% {
			transform: translateX(-100%);
		}
		50% {
			transform: translateX(0%);
		}
		100% {
			transform: translateX(100%);
		}
	}

	.animate-progress-indeterminate {
		animation: progress-indeterminate 12.4s linear;
	}
</style>
