<script lang="ts">
	import { delegates, parties } from "$lib/api";
	import type { Delegate, Party } from "$lib/types";
	import { popup, Autocomplete } from "@skeletonlabs/skeleton";
	import type { PopupSettings, AutocompleteOption } from "@skeletonlabs/skeleton";
	import { onMount } from "svelte";
	import DelegateCard from "@/components/DelegateCard.svelte";

	export let selectedDel: Delegate | undefined;

	let dels: Delegate[];
	let prts: Party[];
	let constituencies: Set<string> = new Set<string>();
	let flavorOptions: AutocompleteOption[] = [];

	import { createEventDispatcher } from "svelte";
	const dispatch = createEventDispatcher();
	const dispatchUpdateDel = () => dispatch("updateDel", selectedDel);

	//$: selectedDel, dispatchUpdateDel();

	onMount(async function () {
		const austrianDelegates = await delegates();
		// use local storage to cache the delegates
		dels = austrianDelegates.filter((delegate) => delegate.council === "nr");

		dels.forEach((del) => {
			flavorOptions.push({ label: del.name, value: del, keywords: del.party });
			constituencies.add(del.constituency);
		});

		prts = await parties();
	});

	let selectedDelName: string | undefined = "";

	let selectedParty: string | undefined = undefined;
	let selectedGender: string | undefined = undefined;
	let selectedConstituency: string | undefined = undefined;

	$: filterDels = dels;

	function onDelegateSelection(event: any): void {
		selectedDel = event.detail.value;
		selectedDelName = selectedDel?.name;
		dispatchUpdateDel();
	}

	let popupSettings: PopupSettings = {
		event: "focus-click",
		target: "popupAutocomplete",
		placement: "bottom",
	};
</script>

{#if dels && prts}
	<div class="flex flex-row w-full gap-[3vw]">
		<div class="flex flex-col gap-[1vh] w-1/2">
			<label class="label">
				<span>Name</span>
				<input
					class="input autocomplete"
					autocomplete="off"
					type="search"
					name="autocomplete-search"
					bind:value={selectedDelName}
					placeholder="Name des/der Abgeordneten"
					use:popup={popupSettings}
				/>
				<!-- TODO: fix the buggy autocomplete mess -->
				<div data-popup="popupAutocomplete">
					<div class="z-50 card w-full max-w-sm max-h-48 p-4 overflow-y-auto">
						<Autocomplete
							bind:input={selectedDelName}
							options={flavorOptions}
							on:selection={onDelegateSelection}
							limit={6}
						/>
					</div>
				</div>
			</label>
			<label class="label">
				<span>Partei</span>
				<select class="select" bind:value={selectedParty}>
					{#each prts as party}
						<option value={party.name}>{party.name}</option>
					{/each}
				</select>
			</label>
			<label class="label">
				<span>Geschlecht</span>
				<select class="select" bind:value={selectedGender}>
					<option value="m">Männlich</option>
					<option value="f">Weiblich</option>
					<option value="d">Divers</option>
				</select>
			</label>
			<label class="label">
				<span>Alter</span>
				<div class="flex flex-row w-[10rem] gap-3">
					<input class="input variant-form-material" type="number" />
					bis
					<input class="input variant-form-material" type="number" />
				</div>
			</label>
			<label class="label">
				<span>Wahlkreis</span>
				<select class="select" bind:value={selectedConstituency}>
					{#each constituencies as constituency}
						<option value={constituency}>{constituency}</option>
					{/each}
				</select>
			</label>
		</div>
		<select class="select w-1/2" size="4" bind:value={selectedDel} on:change={dispatchUpdateDel}>
			{#each filterDels as del}
				<option value={del}>{del.name}</option>
			{/each}
		</select>
	</div>
{:else}
	<loading class="">loading...</loading>
{/if}
