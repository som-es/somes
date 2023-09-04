<script lang="ts">
	import { delegates, parties } from "$lib/api";
	import { getAge, toTSDate } from "$lib/date";
	import type { Delegate, Party } from "$lib/types";
	import { popup, Autocomplete, TabGroup, Tab } from "@skeletonlabs/skeleton";
	import type { PopupSettings, AutocompleteOption } from "@skeletonlabs/skeleton";
	import { onMount } from "svelte";

	export let selectedDel: Delegate | undefined;

	let dels: Delegate[] = [];
	let prts: Party[];
	let constituencies: Set<string> = new Set<string>();
	let flavorOptions: AutocompleteOption[] = [];
	let delegateAgeRange: number[];

	let popupSettings: PopupSettings = {
		event: "focus-click",
		target: "popupAutocomplete",
		placement: "bottom",
	};

	import { createEventDispatcher } from "svelte";
	const dispatch = createEventDispatcher();
	const dispatchUpdateDel = () => dispatch("updateDel", selectedDel);

	onMount(async function () {
		const austrianDelegates = await delegates();
		// use local storage to cache the delegates
		dels = austrianDelegates.filter((delegate) => delegate.council === "nr");

		dels.forEach((del) => {
			flavorOptions.push({ label: del.name, value: del, keywords: del.party });
			constituencies.add(del.constituency);
		});

		prts = await parties();

		delegateAgeRange = [
			getAge(new Date(Math.max(...dels.map((o) => toTSDate(o.birthdate).valueOf())))),
			getAge(new Date(Math.min(...dels.map((o) => toTSDate(o.birthdate).valueOf())))),
		];
		selectedAgeRange = delegateAgeRange.slice(0);
	});

	let selectedName = selectedDel?.name;
	let selectedParty = selectedDel?.party;
	let selectedGender = selectedDel?.gender;
	let selectedConstituency = selectedDel?.constituency;
	let selectedAgeRange: number[] = [Math.min(), Math.max()];

	$: filterDels = dels.filter((del) => {
		if (del.party !== selectedParty && selectedParty !== "all") return false;
		if (del.gender !== selectedGender && selectedGender !== "all") return false;
		if (del.constituency !== selectedConstituency && selectedConstituency !== "all") return false;
		if (
			getAge(toTSDate(del.birthdate)) < selectedAgeRange[0] ||
			getAge(toTSDate(del.birthdate)) > selectedAgeRange[1]
		)
			return false;

		return true;
	});

	function onDelegateSelection(event: any): void {
		selectedDel = event.detail.value;

		selectedName = selectedDel?.name;
		selectedParty = selectedDel?.party;
		selectedGender = selectedDel?.gender;
		selectedConstituency = selectedDel?.constituency;

		dispatchUpdateDel();
	}

	let tabSet: number = 1;
	$: tabSet, (selectedName = "");

	function resetFilters(): void {
		selectedName = "";
		selectedParty = "all";
		selectedGender = "all";
		selectedConstituency = "all";
		selectedAgeRange = delegateAgeRange;
	}
</script>

{#if dels && prts}
	<div class="flex flex-row w-full gap-[3vw]">
		<!-- TODO: fix the weird slight spacing style difference between autocomplete and selects -->
		{#if tabSet === 0}
			<div class="select w-1/2 max-h-[60vh] overflow-y-auto">
				<Autocomplete
					bind:input={selectedName}
					options={flavorOptions}
					on:selection={onDelegateSelection}
					size={4}
				/>
			</div>
		{:else}
			<select class="select w-1/2" size="4" bind:value={selectedDel} on:change={dispatchUpdateDel}>
				{#each filterDels as del}
					<option value={del}>{del.name}</option>
				{/each}
			</select>
		{/if}
		<div class="w-1/2 max-h-[60vh]">
			<TabGroup>
				<Tab bind:group={tabSet} name="name" value={0}>Namenssuche</Tab>
				<Tab bind:group={tabSet} name="filter" value={1}>Filtersuche</Tab>
				<svelte:fragment slot="panel">
					{#if tabSet === 0}
						<label class="label">
							<span>Name</span>
							<input
								class="input autocomplete variant-form-material h-10 pl-3"
								autocomplete="off"
								type="search"
								name="autocomplete-search"
								bind:value={selectedName}
								placeholder="Name des/der Abgeordneten"
								use:popup={popupSettings}
							/>
						</label>
					{:else if tabSet === 1}
						<div class="flex flex-col gap-[1vh]">
							<label class="label">
								<span>Partei</span>
								<select class="select" bind:value={selectedParty}>
									<option value="all" selected>Alle</option>
									{#each prts as party}
										<option value={party.name}>{party.name}</option>
									{/each}
								</select>
							</label>
							<label class="label">
								<span>Geschlecht</span>
								<select class="select" bind:value={selectedGender}>
									<option value="all" selected>Alle</option>
									<option value="m">Männlich</option>
									<option value="f">Weiblich</option>
									<option value="d">Divers</option>
								</select>
							</label>
							<label class="label">
								<span>Alter</span>
								<div class="flex flex-row w-[10rem] gap-3">
									<input
										class="input variant-form-material h-6 pl-3"
										type="number"
										bind:value={selectedAgeRange[0]}
										min={delegateAgeRange[0]}
										max={selectedAgeRange[1]}
									/>
									bis
									<input
										class="input variant-form-material h-6 pl-3"
										type="number"
										bind:value={selectedAgeRange[1]}
										min={selectedAgeRange[0]}
										max={delegateAgeRange[1]}
									/>
								</div>
							</label>
							<label class="label">
								<span>Wahlkreis</span>
								<select class="select" bind:value={selectedConstituency}>
									<option value="all" selected>Alle</option>
									{#each constituencies as constituency}
										<option value={constituency}>{constituency}</option>
									{/each}
								</select>
							</label>
							<button type="button" class="btn variant-filled mt-5" on:click={resetFilters}
								>Filter zurücksetzen</button
							>
						</div>
					{/if}
				</svelte:fragment>
			</TabGroup>
		</div>
	</div>
{:else}
	<loading class="">loading...</loading>
{/if}

<style>
	input[type="number"]::-webkit-inner-spin-button,
	input[type="number"]::-webkit-outer-spin-button {
		opacity: 1;
	}
</style>
