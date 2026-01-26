<script lang="ts">
	import { page as sveltePage } from '$app/state';

	let { maxPage }: { maxPage: number; } = $props();

	let page = $derived(
        Number(sveltePage.url.searchParams.get('page')) || 1
    );

	let isMobile = $state(false);

	const offsets = [
		[0, 11, 4, 4, 4, 4, 4, 7, 0], [0, 10, 3, 3, 3, 3, 3, 6, 0],
		[0, 9, 2, 2, 2, 2, 2, 5, 0], [0, 8, 1, 1, 1, 1, 1, 4, 0],
		[0, 7, 0, 0, 0, 0, 0, 3, 0], [0, 6, 0, 0, 0, 0, 0, 2, 0],
		[0, 5, 0, 0, 0, 0, 0, 1, 0], [0, 4, 0, 0, 0, 0, 0, 0, 0],
		[0, 3, 0, 0, 0, 0, 0, 0, 0], [0, 2, 0, 0, 0, 0, 0, 0, 0],
		[0, 1, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0]
	];

	$effect(() => {
		const handleResize = () => (isMobile = window.innerWidth < 640);
		handleResize();
		window.addEventListener('resize', handleResize);
		return () => window.removeEventListener('resize', handleResize);
	});

	const pageSuggestions = $derived.by(() => {
		const effectivePage = (page >= maxPage && maxPage !== 0) ? maxPage : page;

		if (isMobile) {
			if (effectivePage < 3) {
				return [1, 2, 3, 4, maxPage].filter(v => v > 0 && v <= maxPage);
			}
			return [1, effectivePage - 1, effectivePage, effectivePage + 1, maxPage]
				.filter((el, idx, arr) => arr.indexOf(el) === idx && el > 0 && el <= maxPage);
		}

		const baseLayout = [
			1, effectivePage - 10, effectivePage - 2, effectivePage - 1,
			effectivePage, effectivePage + 1, effectivePage + 2, effectivePage + 10, maxPage
		];

		const offsetFn = (i: number) => {
			if (effectivePage > 0 && effectivePage < 12) {
				return offsets[effectivePage - 1][i];
			}
			if (effectivePage <= maxPage && effectivePage > maxPage - 12) {
				const idx = maxPage - effectivePage;
				return offsets[idx][offsets[0].length - i - 1] * -1;
			}
			return 0;
		};

		const suggestions = baseLayout
			.map((el, i) => el + offsetFn(i))
			.filter((el) => el > 0 && el <= maxPage);
		
		return [...new Set(suggestions)].sort((a, b) => a - b);
	});

	function createHref(page: number): string {
		const nextUrl = new URL(sveltePage.url); 
		nextUrl.searchParams.set('page', page.toString());
		return nextUrl.href;
	}
</script>

<div class="flex flex-row flex-wrap gap-[0.4rem] items-center text-black pagination-wrapper">
	{#each pageSuggestions as suggestion}
		<a
			class="btn mt-5 mb-5 px-2 py-1 text-center rounded-lg! {suggestion === page
				? 'bg-secondary-400'
				: 'bg-tertiary-400'}"
			href="{createHref(suggestion)}"
		>
			<div class="font-bold text-lg w-[30px] h-[30px] items-center flex justify-center">
				{suggestion}
			</div>
		</a>
	{/each}
</div>

<style>
	.pagination-wrapper {
		overflow-x: auto;
		flex-wrap: nowrap;
		max-width: 100vw;
	}
	@media (max-width: 640px) {
		.pagination-wrapper {
			gap: 0.1rem;
		}
		.pagination-wrapper .font-bold {
			font-size: 1rem;
			width: 2rem;
			height: 2rem;
		}
	}
</style>