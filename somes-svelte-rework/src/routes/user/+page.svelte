<script lang="ts">
	import { goto } from '$app/navigation';
	import { base } from '$app/paths';
	import {
		errorToNull,
		get_topics,
		isHasError,
	} from '$lib/api/api';
	import { delete_account, renew_token } from '$lib/api/authed';
	import { jwtStore } from '$lib/caching/stores/stores';
	import { cachedUserTopics } from '$lib/caching/user_topics_cache';
	import Container from '$lib/components/Layout/Container.svelte';
	import SelectableTopics from '$lib/components/Topics/SelectableTopics.svelte';
	import SButton from '$lib/components/UI/SButton.svelte';
	import { gotoHistory } from '$lib/goto';
	import { getUserFromJwt, type BasicUserInfo, type Topic, type UniqueTopic } from '$lib/types';
	import { onMount } from 'svelte';
	import { get } from 'svelte/store';

	let topics: UniqueTopic[] = [];
	let selectedTopics = new Set<number>();
	let user: BasicUserInfo | null;

	onMount(async () => {
		const jwtToken = get(jwtStore);
		if (isHasError(await renew_token()) || jwtToken == null) {
			goto(`${base}/home`);
			return;
		}


		topics = errorToNull(await get_topics()) ?? [];
		user = getUserFromJwt(jwtToken);

		// get interest topics from api
		const data = await cachedUserTopics(true);

		if (data) {
			selectedTopics = new Set<number>(data.map((topic) => topic.id));
		}
		// selectedTopics = new Set<UniqueTopic>(selectedTopics)
	});
</script>

<Container>
	<div class="entry bg-primary-200 dark:bg-primary-400 mt-3 grid-container">
		<div
			class="title-item rounded-xl bg-primary-300 dark:bg-primary-500 px-3 py-3 items-center flex justify-between"
		>
			<h1 class="font-bold text-3xl">Benutzer</h1>
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
		<div class="title-item rounded-xl bg-primary-300 dark:bg-primary-500 px-3 py-3">
			<h1 class="font-bold text-2xl">Wahle deine Interessen</h1>
			todo: Searchbar
			<div class="mt-3">
				{#if topics}
					<SelectableTopics bind:selectedTopics {topics} />
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
