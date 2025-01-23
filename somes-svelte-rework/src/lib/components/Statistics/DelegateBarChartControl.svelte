<script lang="ts">
	import type { DelegateData } from "$lib/types";
	import { filter, ListBox, ListBoxItem, popup, type PopupSettings } from "@skeletonlabs/skeleton";
	import LegisButtons from "../Filtering/LegisButtons.svelte";
	import { onMount } from "svelte";
	import ReactiveDelegateBarChart from "./ReactiveDelegateBarChart.svelte";

    export let makeRequest: (gp: string | null, gender: string | null) => Promise<DelegateData[]>;

    let currentData: DelegateData[] = [];
    let filteredData: DelegateData[] = [];
    let filterParties: string[] = [];

    const addUniqueParties = () => {
        uniqueParties = []
        currentData.forEach(data => {
            if (!uniqueParties.includes(data.party)) {
                uniqueParties.push(data.party)
            }
        })
        uniqueParties = uniqueParties
    }

    onMount(async () => {
        currentData = await makeRequest(null, null);
        filteredData = currentData;
    })

    const popupParty: PopupSettings = {
		event: 'click',
		target: 'popupParty',
		placement: 'bottom',
	};

    const popupGender: PopupSettings = {
		event: 'click',
		target: 'popupGender',
		placement: 'bottom',
        closeQuery: '.listbox-item'
	};

    let selectedPeriod: string = "all";
    let gender: string | undefined = undefined
    let uniqueParties: string[] = []

    $: if (filterParties && filterParties.length > 0) {
        filteredData = currentData.filter(data => {

            return filterParties.includes(data.party)
        })
    } else {
        filteredData = currentData;
    }

    const resetFilterParties = () => {
        filterParties = []
    }


    $: if(selectedPeriod || gender) {
        let gp: string | null = selectedPeriod
        if (selectedPeriod == "all") {
            gp = null
        }
        makeRequest(gp, gender ?? null).then(res => {
            currentData = res;
        });

        resetFilterParties()
    }

    $: if (currentData) {
        filteredData = currentData;
        addUniqueParties();
    }


    const translatePartyFilter = (filterParties: string[]) => {
        if (filterParties.length == 0) {
            return "Alle"
        }
        if (filterParties.length == 1) {
            return filterParties[0]
        }
        return `${filterParties[0]}, ...`
    }

    const translateGenderFilter = (gender: string | undefined) => {
        if (!gender) {
            return "egal"
        }
        if (gender == "f") {
            return "weiblich"
        }
        if (gender == "m") {
            return "männlich"
        }
    }
    
</script>

<LegisButtons bind:selectedPeriod={selectedPeriod} />

<div class="flex flex-wrap gap-6">
	<div>
		<h1 class="text-2xl font-bold">Partei</h1>
		<button
			class="btn variant-filled-secondary w-48 justify-between"
			use:popup={popupParty}
		>
			<span class="capitalize">{translatePartyFilter(filterParties)}</span>
			<span>↓</span>
		</button>
	</div>
	<div>
		<h1 class="text-2xl font-bold">Geschlecht</h1>
		<button
			class="btn variant-filled-secondary w-48 justify-between"
			use:popup={popupGender}
		>
			<span class="capitalize">{translateGenderFilter(gender)}</span>
			<span>↓</span>
		</button>
	</div>
</div>

<div class="z-10 card w-48 shadow-xl py-2" data-popup="popupParty">
	<ListBox
		rounded="rounded-container-token sm:!rounded-token"
		active="variant-filled-secondary"
		hover="hover:variant-soft-secondary"
        multiple
	>
        {#each uniqueParties as party}
            <ListBoxItem bind:group={filterParties} name="partyFilter" value={party}
                >{party}</ListBoxItem
            >
        {/each}
	</ListBox>
</div>
<div class="z-10 card w-48 shadow-xl py-2" data-popup="popupGender">
	<ListBox
		rounded="rounded-container-token sm:!rounded-token"
		active="variant-filled-secondary"
		hover="hover:variant-soft-secondary"
	>
        <ListBoxItem bind:group={gender} name="genderName" value={undefined}
			>egal</ListBoxItem
		>
		<ListBoxItem bind:group={gender} name="genderName" value={"f"}
			>weiblich</ListBoxItem
		>
		<ListBoxItem bind:group={gender} name="genderName" value={"m"}
			>männlich</ListBoxItem
		>
	</ListBox>
</div>

<ReactiveDelegateBarChart delegateData={filteredData} title={"Redezeit in Sekunden"} />

<style>

</style>