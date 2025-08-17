<script lang="ts">
	import QuestionStepper from "./QuestionStepper.svelte";
	import { getModalStore } from "@skeletonlabs/skeleton";

	const modalStore = getModalStore();
	export let parent: any;
</script>

<div class="card p-8 w-2/3">
	<button
		on:click={() => {
			modalStore.close();
		}}
		style="font-size: 34px"
		class="w-5 unselectable float-right"
	>
		&#x2715
	</button>
	<QuestionStepper />
</div>