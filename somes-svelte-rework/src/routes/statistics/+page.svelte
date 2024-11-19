<script lang="ts">
	import VoteParliament2 from '$lib/components/Parliaments/VoteParliament2.svelte';
	import { mockDelegates, mockVoteResult } from '$lib/parliaments/mock';
	// import { Chart } from "frappe-charts";
</script>

<div>
	statistics

	<div class="w-full">
		<VoteParliament2 voteResult={mockVoteResult()} delegates={mockDelegates()} overrideDelegates useOffset={false} noSeats preview />
	</div>
</div>
