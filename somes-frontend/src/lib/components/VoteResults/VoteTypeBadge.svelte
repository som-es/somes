<script lang="ts">
	import type { VoteResult } from '$lib/types';
	import { t } from '$lib/i18n/i18n.svelte';

	export let voteResult: VoteResult;

	let badgeText = '';
	$: {
		if (voteResult.legislative_initiative.voting) {
			switch (voteResult.legislative_initiative.voting) {
				case 'Law':
					badgeText = t('voteType.law');
					break;
				case 'Amendment':
					badgeText = t('voteType.amendment');
					break;
				case 'Resolution':
					badgeText = t('voteType.resolution');
					break;
				case 'Report':
					badgeText = t('voteType.report');
					break;
				default:
					badgeText = '';
			}
		} else {
			badgeText = t('voteType.other');
		}
	}
</script>

<div class="badge bg-tertiary-400 text-black">{badgeText}</div>
