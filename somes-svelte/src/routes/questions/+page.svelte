<script lang="ts">
	import type { Delegate, Party, Question } from "$lib/types";
	import QuestionCard from "../../components/QuestionCard.svelte";
	import QuestionToolbar from "../../components/QuestionToolbar.svelte";
	import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
	import { faPlus } from "@fortawesome/free-solid-svg-icons";
	import { modalStore } from "@skeletonlabs/skeleton";
	import type { ModalSettings } from "@skeletonlabs/skeleton";
	import { t } from "$lib/translations";
	import { delegate_by_id } from "$lib/api";
	import { toAPIDate, toTSDate } from "$lib/date";

	const modal: ModalSettings = {
		type: "component",
		component: "QuestionModal",
	};

	// TODO: use the real questions from the api
	const questions: Question[] = [
		{
			question_id: 1,
			issuer_id: 69420,
			created_on: new Date(),
			delegate_id: 5650,
			title: "Test123",
			body: "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.",
			response: null,
			responded_on: null,
			editable: false,
			last_edited_on: null,
			visible: true,
			likes: 0,
			dislikes: 0,
		},
		{
			question_id: 1,
			issuer_id: 69420,
			created_on: new Date(),
			delegate_id: 5650,
			title: "Testaaaaaaaaaaa",
			body: "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua.",
			response: "Da bin ich mir nicht ganz sicher muss ich sagen",
			responded_on: new Date(),
			editable: false,
			last_edited_on: null,
			visible: true,
			likes: 12,
			dislikes: 3,
		},
		{
			question_id: 1,
			issuer_id: 69420,
			created_on: new Date(),
			delegate_id: 5650,
			title: "Test123",
			body: "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.",
			response:
				"According to all known laws of aviation, there is no way that a bee should be able to fly. Its wings are too small to get its fat little body off the ground. The bee, of course, flies anyways. Because bees don't care what humans think is impossible.",
			responded_on: new Date(),
			editable: false,
			last_edited_on: null,
			visible: true,
			likes: 2,
			dislikes: 7,
		},
		{
			question_id: 1,
			issuer_id: 69420,
			created_on: new Date(),
			delegate_id: 5438,
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

	let search: string = "";
	let sort: "relevance" | "rating" | "newest" = "relevance";
	let selectedDelegate: Delegate | string = "all";
	let selectedParty: Party | string = "all";
	let selectedDateRange: string[] = [toAPIDate(new Date(0)), toAPIDate(new Date())];

	$: filtered = async (): Promise<Question[]> => {
		const delegates = await Promise.all(questions.map((q) => delegate_by_id(q.delegate_id)));

  		const filtered = questions.filter((q, i) => {
			console.log(q.created_on)
			console.log(selectedDateRange[0])

			// TODO: add search bar
			if (q.delegate_id !== selectedDelegate.id && selectedDelegate !== "all") return false;
			if (delegates[i].party !== selectedParty && selectedParty !== "all") return false;
			if (
				q.created_on < toTSDate(selectedDateRange[0]) ||
				q.created_on > toTSDate(selectedDateRange[1], true)
			) return false;

			return true;
		});

		return filtered;
	}
</script>

<div class="flex flex-col py-10">
	<div class="flex flex-col gap-[2vh] w-3/5 self-center">
		<QuestionToolbar
			bind:search={search}
			bind:sort={sort}
			bind:delegate={selectedDelegate}
			bind:party={selectedParty}
			bind:dateRange={selectedDateRange}
		/>

		{#await filtered()}
			<loading class="">loading...</loading>
		{:then value}
			<div class="flex flex-col gap-[5vh] mt-[3vh]">
				{#if value.length === 0}
					<p class="text-center">Es konnten keine Fragen gefunden werden.</p>
				{:else}
					{#each value as question}
						<QuestionCard {question} />
					{/each}
				{/if}
			</div>
		{/await}
	</div>
</div>
<div class="absolute bottom-0 left-0 m-5">
	<button type="button" class="btn btn-lg variant-filled" on:click={triggerQuestionModal}>
		<FontAwesomeIcon icon={faPlus} class="pr-2" /> {$t("common.ask_question")}
	</button>
</div>

<style>
</style>
