<script lang="ts">
	import type { VoteResult } from '$lib/types';
	import { t } from '$lib/i18n/i18n.svelte';
	import { voteTypeLabel } from '$lib/voteBadges';

	export let voteResult: VoteResult;

	$: badgeText = voteTypeLabel(voteResult, t);
</script>

<div class="badge bg-tertiary-400 text-black">{badgeText}</div>
