<script lang="ts">
	import VoteParliament2 from '$lib/components/Parliaments/VoteParliament2.svelte';
	import { mockDelegates, mockVoteResult } from '$lib/parliaments/mock';
	import TestChart from '$lib/components/Statistics/TestChart.svelte';
	// import { Chart } from "frappe-charts";
</script>

<div>
	<TestChart dataNoParty={[7, 9, 5, 1, 2]}> </TestChart>

	<div class="w-full">
		<VoteParliament2 voteResult={mockVoteResult()} delegates={mockDelegates()} overrideDelegates useOffset={false} noSeats preview />
	</div>
</div>
