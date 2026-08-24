<script lang="ts">
	import ModalCloseButton from '$lib/components/UI/ModalCloseButton.svelte';
	import { topicColors, translateTopicToParent } from '$lib/interestColors';
	import type { InterestShare } from '$lib/types';
	import { Dialog } from 'bits-ui';
	import { BarChart } from 'layerchart';
	import { t } from '$lib/i18n/i18n.svelte';

	interface Props {
		detailedInterests?: InterestShare[];
	}

	let { detailedInterests = [] }: Props = $props();

	let dateSeriesData = $derived(
		detailedInterests.map((interest) => {
			return {
				topic: interest.topic,
				occurences: interest.occurences,
				color: topicColors.get(translateTopicToParent(interest.topic)) ?? 'gray'
			};
		})
	);

	let cDomain = $derived(detailedInterests.map((i) => i.topic));

	let cRange = $derived(
		detailedInterests.map((interest) => {
			return topicColors.get(translateTopicToParent(interest.topic)) ?? 'gray';
		})
	);
	const ROW_HEIGHT = 45;
	const VERTICAL_PADDING = 40;

	let dynamicHeight = $derived(
		Math.max(detailedInterests.length * ROW_HEIGHT + VERTICAL_PADDING, 150)
	);

	let windowWidth = $state(window?.innerWidth ?? 1024);
	let isMobile = $derived(windowWidth < 640);
	let chartPaddingLeft = $derived(isMobile ? 185 : 300);
</script>

<svelte:window bind:innerWidth={windowWidth} />

<div class="w-full max-w-7xl card p-4 sm:p-8">
	<div class="flex items-start justify-between gap-2">
		<span class="min-w-0 flex-1 text-xl font-bold sm:text-2xl">{t('interests.detail.title')}</span>
		<Dialog.Close>
			<ModalCloseButton />
		</Dialog.Close>
	</div>

	<span class="text-base sm:text-lg">{t('interests.detail.subtitle')}</span>

	<div style="height: {dynamicHeight}px" class="mt-2 rounded-sm border p-4">
		<BarChart
			data={dateSeriesData}
			{cRange}
			x="occurences"
			y="topic"
			{cDomain}
			c="topic"
			orientation="horizontal"
			renderContext="svg"
			padding={{ left: chartPaddingLeft, top: 20, bottom: 12 }}
			props={{
				xAxis: {
					tickLabelProps: {
						class: 'fill-black dark:fill-white stroke-none stroke-0 text-sm font-semibold'
					}
				},
				yAxis: {
					tickLabelProps: {
						class: 'fill-black dark:fill-white stroke-none stroke-0 font-semibold',
						'font-size': isMobile ? 9 : 11,
						textAnchor: 'end'
					}
				},
				bars: {
					strokeWidth: 0
				}
			}}
		/>
	</div>
</div>
