<script lang="ts">
	import { delegates, parties } from "$lib/api";
	import { t } from "$lib/translations";
	import type { Delegate, Party } from "$lib/types";
	import { onMount } from "svelte";

    let dels: Delegate[] = [];
    let prts: Party[];
    let minDate: Date = new Date(0);
    let maxDate: Date = new Date();

    onMount(async function () {
        const austrianDelegates = await delegates();
		dels = austrianDelegates.filter((delegate) => delegate.council === "nr");

        prts = await parties();
    });
</script>

<!-- TODO: do it like the gmail web client :) -->
{#if dels && prts}
    <div class="flex flex-col gap-[1rem]">
        <div class="flex flex-row h-18 gap-[2vw]">
            <div class="flex flex-row h-[2.5rem] mt-auto gap-[0.5vw]">
                <input
                    class="input rounded-lg p-2"
                    type="text"
                    placeholder={$t("common.enter_search")}
                />
                <button type="button" class="btn variant-filled">{$t("common.search")}</button>
            </div>	
            <label class="label ml-auto w-1/6">
                <span>{$t("common.sort_by")}</span>
                <select class="select">
                    <option value="1">{$t("common.relevance")}</option>
                    <option value="2">{$t("common.user_rating")}</option>
                    <option value="3">{$t("common.newest")}</option>
                </select>
            </label>
        </div>
        <div class="flex flex-row h-18 gap-[2vw]">
            <label class="label">
                <span>{$t("common.delegate")}</span>
                <select class="select">
                    {#each dels as del}
                        <option value={del.id}>{del.name}</option>
                    {/each}
                </select>
            </label>
            <label class="label">
                <span>{$t("common.party")}</span>
                <select class="select">
                    {#each prts as party}
                        <option value={party.name}>{party.name}</option>
                    {/each}
                </select>
            </label>
            <label class="label">
                <span>{$t("common.date_range")}</span>
                <div class="flex flex-row h-10 gap-[0.5vw]">
                    <input class="input rounded-lg p-2" title="Input (date)" type="date" bind:value={minDate} />
                    <div class="mt-2">
                        {$t("common.to")}
                    </div>
                    <input class="input rounded-lg p-2" title="Input (date)" type="date" bind:value={maxDate} />
                </div>
            </label>
        </div>
    </div>
    <hr class="opacity-100 mt-auto" />
{:else}
    <p class="loading">{$t("common.loading")}</p>
{/if}