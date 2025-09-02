<script lang="ts">
	import ReactiveGenericBarChart from '$lib/components/GeneralCharts/ReactiveGenericBarChart.svelte';
	import { topicColors } from '$lib/interestColors';
	import type { InterestShare } from '$lib/types';
	import { getModalStore, type ModalSettings } from '@skeletonlabs/skeleton';

	export let interests: InterestShare[];
	export let detailedInterests: InterestShare[];

	$: detailedInterestsModal = {
		type: 'component',
		component: 'detailedInterests',
		meta: { detailedInterests }
	} as ModalSettings;

	const modalStore = getModalStore();
</script>

<div class="title-item rounded-xl bg-primary-300 dark:bg-primary-500 p-3 gap-1 w-full">
	<span class="font-bold max-lg:text-lg text-2xl">Meist behandelte Themen</span>

	{#if detailedInterests.length > 0}
		<div class="flex justify-between">
			<div></div>
			<button
				class="btn btn-lg variant-filled"
				on:click={() => modalStore.trigger(detailedInterestsModal)}>Details</button
			>
		</div>
	{/if}
	<ReactiveGenericBarChart
		height={300}
		title={''}
		chartData={interests.map((interest) => {
			return {
				label: interest.topic,
				data: interest.occurences,
				color: topicColors.get(interest.topic) ?? 'black'
			};
		})}
	/>
</div>
