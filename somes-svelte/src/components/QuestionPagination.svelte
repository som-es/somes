<script lang="ts">
	import { t } from "$lib/translations";
	import type { Question } from "$lib/types";
    import QuestionCard from "@/components/QuestionCard.svelte";
    import { FontAwesomeIcon } from '@fortawesome/svelte-fontawesome'
    import { faCaretRight } from '@fortawesome/free-solid-svg-icons'
    import { faCaretDown } from '@fortawesome/free-solid-svg-icons'
	import { userStore } from "../stores/stores";
	import { get } from "svelte/store";
    import { Paginator, type PaginationSettings } from '@skeletonlabs/skeleton';
	
    export let questions: Question[];

    const user = get(userStore);
    let expandedOwn: boolean = true;

    function flipExpand() {
        expandedOwn = !expandedOwn;
    }

    // TODO: below is just a temporary solution and will not work, once possible use the actual user id and not the username
    const ownQuestions: Question[] = questions.filter(q => q.issuer_id as unknown as string === user?.username);
    const otherQuestions: Question[] = questions.filter(q => !ownQuestions.includes(q));

    let paginationSettings = {
        offset: 0,
        limit: 10,
        size: otherQuestions.length,
        amounts: [3, 5, 10],
    } satisfies PaginationSettings;

    $: paginatedQuestions = otherQuestions.slice(
        paginationSettings.offset * paginationSettings.limit,
        paginationSettings.offset * paginationSettings.limit + paginationSettings.limit
    );
</script>

<div class="flex flex-col gap-[3vh]">
    {#if questions.length === 0}
        <p class="text-center">Es konnten keine Fragen gefunden werden.</p>
    {:else}
        {#if user && ownQuestions.length > 0}
            <div class="flex flex-col gap-[3vh]">
                <!-- svelte-ignore a11y-label-has-associated-control -->
                <!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
                <label class="label flex flex-row gap-[1rem]" on:click={flipExpand} on:keypress={flipExpand}>
                    <div class="mt-2">
                        {#if expandedOwn}
                            <FontAwesomeIcon icon={faCaretDown} class="" size="xl" />
                        {:else}
                            <FontAwesomeIcon icon={faCaretRight} class="" size="xl" />
                        {/if}
                    </div>
                    <h3 class="h3">Meine Fragen</h3>
                </label>
                {#if expandedOwn}
                    {#each ownQuestions as question}
                        <QuestionCard {question} />
                    {/each}
                {/if}
            </div>
            <hr class="opacity-100 mt-auto" />
        {/if}
        <Paginator
            bind:settings={paginationSettings}
            showFirstLastButtons="{true}"
            showPreviousNextButtons="{true}"
            amountText="Einträge"
        />
        <div class="flex flex-col gap-[5vh] mt-[2vh]">
            {#each paginatedQuestions as question}
                <QuestionCard {question} />
            {/each}
        </div>
    {/if}
</div>
