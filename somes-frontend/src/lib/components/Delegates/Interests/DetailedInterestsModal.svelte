<script lang="ts">
	import ModalCloseButton from '$lib/components/UI/ModalCloseButton.svelte';
	import { topicColors } from '$lib/interestColors';
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
				color: topicColors.get(interest.topic) ?? 'black'
			};
		})
	);

	let cDomain = $derived(detailedInterests.map(i => i.topic));

	let cRange = $derived(
		detailedInterests.map((interest) => {
			return topicColors.get(interest.topic) ?? 'black';
		})
	);
	const ROW_HEIGHT = 45;
	const VERTICAL_PADDING = 40; 

	let dynamicHeight = $derived(
		Math.max(detailedInterests.length * ROW_HEIGHT + VERTICAL_PADDING, 150)
	);
</script>
<div class="card p-8 max-w-7xl w-7xl">
	<div class="flex justify-between">
		<span class="font-bold text-2xl">Detailierte Interessen</span>
		<Dialog.Close>
			<ModalCloseButton />	
		</Dialog.Close>
	</div>
	
	<span class="text-lg">meist behandelte Themen</span>
	
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
			padding={{ left: 300, top: 20, bottom: 12 }}
			props={{
				xAxis: {
					tickLabelProps: {
						class: "fill-black dark:fill-white stroke-none stroke-0 text-sm font-semibold"
					}
				},
				yAxis: {
					tickLabelProps: {
						class: "fill-black dark:fill-white stroke-none stroke-0 text-sm font-semibold"
					}
				},
				bars: {
					strokeWidth: 0
				}	
			}}
			
		/>	
	</div>
</div>
