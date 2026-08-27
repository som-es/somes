<script lang="ts">
	import type { InterestShare, StanceTopicScore } from '$lib/types';
	import ExtendInfoDialog from '../../ExtendInfoDialog.svelte';
	import { t } from '$lib/i18n/i18n.svelte';
	import DetailedLeftRightChart from './DetailedLeftRightChart.svelte';
	import LeftRightSvg from './LeftRightSvg.svelte';

	interface Props {
		stances: StanceTopicScore[];
		interests: InterestShare[];
	}

	let { stances, interests }: Props = $props();

	// Keep only the 8 topics with the strongest stance (highest absolute score),
	// then sort them left (most negative) to right
	const positioned = $derived.by(() => {
		let tempStance: (StanceTopicScore & { occ: number })[] = [];
		if (interests.length > 0) {
			stances.map((s) => {
				const intr = interests.find((i) => i.topic === s.topic);
				tempStance.push({ ...s, occ: intr?.occurences ?? 0 });
			});
		} else {
			stances.map((s) => {
				tempStance.push({ ...s, occ: 0 });
			});
		}
		return tempStance.sort((a, b) => b.occ - a.occ).slice(0, 8);
	});
</script>

<div class="relative w-full h-full rounded-xl bg-primary-300 px-5 pt-12 pb-3 sm:pt-5 dark:bg-primary-500">
	<div class="absolute top-3 right-3">
		{#if stances.length > 8}
			<ExtendInfoDialog title={t('ui.details')}>
				<DetailedLeftRightChart {stances} />
			</ExtendInfoDialog>
		{/if}
	</div>

	<LeftRightSvg stances={positioned} />
</div>
