<script lang="ts">
	import { type VoteResult } from '$lib/types';
	import InfoBadgesCore from './InfoBadgesCore.svelte';
	export let voteResult: VoteResult;
	export let showRequiredMajority: boolean = true;
	export let showGp: boolean = true;
	export let showDate: boolean = true;
	export let showVoteType: boolean = true;
</script>

<div class="mt-1 flex gap-1 max-sm:flex-wrap">
    <InfoBadgesCore {voteResult} showRequiredMajority showGp showDate showVoteType />
</div>

<style>
</style>
