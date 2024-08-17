<script lang="ts">
	import VoteParliament from '$lib/components/Parliaments/VoteParliament.svelte';
	import { mockDelegates, mockVoteResult } from '$lib/parliaments/mock';
</script>

<div>
	statistics hi

	<div class="w-full">
		<VoteParliament voteResult={mockVoteResult()} dels={[]} delsAtDate={mockDelegates()} preview />
	</div>
</div>
