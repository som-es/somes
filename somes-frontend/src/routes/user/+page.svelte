<script lang="ts">
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import {
		delegate_by_id,
		errorToNull,
		get_eurovoc_topics,
		isHasError,
		vote_result_by_id
	} from '$lib/api/api';
	import {
		addUserTopic,
		anonymize_email,
		change_email,
		delete_account,
		getMailSendInfo,
		getUser,
		removeUserTopic,
		renew_token,
		updateMailSendInfo,
		verify_email_change
	} from '$lib/api/authed';
	import { cachedDelegateFavos, cachedLegisInitFavos } from '$lib/caching/favos';
	import { jwtStore } from '$lib/caching/stores/stores.svelte';
	import { cachedUserTopics } from '$lib/caching/user_topics_cache.svelte';
	import DelegateCard from '$lib/components/Delegates/DelegateCard.svelte';
	import Container from '$lib/components/Layout/Container.svelte';
	import SelectableTopics from '$lib/components/Topics/SelectableTopics.svelte';
	import SButton from '$lib/components/UI/SButton.svelte';
	import ExpandablePlaceholder from '$lib/components/VoteResults/Expandable/Placeholders/ExpandablePlaceholder.svelte';
	import VoteResultExpandableBar from '$lib/components/VoteResults/Expandable/VoteResultExpandableBar.svelte';
	import { gotoHistory } from '$lib/goto';
	import {
		getUserFromJwt,
		type BasicUserInfo,
		type DelegateFavo,
		type ExtendedUserInfo,
		type MailSendInfo,
		type UniqueTopic
	} from '$lib/types';
	import { Switch, Popover, Select } from 'bits-ui';
	import { onMount } from 'svelte';
	import SearchBar from '$lib/components/Filtering/SearchBar.svelte';
	import { SvelteMap, SvelteSet } from 'svelte/reactivity';
	import DelegateUserCard from '$lib/components/Delegates/DelegateUserCard.svelte';
	import MailTopicCard from '$lib/components/UI/MailTopicCard.svelte';



	const API_BASE = import.meta.env.VITE_API_URL;

	// State with Svelte 5 runes
	let topics = $state<UniqueTopic[]>([]);
	let selectedTopics = $state<SvelteSet<number>>(new SvelteSet<number>());
	let user = $state<BasicUserInfo | null>(null);
	let extendedUser = $state<ExtendedUserInfo | null>(null);
	let mailSendInfo = $state<MailSendInfo | null>(null);
	let favoDelegates = $state<SvelteMap<number, DelegateFavo> | null>(null);
	let favoLegisInits = $state<SvelteSet<number> | null>(null);

	let topicSearchValue = $state('');
	let isSearchPopupOpen = $state(false);

	let anonymizeEmail = $state<boolean>(!!extendedUser?.is_email_hashed);
	let showChangeEmail = $state(false);
	let newEmail = $state('');
	let otp = $state('');
	let otpStep = $state(false);
	let error = $state('');
	let success = $state('');
	let sent = $state(false);
	

	// Filtered topics based on search input
	let filteredTopics = $derived(
		topics.filter((t) => t.topic.toLowerCase().includes(topicSearchValue.toLowerCase()))
	);

	// Selected topics first, then unselected
	let selectedFilteredTopics = $derived(filteredTopics.filter((t) => selectedTopics.has(t.id)));
	let unselectedFilteredTopics = $derived(filteredTopics.filter((t) => !selectedTopics.has(t.id)));

	let searchWrapper: HTMLDivElement | undefined = $state();
	function handleFocusOut(e: FocusEvent) {
		const relatedTarget = e.relatedTarget as Node | null;
		if (relatedTarget) {
			if (searchWrapper?.contains(relatedTarget)) return;
			if ((relatedTarget as Element).closest('.search-filter-portal')) return;
		}
		isSearchPopupOpen = false;
	}

	onMount(async () => {
		const jwtToken = jwtStore.value;
		if (isHasError(await renew_token()) || jwtToken == null) {
			goto(resolve('/home'));
			return;
		}

		topics = errorToNull(await get_eurovoc_topics()) ?? [];
		user = getUserFromJwt(jwtToken);
		mailSendInfo = errorToNull(await getMailSendInfo());
		extendedUser = errorToNull(await getUser());
		favoDelegates = errorToNull(await cachedDelegateFavos(true));
		favoLegisInits = errorToNull(await cachedLegisInitFavos(true));

		// get interest topics from api
		const data = await cachedUserTopics(true);

		if (data) {
			selectedTopics = new SvelteSet<number>(data.map((topic) => topic.id));
		}
	});

	const allMailFields: (keyof MailSendInfo)[] = [
		'send_new_vote_results_mails',
		'send_new_vote_result_by_favo_mails',
		'send_new_delegate_activity_mails',
		'send_new_ministrial_prop_mails',
		'send_new_ministrial_prop_by_favo_mails',
		'send_new_decree_mails',
		'send_new_decree_by_favo_mails',
		'send_new_proposal_mails',
		'send_new_proposal_by_favo_mails'
	];

	let allChecked = $derived(!!mailSendInfo && allMailFields.every((f) => mailSendInfo![f]));

	const toggleAll = async (checked: boolean) => {
		if (!mailSendInfo) return;
		for (const field of allMailFields) {
			mailSendInfo[field] = checked;
		}
		await updateMailSendInfo(mailSendInfo);
	};

	const updateThisMailSendInfo = async () => {
		if (!mailSendInfo) {
			return;
		}

		await updateMailSendInfo(mailSendInfo);
	};

	function handleTopicToggle(topic: UniqueTopic) {
		if (selectedTopics.has(topic.id)) {
			selectedTopics.delete(topic.id);
			removeUserTopic({ id: topic.id, topic: '' });
		} else {
			selectedTopics.add(topic.id);
			addUserTopic({ id: topic.id, topic: '' });
		}
		// Trigger reactivity
		selectedTopics = new SvelteSet(selectedTopics);
	}

	async function handleLogout() {
		jwtStore.value = null;
		gotoHistory('/home');
	}

	async function handleDeleteAccount() {
		await delete_account();
		jwtStore.value = null;
		gotoHistory('/home');
	}

	async function toggleEmailAnonymization(checked: boolean) {
		anonymizeEmail = checked;
		await anonymize_email(checked);
	}

async function changeEmail() {
	if (!newEmail) return;

	error = '';
	success = '';
	sent = false;

	try {
		const result = await change_email(newEmail);
		
		if (isHasError(result)) {
			if (result.error.includes('fehlt')) {
				error = 'E-Mail-Adresse fehlt';
			} else if (result.error.includes('Fehlerhafte')) {
				error = 'Fehlerhafte E-Mail-Adresse';
			} else {
				error = 'Ein serverseitiger Fehler ist aufgetreten. Es kann nicht fortgefahren werden.';
			}
		} else {
			// Success - OTP sent
			otpStep = true;
			sent = true;
			success = 'An deine E-Mail-Adresse wurde ein One-Time Passwort gesendet.';
		}
	} catch (e) {
		error = 'Ein serverseitiger Fehler ist aufgetreten. Es kann nicht fortgefahren werden.';
		console.error('Email change error:', e);
	}
}

async function verifyOtp() {
	if (!otp || !newEmail) return;

	error = '';
	success = '';

	try {
		const result = await verify_email_change(newEmail, otp);

		if (isHasError(result)) {
			error = 'Ein serverseitiger Fehler ist aufgetreten.';
			return;
		}

		if (result.access_token) {
			jwtStore.value = result.access_token;
			user = getUserFromJwt(result.access_token)
		}

		extendedUser = errorToNull(await getUser());

		success = 'E-Mail-Adresse erfolgreich geändert.';

		setTimeout(() => {
			showChangeEmail = false;
			newEmail = '';
			otp = '';
			otpStep = false;
			error = '';
			success = '';
		}, 1500);

	} catch (e) {
		error = 'Ein serverseitiger Fehler ist aufgetreten.';
		console.error(e);
	}
}
</script>

<svelte:head>
	<title>Benutzerprofil</title>
	<meta name="description" content="Dein persönliches Benutzerprofil und Einstellungen" />
</svelte:head>

<Container>
	{#if extendedUser}
		<!-- Header Section -->
		<h1 class="px-1 pt-2 text-3xl font-bold sm:p-0 sm:text-4xl">Benutzerprofil</h1>
		<span class="mb-2 ml-1 block text-base text-gray-800 sm:mt-1 sm:ml-0 dark:text-gray-300">
			Verwalte deine Einstellungen und Präferenzen
		</span>

		<div class="mt-5 flex flex-col gap-4">
			<!-- User Info Card -->
			<div class="w-full rounded-xl bg-primary-300 p-4 dark:bg-primary-500">
				<div class="flex flex-col gap-3">
					<!-- OBERE ZEILE -->
					<div class="flex flex-wrap items-center justify-between gap-3">
						<div class="flex flex-wrap items-center gap-3">
							<h2 class="text-xl font-bold text-gray-900 dark:text-gray-50">Benutzerinfos</h2>
							<div class="flex items-center gap-2 text-base text-gray-800 dark:text-gray-200">
								<span class="font-medium">E-Mail:</span>
								{#if extendedUser?.is_email_hashed}
									<span class="font-serif">anonymisiert</span>
									{#if user}
										<span class="text-sm text-gray-600 dark:text-gray-400"
											>...{user.sub.slice(36, 60)}...</span
										>
									{/if}
								{:else if user}
									<span>{user.sub}</span>
								{/if}
							</div>
						</div>

						<div class="mt-3 flex items-center gap-3">
							<span class="text-sm text-gray-700 dark:text-gray-300">
								E-Mail anonymisieren
							</span>

							<Switch.Root
								checked={anonymizeEmail}
								onCheckedChange={toggleEmailAnonymization}
								class="peer inline-flex h-[24px] w-[44px] shrink-0 cursor-pointer items-center rounded-full border-2 border-transparent transition-colors data-[state=checked]:bg-secondary-500 data-[state=unchecked]:bg-gray-300"
							>
								<Switch.Thumb
									class="block h-5 w-5 rounded-full bg-white shadow transition-transform data-[state=checked]:translate-x-5"
								/>
							</Switch.Root>

							<span class="text-xs text-gray-500">
								{anonymizeEmail ? 'Ja' : 'Nein'}
							</span>
						</div>

						<div class="flex gap-2">
							<SButton
								class="bg-secondary-500 text-white hover:bg-secondary-600"
								onclick={handleLogout}
							>
								Abmelden
							</SButton>
							<SButton
								class="bg-tertiary-500 text-black hover:bg-tertiary-600"
								onclick={() => (showChangeEmail = !showChangeEmail)}
							>
								E-Mail wechseln
							</SButton>
							
						</div>
					</div> <!-- ENDE obere Zeile -->
						{#if showChangeEmail}
						<div class="flex justify-end items-end gap-4 border-t pt-3">

							<!-- EMAIL -->
							<div class="flex flex-col items-end">
								<label for="email" class="text-l font-medium text-gray-700">
									Neue E-Mail
								</label>
								<input
									id="email"
									type="email"
									placeholder="dergertrud@gmail.com"
									class="w-48 rounded-lg border px-3 py-2 text-sm text-right dark:bg-gray-800"
									bind:value={newEmail}
								/>
							</div>

							<!-- OTP -->
							{#if otpStep}
								<div class="flex flex-col items-end">
									<label for="otp" class="text-l font-medium text-gray-700">
										One-Time Passwort (OTP)
									</label>
									<input
										id="otp"
										type="text"
										placeholder="MAS DS5 4DA"
										class="w-48 rounded-lg border px-3 py-2 text-sm text-right dark:bg-gray-800"
										bind:value={otp}
									/>
								</div>
							{/if}

							<!-- BUTTONS -->
							<div class="flex items-end gap-2">
								{#if !otpStep}
									<SButton class="bg-secondary-500 text-white" onclick={changeEmail}>
										Weiter
									</SButton>
								{:else}
									<SButton class="bg-secondary-500 text-white" onclick={verifyOtp}>
										Speichern
									</SButton>
								{/if}

								<SButton
									class="bg-gray-300"
									onclick={() => {
										showChangeEmail = false;
										newEmail = '';
										otp = '';
										otpStep = false;
										error = '';
										success = '';
										sent = false;
									}}
								>
									Abbrechen
								</SButton>
							</div>
						</div>

						<!-- Success and Error Messages -->
						{#if sent && success}
							<div
								class="rounded-lg border border-green-200 bg-green-50 px-4 py-3 text-sm text-green-700"
							>
								{success}
							</div>
						{/if}

						{#if error}
							<p class="text-sm text-red-500">{error}</p>
						{/if}
					{/if}
				</div>
			</div>

			<!-- Email Notifications Card -->
			<div class="w-full rounded-xl bg-primary-300 p-4 dark:bg-primary-500">
				<div class="flex items-center justify-between">
					<h2 class="text-xl font-bold text-gray-900 dark:text-gray-50">E-Mail Benachrichtigungen</h2>
					<div class="flex items-center gap-2">
						<p class="text-sm">Alle</p>
						<Switch.Root
							checked={allChecked}
							onCheckedChange={toggleAll}
							class="peer inline-flex h-[24px] w-[44px] shrink-0 cursor-pointer items-center rounded-full border-2 border-transparent transition-colors focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:ring-offset-white focus-visible:outline-none disabled:cursor-not-allowed disabled:opacity-50 data-[state=checked]:bg-secondary-500 data-[state=unchecked]:bg-gray-300"
						>
							<Switch.Thumb
								class="pointer-events-none block h-5 w-5 rounded-full bg-white shadow-lg ring-0 transition-transform data-[state=checked]:translate-x-5 data-[state=unchecked]:translate-x-0"
							/>
						</Switch.Root>
					</div>
				</div>

				{#if !extendedUser?.is_email_hashed}
					{#if mailSendInfo}
						<div>
							<p class="text-sm font-semibold text-gray-600 dark:text-gray-300">Nach Interessen & Themen</p>
							<div class="flex flex-wrap gap-2 mt-2">
								<MailTopicCard title="Abstimmungen" description="Neue Abstimmungen nach deinen Interessen" bind:checked={mailSendInfo.send_new_vote_results_mails} onchange={updateThisMailSendInfo} />
								<MailTopicCard title="Ministerialentwürfe" description="Neue Ministerialentwürfe nach deinen Interessen" bind:checked={mailSendInfo.send_new_ministrial_prop_mails} onchange={updateThisMailSendInfo} />
								<MailTopicCard title="Verordnungen" description="Neue Verordnungen nach deinen Themen" bind:checked={mailSendInfo.send_new_decree_mails} onchange={updateThisMailSendInfo} />
								<MailTopicCard title="Anträge" description="Neue Anträge nach deinen Themen" bind:checked={mailSendInfo.send_new_proposal_mails} onchange={updateThisMailSendInfo} />
							</div>
						</div>
						<div>
							<p class="text-sm font-semibold text-gray-600 dark:text-gray-300 mt-3">Nach favourisierten Ministern & Personen</p>
							<div class="flex flex-wrap gap-2 mt-2">
								<MailTopicCard title="Abstimmungen" description="Neue Abstimmungen nach favorisierten Personen" bind:checked={mailSendInfo.send_new_vote_result_by_favo_mails} onchange={updateThisMailSendInfo} />
								<MailTopicCard title="Ministerialentwürfe" description="Neue Ministerialentwürfe nach favorisierten Ministern" bind:checked={mailSendInfo.send_new_ministrial_prop_by_favo_mails} onchange={updateThisMailSendInfo} />
								<MailTopicCard title="Verordnungen" description="Neue Verordnungen nach favorisierten Ministern" bind:checked={mailSendInfo.send_new_decree_by_favo_mails} onchange={updateThisMailSendInfo} />
								<MailTopicCard title="Anträge" description="Neue Anträge nach favorisierten Personen" bind:checked={mailSendInfo.send_new_proposal_by_favo_mails} onchange={updateThisMailSendInfo} />
								<MailTopicCard title="Aktivitäten" description="Neue Aktivitäten nach favorisierten Personen" bind:checked={mailSendInfo.send_new_delegate_activity_mails} onchange={updateThisMailSendInfo} />
							</div>
						</div>
					{/if}
				{:else}
					<p class="mt-3 text-gray-600 dark:text-gray-300">
						nicht verfügbar: Anonymisierung durch Mail-Wechsel aufheben
					</p>
				{/if}
			</div>

			<!-- Interest Topics Card -->
			<div class="w-full rounded-xl bg-primary-300 p-4 dark:bg-primary-500">
				<div class="flex flex-wrap items-center justify-between gap-2">
					<h2 class="text-xl font-bold text-gray-900 dark:text-gray-50">Wähle deine Interessen</h2>
					{#if selectedTopics.size > 0}
						<span
							class="rounded-full bg-secondary-500 px-2.5 py-0.5 text-sm font-semibold text-white"
						>
							{selectedTopics.size} ausgewählt
						</span>
					{/if}
				</div>

				<!-- Inline search with dropdown (like MultiValuesFilter) -->
				<!-- svelte-ignore a11y_no_static_element_interactions -->
				<div class="relative mt-3" bind:this={searchWrapper} onfocusout={handleFocusOut}>
					<SearchBar
						bind:searchValue={topicSearchValue}
						placeholder="Themen suchen..."
						name="topic-search"
						onfocus={() => (isSearchPopupOpen = true)}
						oninput={() => (isSearchPopupOpen = true)}
					/>
					{#if isSearchPopupOpen}
						<div
							class="absolute top-full left-0 z-[1000] mt-1.5 w-full rounded-xl border border-gray-300 bg-surface-50 shadow-lg dark:bg-surface-600"
							onmousedown={(e) => e.preventDefault()}
						>
							<div class="flex max-h-72 flex-col gap-1 overflow-y-auto px-3 py-2">
								{#if filteredTopics.length === 0}
									<p class="py-2 text-center text-sm text-gray-500 dark:text-gray-400">
										Keine Themen gefunden
									</p>
								{:else}
									<!-- Selected topics first -->
									{#each selectedFilteredTopics as topic}
										<button
											class="flex cursor-pointer items-center gap-2 rounded-lg px-2 py-1.5 hover:bg-primary-100 dark:hover:bg-primary-700"
											onclick={() => handleTopicToggle(topic)}
										>
											<div class="min-h-4 min-w-4 rounded-md bg-secondary-500"></div>
											<span class="text-left text-sm font-semibold text-gray-800 dark:text-gray-200"
												>{topic.topic}</span
											>
										</button>
									{/each}
									<!-- Unselected topics -->
									{#each unselectedFilteredTopics as topic}
										<button
											class="flex cursor-pointer items-center gap-2 rounded-lg px-2 py-1.5 hover:bg-primary-100 dark:hover:bg-primary-700"
											onclick={() => handleTopicToggle(topic)}
										>
											<div
												class="min-h-4 min-w-4 rounded-md border-[2px] border-secondary-500"
											></div>
											<span class="text-left text-sm text-gray-800 dark:text-gray-300"
												>{topic.topic}</span
											>
										</button>
									{/each}
								{/if}
							</div>
						</div>
					{/if}
				</div>

				<div class="mt-4">
					{#if topics.length > 0}
						<SelectableTopics bind:selectedTopics {topics} />
					{/if}
				</div>
			</div>

			<!-- Favorite Delegates Card -->
			<div class="w-full rounded-xl bg-primary-300 p-4 dark:bg-primary-500">
				<h2 class="text-xl font-bold text-gray-900 dark:text-gray-50">Favorisierte Personen</h2>
				<div class="mt-3 flex flex-wrap gap-3">
					{#if favoDelegates}
						{#if favoDelegates.size == 0}
							<p class="text-gray-600 dark:text-gray-300">
								Keine favorisierten Personen vorhanden.
							</p>
						{:else}
							{#each favoDelegates as favoDelegateId (favoDelegateId[0])}
								{#await delegate_by_id(favoDelegateId[0])}
									<ExpandablePlaceholder class="!w-80" />
								{:then maybeDelegate}
									{#if !isHasError(maybeDelegate)}
										<DelegateUserCard
											delegate={maybeDelegate}
											currentNotifyInfoDays={favoDelegateId[1].user_info_days}
										/>
									{/if}
								{/await}
							{/each}
						{/if}
					{:else}
						<ExpandablePlaceholder />
					{/if}
				</div>
			</div>

			<!-- Favorite Votes Card -->
			<h2 class="text-xl font-bold text-gray-900 dark:text-gray-50">Favorisierte Abstimmungen</h2>
			<div class="flex flex-wrap gap-3">
				{#if favoLegisInits}
					{#if favoLegisInits.size == 0}
						<p class="text-gray-600 dark:text-gray-300">
							Keine favorisierten Abstimmungen vorhanden.
						</p>
					{:else}
						{#each favoLegisInits as favoLegisInitId}
							{#await vote_result_by_id(favoLegisInitId.toString())}
								<ExpandablePlaceholder class="w-80!" />
							{:then voteResult}
								{#if !isHasError(voteResult)}
									<VoteResultExpandableBar {voteResult} class="mt-1!" />
								{/if}
							{/await}
						{/each}
					{/if}
				{:else}
					<ExpandablePlaceholder />
				{/if}
			</div>

			<!-- Favorite gov proposals Card -->
			<!-- <h2 class="text-xl font-bold text-gray-900 dark:text-gray-50">Favorisierte Ministerialentwürfe</h2>
			<div class="flex flex-wrap gap-3">
				{#if favoLegisInits}
					{#if favoLegisInits.size == 0}
						<p class="text-gray-600 dark:text-gray-300">
							Keine favorisierten Abstimmungen vorhanden.
						</p>
					{:else}
						{#each favoLegisInits as favoLegisInitId}
							{#await vote_result_by_id(favoLegisInitId.toString())}
								<ExpandablePlaceholder class="!w-80" />
							{:then voteResult}
								{#if !isHasError(voteResult)}
									<VoteResultExpandableBar {voteResult} class="mt-1!" />
								{/if}
							{/await}
						{/each}
					{/if}
				{:else}
					<ExpandablePlaceholder />
				{/if}
			</div> -->

			<!-- Danger Zone Card -->
			<div
				class="mt-7 w-full rounded-xl border border-error-300 bg-error-50 p-4 dark:border-error-500 dark:bg-error-900/20"
			>
				<h2 class="text-xl font-bold text-error-700 dark:text-error-400">Gefahrenbereich</h2>
				<p class="mt-1 text-sm text-gray-600 dark:text-gray-300">
					Diese Aktion kann nicht rückgängig gemacht werden.
				</p>
				<div class="mt-3">
					<SButton class="bg-error-500 text-white hover:bg-error-600" onclick={handleDeleteAccount}>
						Account löschen
					</SButton>
				</div>
			</div>
		</div>
	{/if}
</Container>
