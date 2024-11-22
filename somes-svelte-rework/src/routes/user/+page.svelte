<script lang="ts">
	import { errorToNull, get_topics } from '$lib/api';
	import { jwtStore } from '$lib/caching/stores/stores';
	import Container from '$lib/components/Layout/Container.svelte';
	import SelectableTopics from '$lib/components/Topics/SelectableTopics.svelte';
	import SButton from '$lib/components/UI/SButton.svelte';
	import { gotoHistory } from '$lib/goto';
	import type { Topic } from '$lib/types';
	import { onMount } from 'svelte';

	let topics: Topic[] = [];
	let selectedTopics = new Set<Topic>();

	onMount(async () => {
		topics = errorToNull(await get_topics()) ?? [];

		// get topics from api
	});

</script>

<Container>
	<div class=" entry bg-primary-200 dark:bg-primary-400 mt-3 grid-container">
		<div class="title-item rounded-xl bg-primary-300 dark:bg-primary-500 px-3 py-3 flex justify-between ">
			<h1 class="font-bold text-3xl">
				Benutzer
			</h1>
			<SButton
				class="bg-tertiary-500 text-black"
				on:click={() => {
					jwtStore.set(null);
					gotoHistory("/home");
			}}>
				Abmelden
			</SButton>
			<!-- <span class="text-xl">{voteResult.legislative_initiative.description}</span> -->
		</div>
		<div class="title-item rounded-xl bg-primary-300 dark:bg-primary-500 px-3 py-3 ">
			<h1 class="font-bold text-2xl">
				Deine Interessen
			</h1>
			<div class="mt-3">
				{#if topics}
					<SelectableTopics bind:selectedTopics topics={topics} />
				{/if}
			</div>
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