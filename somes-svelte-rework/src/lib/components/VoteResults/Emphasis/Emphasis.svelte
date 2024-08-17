<script lang="ts">
	export let emphasis: string[] | undefined;
</script>

{#if emphasis}
	{#if emphasis.length > 0}
		<div class="emphasis-item rounded-xl bg-primary-300 dark:bg-primary-500 px-10 pt-3 pb-3">
			<h1 class="font-bold text-2xl">Schwerpunkte</h1>
			<ul class="mt-1 list fill-primary-400 ">
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
