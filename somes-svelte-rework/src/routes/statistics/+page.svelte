<script lang="ts">
	import Container from '$lib/components/Layout/Container.svelte';
	import VoteParliament2 from '$lib/components/Parliaments/VoteParliament2.svelte';
	import { mockDelegates, mockVoteResult } from '$lib/parliaments/mock';
	// import { Chart } from "frappe-charts";
</script>

<Container>
	<div class="entry bg-primary-200 dark:bg-primary-400 gap-3 flex flex-wrap">
		<div class="flex-item rounded-xl bg-primary-300 dark:bg-primary-500 p-3">
			<h1 class="font-bold text-3xl">Statistiken</h1>
		</div>

		<div class="rounded-xl min-w-40 min-h-40 w-[20rem] h-[20rem] bg-primary-300 dark:bg-primary-500 flex items-center justify-center">Sprachkomplexität</div>
		<div class="rounded-xl min-w-40 min-h-40 w-[20rem] h-[20rem] bg-primary-300 dark:bg-primary-500 flex items-center justify-center">Aktivitätsscoring</div>
		<div class="rounded-xl min-w-40 min-h-40 w-[20rem] h-[20rem] bg-primary-300 dark:bg-primary-500 flex items-center justify-center">Redezeit</div>
		<div class="rounded-xl min-w-40 min-h-40 w-[20rem] h-[20rem] bg-primary-300 dark:bg-primary-500 flex items-center justify-center">Ordnungsrufe</div>
	</div>
</Container>
<div>
	<div class="w-full">
		<VoteParliament2 voteResult={mockVoteResult()} delegates={mockDelegates()} overrideDelegates useOffset={false} noSeats preview />
	</div>
</div>

<style>
	.entry {
		border-radius: 0.9rem;
		box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
		padding: 20px;
		gap: 10px;
	}
	
	.flex-item {
		flex-basis: 100%;
	}

	.square-item {
		flex-basis: 50%;
	}
</style>