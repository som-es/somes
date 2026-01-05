<script lang="ts">
	import type { GeneralDelegateInfo } from "$lib/types";
	// import { Tab, Tabs } from "@skeletonlabs/skeleton-svelte";
	import Stances from "./Stances.svelte";

    export let delegateInfo: GeneralDelegateInfo;

    let tabSet: number = 0;
</script>

<div
	class="bg-primary-300 flex flex-wrap dark:bg-primary-500 {delegateInfo.left_right_stances.length > 0 || delegateInfo.stance_topic_scores.length > 0
		? 'p-4'
		: ''} rounded-xl"
>
	<!-- <Tabs>
		<Tab bind:group={tabSet} name="infavor" value={0}>
			<span>Haltung</span>
		</Tab>
		<Tab bind:group={tabSet} name="left_right" value={1}>Richtung</Tab>
		<svelte:fragment slot="panel">
			{#if tabSet === 0}
				<Stances leftLabel={"Dagegen"} rightLabel={"Dafür"} stances={delegateInfo.stance_topic_scores} />
			{:else if tabSet === 1}
				<Stances stances={delegateInfo.left_right_stances} />
			{/if}
		</svelte:fragment>
	</Tabs> -->
</div>