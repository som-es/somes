<script lang="ts">
	import type { GeneralDelegateInfo } from "$lib/types";
	import Stances from "./Stances.svelte";

    export let delegateInfo: GeneralDelegateInfo;

</script>

<div
	class="bg-primary-300 flex flex-wrap dark:bg-primary-500 {delegateInfo.left_right_stances.length > 0 || delegateInfo.stance_topic_scores.length > 0
		? 'p-4'
		: ''} rounded-xl"
>
	<!-- <Tabs tabStyle="underline" class="bg-inherit!" contentClass="bg-inherit!">
		<TabItem open title="Haltung" >
			<Stances leftLabel={"Dagegen"} rightLabel={"Dafür"} stances={delegateInfo.stance_topic_scores} />
		</TabItem>
		<TabItem title="Richtung">
			<Stances stances={delegateInfo.left_right_stances} />
		</TabItem>
	</Tabs> -->
</div>