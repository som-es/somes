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
		delete_account,
		getMailSendInfo,
		getUser,
		removeUserTopic,
		renew_token,
		updateMailSendInfo
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
		type ExtendedUserInfo,
		type MailSendInfo,
		type UniqueTopic
	} from '$lib/types';
	import { Switch, Popover } from 'bits-ui';
	import { onMount } from 'svelte';
	import searchIcon from '$lib/assets/misc_icons/search-glass.svg?raw';
	import { SvelteSet } from 'svelte/reactivity';

	// State with Svelte 5 runes
	let topics = $state<UniqueTopic[]>([]);
	let selectedTopics = $state<SvelteSet<number>>(new SvelteSet<number>());
	let user = $state<BasicUserInfo | null>(null);
	let extendedUser = $state<ExtendedUserInfo | null>(null);
	let mailSendInfo = $state<MailSendInfo | null>(null);
	let favoDelegates = $state<SvelteSet<number> | null>(null);
	let favoLegisInits = $state<SvelteSet<number> | null>(null);

	let topicSearchValue = $state('');
	let isSearchPopupOpen = $state(false);

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
					<div class="flex gap-2">
						<SButton
							class="bg-secondary-500 text-white hover:bg-secondary-600"
							onclick={handleLogout}
						>
							Abmelden
						</SButton>
						<!-- <SButton
							class="bg-tertiary-500 text-black hover:bg-tertiary-600"
							onclick={() => {
								jwtStore.value = null;
								gotoHistory('/home');
							}}
						>
							E-Mail wechseln
						</SButton> -->
					</div>
				</div>
			</div>

			<!-- Email Notifications Card -->
			<div class="w-full rounded-xl bg-primary-300 p-4 dark:bg-primary-500">
				<h2 class="text-xl font-bold text-gray-900 dark:text-gray-50">E-Mail Benachrichtigungen</h2>

				{#if !extendedUser?.is_email_hashed}
					<div class="mt-4 grid gap-4 sm:grid-cols-2">
						{#if mailSendInfo}
							<div class="flex items-start gap-3">
								<Switch.Root
									bind:checked={mailSendInfo.send_new_vote_results_mails}
									onCheckedChange={updateThisMailSendInfo}
									class="peer inline-flex h-[24px] w-[44px] shrink-0 cursor-pointer items-center rounded-full border-2 border-transparent transition-colors focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:ring-offset-white focus-visible:outline-none disabled:cursor-not-allowed disabled:opacity-50 data-[state=checked]:bg-secondary-500 data-[state=unchecked]:bg-gray-300"
									id="sendVoteResultInfoMail"
								>
									<Switch.Thumb
										class="pointer-events-none block h-5 w-5 rounded-full bg-white shadow-lg ring-0 transition-transform data-[state=checked]:translate-x-5 data-[state=unchecked]:translate-x-0"
									/>
								</Switch.Root>
								<label class="cursor-pointer" for="sendVoteResultInfoMail">
									<div class="flex flex-col">
										<span class="font-semibold text-gray-900 dark:text-gray-50"
											>Zu neuen Abstimmungen</span
										>
										<span class="text-sm text-gray-600 dark:text-gray-300"
											>nach ausgewählten Interessen</span
										>
									</div>
								</label>
							</div>
							<div class="flex items-start gap-3">
								<Switch.Root
									bind:checked={mailSendInfo.send_new_vote_result_by_favo_mails}
									onCheckedChange={updateThisMailSendInfo}
									class="peer inline-flex h-[24px] w-[44px] shrink-0 cursor-pointer items-center rounded-full border-2 border-transparent transition-colors focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:ring-offset-white focus-visible:outline-none disabled:cursor-not-allowed disabled:opacity-50 data-[state=checked]:bg-secondary-500 data-[state=unchecked]:bg-gray-300"
									id="sendVoteResultInfoMailByPerson"
								>
									<Switch.Thumb
										class="pointer-events-none block h-5 w-5 rounded-full bg-white shadow-lg ring-0 transition-transform data-[state=checked]:translate-x-5 data-[state=unchecked]:translate-x-0"
									/>
								</Switch.Root>
								<label class="cursor-pointer" for="sendVoteResultInfoMail">
									<div class="flex flex-col">
										<span class="font-semibold text-gray-900 dark:text-gray-50"
											>Zu neuen Abstimmungen</span
										>
										<span class="text-sm text-gray-600 dark:text-gray-300"
											>nach favorisierten Personen</span
										>
									</div>
								</label>
							</div>
							<div class="flex items-start gap-3">
								<Switch.Root
									bind:checked={mailSendInfo.send_new_ministrial_prop_mails}
									onCheckedChange={updateThisMailSendInfo}
									class="peer inline-flex h-[24px] w-[44px] shrink-0 cursor-pointer items-center rounded-full border-2 border-transparent transition-colors focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:ring-offset-white focus-visible:outline-none disabled:cursor-not-allowed disabled:opacity-50 data-[state=checked]:bg-secondary-500 data-[state=unchecked]:bg-gray-300"
									id="sendMinistrialPropInfoMails"
								>
									<Switch.Thumb
										class="pointer-events-none block h-5 w-5 rounded-full bg-white shadow-lg ring-0 transition-transform data-[state=checked]:translate-x-5 data-[state=unchecked]:translate-x-0"
									/>
								</Switch.Root>
								<label class="cursor-pointer" for="sendMinistrialPropInfoMails">
									<div class="flex flex-col">
										<span class="font-semibold text-gray-900 dark:text-gray-50"
											>Zu neuen Ministerialentwürfen</span
										>
										<span class="text-sm text-gray-600 dark:text-gray-300"
											>nach ausgewählten Interessen</span
										>
									</div>
								</label>
							</div>
							<div class="flex items-start gap-3">
								<Switch.Root
									bind:checked={mailSendInfo.send_new_ministrial_prop_by_favo_mails}
									onCheckedChange={updateThisMailSendInfo}
									class="peer inline-flex h-[24px] w-[44px] shrink-0 cursor-pointer items-center rounded-full border-2 border-transparent transition-colors focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:ring-offset-white focus-visible:outline-none disabled:cursor-not-allowed disabled:opacity-50 data-[state=checked]:bg-secondary-500 data-[state=unchecked]:bg-gray-300"
									id="sendMinistrialPropByFavoMails"
								>
									<Switch.Thumb
										class="pointer-events-none block h-5 w-5 rounded-full bg-white shadow-lg ring-0 transition-transform data-[state=checked]:translate-x-5 data-[state=unchecked]:translate-x-0"
									/>
								</Switch.Root>
								<label class="cursor-pointer" for="sendMinistrialPropByFavoMails">
									<div class="flex flex-col">
										<span class="font-semibold text-gray-900 dark:text-gray-50"
											>Zu neuen Ministerialentwürfen</span
										>
										<span class="text-sm text-gray-600 dark:text-gray-300"
											>nach favorisierten Ministern</span
										>
									</div>
								</label>
							</div>
							<div class="flex items-start gap-3">
								<Switch.Root
									bind:checked={mailSendInfo.send_new_decree_by_favo_mails}
									onCheckedChange={updateThisMailSendInfo}
									class="peer inline-flex h-[24px] w-[44px] shrink-0 cursor-pointer items-center rounded-full border-2 border-transparent transition-colors focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:ring-offset-white focus-visible:outline-none disabled:cursor-not-allowed disabled:opacity-50 data-[state=checked]:bg-secondary-500 data-[state=unchecked]:bg-gray-300"
									id="sendMinistrialPropByFavoMails"
								>
									<Switch.Thumb
										class="pointer-events-none block h-5 w-5 rounded-full bg-white shadow-lg ring-0 transition-transform data-[state=checked]:translate-x-5 data-[state=unchecked]:translate-x-0"
									/>
								</Switch.Root>
								<label class="cursor-pointer" for="sendMinistrialPropByFavoMails">
									<div class="flex flex-col">
										<span class="font-semibold text-gray-900 dark:text-gray-50"
											>Zu neuen Verordnungen</span
										>
										<span class="text-sm text-gray-600 dark:text-gray-300"
											>nach favorisierten Ministern</span
										>
									</div>
								</label>
							</div>
							<div class="flex items-start gap-3">
								<Switch.Root
									bind:checked={mailSendInfo.send_new_decree_mails}
									onCheckedChange={updateThisMailSendInfo}
									class="peer inline-flex h-[24px] w-[44px] shrink-0 cursor-pointer items-center rounded-full border-2 border-transparent transition-colors focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:ring-offset-white focus-visible:outline-none disabled:cursor-not-allowed disabled:opacity-50 data-[state=checked]:bg-secondary-500 data-[state=unchecked]:bg-gray-300"
									id="sendMinistrialPropByFavoMails"
								>
									<Switch.Thumb
										class="pointer-events-none block h-5 w-5 rounded-full bg-white shadow-lg ring-0 transition-transform data-[state=checked]:translate-x-5 data-[state=unchecked]:translate-x-0"
									/>
								</Switch.Root>
								<label class="cursor-pointer" for="sendMinistrialPropByFavoMails">
									<div class="flex flex-col">
										<span class="font-semibold text-gray-900 dark:text-gray-50"
											>Zu neuen Verordnungen</span
										>
										<span class="text-sm text-gray-600 dark:text-gray-300"
											>nach favorisierten Themen</span
										>
									</div>
								</label>
							</div>
							<div class="flex items-start gap-3">
								<Switch.Root
									bind:checked={mailSendInfo.send_new_proposal_mails}
									onCheckedChange={updateThisMailSendInfo}
									class="peer inline-flex h-[24px] w-[44px] shrink-0 cursor-pointer items-center rounded-full border-2 border-transparent transition-colors focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:ring-offset-white focus-visible:outline-none disabled:cursor-not-allowed disabled:opacity-50 data-[state=checked]:bg-secondary-500 data-[state=unchecked]:bg-gray-300"
									id="sendProposalMails"
								>
									<Switch.Thumb
										class="pointer-events-none block h-5 w-5 rounded-full bg-white shadow-lg ring-0 transition-transform data-[state=checked]:translate-x-5 data-[state=unchecked]:translate-x-0"
									/>
								</Switch.Root>
								<label class="cursor-pointer" for="sendProposalMails">
									<div class="flex flex-col">
										<span class="font-semibold text-gray-900 dark:text-gray-50"
											>Zu neuen Anträgen</span
										>
										<span class="text-sm text-gray-600 dark:text-gray-300"
											>nach ausgewählten Themen</span
										>
									</div>
								</label>
							</div>
							<div class="flex items-start gap-3">
								<Switch.Root
									bind:checked={mailSendInfo.send_new_proposal_by_favo_mails}
									onCheckedChange={updateThisMailSendInfo}
									class="peer inline-flex h-[24px] w-[44px] shrink-0 cursor-pointer items-center rounded-full border-2 border-transparent transition-colors focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:ring-offset-white focus-visible:outline-none disabled:cursor-not-allowed disabled:opacity-50 data-[state=checked]:bg-secondary-500 data-[state=unchecked]:bg-gray-300"
									id="sendProposalMailsByDelegateFavo"
								>
									<Switch.Thumb
										class="pointer-events-none block h-5 w-5 rounded-full bg-white shadow-lg ring-0 transition-transform data-[state=checked]:translate-x-5 data-[state=unchecked]:translate-x-0"
									/>
								</Switch.Root>
								<label class="cursor-pointer" for="sendProposalMailsByDelegateFavo">
									<div class="flex flex-col">
										<span class="font-semibold text-gray-900 dark:text-gray-50"
											>Zu neuen Anträgen</span
										>
										<span class="text-sm text-gray-600 dark:text-gray-300"
											>nach favorisierten Personen</span
										>
									</div>
								</label>
							</div>
							<div class="flex items-start gap-3">
								<Switch.Root
									bind:checked={mailSendInfo.send_new_delegate_activity_mails}
									onCheckedChange={updateThisMailSendInfo}
									class="peer inline-flex h-[24px] w-[44px] shrink-0 cursor-pointer items-center rounded-full border-2 border-transparent transition-colors focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:ring-offset-white focus-visible:outline-none disabled:cursor-not-allowed disabled:opacity-50 data-[state=checked]:bg-secondary-500 data-[state=unchecked]:bg-gray-300"
									id="sendnewDelegateInfo"
								>
									<Switch.Thumb
										class="pointer-events-none block h-5 w-5 rounded-full bg-white shadow-lg ring-0 transition-transform data-[state=checked]:translate-x-5 data-[state=unchecked]:translate-x-0"
									/>
								</Switch.Root>
								<label class="cursor-pointer" for="sendnewDelegateInfo">
									<div class="flex flex-col">
										<span class="font-semibold text-gray-900 dark:text-gray-50">Zu Aktivitäten</span
										>
										<span class="text-sm text-gray-600 dark:text-gray-300"
											>nach favorisierten Personen</span
										>
									</div>
								</label>
							</div>
						{/if}
					</div>
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
					<div
						class="flex w-full items-center gap-2 rounded-xl border border-gray-300 bg-white px-3 py-2 focus-within:border-secondary-500 focus-within:ring-2 focus-within:ring-secondary-500/20 dark:bg-surface-100"
					>
						<div
							class="flex h-5 w-5 shrink-0 items-center justify-center text-gray-500 dark:text-gray-500"
						>
							{@html searchIcon}
						</div>
						<input
							class="w-full bg-transparent text-base text-gray-900 placeholder:text-gray-500 focus:outline-none dark:text-gray-800 dark:placeholder:text-gray-400"
							type="search"
							name="topic-search"
							bind:value={topicSearchValue}
							placeholder="Themen suchen..."
							onfocus={() => (isSearchPopupOpen = true)}
							oninput={() => (isSearchPopupOpen = true)}
						/>
					</div>
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
							{#each favoDelegates as favoDelegateId}
								{#await delegate_by_id(favoDelegateId)}
									<ExpandablePlaceholder class="!w-80" />
								{:then maybeDelegate}
									{#if !isHasError(maybeDelegate)}
										<DelegateCard delegate={maybeDelegate} showMoreDetailsBtn onlyTop={true} />
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
