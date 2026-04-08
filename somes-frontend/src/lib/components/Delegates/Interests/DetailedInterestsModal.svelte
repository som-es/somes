<script lang="ts">
	import ModalCloseButton from '$lib/components/UI/ModalCloseButton.svelte';
	import { topicColors, translateTopicToParent } from '$lib/interestColors';
	import type { InterestShare } from '$lib/types';
	import { Dialog } from 'bits-ui';
	import { BarChart } from 'layerchart';


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

	let cDomain = $derived(detailedInterests.map(i => i.topic));

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

<div class="card p-4 sm:p-8 w-full max-w-7xl">
	<div class="flex justify-between items-start gap-2">
		<span class="font-bold text-xl sm:text-2xl flex-1 min-w-0">Detailierte Interessen</span>
		<Dialog.Close>
			<ModalCloseButton />	
		</Dialog.Close>
	</div>
	
	<span class="text-base sm:text-lg">meist behandelte Themen</span>
	
	<div style="height: {dynamicHeight}px" class="p-4 border rounded-sm mt-2">
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
						class: "fill-black dark:fill-white stroke-none stroke-0 text-sm font-semibold"
					}
				},
				yAxis: {
					tickLabelProps: {
						class: "fill-black dark:fill-white stroke-none stroke-0 font-semibold",
						"font-size": isMobile ? 9 : 11,
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
