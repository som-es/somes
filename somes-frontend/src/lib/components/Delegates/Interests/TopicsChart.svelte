<script lang="ts">
	import { topicColors } from '$lib/interestColors';
	import type { InterestShare } from '$lib/types';
	import { Axis, BarChart, Bars, Canvas, Group, Svg } from 'layerchart';
	import { color } from 'three/tsl';
	import ExtendInfoDialog from '../ExtendInfoDialog.svelte';
	import DetailedInterestsModal from './DetailedInterestsModal.svelte';

	interface Props {
		interests: InterestShare[];
		detailedInterests: InterestShare[];
	}

	let { interests, detailedInterests }: Props = $props();

	let dateSeriesData = $derived(
		interests.map((interest) => {
			return {
				topic: interest.topic,
				occurences: interest.occurences,
				color: topicColors.get(interest.topic) ?? 'black'
			};
		})
	);

	let cDomain = $derived(interests.map(i => i.topic));

	let cRange = $derived(
		interests.map((interest) => {
			return topicColors.get(interest.topic) ?? 'black';
		})
	);
</script>

<div class="title-item rounded-xl bg-primary-300 dark:bg-primary-500 p-3 gap-1 w-full">

	<div class="flex justify-between">
		<span class="font-bold max-lg:text-lg text-2xl">Meist behandelte Themen</span>

		{#if detailedInterests.length > 0}
			<ExtendInfoDialog title="Details">
				<DetailedInterestsModal {detailedInterests} />
			</ExtendInfoDialog>	
		{/if}
	</div>
	<div class="h-[300px] p-4 border rounded-sm mt-2">
  		<BarChart 
			data={dateSeriesData} 
			{cRange} 
			x="topic" 
			y="occurences" 
			{cDomain} 
			c="topic" 
			renderContext="svg"
			padding={{ left: 40, top: 20, bottom: 120 }}
			props={{
				xAxis: {
					tickLabelProps: {
						rotate: -30,
						textAnchor: 'end',
						dx: -8,
						dy: 8,
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
		>	
		</BarChart>
	</div>
</div>
