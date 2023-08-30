<script lang="ts">
	import type { Delegate } from "$lib/types";
	import { Stepper, Step } from "@skeletonlabs/skeleton";
	import DelegateCard from "@/components/DelegateCard.svelte";
	import DelegateSelect from "@/components/DelegateSelect.svelte";
	import QuestionForm from "@/components/QuestionForm.svelte";
	import QuestionConfirm from "@/components/QuestionConfirm.svelte";

	let selectedDel: Delegate | undefined = undefined;
	let question: string = "";
	let details: string = "";
	let disclaimerAcknowledged: boolean = false;

	const handleUpdateDel = (e: any) => (selectedDel = e.detail);
</script>

<div class="flex flex-col gap-[5vh]">
	<div class=" w-4/5 self-center">
		<Stepper
			stepTerm="Schritt"
			buttonBackLabel="← Zurück"
			buttonNextLabel="Nächster Schritt →"
			buttonCompleteLabel="Frage abschicken"
		>
			<Step locked={selectedDel === undefined}>
				<svelte:fragment slot="header">Abgeordete*n auswählen</svelte:fragment>
				<div class="flex flex-row gap-[3vw]">
					<DelegateCard delegate={selectedDel} />
					<DelegateSelect {selectedDel} on:updateDel={handleUpdateDel} />
				</div>
			</Step>
			<Step>
				<svelte:fragment slot="header">Frage verfassen</svelte:fragment>
				<div class="flex flex-row gap-[3vw]">
					<DelegateCard delegate={selectedDel} />
					<QuestionForm bind:question bind:details />
				</div>
			</Step>
			<Step locked={!disclaimerAcknowledged}>
				<svelte:fragment slot="header">Frage überprüfen</svelte:fragment>
				<div class="flex flex-row gap-[3vw]">
					<DelegateCard delegate={selectedDel} />
					<QuestionConfirm
						delegate={selectedDel}
						{question}
						{details}
						bind:disclaimerAcknowledged
					/>
				</div>
			</Step>
		</Stepper>
	</div>
</div>

<style>
</style>
