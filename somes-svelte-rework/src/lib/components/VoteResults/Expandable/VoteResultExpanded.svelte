<script lang="ts">
	import VoteParliament from "$lib/components/Parliaments/VoteParliament.svelte";
	import Topics from "$lib/components/Topics/Topics.svelte";
	import SButton from "$lib/components/UI/SButton.svelte";
	import crossmarkIcon from '$lib/assets/misc_icons/crossmark.svg?raw';
	import checkmarkIcon from '$lib/assets/misc_icons/checkmark.svg?raw';
	import type { Delegate, VoteResult } from "$lib/types";

	export let voteResult: VoteResult;
	export let dels: Delegate[];

	const emphasis = voteResult.legislative_initiative.emphasis
		?.split('\n\t')
		.filter((x) => x.length > 0);

</script>

<div class="lg:!hidden entry bg-primary-200 dark:bg-primary-400 mt-3">
    <div class="">
        {#if emphasis}
            <ul>
                {#each emphasis as emph}
                    <li>- {emph}</li>
                {/each}
            </ul>
        {/if}
    </div>
    <div class="rounded-md w-full">
        <VoteParliament {dels} {voteResult} preview={true} />
    </div>

    <div class="flex justify-between"> 
        <div class="accepted-item bg-primary-300">Angenommen: {voteResult.legislative_initiative.accepted}</div>
        <div class="ml-auto more-info-item"><SButton class="bg-tertiary-500">Details anzeigen</SButton></div>
    </div>

</div>
<div class="max-lg:!hidden entry bg-primary-200 dark:bg-primary-400 mt-3  grid-container">
    <!-- Inneres Migration Frauen Klimaschutz -->

    {#if emphasis}
        {#if emphasis.length > 0}
            <div class="emphasis-item bg-primary-300 px-10">
                <ul class="mt-1">
                    {#each emphasis as emph}
                        <li>- {emph}</li>
                    {/each}
                </ul>
            </div>
        {:else}
            <div class="emphasis-item"></div>
        {/if}
    {/if}

    <div class="topics-item flex justify-center items-center bg-primary-300 px-4">
        <Topics topics={voteResult.topics} />
    </div>

    <div class="rounded-md w-80 max-w-full ml-auto parliament-item bg-primary-100">
        <VoteParliament {dels} {voteResult} preview={true} />
    </div>
    <div class="flex info-item gap-3">

        <div class="accepted-item square bg-primary-300">
            <div class="flex flex-col items-center justify-center">
                {#if voteResult.legislative_initiative.accepted}	
                    {@html checkmarkIcon}
                    <div>
                        Angenommen
                    </div>
                {:else}
                    {@html crossmarkIcon}
                    <div>
                        Abgelehnt
                    </div>
                {/if}
            </div>
            
            <!-- Angenommen: {voteResult.legislative_initiative.accepted} -->
        </div>
        <div class="majority-item square flex bg-primary-300">Notwendige Mehrheit: {voteResult.legislative_initiative.requires_simple_majority ? "1/2" : "2/33" }</div>
        <div class="accepted-item square flex bg-primary-300">Abgestimmt am {voteResult.legislative_initiative.created_at}</div>
    </div>
    <div class="ml-auto details-item mt-auto"><SButton class="bg-tertiary-500">Details anzeigen</SButton></div>
</div>

<style>

    .grid-container {
		box-sizing: border-box;
		display: grid;
		grid-template-areas:
			'e e e e e p p p' /* e: emphasis, p: parliament */
			'e e e e e p p p'
			'e e e e e p p p'
			'i i i i t t d d'; /* a: accepted, m: majority? 2/3, 1/2, dt: date, d: details */
		/* "i i i a"; */
		padding: 10px;
        gap: 10px;
	}

	.square {
		/* aspect-ratio: 1/ 1; */
        width: 200px;
        height: 200px;
		/* padding: 5%; */
        display: flex;
        justify-content: center;
        align-content: center;
		color: #fff;
	}

	.parliament-item {
		grid-area: p;
		border-radius: 2rem;
	}

	.topics-item {
		grid-area: t;
		border-radius: 2rem;
        overflow: hidden;  /* NEW */
        min-width: 0;      /* NEW; needed for Firefox */
	}

	.emphasis-item {
		grid-area: e;
		border-radius: 2rem;
	}

	.accepted-item {
		grid-area: a;
	}

	.majority-item {
		grid-area: m;
	}

	.date-item {
		grid-area: dt;
	}

	.info-item {
		grid-area: i;
        overflow: hidden;  /* NEW */
        min-width: 0;      /* NEW; needed for Firefox */
	}

	.details-item {
		grid-area: d;
	}

	.item {
		grid-column: 1fr;
	}
</style>