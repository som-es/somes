<script lang="ts">
	// import ReactiveGenericBarChart from '$lib/components/GeneralCharts/ReactiveGenericBarChart.svelte';
	import { topicColors } from '$lib/interestColors';
	import type { InterestShare } from '$lib/types';
	// export let parent;

	// const modalStore = getModalStore();

	let detailedInterests: InterestShare[] = [];
	// $: if ($modalStore.length > 0 && $modalStore[0].meta) {
	// 	detailedInterests = $modalStore[0].meta.detailedInterests;
	// }
</script>
<!-- 
{#if $modalStore.length > 0 && $modalStore[0].meta}
	<div class="card p-8 max-w-7xl w-7xl">
		<span class="font-bold text-2xl">Detailierte Interessen</span>
		<button
			on:click={() => {
				modalStore.close();
			}}
			style="font-size: 34px"
			class="w-5 unselectable float-right"
		>
			✕
		</button>
		<br />
		<span class="text-lg">meist behandelte Themen</span>
		<ReactiveGenericBarChart
			horizontalBars={true}
			height={5000}
			title={''}
			chartData={detailedInterests.map((interest) => {
				return {
					label: interest.topic,
					data: interest.occurences,
					color: topicColors.get(interest.topic) ?? 'gray'
				};
			})}
		/>
	</div>
{/if} -->
