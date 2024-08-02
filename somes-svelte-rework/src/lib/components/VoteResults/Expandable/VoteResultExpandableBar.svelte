<script lang="ts">
	import type { Delegate, VoteResult } from '$lib/types';
	import VoteParliament from '../../Parliaments/VoteParliament.svelte';
	import collapse from 'svelte-collapse';
	import upArrowIcon from '$lib/assets/misc_icons/up-arrow.svg?raw';
	import rightArrowIcon from '$lib/assets/misc_icons/right-arrow.svg?raw';
	import crossmarkIcon from '$lib/assets/misc_icons/crossmark.svg?raw';
	import checkmarkIcon from '$lib/assets/misc_icons/checkmark.svg?raw';
	import downArrowIcon from '$lib/assets/misc_icons/down-arrow.svg?raw';
	import SButton from '$lib/components/UI/SButton.svelte';
	import Topics from '$lib/components/Topics/Topics.svelte';

	export let voteResult: VoteResult;
	export let dels: Delegate[];
	let clazz;
	export { clazz as class };
	let open = false;
	let duration = 0.35;

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
			<div id="{open ? "open" : "closed"}">
				{@html rightArrowIcon}
			</div>
		</div>
		<div>{voteResult.legislative_initiative.title}</div>
		<div class="w-20 bg-primary-100 dark:bg-primary-300 rounded-md">
			<VoteParliament {dels} {voteResult} preview={true} />
		</div>
	</div>

	<div use:collapse={{ open, duration }}>
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

			<div class="topics-item bg-primary-300 px-4">
				<Topics topics={voteResult.topics} />
			</div>

			<div class="rounded-md w-80 max-w-full ml-auto parliament-item bg-primary-100">
				<VoteParliament {dels} {voteResult} preview={true} />
			</div>
			<div class="flex info-item gap-3">
				<div class="accepted-item square bg-primary-300">
					{#if voteResult.legislative_initiative.accepted}	
						<!-- {@html } -->
					{/if}
					Angenommen: {voteResult.legislative_initiative.accepted}
				</div>
				<div class="majority-item square bg-primary-300">Notwendige Mehrheit: {voteResult.legislative_initiative.requires_simple_majority ? "1/2" : "2/33" }</div>
				<div class="accepted-item square bg-primary-300">Abgestimmt am {voteResult.legislative_initiative.created_at}</div>
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
		grid-template-areas:
			'e e e e e p p p' /* e: emphasis, p: parliament */
			'e e e e e p p p'
			'e e e e e p p p'
			'i i i i t t d d'; /* a: accepted, m: majority? 2/3, 1/2, dt: date, d: details */
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
		border-radius: 2rem;
	}

	#open :global(.right-arrow)  {
		transform: rotate(90deg);
		transition: transform 0.35s;
	}
	
	#closed :global(.right-arrow)  {
		transform: rotate(0deg);
		transition: transform 0.35s;
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
