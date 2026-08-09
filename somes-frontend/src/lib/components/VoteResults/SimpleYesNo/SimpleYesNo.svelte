<script lang="ts">
	import { partyToColor } from '$lib/partyColor';
	import type { Vote } from '$lib/types';
	import BadgeList from './BadgeList.svelte';

	export let votes: Vote[];

	votes.sort((a, b) => b.fraction - a.fraction);
</script>

<div>
	<span class="text-lg font-bold">Dafür</span>
	<div class="mt-0 flex flex-wrap gap-3 py-2">
		<BadgeList {votes} infavor />
	</div>
</div>
<div>
	<span class="text-lg font-bold">Dagegen</span>
	<div class="mt-0 flex flex-wrap gap-3 py-2">
		<BadgeList {votes} />
	</div>
</div>
