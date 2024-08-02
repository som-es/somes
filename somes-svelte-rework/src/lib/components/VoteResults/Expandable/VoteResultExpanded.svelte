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

    function dashDateToDotDate(date: string): string {
        const dateParts = date.split('-');

        return `${dateParts[2]}.${dateParts[1]}.${dateParts[0]}`
    }

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
                <ul class="mt-1 list fill-primary-400">
                    {#each emphasis as emph}
                        <li>
                            <span class="badge bg-primary-500"></span>
                            <span>{emph}</span>
                        </li>
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
        <div class="majority-item square flex bg-primary-300">
            <div class="flex flex-col items-center justify-center">
                <div class="donut" id="{voteResult.legislative_initiative.requires_simple_majority ? "donut-simple-majority": "donut-other-majority"}"></div>
                <div>
                    Notwendige
                </div>
                <div>
                    Mehrheit
                </div>
                <!-- {voteResult.legislative_initiative.requires_simple_majority ? "1/2" : "2/3" } -->

            </div> 
        </div>
        <div class="accepted-item square bg-primary-300">
            <div class="flex flex-col items-center justify-center">
                <div class="bold font-bold text-lg">
                    {dashDateToDotDate(voteResult.legislative_initiative.created_at.toString())}
                </div>
                <div>
                    Abgestimmt am 
                </div>
            </div>
        </div>
    </div>
    <div class="ml-auto details-item mt-auto"><SButton class="bg-tertiary-500">Details anzeigen</SButton></div>
</div>

<style>
    .entry {
		border-radius: 0.9rem;
		box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
		padding: 20px;
		gap: 10px;
	}
    .grid-container {
		box-sizing: border-box;
		display: grid;
		grid-template-areas:
			'e e e e e p p p' /* e: emphasis, p: parliament */
			'e e e e e p p p'
			'e e e e e p p p'
			'i i i i i t d d'; /* a: accepted, m: majority? 2/3, 1/2, dt: date, d: details */
		/* "i i i a"; */
		padding: 10px;
	}

	.square {
		/* aspect-ratio: 1/ 1; */
        min-width: 140px;
        min-height: 140px;
		/* padding: 5%; */
        display: flex;
        justify-content: center;
        align-content: center;
        border-radius: 1rem;
	}

	.parliament-item {
		grid-area: p;
		border-radius: 2rem;
	}

	.topics-item {
		grid-area: t;
		border-radius: 2rem;
        /* overflow: hidden; */
        /* min-width: 0;*/
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

    #donut-simple-majority {
        width: 60px; height: 60px;
        border-radius: 50%;

        background: conic-gradient(
            rgb(var(--color-secondary-400)) 0deg 180deg,
            rgb(var(--color-primary-600)) 180deg 360deg
        );
    }
    #donut-other-majority {
        width: 60px; height: 60px;
        border-radius: 50%;

        background: conic-gradient(
            rgb(var(--color-secondary-400)) 0deg 240deg,
            rgb(var(--color-primary-600))240deg 360deg
        );
    }
    
    .donut::before {
        content: "";
        width: 40px; height: 40px;
        border-radius: 50%;
        /* background: rgb(var(--bg-primary-300)); */
        background: rgb(var(--color-primary-300));
    }
    .donut {
        display: flex;
        align-items: center;
        justify-content: center;
    }
</style>