<script lang="ts">
	import VoteResults from '$lib/components/VoteResults/VoteResults.svelte';
	import { filteredDelegates } from '$lib/caching/delegates';
	import type { Delegate, VoteResult } from '$lib/types';
	import { onMount } from 'svelte';
	import { cachedLatestVoteResults } from '$lib/caching/vote_results';
	import Container from '$lib/components/Layout/Container.svelte';

	let dels: Delegate[] | null = null;
	let voteResults: VoteResult[] | null = null;
	onMount(async function () {
		// await updateColorStorage();
		dels = (await filteredDelegates())?.nr ?? null;
		voteResults = await cachedLatestVoteResults();
	});
</script>

<Container>
	home
	{#if voteResults && dels}
		<VoteResults {dels} {voteResults} />
	{:else}
		<section class="card w-full animate-pulse">
			<div class="p-4 space-y-4">
				<div class="placeholder" />
				<div class="grid grid-cols-3 gap-8">
					<div class="placeholder" />
					<div class="placeholder" />
					<div class="placeholder" />
				</div>
				<div class="grid grid-cols-4 gap-4">
					<div class="placeholder" />
					<div class="placeholder" />
					<div class="placeholder" />
					<div class="placeholder" />
				</div>
				<div class="grid grid-cols-3 gap-8">
					<div class="placeholder" />
					<div class="placeholder" />
					<div class="placeholder" />
				</div>
				<div class="grid grid-cols-2 gap-5">
					<div class="placeholder" />
					<div class="placeholder" />
				</div>
				<div class="grid grid-cols-3 gap-7">
					<div class="placeholder" />
					<div class="placeholder" />
					<div class="placeholder" />
				</div>
				<div class="grid grid-cols-4 gap-3">
					<div class="placeholder" />
					<div class="placeholder" />
					<div class="placeholder" />
					<div class="placeholder" />
				</div>
			</div>
		</section>
	{/if}
</Container>
