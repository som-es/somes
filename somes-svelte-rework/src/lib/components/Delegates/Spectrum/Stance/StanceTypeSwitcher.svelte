<script lang="ts">
	import type { GeneralDelegateInfo } from "$lib/types";
	import { Tab, TabGroup } from "@skeletonlabs/skeleton";
	import Stances from "./Stances.svelte";

    export let delegateInfo: GeneralDelegateInfo;

    let tabSet: number = 0;
</script>

<div
	class="bg-primary-300 flex flex-wrap dark:bg-primary-500 w-full {delegateInfo.left_right_stances.length > 0 || delegateInfo.stance_topic_scores.length > 0
		? 'p-4'
		: ''} rounded-xl"
>
	<TabGroup>
		<Tab bind:group={tabSet} name="infavor" value={0}>
			<!-- <svelte:fragment slot="lead">(icon)</svelte:fragment> -->
			<span>Haltung</span>
		</Tab>
		<Tab bind:group={tabSet} name="left_right" value={1}>Richtung</Tab>
		<!-- Tab Panels --->
		<svelte:fragment slot="panel">
			{#if tabSet === 0}
				<Stances leftLabel={"Dagegen"} rightLabel={"Dafür"} stances={delegateInfo.stance_topic_scores} />
			{:else if tabSet === 1}
				<Stances stances={delegateInfo.left_right_stances} />
			{/if}
		</svelte:fragment>
	</TabGroup>
</div>