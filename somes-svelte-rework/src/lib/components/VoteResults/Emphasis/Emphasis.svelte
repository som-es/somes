<script lang="ts">
	import { popup, type PopupSettings } from "@skeletonlabs/skeleton";

	export let rawEmphasis: string | null;
	export let isAiGenerated: boolean = false;
	export let useTitleHover: boolean = false;
	
	$: emphasis = rawEmphasis
		?.split('\n')
		.filter((x) => x.length > 10)
		.map((x) => {
			let trim = x.trim();
			if (trim.startsWith("-")) {
				trim = trim.slice(1).trim()
			}
			return trim;
		});

	$: isAiGenerated
	
	const popupFeatured: PopupSettings = {
		event: 'hover',
		target: 'emphasisAi',
		placement: 'bottom'
	};

	const aiGenText = "Diese Schwerpunkte wurden mittels KI aus dem jeweiligen Gesetzestext zusammengefasst.";
	const titleHover = useTitleHover ? aiGenText : "";
</script>

{#if emphasis}
	{#if emphasis.length > 0}
		<div class="emphasis-item rounded-xl bg-primary-300 dark:bg-primary-500 px-10 pt-3 pb-3">
			<div class="flex justify-between">
				<h1 class="font-bold text-2xl">Schwerpunkte</h1>

				<div class="!z-50 card p-4 w-72 shadow-xl" data-popup="emphasisAi">
					<div class="z-50 font-bold text-xl">{aiGenText}</div>
				</div>
				
				<button class="text-4xl" title={titleHover} use:popup={popupFeatured}>&#9432;</button>
			</div>
			<ul class="mt-1 list fill-primary-400">
				{#each emphasis as emph}
					<li>
						<span class="badge bg-primary-500 dark:bg-primary-300"></span>
						<span>{emph}</span>
					</li>
				{/each}
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
