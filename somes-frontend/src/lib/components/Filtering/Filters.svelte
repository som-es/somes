<script lang="ts" , generics="T">
	import ActiveFilter from './ActiveFilter.svelte';
	import SButton from '../UI/SButton.svelte';
	import FilterOpener from './FilterOpener.svelte';
	import FilterRadioGroup from './FilterRadioGroup.svelte';
	import LegisButtons from './LegisButtons.svelte';
	import type { FilterInfo } from './types';

	export let filters: FilterInfo<T>[];
	export let selectedPeriod: string;
	export let searchValue: string;
	export let update: () => void;
</script>

<!-- Small Screen PopUps (keep them out of <div>...</div> as much as possible) -->
<div
	class="z-10 card w-full p-5 self-center md:max-w-136 lg:max-w-200 shadow-xl py-2"
	data-popup="mobileFilter"
>
	<!-- {#each filters as filter}
		<div class="z-20 card w-48 shadow-xl py-2" data-popup={filter.popup.target}>
			<FilterListBox bind:filter />
		</div>
	{/each} -->

	<div class="lg:hidden flex flex-wrap gap-6">
		{#each filters as filter}
			<FilterOpener {filter} />
		{/each}
	</div>

	<!-- LEGIS PERIODS -->
	<div class="mt-10">
		<h2 class="font-bold text-2xl mb-1">Legislaturperioden</h2>
		<LegisButtons bind:selectedPeriod />
	</div>
</div>

<!-- FILTER OPTIONS -->
<!-- Large Screens-->
<div class="max-lg:hidden flex flex-wrap mt-1">
	{#each filters as filter}
		<FilterRadioGroup bind:filter />
	{/each}
</div>
<!-- LEGIS PERIODS -->
<div class="max-lg:hidden mt-5">
	<h2 class="font-bold text-2xl mb-1">Legislaturperioden</h2>
	<LegisButtons bind:selectedPeriod />
</div>

<!-- SEARCH OPTION -->
<div class="mt-4 sm:mt-8">
	<div class="flex flex-row gap-3">
		<input
			class="rounded-lg !bg-surface-200-800 w-full h-12 px-2 placeholder-gray-500"
			type="search"
			name="ac-demo"
			bind:value={searchValue}
			on:change={update}
			placeholder="Suche..."
		/>
		<div class="flex flex-row gap-2">
			<SButton class="bg-secondary-500 px-1.5! text-black" on:click={update}>Suchen</SButton>
			<!-- <div use:popup={mobileFilter} class="lg:hidden">
				<SButton class="bg-secondary-500 text-black">{@html filterIcon}</SButton>
			</div> -->
		</div>
	</div>

	<!-- Remove hardcoding of filter html -->
	<div class="mt-2 flex flex-wrap gap-2">
		{#if selectedPeriod !== 'all'}
			<button
				class="badge p-3 bg-secondary-400 text-black cursor-pointer"
				on:click={() => (selectedPeriod = 'all')}
			>
				{selectedPeriod} <span class="ml-1" style="font-size: 18px;">✕</span>
			</button>
		{/if}
		{#each filters as filter}
			<ActiveFilter bind:filter />
		{/each}
	</div>
</div>
