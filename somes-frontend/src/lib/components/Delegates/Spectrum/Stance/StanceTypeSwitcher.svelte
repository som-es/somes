<script lang="ts">
	import type { GeneralDelegateInfo } from "$lib/types";
	import { Tabs } from "bits-ui";
	import Stances from "./Stances.svelte";

    export let delegateInfo: GeneralDelegateInfo;

</script>

<div
	class="bg-primary-300 flex flex-wrap dark:bg-primary-500 {delegateInfo.left_right_stances.length > 0 || delegateInfo.stance_topic_scores.length > 0
		? 'p-4'
		: ''} rounded-xl"
>
	<Tabs.Root value="stance">
		<Tabs.List>
			<Tabs.Trigger
				value="stance"
				class="data-[state=active]:border-b-2 p-1 px-3"
			>
				Haltung
			</Tabs.Trigger>
			<Tabs.Trigger
				value="direction"
				class="data-[state=active]:border-b-2 p-1 px-3"
			>
				Richtung
			</Tabs.Trigger>
		</Tabs.List>
		<Tabs.Content value="stance" class="mt-4">
			<Stances
				leftLabel={"Dagegen"}
				rightLabel={"Dafür"}
				stances={delegateInfo.stance_topic_scores}
			/>
		</Tabs.Content>
		<Tabs.Content value="direction" class="mt-4">
			<Stances stances={delegateInfo.left_right_stances} />
		</Tabs.Content>
	</Tabs.Root>
	<!-- <Tabs tabStyle="underline" class="bg-inherit!" contentClass="bg-inherit!">
		<TabItem open title="Haltung" >
			<Stances leftLabel={"Dagegen"} rightLabel={"Dafür"} stances={delegateInfo.stance_topic_scores} />
		</TabItem>
		<TabItem title="Richtung">
			<Stances stances={delegateInfo.left_right_stances} />
		</TabItem>
	</Tabs> -->
</div>