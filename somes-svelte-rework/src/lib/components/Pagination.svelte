<script lang="ts">
	import SButton from './UI/SButton.svelte';
	import rightArrowIcon from '$lib/assets/misc_icons/right-arrow-small.svg?raw';

	export let maxPage: number;
	export let page: number;

	const offsets = [
		[0, 11, 4, 4, 4, 4, 4, 7, 0],
		[0, 10, 3, 3, 3, 3, 3, 6, 0],
		[0, 9, 2, 2, 2, 2, 2, 5, 0],
		[0, 8, 1, 1, 1, 1, 1, 4, 0],
		[0, 7, 0, 0, 0, 0, 0, 3, 0],
		[0, 6, 0, 0, 0, 0, 0, 2, 0],
		[0, 5, 0, 0, 0, 0, 0, 1, 0],
		[0, 4, 0, 0, 0, 0, 0, 0, 0],
		[0, 3, 0, 0, 0, 0, 0, 0, 0],
		[0, 2, 0, 0, 0, 0, 0, 0, 0],
		[0, 1, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0]
	];
	let pageSuggestions: number[] = [];
	let writtenPage = `${page}`;
	$: if (page || maxPage) {
		// maxPage != 0, if no elements are found (maxPage=0) than do not display anything
		if (page >= maxPage && maxPage != 0) {
			page = maxPage;
		}
		const otherPage = page + 0;
		const baseLayout = [
			1,
			otherPage - 10,
			otherPage - 2,
			otherPage - 1,
			otherPage,
			otherPage + 1,
			otherPage + 2,
			otherPage + 10,
			maxPage
		];
		const offsetFn = (i: number) => {
			if (page > 0 && page < 12) {
				return offsets[page - 1][i];
			}
			if (page <= maxPage && page > maxPage - 12) {
				return offsets[maxPage - page][offsets[0].length - i - 1] * -1;
			}
			return 0;
		};

		// filter removes duplicates
		pageSuggestions = baseLayout.map((el, i) => el + offsetFn(i)).filter((el) => el != maxPage);
		pageSuggestions.push(maxPage);
		writtenPage = `${page}`;
	}

	const isActive = (suggestion: number) => {
		return suggestion == page;
	};

	const onInput = (e: Event) => {
		try {
			console.log(writtenPage);
			if (writtenPage == '' || writtenPage == null) {
				return;
			}
			const newPage = +writtenPage;
			if (newPage > 0 && newPage <= maxPage) {
				page = newPage;
			} else {
				writtenPage = `${page}`;
			}
		} catch (error) {
			writtenPage = '1';
		}
	};
</script>

<div class="flex flex-row flex-wrap gap-[0.35rem] items-center text-black">
	<SButton
		class="mt-5 mb-5 bg-tertiary-500 text-center"
		title="vorherige Seite"
		on:click={() => {
			if (page > 1) page--;
		}}
	>
		<div class="w-[30px] h-[30px]">
			<div class="w-[30px] h-[30px] left-arrow">
				{@html rightArrowIcon}
			</div>
		</div>
	</SButton>
	{#each pageSuggestions as suggestion}
		{#if suggestion > 0 && suggestion <= maxPage}
			<SButton
				class="mt-5 mb-5 text-center !rounded-lg {isActive(suggestion)
					? 'bg-secondary-300'
					: 'bg-tertiary-300'}"
				on:click={() => {
					page = suggestion;
				}}>
				<div class="font-bold text-lg w-[30px] h-[30px] items-center flex justify-center">
					{suggestion}
				</div>
			</SButton
			>
		{/if}
	{/each}
	<SButton
		class="mt-5 mb-5 bg-tertiary-500 "
		title="nächste Seite"
		on:click={() => {
			if (page < maxPage) page++;
		}}
	>
		<div class="w-[30px] h-[30px]">
			<div class="w-[30px] h-[30px]">
				{@html rightArrowIcon}
			</div>
		</div>
		<!-- {'>'} -->
	</SButton
	>
	<!-- <input style="color: black;" class="mx-3 w-11 h-11" bind:value={writtenPage} on:input={onInput} type="number" /> -->
</div>

<style>
	.left-arrow {
		transform: rotate(180deg);
	}
</style>
