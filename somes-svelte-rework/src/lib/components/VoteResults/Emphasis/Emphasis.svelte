<script lang="ts">
	import type { Keypoint } from '$lib/ai_summary_types';
	import { popup, type PopupSettings } from '@skeletonlabs/skeleton';
	import collapse from 'svelte-collapse';

	export let emphasis: Keypoint[] | null;
	export let useTitleHover: boolean = false;

	const popupFeatured: PopupSettings = {
		event: 'hover',
		target: 'emphasisAi',
		placement: 'bottom'
	};

	const aiGenText =
		'Diese Schwerpunkte wurden mittels KI aus den jeweiligen Dokumenten zusammengefasst.';
	const titleHover = useTitleHover ? aiGenText : '';

	let open = false;
	let firstThreePoints: Keypoint[] = [];
	let restPoints: Keypoint[] = [];
	$: if (emphasis) {
		firstThreePoints = emphasis.slice(0, 3);
		restPoints = emphasis.slice(3);
	}
</script>

{#if emphasis}
	{#if emphasis.length > 0}
		<div class="emphasis-item rounded-xl bg-primary-300 dark:bg-primary-500 px-3 pt-3 pb-3">
			<div class="flex justify-between">
				<h1 class="font-bold text-lg md:text-2xl">Schwerpunkte</h1>

				<div class="!z-50 card p-4 w-72 shadow-xl" data-popup="emphasisAi">
					<div class="z-50 font-bold text-xl">{aiGenText}</div>
				</div>

				<button class="text-4xl" title={titleHover} use:popup={popupFeatured}>⚠</button>
			</div>

			<ul class="mt-1 list fill-primary-400 px-7">
				{#each firstThreePoints as emph}
					<li class="my-3">
						<span class="badge bg-primary-500 dark:bg-primary-300"></span>
						<span>{emph.point}</span>
					</li>
				{/each}

				<div use:collapse={{ open }}>
					{#each restPoints as emph}
						<li class="my-3">
							<span class="badge bg-primary-500 dark:bg-primary-300"></span>
							<span>{emph.point}</span>
						</li>
					{/each}
				</div>

				{#if emphasis.length > 3}
					<button class=" font-bold text-xl" on:click={() => (open = !open)}>
						<span>{open ? 'Weniger' : 'Mehr'} anzeigen</span>
					</button>
				{/if}
			</ul>
		</div>
	{:else}
		<div class="emphasis-item"></div>
	{/if}
{/if}

<style>
	.emphasis-item {
		grid-area: e;
	}
</style>
