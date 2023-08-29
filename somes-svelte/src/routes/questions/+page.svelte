<script lang="ts">
	import type { Delegate } from "$lib/types";
	import { Stepper, Step } from "@skeletonlabs/skeleton";
	import DelegateCard from "@/components/DelegateCard.svelte";
	import DelegateSelect from "@/components/DelegateSelect.svelte";

	let selectedDel: Delegate | undefined = undefined;

	const handleUpdateDel = (e: any) => selectedDel = e.detail;
</script>

<div class="flex flex-col gap-[5vh]">
	<div class=" w-4/5 self-center">
		<Stepper>
			<Step>
				<div class="flex flex-row gap-[5vw]">
					<DelegateCard delegate={selectedDel} />
					<DelegateSelect 
						selectedDel={selectedDel}
						on:updateDel={handleUpdateDel} 
					/>
				</div>
			</Step>
			<Step>
				<div class="flex flex-row gap-[5vw]">
					<DelegateCard delegate={selectedDel} />
					<div class="w-full">Lorem Ipsum</div>
				</div>
			</Step>
		</Stepper>
	</div>
</div>

<style>
	
</style>
