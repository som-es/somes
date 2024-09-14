<script lang="ts">
	import { walo_questions } from "$lib/api";
	import Container from "$lib/components/Layout/Container.svelte";
	import SButton from "$lib/components/UI/SButton.svelte";
	import type { WaloQuestion } from "$lib/types";
	import { ListBox, ListBoxItem } from "@skeletonlabs/skeleton";
	import { onMount } from "svelte";

	let waloQuestions: WaloQuestion[];
	let allTickedQuestions: string[][];
	let justificationsOrderCache: [string | null, number][][];
	onMount(async () => {
		const fetchedWaloQuestions = await walo_questions();
		if (fetchedWaloQuestions) {
			waloQuestions = fetchedWaloQuestions;
	 		allTickedQuestions = Array.from({ length: waloQuestions.length }, () => []);
	 		justificationsOrderCache = Array.from({ length: waloQuestions.length }, () => []);
		}
	});

	let idx = 0;


	const next = () => {
		if (idx < waloQuestions.length) {
			idx += 1;
		}
	};
	function shuffleArray<T>(array: T[]): T[] {
		const shuffledArray = array.slice(); 
		for (let i = shuffledArray.length - 1; i > 0; i--) {
			const j = Math.floor(Math.random() * (i + 1));
			[shuffledArray[i], shuffledArray[j]] = [shuffledArray[j], shuffledArray[i]];
		}
		return shuffledArray;
	}

	let justifications: [string | null, number][] = [];

	let valueMultiple: string[] = [];

	const updateJustifications = (idx: number) => {
		if (justificationsOrderCache[idx].length > 0) {
			justifications = justificationsOrderCache[idx];
			return
		};

		const question = waloQuestions[idx];
		justifications = shuffleArray([
			[question.spoe_justification, 0], 
			[question.gruene_justification, 1], 
			[question.neos_justification, 2], 
			[question.fpoe_justification, 3], 
			[question.oevp_justification, 4]
		]);
		justificationsOrderCache[idx] = justifications;

	};

	$: if (waloQuestions || idx) {
		updateJustifications(idx);
	}
	$: if (allTickedQuestions) {
		valueMultiple = allTickedQuestions[idx];
		// valueMultiple.forEach(partyIdx => {
		// 	allTickedQuestions[idx][+partyIdx] = 1;
		// });
		console.log(allTickedQuestions);
	} 


</script>

{#if waloQuestions}
<div class="p-4 max-w-[800px] w-full mx-auto">
	<h1 class="font-bold text-4xl">{waloQuestions[idx].question_statement}</h1>
	<h4 class="text-xl">{waloQuestions[idx].erklaerbaer}</h4>

	<div class="flex justify-between flex-wrap felx-row">
		<ListBox multiple>
			{#each justifications as justification}
				<ListBoxItem bind:group={valueMultiple} name="justification" value="{justification[1]}">{justification[0]}</ListBoxItem>
			{/each}
		</ListBox>
	</div>

	<div class="flex justify-between mt-10">
		{#if idx > 0} 
			<SButton 
				class="mt-5 mb-5 bg-tertiary-500 text-black"
				on:click={() => idx -= 1}
			>
				Vorherige Frage
			</SButton>
		{:else}
			<div></div>
		{/if}

		{#if idx+1 < waloQuestions.length} 
			<SButton 
				class="mt-5 mb-5 bg-tertiary-500 text-black"
				on:click={() => idx += 1}
			>
				Nächste Frage
			</SButton>
		{/if}

	</div>
</div>
{/if}
