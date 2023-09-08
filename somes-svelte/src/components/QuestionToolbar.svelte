<script lang="ts">
	import { delegates, parties } from "$lib/api";
	import { t } from "$lib/translations";
	import type { Delegate, Party } from "$lib/types";
	import { onMount } from "svelte";
    import { modalStore } from "@skeletonlabs/skeleton";
	import type { ModalSettings } from "@skeletonlabs/skeleton";

    export let search: string;
	export let sort: "relevance" | "rating" | "newest";
	export let delegate: Delegate | string;
	export let party: Party | string;
	export let dateRange: string[];

    let dels: Delegate[] = [];
    let prts: Party[];

    const modal: ModalSettings = {
		type: "component",
		component: "DelegateSelectModal",
	};

    onMount(async function () {
        const austrianDelegates = await delegates();
		dels = austrianDelegates.filter((delegate) => delegate.council === "nr");

        prts = await parties();
    });

    function triggerDelegateModal() {
		modalStore.trigger(modal);
	}
</script>

<!-- TODO: do it like the gmail web client :) -->
{#if dels && prts}
    <div class="flex flex-col gap-[1rem]">
        <div class="flex flex-row h-18 gap-[2vw]">
            <div class="flex flex-row h-[2.5rem] mt-auto gap-[0.5vw] w-1/3">
                <input
                    class="input rounded-lg p-2"
                    type="text"
                    placeholder={$t("common.enter_search")}
                    bind:value={search}
                />
                <button type="button" class="btn variant-filled">{$t("common.search")}</button>
            </div>	
            <label class="label ml-auto w-1/6">
                <span>{$t("common.sort_by")}</span>
                <select class="select" bind:value={sort}>
                    <option value="relevance">{$t("common.relevance")}</option>
                    <option value="rating">{$t("common.user_rating")}</option>
                    <option value="newest">{$t("common.newest")}</option>
                </select>
            </label>
        </div>
        <div class="flex flex-row h-18 gap-[2vw]">
            <div class="flex flex-row gap-[0.5vw] w-1/2">
                <label class="label">
                    <span>{$t("common.delegate")}</span>
                    <select class="select" bind:value={delegate}>
                        <option value="all" selected>Alle</option>
                        {#each dels as del}
                            <option value={del}>{del.name}</option>
                        {/each}
                    </select>
                </label>
                <button type="button" class="btn variant-filled h-[2.5rem] mt-auto" on:click={triggerDelegateModal}>Delegate Search</button>
            </div>
            <label class="label">
                <span>{$t("common.party")}</span>
                <select class="select" bind:value={party}>
                    <option value="all" selected>Alle</option>
                    {#each prts as party}
                        <option value={party.name}>{party.name}</option>
                    {/each}
                </select>
            </label>
            <label class="label">
                <span>{$t("common.date_range")}</span>
                <div class="flex flex-row h-10 gap-[0.5vw]">
                    <input class="input rounded-lg p-2" title="Input (date)" type="date" bind:value={dateRange[0]} />
                    <div class="mt-2">
                        {$t("common.to")}
                    </div>
                    <input class="input rounded-lg p-2" title="Input (date)" type="date" bind:value={dateRange[1]} />
                </div>
            </label>
        </div>
    </div>
    <hr class="opacity-100 mt-auto" />
{:else}
    <p class="loading">{$t("common.loading")}</p>
{/if}