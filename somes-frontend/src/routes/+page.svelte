<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { fade, fly, slide } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';
	import { resolve } from '$app/paths';

	// --- TYPE DEFINITIONS ---
	type PlatformItemType = 'vote' | 'proposal' | 'decree';

	interface PlatformItem {
		id: number;
		type: PlatformItemType;
		title: string;
		date: string;
		status?: 'accepted' | 'rejected' | 'pending';
	}

	interface Event {
		id: number;
		title: string;
		location: string;
		date: string; // ISO format YYYY-MM-DD
		time: string;
		description: string;
		image?: string;
		requiresMembership?: boolean;
		requiresRegistration?: boolean;
	}

	interface Member {
		name: string;
		role: string;
		subRole?: string;
		image?: string;
	}

	// --- CONFIGURATION / MOCK DATA (As requested in script tag) ---

	// 1. Internal Info / Next Plenary Date
	let nextPlenaryDate = $state(new Date('2026-02-15T09:00:00'));
	let developmentProgress = $state(30); // 30% Progress

	// 2. Platform Items (Mock API Data)
	const rawPlatformItems: PlatformItem[] = [
		{ id: 1, type: 'vote', title: 'Änderung des Ökostromgesetzes 2012', date: '2025-01-10', status: 'accepted' },
		{ id: 2, type: 'vote', title: 'Bundesfinanzgesetz 2025', date: '2025-01-08', status: 'rejected' },
		{ id: 3, type: 'proposal', title: 'Antrag betreffend Ausbau der Kinderbetreuung', date: '2025-01-12', status: 'pending' },
		{ id: 4, type: 'proposal', title: 'Dringliche Anfrage zur Bildungspolitik', date: '2025-01-11', status: 'pending' },
		{ id: 5, type: 'decree', title: 'Verordnung über Luftreinhaltung', date: '2025-01-05' },
		{ id: 6, type: 'vote', title: 'Novelle zum Universitätsgesetz', date: '2025-01-02', status: 'accepted' },
	];

	// 3. Association Events
	const eventsData: Event[] = [
		{
			id: 1,
			title: 'EU Wahl 2024 - Podiumsdiskussion',
			location: 'HTL Hollabrunn, Stadtsaal',
			date: '2024-04-22',
			time: '10:00',
			description: 'Podiumsdiskussion zur EU-Wahl 2024 🇪🇺. Wir wählen in diesem Jahr das Europäische Parlament! Deshalb veranstalten wir von der Schülervertretung eine Podiumsdiskussion zur Europawahl. Wir veranstalten diese Diskussion zusammen mit @somes.at. Es kommen spannende Kandidaten und ihr werdet die Möglichkeit haben, diese Diskussion mitzugestalten.',
			requiresMembership: false,
		},
		{
			id: 2,
			title: 'Nationalratswahl 2024 - Podiumsdiskussion',
			location: 'HTL Hollabrunn, Stadtsaal',
			date: '2024-09-17',
			time: '09:00',
			description: 'Das war die Podiumsdiskussion der Schülervertretung zur Nationalratswahl 🇦🇹! Fünf Parteien stellten sich den spannenden Fragen. Rund 790 SchülerInnen der @htlhollabrunn nahmen an dieser lebhaften Debatte teil. Besonders gefreut haben wir uns über die Anwesenheit des Vize-Kanzlers.',
			requiresMembership: false,
		},
		{
			id: 3,
			title: 'Demokratietag 2025',
			location: 'HTL Hollabrunn, Stadtsaal',
			date: '2025-03-24',
			time: '08:30',
			description: 'Danke fürs Dabeisein! 🙌 Über 650 Schüler:innen, spannende Gäste 🎤, ein interaktives Politikquiz 🧠 & ein offener Gesprächskreis. Organisiert von der @sv_htlhollabrunn & @somes.at, mit dem Ziel: Positive Perspektiven geben und Demokratie erlebbar machen. Interesse an Somes? Schreib uns!',
			requiresMembership: false,
		},
		{
			id: 4,
			title: 'Parlamentsführung & Ausklang',
			location: 'Parlament Wien / Cafe Rathaus',
			date: '2026-01-24',
			time: '14:00',
			description: 'Parlamentsführung und abschließend Ausklang im Cafe Rathaus mit Nationalratsabgeordneten Paul Stich (SPÖ).',
			requiresMembership: true,
		}
	];

	// 4. Board Members
	const boardMembers: Member[] = [
		{ name: 'Daniel Atzwanger', role: 'Obperson', subRole: 'Kommunikation, Organisation', image: '' },
		{ name: 'Clemens Bauer', role: 'Obperson', subRole: 'Moderation, Öffentlichkeitsarbeit', image: '' },
		{ name: 'Florian Nagy', role: 'Obperson', subRole: 'Plattformentwicklung, technische Leitung', image: '' },
		{ name: 'Martin Zambal', role: 'Kassier', subRole: 'Finanzen, Förderanträge', image: '' }
	];

	const devTeamMembers: Member[] = [
		{ name: 'Fabian Cemerka', role: 'Fullstack Developer', image: '' },
		{ name: 'Christoph Handschuh', role: 'Frontend & KI Developer', image: '' },
		{ name: 'Florian Nagy', role: 'Data Scraping, Fullstack & KI Developer', image: '' },
		{ name: 'Jakob Pronebner', role: 'Mobile Developer', image: '' },
	];

	// --- STATE MANAGEMENT (RUNES) ---

	// Platform Widget Logic
	let selectedPlatformType: PlatformItemType  = $state('vote');
	let platformTickerIndex = $state(0);
	let tickerInterval: any;

	// Filtered Platform Items
	let currentPlatformItems = $derived(
		rawPlatformItems.filter(i => i.type === selectedPlatformType)
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
	
	let upcomingEvents = $derived(eventsData.filter(e => new Date(e.date) >= today).sort((a,b) => new Date(a.date).getTime() - new Date(b.date).getTime()));
	let pastEvents = $derived(eventsData.filter(e => new Date(e.date) < today).sort((a,b) => new Date(b.date).getTime() - new Date(a.date).getTime())); // Newest past first

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

	onMount(() => {
		startTicker();
	});

	onDestroy(() => {
		stopTicker();
	});
</script>

<div class="min-h-screen text-base-font-color font-base pb-20 dark:bg-surface-950 dark:text-surface-50">
	
	<!-- NAVIGATION / HEADER -->
	<header class="sticky top-0 z-50 bg-surface-50/90 backdrop-blur-md border-b border-surface-200 shadow-sm dark:bg-surface-900/90 dark:border-surface-700">
		<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 h-16 flex items-center justify-between">
			<nav class="hidden md:flex gap-6 text-sm font-medium text-surface-600 dark:text-surface-300">
				<a href="{resolve("/home")}" class="hover:text-primary-600 dark:hover:text-primary-400 transition-colors">Plattform</a>
				<a href="{resolve("/")}#events" class="hover:text-primary-600 dark:hover:text-primary-400 transition-colors">Events</a>
				<a href="{resolve("/")}#team" class="hover:text-primary-600 dark:hover:text-primary-400 transition-colors">Team</a>
				<a href="{resolve("/")}#join" class="hover:text-primary-600 dark:hover:text-primary-400 transition-colors">Mitmachen</a>
			</nav>
			<a href="https://forms.gle/mVx8K2cm3TPx8CJP9" target="_blank" class="px-4 py-2 rounded-container bg-primary-600 text-white text-sm font-bold hover:bg-primary-700 transition-all shadow-md">
				Mitglied werden
			</a>
		</div>
	</header>

	<!-- HERO SECTION -->
	<section class="relative pt-20 pb-32 overflow-hidden">
		<!-- Background Elements -->
		<div class="absolute top-[-10%] right-[-5%] w-[500px] h-[500px] bg-secondary-200/40 rounded-full blur-3xl -z-10 dark:bg-secondary-900/20"></div>
		<div class="absolute bottom-[-10%] left-[-10%] w-[600px] h-[600px] bg-tertiary-200/60 rounded-full blur-3xl -z-10 dark:bg-tertiary-900/20"></div>

		<div class="max-w-5xl mx-auto px-4 text-center space-y-8">
			<div class="inline-block px-3 py-1 rounded-full bg-tertiary-200 text-tertiary-900 text-xs font-bold uppercase tracking-wider mb-4 border border-tertiary-300 dark:bg-tertiary-900/50 dark:text-tertiary-100 dark:border-tertiary-700">
				Verein für Demokratie und Transparenz
			</div>
			<h1 class="font-heading text-5xl md:text-7xl font-extrabold text-primary-900 dark:text-primary-50 leading-tight">
				Demokratie <span class="text-transparent bg-clip-text bg-gradient-to-r from-secondary-500 to-error-500">verstehen.</span><br>
				Zukunft <span class="text-transparent bg-clip-text bg-gradient-to-r from-primary-500 to-success-600">gestalten.</span>
			</h1>
			<p class="text-lg md:text-xl text-surface-600 max-w-2xl mx-auto leading-relaxed dark:text-surface-300">
				Wir bringen politische Bildung direkt zu dir. Mit der somes Plattform und interaktiven Events machen wir Politik transparent und greifbar.
			</p>
			
			<div class="flex flex-wrap justify-center gap-4 pt-4">
				<a href="{resolve("/home")}" class="flex items-center gap-2 px-6 py-3 rounded-full bg-surface-900 text-white font-bold text-lg hover:scale-105 transition-transform shadow-xl dark:bg-surface-50 dark:text-surface-900">
					Zur Somes Plattform
					<svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 8l4 4m0 0l-4 4m4-4H3" /></svg>
				</a>
				<a href="{resolve("/")}#events" class="px-6 py-3 rounded-full bg-white text-surface-900 border border-surface-200 font-bold text-lg hover:bg-surface-50 transition-colors shadow-sm dark:bg-surface-800 dark:text-surface-50 dark:border-surface-600 dark:hover:bg-surface-700">
					Unsere Events
				</a>
			</div>
		</div>
	</section>

	<!-- PLATFORM WIDGET SECTION -->
	<section id="platform" class="py-16 bg-surface-900 text-white relative">
		<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
			
			<div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
				
				<!-- Left: Intro & Progress -->
				<div class="lg:col-span-1 space-y-6">
					<h2 class="font-heading text-3xl font-bold text-secondary-300">Somes Plattform</h2>
					<p class="text-surface-300 leading-relaxed">
						Unsere Open-Source Web-App macht das Parlament gläsern. Analysiere Abstimmungen, lies KI-Zusammenfassungen von Gesetzen und verfolge, was deine Abgeordneten tun.
					</p>
					
					<!-- Development Progress -->
					<div class="bg-surface-800 p-5 rounded-xl border border-surface-700 shadow-inner">
						<div class="flex justify-between items-end mb-2">
							<span class="text-sm font-medium text-surface-300">Entwicklung v1.0 - 31.04.2026</span>
							<span class="text-2xl font-bold text-primary-300">{developmentProgress}%</span>
						</div>
						<div class="w-full bg-surface-700 rounded-full h-2.5 overflow-hidden">
							<div class="bg-gradient-to-r from-primary-400 to-secondary-400 h-2.5 rounded-full transition-all duration-1000 ease-out" style="width: {developmentProgress}%"></div>
						</div>
						<p class="text-xs text-surface-400 mt-2"></p>
					</div>

					<div class="bg-primary-900/50 p-4 rounded-xl border border-primary-700/50">
						<span class="block text-xs uppercase tracking-wide text-primary-300 mb-1">Nächste Nationalratssitzung</span>
						<span class="text-xl font-mono font-bold text-white">
							{nextPlenaryDate.toLocaleDateString('de-AT', { weekday: 'long', day: 'numeric', month: 'long' })}
						</span>
					</div>
				</div>

				<!-- Right: The "Live" Ticker Widget -->
				<div class="lg:col-span-2">
					<div class="bg-surface-800 rounded-2xl border border-surface-700 overflow-hidden shadow-2xl flex flex-col h-full min-h-[400px]">
						
						<!-- Widget Header / Controls -->
						<div class="p-4 border-b border-surface-700 flex flex-wrap gap-2 items-center justify-between bg-surface-850">
                            <div></div>
							<!-- <div class="flex gap-2">
								<div class="w-3 h-3 rounded-full bg-error-500"></div>
								<div class="w-3 h-3 rounded-full bg-warning-500"></div>
								<div class="w-3 h-3 rounded-full bg-success-500"></div>
							</div> -->
							<div class="flex bg-surface-900 rounded-lg p-1 gap-1">
								<button 
									class="px-3 py-1 rounded-md text-sm transition-colors {selectedPlatformType === 'vote' ? 'bg-secondary-600 text-white shadow-md' : 'text-surface-400 hover:text-white'}"
									onclick={() => setPlatformType('vote')}>
									Abstimmungen
								</button>
								<button 
									class="px-3 py-1 rounded-md text-sm transition-colors {selectedPlatformType === 'proposal' ? 'bg-primary-600 text-white shadow-md' : 'text-surface-400 hover:text-white'}"
									onclick={() => setPlatformType('proposal')}>
									Entwürfe
								</button>
								<button 
									class="px-3 py-1 rounded-md text-sm transition-colors {selectedPlatformType === 'decree' ? 'bg-tertiary-600 text-white shadow-md' : 'text-surface-400 hover:text-white'}"
									onclick={() => setPlatformType('decree')}>
									Erlasse
								</button>
							</div>
						</div>

						<!-- Widget Content Area -->
						<div class="relative flex-1 p-8 flex items-center justify-center bg-gradient-to-br from-surface-800 to-surface-900">
							{#key currentTickerItem}
								<div 
									in:fly={{ y: 20, duration: 400, delay: 100, opacity: 0 }} 
									out:fade={{ duration: 200 }}
									class="absolute w-full max-w-lg text-center"
								>
									{#if currentTickerItem}
										<!-- Badge -->
										<span class="inline-flex items-center px-3 py-1 rounded-full text-xs font-bold uppercase tracking-wider mb-6
											{currentTickerItem.type === 'vote' ? 'bg-secondary-500/20 text-secondary-300 border border-secondary-500/30' : 
											 currentTickerItem.type === 'proposal' ? 'bg-primary-500/20 text-primary-300 border border-primary-500/30' : 
											 'bg-tertiary-500/20 text-tertiary-300 border border-tertiary-500/30'}">
											{currentTickerItem.type === 'vote' ? 'Abstimmung' : currentTickerItem.type === 'proposal' ? 'Ministerialentwurf' : 'Erlass'}
										</span>

										<!-- Title -->
										<h3 class="text-2xl md:text-3xl font-heading font-bold text-white mb-4 leading-tight">
											{currentTickerItem.title}
										</h3>

										<!-- Meta Info -->
										<div class="flex justify-center gap-4 text-sm text-surface-400 font-mono">
											<span>{formatDate(currentTickerItem.date)}</span>
											{#if currentTickerItem.status}
												<span class="mx-2">•</span>
												<span class="{currentTickerItem.status === 'accepted' ? 'text-success-400' : currentTickerItem.status === 'rejected' ? 'text-error-400' : 'text-warning-400'}">
													{currentTickerItem.status === 'accepted' ? 'Angenommen' : currentTickerItem.status === 'rejected' ? 'Abgelehnt' : 'In Bearbeitung'}
												</span>
											{/if}
										</div>

										<!-- Progress Indicator for Timer -->
										<div class="absolute -bottom-16 left-1/2 -translate-x-1/2 w-32 h-1 bg-surface-700 rounded-full overflow-hidden">
											<div class="h-full bg-white/20 animate-progress-indeterminate"></div> 
											<!-- Note: Using custom animation defined in theme or standard CSS -->
										</div>
									{:else}
										<p class="text-surface-500">Keine Daten verfügbar.</p>
									{/if}
								</div>
							{/key}
						</div>
						
						<!-- Footer Link -->
						<div class="p-3 bg-surface-850 border-t border-surface-700 text-center">
							<a href="https://somes.at" class="text-xs text-primary-400 hover:text-primary-300 flex items-center justify-center gap-1 group">
								Alle Details auf somes.at ansehen
								<span class="group-hover:translate-x-1 transition-transform">→</span>
							</a>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- EVENTS SECTION -->
	<section id="events" class="py-20 max-w-6xl mx-auto px-4 sm:px-6">
		<div class="text-center mb-16">
			<h2 class="text-primary-600 font-bold uppercase tracking-widest text-sm mb-2">Engagiere dich</h2>
			<h3 class="font-heading text-4xl font-bold text-surface-900 dark:text-surface-50">Events & Diskussionen</h3>
		</div>

		<!-- Upcoming Events -->
		{#if upcomingEvents.length > 0}
			<div class="mb-16">
				<div class="flex items-center gap-4 mb-6">
					<h4 class="text-2xl font-bold text-primary-800 dark:text-primary-200">Kommende Termine</h4>
					<div class="h-px flex-1 bg-primary-200 dark:bg-primary-800"></div>
				</div>
				
				<div class="grid gap-6 md:grid-cols-2">
					{#each upcomingEvents as event}
						<div class="group bg-white rounded-2xl p-6 shadow-lg border border-primary-100 hover:border-secondary-300 transition-all hover:-translate-y-1 relative overflow-hidden dark:bg-surface-800 dark:border-surface-700">
							<div class="absolute top-0 left-0 w-2 h-full bg-secondary-500"></div>
							
							<div class="flex justify-between items-start mb-4 pl-4">
								<div class="bg-secondary-50 rounded-lg px-3 py-1 text-secondary-800 font-bold text-center border border-secondary-100 dark:bg-secondary-900 dark:text-secondary-100 dark:border-secondary-700">
									<div class="text-xs uppercase">{new Date(event.date).toLocaleDateString('de-AT', { month: 'short' })}</div>
									<div class="text-xl">{new Date(event.date).getDate()}</div>
								</div>
								<span class="text-sm font-medium text-surface-500 flex items-center gap-1 dark:text-surface-400">
									<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z"/><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 11a3 3 0 11-6 0 3 3 0 016 0z"/></svg>
									{event.location}
								</span>
							</div>

							<div class="pl-4">
								<h5 class="text-xl font-bold text-surface-900 mb-2 group-hover:text-secondary-600 transition-colors dark:text-surface-100 dark:group-hover:text-secondary-400">{event.title}</h5>
								<p class="text-surface-600 text-sm line-clamp-3 mb-4 dark:text-surface-300">{event.description}</p>
								
								{#if event.requiresMembership}
									<div class="text-xs text-primary-500 font-medium mb-2">
										🔒 Mitgliedschaft erforderlich
									</div>
								{:else if !event.requiresMembership}
									<div class="text-xs text-success-500 font-medium mb-2">
										✅ Offen für alle - einfach kommen!
									</div>
								{/if}
								{#if event.requiresRegistration}
									<div class="text-xs text-primary-500 font-medium mb-2">
										📝 Anmeldung erforderlich
									</div>
								{/if}
								<!-- <button class="text-primary-600 font-bold text-sm flex items-center gap-1 group/btn">
									Details & Anmeldung
									<span class="group-hover/btn:translate-x-1 transition-transform">→</span>
								</button> -->
							</div>
						</div>
					{/each}
				</div>
			</div>
		{/if}

		<!-- Past Events -->
		<div>
			<div class="flex items-center gap-4 mb-6">
				<h4 class="text-2xl font-bold text-surface-500 dark:text-surface-400">Vergangene Events</h4>
				<div class="h-px flex-1 bg-surface-200 dark:bg-surface-700"></div>
			</div>
			
			<div class="space-y-4">
				{#each pastEvents as event}
					<div class="bg-surface-50 rounded-xl p-5 flex flex-col md:flex-row gap-4 border border-surface-200/50 hover:bg-white transition-colors dark:bg-surface-800/50 dark:border-surface-700 dark:hover:bg-surface-800">
						<div class="shrink-0 flex md:flex-col items-center gap-2 md:gap-0 md:justify-center md:w-24 text-surface-400 dark:text-surface-400">
							<span class="font-bold">{formatDate(event.date)}</span>
						</div>
						<div class="flex-1">
							<h5 class="text-lg font-bold text-surface-700 dark:text-surface-100">{event.title}</h5>
							<p class="text-sm text-surface-500 mt-1 dark:text-surface-400">{event.description}</p>
							<div class="flex justify-between flex-wrap">
								<div class="mt-2 text-xs text-primary-500 font-medium">
									📍 {event.location}
								</div>
								{#if event.requiresMembership}
									<div class="mt-2 text-xs text-primary-500 font-medium">
										🔒 Mitgliedschaft erforderlich
									</div>
								{/if}
								{#if event.requiresRegistration}
									<div class="mt-2 text-xs text-primary-500 font-medium">
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
	<section id="join" class="py-4 px-4">
		<div class="max-w-4xl mx-auto bg-primary-800 rounded-3xl p-8 md:p-12 text-center text-white shadow-2xl relative overflow-hidden">
			<!-- Decorative Circles -->
			<div class="absolute top-0 right-0 w-64 h-64 bg-primary-600 rounded-full blur-3xl -translate-y-1/2 translate-x-1/2 opacity-50"></div>
			<div class="absolute bottom-0 left-0 w-64 h-64 bg-secondary-600 rounded-full blur-3xl translate-y-1/2 -translate-x-1/2 opacity-50"></div>

			<div class="relative z-10">
				<h2 class="font-heading text-3xl md:text-5xl font-bold mb-6">Werde Teil von Somes</h2>
				<p class="text-lg text-primary-100 mb-8 max-w-2xl mx-auto">
					Unterstütze uns dabei, den einfachen Zugang zu politischen Inhalten zu revolutionieren. Als Mitglied bei 
					<span class="font-bold italic">somes - Verein für Demokratie und politische Transparenz</span> 
					trägst du aktiv zur Gestaltung unserer Zukunft bei.
				</p>
				
				<div class="flex flex-col sm:flex-row justify-center gap-4">
					<a href="https://forms.gle/mVx8K2cm3TPx8CJP9" target="_blank" class="px-8 py-4 rounded-xl bg-secondary-500 hover:bg-secondary-400 text-white font-bold text-lg shadow-lg hover:shadow-secondary-500/30 transition-all transform hover:-translate-y-0.5">
						Mitglied werden
					</a>
					<a href="mailto:somes.austria@gmail.com" class="px-8 py-4 rounded-xl bg-primary-700 hover:bg-primary-600 text-white font-bold text-lg border border-primary-500 transition-all">
						Kontakt aufnehmen
					</a>
				</div>
			</div>
		</div>
	</section>

	<!-- TEAM SECTION -->
	<section id="team" class="py-10 mt-24 bg-tertiary-100 dark:bg-surface-900">
		<div class="max-w-7xl mx-auto px-4">
			<h2 class="font-heading text-3xl text-center font-bold text-primary-900 mb-12 dark:text-primary-100">Vorstand</h2>
			
			<div class="flex flex-wrap justify-center gap-8 md:gap-12">
				{#each boardMembers as member}
					<div class="flex flex-col items-center group">
						<div class="w-32 h-32 rounded-full bg-gradient-to-br from-tertiary-300 to-tertiary-400 border-4 border-white shadow-lg mb-4 overflow-hidden relative dark:border-surface-800">
							{#if member.image}
								<img src={member.image} alt={member.name} class="w-full h-full object-cover" />
							{:else}
								<div class="w-full h-full flex items-center justify-center text-tertiary-600 font-bold text-2xl dark:text-tertiary-800">
									{member.name.charAt(0)}
								</div>
							{/if}
						</div>
						<h3 class="font-bold text-lg text-primary-900 dark:text-primary-100">{member.name}</h3>
						{#if member.role}
							<span class="text-sm text-secondary-600 font-medium uppercase tracking-wide dark:text-secondary-400">{member.role}</span>
						{/if}
						{#if member.subRole}
							<span class="text-xs text-surface-500 dark:text-surface-400">{member.subRole}</span>
						{/if}
					</div>
				{/each}
			</div>
		</div>
	</section>
	<section id="team" class="py-10 bg-tertiary-100 dark:bg-surface-900">
		<div class="max-w-7xl mx-auto px-4">
			<h2 class="font-heading text-3xl text-center font-bold text-primary-900 mb-12 dark:text-primary-100">Netidee Entwicklungsteam</h2>
			
			<div class="flex flex-wrap justify-center gap-8 md:gap-12">
				{#each devTeamMembers as member}
					<div class="flex flex-col items-center group">
						<div class="w-32 h-32 rounded-full bg-gradient-to-br from-tertiary-300 to-tertiary-400 border-4 border-white shadow-lg mb-4 overflow-hidden relative dark:border-surface-800">
							{#if member.image}
								<img src={member.image} alt={member.name} class="w-full h-full object-cover" />
							{:else}
								<div class="w-full h-full flex items-center justify-center text-tertiary-600 font-bold text-2xl dark:text-tertiary-800">
									{member.name.charAt(0)}
								</div>
							{/if}
						</div>
						<h3 class="font-bold text-lg text-primary-900 dark:text-primary-100">{member.name}</h3>
						{#if member.role}
							<span class="text-sm text-secondary-600 font-medium uppercase tracking-wide dark:text-secondary-400">{member.role}</span>
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
		0% { transform: translateX(-100%); }
		50% { transform: translateX(0%); }
		100% { transform: translateX(100%); }
	}
	
	.animate-progress-indeterminate {
		animation: progress-indeterminate 12.4s linear;
	}
</style>