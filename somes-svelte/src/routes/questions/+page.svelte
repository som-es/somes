<script lang="ts">
	import type { Question } from "$lib/types";
	import QuestionCard from "../../components/QuestionCard.svelte";
	import QuestionToolbar from "../../components/QuestionToolbar.svelte";

	// TODO: use the real questions from the api
	const questions: Question[] = [
		{
			question_id: 1,
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
		},
		{
			question_id: 1,
			issuer_id: 69420,
			created_on: new Date(),
			delegate_id: 4,
			title: "Testaaaaaaaaaaa",
			body: "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.",
			response: "Da bin ich mir nicht ganz sicher muss ich sagen",
			responded_on: new Date(),
			editable: false,
			last_edited_on: null,
			visible: true,
		},
		{
			question_id: 1,
			issuer_id: 69420,
			created_on: new Date(),
			delegate_id: 4,
			title: "Test123",
			body: "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.",
			response: "According to all known laws of aviation, there is no way that a bee should be able to fly. Its wings are too small to get its fat little body off the ground. The bee, of course, flies anyways. Because bees don't care what humans think is impossible.",
			responded_on: new Date(),
			editable: false,
			last_edited_on: null,
			visible: true,
		},
		{
			question_id: 1,
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
		},
	] 
</script>

<div class="flex flex-col pt-10">
	<div class="flex flex-col gap-[2vh] w-3/5 self-center">
		<QuestionToolbar />
	
		<div class="flex flex-col gap-[5vh] mt-[3vh]">
			{#each questions as question}
				<QuestionCard {question} />
			{/each}
		</div>
	</div>
</div>

<style>
</style>
