<script lang="ts">
	import { createVoteResultPath, type Delegate, type VoteResult } from '$lib/types';
	import VoteParliament2 from '$lib/components/Parliaments/VoteParliament2.svelte';
	import VoteTypeBadge from '$lib/components/VoteResults/VoteTypeBadge.svelte';

	interface Props {
		voteResult: VoteResult;
		dels: Delegate[];
		tabindex: number;
		allSeats: Map<string, number[]> | null;
	}

	let { voteResult, dels, tabindex, allSeats }: Props = $props();
</script>

<a
	class="tile card hover:cursor-pointer"
	href={createVoteResultPath(voteResult)}
	tabindex={10 + tabindex}
>
	<div class="tile-content">
		<div class="w-[360px]">
			<VoteParliament2
				showGovs
				{voteResult}
				{allSeats}
				noSeats={false}
				useOffset={true}
				show3D
			/>
		</div>
		<div class="mx-3 my-1 text-left">
			<span>{voteResult.legislative_initiative.description}</span>
			<VoteTypeBadge {voteResult} />
		</div>
	</div>
</a>

<style>
	.tile {
		box-sizing: border-box;
		padding: 0;
		border-radius: 25px;
		position: relative;
		z-index: 1;
		overflow: hidden;
		width: 25rem;
		margin: 0.8rem;
	}

	.tile-content {
		display: flex;
		justify-content: center;
		flex-direction: column;
		align-items: center;
	}
</style>
