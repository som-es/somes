<script lang="ts">
	import type { DelegateData } from "$lib/types";
	import { filter, ListBox, ListBoxItem, popup, type PopupSettings } from "@skeletonlabs/skeleton";
	import LegisButtons from "../Filtering/LegisButtons.svelte";
	import { onMount } from "svelte";

    export let makeRequest: (gp: string | null, gender: string | null) => Promise<DelegateData[]>;

    let x = 0;

    let currentData: DelegateData[];
    let filteredData: DelegateData[];
    let filterParties: string[];

    onMount(async () => {
        currentData = await makeRequest(null, null);
    })

    const popupParty: PopupSettings = {
		event: 'click',
		target: 'popupParty',
		placement: 'bottom',
		closeQuery: '.listbox-item'
	};

    let selectedPeriod: string = "all";
    let gender: string | null = null
    let uniqueParties: string[] = []


    $: {
        let gp: string | null = selectedPeriod
        if (selectedPeriod == "all") {
            gp = null
        }
        makeRequest(gp, gender).then(res => {
            currentData = res;
        });

        filterParties = []
    }

    $: if (currentData) {
        filteredData = currentData;

        currentData.forEach(data => {
            if (!uniqueParties.includes(data.party)) {
                uniqueParties.push(data.party)
            }
        })
    }

    $: if (filterParties) {
        filteredData = filteredData.filter(data => {
            return filterParties.includes(data.party)
        })
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
</div>

<div class="card w-48 shadow-xl py-2" data-popup="popupParty">
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

<style>

</style>