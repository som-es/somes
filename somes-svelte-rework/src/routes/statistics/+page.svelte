<script lang="ts">
	import VoteParliament from '$lib/components/Parliaments/VoteParliament.svelte';
	import { mockDelegates, mockVoteResult } from '$lib/parliaments/mock';
	import { Chart } from "frappe-charts";
</script>

<div>
	statistics

	<div class="w-full">
		<VoteParliament voteResult={mockVoteResult()} dels={[]} delsAtDate={mockDelegates()} preview />
	</div>
</div>
