<script lang="ts">
	import type { Decree } from '$lib/types';

	export let decree: Decree;
	export let open: boolean;

</script>

<div class="entry bg-primary-200 dark:bg-primary-400 mt-3 flex flex-wrap">
	
	<!-- <VoteResults {voteResults} dels={[]} /> -->
</div>

<style>
	.entry {
		border-radius: 0.9rem;
		box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
		padding: 20px;
		gap: 10px;
	}
</style>
