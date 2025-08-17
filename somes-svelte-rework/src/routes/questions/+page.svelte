<script lang="ts">
	import type { Question } from "$lib/types";
	import QuestionCard from "../../lib/components/Questions/QuestionCard.svelte";
	import QuestionToolbar from "../../lib/components/Questions/QuestionToolbar.svelte";
	import { getModalStore } from "@skeletonlabs/skeleton";
	import type { ModalSettings } from "@skeletonlabs/skeleton";

	const modalStore = getModalStore();

	const modal: ModalSettings = {
		type: "component",
		component: "questionModal",
	};

	// TODO: use the real questions from the api
	const questions: Question[] = [
		{
			question_id: 1,
			issuer_id: 69420,
			created_on: new Date(),
			delegate_id: 4,
			title: "Test123",
			body: "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.",
			response: "Da bin ich mir nicht ganz sicher muss ich sagen",
			responded_on: new Date(),
			editable: false,
			last_edited_on: null,
			visible: true,
			likes: 12,
			dislikes: 3,
		},
		{
			question_id: 2,
			issuer_id: 69420,
			created_on: new Date(),
			delegate_id: 4,
			title: "Test123",
			body: "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.",
			response: null,
			responded_on: null,
			editable: false,
			last_edited_on: null,
			visible: true,
			likes: 2,
			dislikes: 0,
		},
	];

	function triggerQuestionModal() {
		modalStore.trigger(modal);
	}
</script>

<svelte:head>
	<title>Fragen - Somes</title>
</svelte:head>

<div class="flex flex-col py-10">
	<div class="flex flex-col gap-[2vh] w-3/5 self-center">
		<QuestionToolbar />

		<div class="flex flex-col gap-[5vh] mt-[3vh]">
			{#each questions as question}
				<QuestionCard {question} />
			{/each}
		</div>
	</div>
</div>

<div class="fixed bottom-0 left-0 m-5">
	<button type="button" class="btn btn-lg variant-filled" on:click={triggerQuestionModal}>
		<span class="pr-2">+</span> Frage stellen
	</button>
</div>