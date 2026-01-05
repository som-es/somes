<script lang="ts">
	import type { Glossary, Keypoint } from '$lib/ai_summary_types';
	import GlossaryText from '$lib/components/UI/GlossaryText.svelte';
	// import collapse from 'svelte-collapse';

	export let emphasis: Keypoint[] | null;
	export let glossary: Glossary | null = null;

	let open = false;
	let firstThreePoints: Keypoint[] = [];
	let restPoints: Keypoint[] = [];
	$: if (emphasis) {
		firstThreePoints = emphasis.slice(0, 2);
		restPoints = emphasis.slice(2);
	}
</script>

{#if emphasis}
	{#if emphasis.length > 0}
		<div class="emphasis-item rounded-xl bg-primary-300 dark:bg-primary-500 px-3 pt-3 pb-3">
			<div class="flex justify-between">
				<h1 class="font-bold text-md md:text-xl">Schwerpunkte</h1>
			</div>

			<ul class="list fill-primary-400 px-7">
				{#each firstThreePoints as emph}
					<li class="mb-3">
						<span class="badge text-sm lg:text-base  bg-primary-500 dark:bg-primary-300"></span>
						{#if glossary}
							<span>
								<GlossaryText text={emph.point} glossary={glossary} />
							</span>
						{:else}
							<span>{emph.point}</span>
						{/if}
					</li>
				{/each}

				<!-- <div use:collapse={{ open }}>
					{#each restPoints as emph}
						<li class="my-3">
							<span class="badge text-sm lg:text-base bg-primary-500 dark:bg-primary-300"></span>
							{#if glossary}
								<span>
									<GlossaryText text={emph.point} glossary={glossary} />
								</span>
							{:else}
								<span>{emph.point}</span>
							{/if}
						</li>
					{/each}
				</div> -->

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
