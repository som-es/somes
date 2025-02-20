<script lang="ts">
	import { goto } from '$app/navigation';
	import { base } from '$app/paths';
	import {
	delegate,
		errorToNull,
		get_topics,
		isHasError,
		vote_result_by_id,
	} from '$lib/api/api';
	import { addUserTopic, delete_account, getMailSendInfo, getUser, removeUserTopic, renew_token, updateMailSendInfo } from '$lib/api/authed';
	import { cachedDelegateFavos, cachedLegisInitFavos } from '$lib/caching/favos';
	import { jwtStore } from '$lib/caching/stores/stores';
	import { cachedUserTopics } from '$lib/caching/user_topics_cache';
	import AutocompleteMultiselect from '$lib/components/Autocompletion/AutocompleteMultiselect.svelte';
	import { filterOptionsMultiSelect } from '$lib/components/Autocompletion/filtering';
	import type { AutocompleteOptionMultiselect } from '$lib/components/Autocompletion/types';
	import DelegateCard from '$lib/components/Delegates/DelegateCard.svelte';
	import Container from '$lib/components/Layout/Container.svelte';
	import SelectableTopics from '$lib/components/Topics/SelectableTopics.svelte';
	import SButton from '$lib/components/UI/SButton.svelte';
	import ExpandablePlaceholder from '$lib/components/VoteResults/Expandable/Placeholders/ExpandablePlaceholder.svelte';
	import VoteResult from '$lib/components/VoteResults/VoteResult.svelte';
	import { gotoHistory } from '$lib/goto';
	import { getUserFromJwt, type BasicUserInfo, type ExtendedUserInfo, type MailSendInfo, type UniqueTopic } from '$lib/types';
	import { popup, SlideToggle, type PopupSettings } from '@skeletonlabs/skeleton';
	import { onMount } from 'svelte';
	import { get } from 'svelte/store';

	let topics: UniqueTopic[] = [];
	let selectedTopics = new Set<number>();
	let user: BasicUserInfo | null;
	let extendedUser: ExtendedUserInfo | null;
	let mailSendInfo: MailSendInfo | null;
	let favoDelegates: Set<number> | null;
	let favoLegisInits: Set<number> | null;

	let autocompleteOptions: AutocompleteOptionMultiselect<string, UniqueTopic>[] = [];
	let inputValue = '';
	let allOwnTopics: UniqueTopic[] = [];
	
	function delegateFilter(): AutocompleteOptionMultiselect<string, UniqueTopic>[] {
		let _options = [...autocompleteOptions];
		let _inputValue = `${String(inputValue).toLowerCase().trim()} `;
		return filterOptionsMultiSelect(_options, _inputValue);
	}

	let popupSettings: PopupSettings = {
		event: 'focus-click',
		target: 'popupAutocomplete',
		placement: 'bottom-start',
		closeQuery: ''
	}

	export function convertDelegatesToAutocompleteOptions(): AutocompleteOptionMultiselect<string, UniqueTopic>[] {
		return topics.map((topic) => {
			return {
				right_label: "",
				isSelected: selectedTopics.has(topic.id),
				label: topic.topic,
				value: topic.id.toString(),
				keywords: `${topic.topic}`,
				meta: topic
			};
		});
	}

	$: if (selectedTopics) autocompleteOptions = convertDelegatesToAutocompleteOptions();

	onMount(async () => {
		const jwtToken = get(jwtStore);
		if (isHasError(await renew_token()) || jwtToken == null) {
			goto(`${base}/home`);
			return;
		}


		topics = errorToNull(await get_topics()) ?? [];
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
		// selectedTopics = new Set<UniqueTopic>(selectedTopics)
	});

	const updateThisMailSendInfo = async () => {
		if (!mailSendInfo) {
			return
		}
		
		await updateMailSendInfo(mailSendInfo);
	}

</script>

<Container>
	<div class="entry bg-primary-200 dark:bg-primary-400 mt-3 grid-container">
		<div
			class="title-item rounded-xl bg-primary-300 dark:bg-primary-500 px-3 py-3 items-center flex justify-between"
		>
			<h1 class="font-bold text-5xl">Benutzer</h1>
			<SButton
				class="bg-tertiary-500 text-black"
				on:click={() => {
					jwtStore.set(null);
					gotoHistory('/home');
				}}
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
					{#if user}
						{user.sub}
					{/if}
					
					{#if extendedUser?.is_email_hashed}
						<span class="ml-3 text-sm font-serif">unkenntlich</span>
					{/if}
				</div>
			</div>
			<div>
				<SButton
					class="bg-tertiary-500 text-black"
					on:click={() => {
						jwtStore.set(null);
						gotoHistory('/home');
					}}
				>
					todo: E-Mail wechseln
				</SButton>
			</div>
		</div>

		{#if !extendedUser?.is_email_hashed}
		<div
			class="title-item rounded-xl bg-primary-300 dark:bg-primary-500 px-3 py-3 items-center flex justify-between"
		>
			<div class="flex flex-wrap items-center">
				<h1 class="font-bold text-2xl">E-Mail Benachrichtigungen</h1>
				<div class="flex flex-wrap items-center gap-8 ml-5">
					{#if mailSendInfo}
						<SlideToggle active="bg-secondary-400" name="sendVoteResultInfoMail" on:change={updateThisMailSendInfo} bind:checked={mailSendInfo.send_new_vote_results_mails}>
							<span class="font-bold">
								Zu neuen Abstimmungen
							</span>	
							<br>
							<span class="text-sm">nach ausgewählten Interessen</span>
						</SlideToggle>
						<SlideToggle active="bg-secondary-400" name="sendnewDelegateInfo" on:change={updateThisMailSendInfo} bind:checked={mailSendInfo.send_new_delegate_activity_mails}>
							<span class="font-bold">
								Zu Abgeordnetenaktivitäten
							</span>	
							<br>
							<span class="text-sm">nach favorisierten Abgeordneten</span>
						</SlideToggle>
						<SlideToggle active="bg-secondary-400" name="sendMinistrialPropInfoMails" on:change={updateThisMailSendInfo} bind:checked={mailSendInfo.send_new_ministrial_prop_mails}>
							<span class="font-bold">
								Zu neuen Minisiterialentwürfen
							</span>	
							<br>
							<span class="text-sm">nach ausgewählten Interessen</span>
						</SlideToggle>
					{/if}
				</div>
			</div>
		</div>
		{/if}
		<div class="title-item rounded-xl bg-primary-300 dark:bg-primary-500 px-3 py-3">
			<h1 class="font-bold text-2xl">Wahle deine Interessen</h1>
			<!-- todo: Searchbar -->
			<input
				class="input w-[28rem] h-9 px-2"
				type="search"
				name="ac-demo"
				bind:value={inputValue}
				placeholder="Suchen..."
				use:popup={popupSettings}
			/>
			{#if autocompleteOptions}
				<div
					class="z-10 card w-full max-w-sm max-h-64 p-4 overflow-y-auto"
					data-popup="popupAutocomplete"
				>
					<AutocompleteMultiselect
						bind:input={inputValue}
						options={autocompleteOptions}
						on:selection={(event) => { 
							if (event.detail.meta) {
								if (event.detail.isSelected) {
									selectedTopics.delete(event.detail.meta.id);
									removeUserTopic({id: event.detail.meta.id, topic: ""})
								} else {
									selectedTopics.add(event.detail.meta.id); 
									addUserTopic({id: event.detail.meta.id, topic: ""})
								}

							}
							selectedTopics = selectedTopics
						}}
						emptyState={'Keine Themen gefunden'}
						filter={delegateFilter}
					/>
				</div>
			{/if}

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
							{#await delegate(favoDelegateId)}
								<ExpandablePlaceholder class="!w-80" />
							{:then maybeDelegate}
								{#if !isHasError(maybeDelegate) }
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
							{:then maybeDelegate}
								{#if !isHasError(maybeDelegate) }
									<VoteResult dels={[]} voteResult={maybeDelegate} tabindex={i} />
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
				on:click={async () => {
					await delete_account();
					jwtStore.set(null);
					gotoHistory('/home');
				}}
			>
				Account löschen
			</SButton>
		</div>
	</div>
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
