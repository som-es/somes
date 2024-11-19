<script lang="ts">
	import { jwtStore } from '$lib/caching/stores/stores';
	import Container from '$lib/components/Layout/Container.svelte';
	import VoteParliament2 from '$lib/components/Parliaments/VoteParliament2.svelte';
	import SButton from '$lib/components/UI/SButton.svelte';
	import { gotoHistory } from '$lib/goto';
	import { mockDelegates, mockVoteResult } from '$lib/parliaments/mock';

	

</script>

<Container>
	user
	<SButton
		class="bg-tertiary-500 text-black"
		on:click={() => {
			jwtStore.set(null);
			gotoHistory("/home");
		}}>
	Abmelden</SButton>
	<div class="w-full">
		<VoteParliament2 voteResult={mockVoteResult()} delegates={mockDelegates()} overrideDelegates useOffset={false} noSeats preview />
	</div>
</Container>
