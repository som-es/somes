<script lang="ts">
	import { partyColors, partyToColor } from '$lib/partyColor';
	import type { Vote } from '$lib/types';

	export let votes: Vote[];
	export let infavor: boolean = false;
</script>
{#if partyColors.size > 0}
	{#each votes as vote}
		{#if vote.infavor == infavor}
			<div class="flex flex-row">
				<span style="background-color: {partyToColor(vote.party)}" class="px-4 py-3 rounded-lg">
				</span>
				<div class="flex flex-col">
					<span class="ml-1">
						{vote.party}
					</span>

					<span class="ml-1">
						({vote.fraction})
					</span>
				</div>
			</div>
		{/if}
	{/each}
{/if}
