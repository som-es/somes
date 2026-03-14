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
	import { filterOptionsMultiSelect } from '$lib/components/Autocompletion/filtering';
	import type { AutocompleteOptionMultiselect } from '$lib/components/Autocompletion/types';
	import AutocompleteMultiselect from '$lib/components/Autocompletion/AutocompleteMultiselect.svelte';
	import DelegateCard from '$lib/components/Delegates/DelegateCard.svelte';
	import Container from '$lib/components/Layout/Container.svelte';
	import SelectableTopics from '$lib/components/Topics/SelectableTopics.svelte';
	import SButton from '$lib/components/UI/SButton.svelte';
	import ExpandablePlaceholder from '$lib/components/VoteResults/Expandable/Placeholders/ExpandablePlaceholder.svelte';
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
	import VoteResultExpandableBar from '$lib/components/VoteResults/Expandable/VoteResultExpandableBar.svelte';

	// State with Svelte 5 runes
	let topics = $state<UniqueTopic[]>([]);
	let selectedTopics = $state<Set<number>>(new Set<number>());
	let user = $state<BasicUserInfo | null>(null);
	let extendedUser = $state<ExtendedUserInfo | null>(null);
	let mailSendInfo = $state<MailSendInfo | null>(null);
	let favoDelegates = $state<Set<number> | null>(null);
	let favoLegisInits = $state<Set<number> | null>(null);

	let autocompleteOptions = $state<AutocompleteOptionMultiselect<string, UniqueTopic>[]>([]);
	let inputValue = $state('');
	let allOwnTopics = $state<UniqueTopic[]>([]);

	function delegateFilter(): AutocompleteOptionMultiselect<string, UniqueTopic>[] {
		let _options = [...autocompleteOptions];
		let _inputValue = `${String(inputValue).toLowerCase().trim()} `;
		return filterOptionsMultiSelect(_options, _inputValue);
	}

	function convertDelegatesToAutocompleteOptions(): AutocompleteOptionMultiselect<
		string,
		UniqueTopic
	>[] {
		return topics.map((topic) => {
			return {
				right_label: '',
				isSelected: selectedTopics.has(topic.id),
				label: topic.topic,
				value: topic.id.toString(),
				keywords: `${topic.topic}`,
				meta: topic
			};
		});
	}

	$effect(() => {
		if (selectedTopics) {
			autocompleteOptions = convertDelegatesToAutocompleteOptions();
		}
	});

	onMount(async () => {
		const jwtToken = jwtStore.value;
		if (isHasError(await renew_token()) || jwtToken == null) {
			goto(resolve("/home"))
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
			allOwnTopics = data;
			selectedTopics = new Set<number>(data.map((topic) => topic.id));
		}
	});

	const updateThisMailSendInfo = async () => {
		if (!mailSendInfo) {
			return;
		}

		await updateMailSendInfo(mailSendInfo);
	};

	function handleTopicSelection(event: AutocompleteOptionMultiselect<string, UniqueTopic>) {
		if (event.meta) {
			if (event.isSelected) {
				selectedTopics.delete(event.meta.id);
				removeUserTopic({ id: event.meta.id, topic: '' });
			} else {
				selectedTopics.add(event.meta.id);
				addUserTopic({ id: event.meta.id, topic: '' });
			}
		}
		// Trigger reactivity
		selectedTopics = selectedTopics;
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

<Container>
	{#if extendedUser}
		<div class="entry bg-primary-200 dark:bg-primary-400 mt-3 grid-container">
			<div
				class="title-item rounded-xl bg-primary-300 dark:bg-primary-500 px-3 py-3 items-center flex justify-between"
			>
				<h1 class="font-bold text-5xl">Benutzer</h1>
				<SButton
					class="bg-tertiary-500 text-black"
					onclick={handleLogout}
				>
					Abmelden
				</SButton>
			</div>
			<div
				class="title-item rounded-xl bg-primary-300 dark:bg-primary-500 px-3 py-3 items-center flex justify-between"
			>
				<div class="flex flex-wrap items-center">
					<h1 class="font-bold text-2xl">Benutzerinfos</h1>
					<div class="ml-5 text-xl">E-Mail</div>
					<div class="mx-4 text-xl">
						{#if extendedUser?.is_email_hashed}
							<span class="ml-3 font-serif">anonymisiert</span>
							{#if user}
								<span class="ml-1 text-sm text-wrap font-serif">...{user.sub.slice(36, 60)}...</span
								>
							{/if}
						{:else if user}
							{user.sub}
						{/if}
					</div>
				</div>
				<div>
					<SButton
						class="bg-tertiary-500 text-black"
						onclick={() => {
							jwtStore.value = null;
							gotoHistory('/home');
						}}
					>
						todo: E-Mail wechseln
					</SButton>
				</div>
			</div>

			<div
				class="title-item rounded-xl bg-primary-300 dark:bg-primary-500 px-3 py-3 items-center flex justify-between"
			>
				<div class="flex flex-wrap items-center">
					<h1 class="font-bold md:text-2xl">E-Mail Benachrichtigungen</h1>

					{#if !extendedUser?.is_email_hashed}
						<div class="flex flex-wrap items-center gap-x-6 gap-y-3 ml-5">
							{#if mailSendInfo}
								<div class="flex items-center gap-3">
									<Switch.Root
										bind:checked={mailSendInfo.send_new_vote_results_mails}
										onCheckedChange={updateThisMailSendInfo}
										class="peer inline-flex h-[24px] w-[44px] shrink-0 cursor-pointer items-center rounded-full border-2 border-transparent transition-colors focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:ring-offset-white focus-visible:outline-none disabled:cursor-not-allowed disabled:opacity-50 data-[state=checked]:bg-secondary-400 data-[state=unchecked]:bg-gray-300"
										id="sendVoteResultInfoMail"
									>
										<Switch.Thumb
											class="pointer-events-none block h-5 w-5 rounded-full bg-white shadow-lg ring-0 transition-transform data-[state=checked]:translate-x-5 data-[state=unchecked]:translate-x-0"
										/>
									</Switch.Root>
									<label class="cursor-pointer" for="sendVoteResultInfoMail">
										<div class="flex flex-col">
											<span class="font-bold">Zu neuen Abstimmungen</span>
											<span class="text-sm">nach ausgewählten Interessen</span>
										</div>
									</label>
								</div>
								<div class="flex items-center gap-3">
									<Switch.Root
										bind:checked={mailSendInfo.send_new_delegate_activity_mails}
										onCheckedChange={updateThisMailSendInfo}
										class="peer inline-flex h-[24px] w-[44px] shrink-0 cursor-pointer items-center rounded-full border-2 border-transparent transition-colors focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:ring-offset-white focus-visible:outline-none disabled:cursor-not-allowed disabled:opacity-50 data-[state=checked]:bg-secondary-400 data-[state=unchecked]:bg-gray-300"
										id="sendnewDelegateInfo"
									>
										<Switch.Thumb
											class="pointer-events-none block h-5 w-5 rounded-full bg-white shadow-lg ring-0 transition-transform data-[state=checked]:translate-x-5 data-[state=unchecked]:translate-x-0"
										/>
									</Switch.Root>
									<label class="cursor-pointer" for="sendnewDelegateInfo">
										<div class="flex flex-col">
											<span class="font-bold">Zu Abgeordnetenaktivitäten</span>
											<span class="text-sm">nach favorisierten Abgeordneten</span>
										</div>
									</label>
								</div>
								<div class="flex items-center gap-3">
									<Switch.Root
										bind:checked={mailSendInfo.send_new_ministrial_prop_mails}
										onCheckedChange={updateThisMailSendInfo}
										class="peer inline-flex h-[24px] w-[44px] shrink-0 cursor-pointer items-center rounded-full border-2 border-transparent transition-colors focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:ring-offset-white focus-visible:outline-none disabled:cursor-not-allowed disabled:opacity-50 data-[state=checked]:bg-secondary-400 data-[state=unchecked]:bg-gray-300"
										id="sendMinistrialPropInfoMails"
									>
										<Switch.Thumb
											class="pointer-events-none block h-5 w-5 rounded-full bg-white shadow-lg ring-0 transition-transform data-[state=checked]:translate-x-5 data-[state=unchecked]:translate-x-0"
										/>
									</Switch.Root>
									<label class="cursor-pointer" for="sendMinistrialPropInfoMails">
										<div class="flex flex-col">
											<span class="font-bold">Zu neuen Ministerialentwürfen</span>
											<span class="text-sm">nach ausgewählten Interessen</span>
										</div>
									</label>
								</div>
								<div class="flex items-center gap-3">
									<Switch.Root
										bind:checked={mailSendInfo.send_new_ministrial_prop_by_favo_mails}
										onCheckedChange={updateThisMailSendInfo}
										class="peer inline-flex h-[24px] w-[44px] shrink-0 cursor-pointer items-center rounded-full border-2 border-transparent transition-colors focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:ring-offset-white focus-visible:outline-none disabled:cursor-not-allowed disabled:opacity-50 data-[state=checked]:bg-secondary-400 data-[state=unchecked]:bg-gray-300"
										id="sendMinistrialPropByFavoMails"
									>
										<Switch.Thumb
											class="pointer-events-none block h-5 w-5 rounded-full bg-white shadow-lg ring-0 transition-transform data-[state=checked]:translate-x-5 data-[state=unchecked]:translate-x-0"
										/>
									</Switch.Root>
									<label class="cursor-pointer" for="sendMinistrialPropByFavoMails">
										<div class="flex flex-col">
											<span class="font-bold">Zu neuen Ministerialentwürfen</span>
											<span class="text-sm">nach favorisierten Ministern</span>
										</div>
									</label>
								</div>
							{/if}
						</div>
					{:else}
						<span class="ml-3 font-serif"
							>nicht verfügbar: Anonymisierung durch Mail-Wechsel aufheben</span
						>
					{/if}
				</div>
			</div>
			<div class="title-item rounded-xl bg-primary-300 dark:bg-primary-500 px-3 py-3">
				<h1 class="font-bold text-2xl">Wahle deine Interessen</h1>
				<!-- Searchbar with Popover -->
				<Popover.Root>
					<Popover.Trigger>
						<input
							class="input w-[28rem] h-9 px-2"
							type="search"
							name="ac-demo"
							bind:value={inputValue}
							placeholder="Suchen..."
						/>
					</Popover.Trigger>
					<Popover.Portal>
						<Popover.Content class="z-10 card w-full max-w-sm max-h-64 p-4 overflow-y-auto bg-white border border-gray-200 rounded-lg shadow-xl">
							{#if autocompleteOptions}
								<AutocompleteMultiselect
									input={inputValue}
									options={autocompleteOptions}
									onselection={handleTopicSelection}
									emptyState={'Keine Themen gefunden'}
									filter={delegateFilter}
								/>
							{/if}
						</Popover.Content>
					</Popover.Portal>
				</Popover.Root>

				<div class="mt-3">
					{#if topics}
						<SelectableTopics bind:selectedTopics {topics} />
					{/if}
				</div>
			</div>

			<div class="title-item rounded-xl bg-primary-300 dark:bg-primary-500 px-3 py-3">
				<!-- make expandable -->
				<h1 class="font-bold text-2xl">Favorisierte Abgeordnete</h1>
				<div class="flex flex-wrap mt-3 gap-3">
					{#if favoDelegates}
						{#if favoDelegates.size == 0}
							Keine favorisierten Abgeordnete vorhanden.
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

			<div class="title-item rounded-xl bg-primary-300 dark:bg-primary-500 px-3 py-3">
				<!-- make expandable -->
				<h1 class="font-bold text-2xl">Favorisierte Abstimmungen</h1>
				<div class="flex flex-wrap mt-3 gap-3">
					{#if favoLegisInits}
						{#if favoLegisInits.size == 0}
							Keine favorisierte Abstimmungen vorhanden.
						{:else}
							{#each favoLegisInits as favoLegisInitId, i}
								{#await vote_result_by_id(favoLegisInitId.toString())}
									<ExpandablePlaceholder class="!w-80" />
								{:then voteResult}
									{#if !isHasError(voteResult)}
										<VoteResultExpandableBar {voteResult} />
									{/if}
								{/await}
							{/each}
						{/if}
					{:else}
						<ExpandablePlaceholder />
					{/if}
				</div>
			</div>

			<div class="title-item rounded-xl bg-primary-300 dark:bg-primary-500 px-3 py-3">
				<SButton
					class="bg-error-300 text-black"
					onclick={handleDeleteAccount}
				>
					Account löschen
				</SButton>
			</div>
		</div>
	{/if}
</Container>

<style>
	.title-item {
		flex-basis: 100%;
	}
	.entry {
		border-radius: 0.9rem;
		box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
		padding: 20px;
		gap: 10px;
	}

	.grid-container {
		display: flex;
		flex-wrap: wrap;
	}
</style>
