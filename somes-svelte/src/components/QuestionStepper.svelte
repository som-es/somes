<script lang="ts">
    import { Stepper, Step } from "@skeletonlabs/skeleton";
	import DelegateCard from "@/components/DelegateCard.svelte";
	import DelegateSelect from "@/components/DelegateSelect.svelte";
	import QuestionForm from "@/components/QuestionForm.svelte";
	import QuestionConfirm from "@/components/QuestionConfirm.svelte";
    import type { Delegate } from "$lib/types";
	import { onMount } from "svelte";

    export let delegate: Delegate | undefined = undefined;
	let delegateOld: Delegate | undefined;
	let question: string = "";
	let details: string = "";
	let disclaimerAcknowledged: boolean = false;

	const handleUpdateDel = (e: any) => (delegate = e.detail);

	// deepcopy on mount
	onMount(() => {
		if (delegate) delegateOld = Object.assign({}, delegate);
	});
</script>


<Stepper
	stepTerm="Schritt"
	buttonBackLabel="← Zurück"
	buttonNextLabel="Nächster Schritt →"
	buttonCompleteLabel="Frage abschicken"
>
	{#if !delegateOld}
		<Step locked={delegate === undefined}>
			<svelte:fragment slot="header">Abgeordete*n auswählen</svelte:fragment>
			<div class="flex flex-row gap-[3vw]">
				<DelegateCard {delegate} />
				<DelegateSelect selectedDel={delegate} on:updateDel={handleUpdateDel} />
			</div>
		</Step>
	{/if}
	<Step>
		<svelte:fragment slot="header">Frage verfassen</svelte:fragment>
		<div class="flex flex-row gap-[3vw]">
			<DelegateCard {delegate} />
			<QuestionForm bind:question bind:details />
		</div>
	</Step>
	<Step locked={!disclaimerAcknowledged}>
		<svelte:fragment slot="header">Frage überprüfen</svelte:fragment>
		<div class="flex flex-row gap-[3vw]">
			<DelegateCard {delegate} />
			<QuestionConfirm {delegate} {question} {details} bind:disclaimerAcknowledged />
		</div>
	</Step>
</Stepper>
