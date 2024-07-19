<script lang="ts">
	import type { Delegate, VoteResult } from '$lib/types';
	import VoteParliament from '../../Parliaments/VoteParliament.svelte';
	import collapse from 'svelte-collapse';
	import upArrowIcon from '$lib/assets/misc_icons/up-arrow.svg?raw';
	import downArrowIcon from '$lib/assets/misc_icons/down-arrow.svg?raw';
	import SButton from '$lib/components/UI/SButton.svelte';

	export let voteResult: VoteResult;
	export let dels: Delegate[];
	let clazz;
	export { clazz as class };
	let open = false;

	const emphasis = voteResult.legislative_initiative.emphasis
		?.split('\n\t')
		.filter((x) => x.length > 0);
</script>

<div class="gap-3 mt-5">
	<div
		on:click={() => (open = !open)}
		on:keypress={() => (open = !open)}
		role="button"
		tabindex="0"
		class="entry bg-primary-300 dark:bg-primary-500 flex justify-between items-center"
	>
		<div>
			{#if open}
				{@html upArrowIcon}
			{:else}
				{@html downArrowIcon}
			{/if}
		</div>
		<div>{voteResult.legislative_initiative.title}</div>
		<div class="w-20 bg-primary-400 dark:bg-primary-600 rounded-md">
			<VoteParliament {dels} {voteResult} preview={true} />
		</div>
	</div>

	<div use:collapse={{ open }}>
		<div class="sm:!hidden entry bg-primary-200 dark:bg-primary-400 mt-3">
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
				<!-- <div class="accepted-item square bg-primary-300">Angenommen: {voteResult.legislative_initiative.accepted}</div> -->
				<div class="ml-auto more-info-item"><SButton class="bg-tertiary-500">Details anzeigen</SButton></div>
			</div>

		</div>
		<div class="max-sm:!hidden entry bg-primary-200 dark:bg-primary-400 mt-3 grid-container">
			<!-- Inneres Migration Frauen Klimaschutz -->
			<div class="emphasis-item">
				{#if emphasis}
					<ul>
						{#each emphasis as emph}
							<li>- {emph}</li>
						{/each}
					</ul>
				{/if}
			</div>
			<div class="rounded-md w-96 max-w-full ml-auto parliament-item">
				<VoteParliament {dels} {voteResult} preview={true} />
			</div>
			<div class="flex info-item gap-3">
				<div class="accepted-item square bg-primary-300">Angenommen: {voteResult.legislative_initiative.accepted}</div>
				<div class="majority-item square bg-primary-300">1/2 mehrheit, maybe 2/3</div>
				<div class="accepted-item square bg-primary-300">Abestimmt am {voteResult.legislative_initiative.created_at}</div>
			</div>
			<div class="ml-auto details-item mt-auto"><SButton class="bg-tertiary-500">Details anzeigen</SButton></div>
		</div>
	</div>
</div>

<style>
	.entry {
		border-radius: 0.9rem;
		box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
		padding: 20px;
		gap: 10px;
	}
	/* .grid-container {
		display: grid;
		grid-template-columns: 2fr 1fr;
	} */
	
	.grid-container {
		box-sizing: border-box;
		display: grid;
        min-width: 0;
        min-height: 0;
		grid-template-areas:
			'e e e e e p p p' /* e: emphasis, p: parliament */
			'e e e e e p p p'
			'e e e e e p p p'
			'i i i . . . d d'; /* a: accepted, m: majority? 2/3, 1/2, dt: date, d: details */
		/* "i i i a"; */
		padding: 10px;
	}

	.square {
		aspect-ratio: 1/ 1;
		display: flex;
		align-items: center;
		padding: 5%;
		color: #fff;
	}

	.parliament-item {
		grid-area: p;
	}

	.emphasis-item {
		grid-area: e;
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
	}

	.details-item {
		grid-area: d;
	}

	.item {
		grid-column: 1fr;
	}
</style>
