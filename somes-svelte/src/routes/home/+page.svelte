<script lang="ts">
	import AutoSelectParliament from "@/components/AutoSelectParliament.svelte";
	import type { Delegate, Party, VoteResult } from "$lib/types";
	import { delegates, latest_vote_results, parties } from "$lib/api";
	import { onMount } from "svelte";
	import { get, type Writable } from "svelte/store";
	import { localStorageStore } from "@skeletonlabs/skeleton";
	import VoteParliament from "@/components/VoteParliament.svelte";
	import { goto } from "$app/navigation";
	import CallToOrdersPerPartyDelegates from "@/components/CallToOrdersPerPartyDelegates.svelte";
	import CallToOrders from "@/components/CallToOrders.svelte";
	import SpeakersByHours from "@/components/SpeakersByHours.svelte";
	import { maybeGetUser } from "$lib/api/user";
	import { userStore } from "../../stores/stores";
	import { t } from "$lib/translations";
	import { browser } from "$app/environment";
	import { base } from "$app/paths";

	let dels: Delegate[];

	const currentLegisInitStorage: Writable<VoteResult | null> = localStorageStore(
		"selectedVoteResult",
		null,
	);
	const partyColorStorage: Writable<string> = localStorageStore("parties", "");

	let voteResults: VoteResult[];

	async function updateColorStorage() {
		let partyToColor = new Map<string, string>();
		(await parties()).forEach((party) => {
			partyToColor.set(party.name, party.color);
		});
		partyColorStorage.set(JSON.stringify(Array.from(partyToColor.entries())));
	}
	$: welcomeMessage = $t("common.greeting");
	let welcomeName = "";

	const user = get(userStore);
	if (user != null) {
		welcomeMessage = $t("common.greeting_concat");
		welcomeName = user.username;
	}

	onMount(async function () {
		await updateColorStorage();

		const austrianDelegates = await delegates();
		dels = austrianDelegates.filter((delegate) => delegate.council === "nr");

		voteResults = (await latest_vote_results()).slice(0, 6);
	});
</script>

<div class="container mx-auto px-4 mt-5">
	<h1 class="text-primary-400">
		{welcomeMessage}{#if welcomeName} <span class="text-tertiary-400">{welcomeName}</span>!{/if}
	</h1>
	<h2 class="mt-5">{$t("common.national_council")}</h2>
	{$t("common.current_news")}

	{#if voteResults}
		{#if voteResults.length == 0}
			<p class="no-news">{$t("common.news_unavailable")}</p>
		{/if}
		<div class="card-container">
			{#each voteResults as voteResult, i}
				<span class="card tile">
					<div class="tile-content">
						<div
							class="mx-1 w-[360px]"
							on:click={() => {
								currentLegisInitStorage.set(voteResult);
								$: if (browser) {
									goto(`${base}/vote`)
								}
							}}
							on:keypress={() => {
								currentLegisInitStorage.set(voteResult);
								$: if (browser) {
									goto(`${base}/vote`)
								}
							}}
							role="link"
							tabindex={10 + i}
						>
							<VoteParliament {dels} seats={[20, 27, 37, 43, 48, 54]} {voteResult} preview={true} />
						</div>
						<span class="mx-3 text-left">{voteResult.legislative_initiative.description}</span>
					</div>
				</span>
			{/each}
		</div>
	{:else}
		<p class="loading">{$t("common.loading")}</p>
		<!--
    <div class="flex flex-wrap">
        <LegisInitCard voteResult={voteResults[0]} dels={dels} />
        <LegisInitCard voteResult={voteResults[1]} dels={dels} />
        <LegisInitCard voteResult={voteResults[2]} dels={dels} />
    </div>
    -->
	{/if}

	<!--<div class="grid-container gap-5">
        <div class="grid-item item2 rounded">1</div>
        <div class="grid-item rounded">2</div>
        <div class="grid-item rounded">2</div>
    </div>-->

	<h2 class="mt-5">{$t("common.representatives")}</h2>

	{#if dels}
		<AutoSelectParliament {dels} seats={[20, 27, 37, 43, 48, 54]} />
	{:else}
		<p class="loading">{$t("common.loading")}</p>
	{/if}

	<h2 class="mt-5">{$t("common.statistics")}</h2>
	<h4>{$t("common.statistics_description")}</h4>
	<div>
		<p class="mt-3">
			{$t("common.top_cto")}
			<CallToOrders />
		</p>
		<p class="mt-3">
			{$t("common.top_speakers")}
			<SpeakersByHours />
		</p>

		<p class="mt-3">
			<!-- TODO: add translation -->
			Call to orders of parties relative to the number of delegates in the Nationalrat = get issuer of
			call to order (wie sind die CTOs verteilt in Bezug auf den aktiven Präsidenten? (ÖVP, SPÖ, FPÖ))
			"Forschungsfrage": Warum hat die ÖVP so wenige CTOs über die Zeit?
			<CallToOrdersPerPartyDelegates />
		</p>
	</div>
</div>

<style>
	.tile {
		box-sizing: border-box;
		padding: 0;
		border-radius: 25px;
		position: relative;
		z-index: 1;
		overflow: hidden;
		width: 25rem;
		margin: 0.8rem;
	}

	.tile-content {
		display: flex;
		justify-content: center;
		flex-direction: column;
		align-items: center;
	}

	.legis-init-label {
		width: 5%;
	}

	.card-container {
		margin: auto;
		display: flex;
		flex-direction: row;
		flex-wrap: wrap;
	}

	@media (max-width: 600px) {
		.legis-init-label {
			width: 100%; /* Adjust the width as per your requirement */
		}
	}
</style>
