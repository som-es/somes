<script lang="ts">
    import { cachedDelegates } from "$lib/caching/delegates";
	import { cachedLatestVoteResults } from "$lib/caching/vote_results";

    setInterval(async () => {
        cachedDelegates(true);
    }, 1000 * 60 * 20);
    setInterval(async () => {
        cachedDelegates(true);
        cachedLatestVoteResults(true); 
    }, 1000 * 60 * 4);
</script>