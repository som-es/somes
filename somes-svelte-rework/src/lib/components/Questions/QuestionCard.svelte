<script lang="ts">
	import type { Question } from "$lib/types";
	import { dashDateToDotDate } from "$lib/date";

	export let question: Question;

	$: score = question.likes - question.dislikes;
	
	function formatDate(date: Date): string {
		return new Date(date).toLocaleDateString('de-DE', {
			year: 'numeric',
			month: '2-digit',
			day: '2-digit',
			hour: '2-digit',
			minute: '2-digit'
		});
	}
</script>

<div class="card w-full flex flex-col {question.response ? "border-2 border-emerald-500" : ""}">
	<div class="p-4">
		<header class="card-header font-bold text-xl">{question.title}</header>
		<section class="p-4">
			{question.body}
			<br><i class="text-slate-600">{formatDate(question.created_on)}</i>
		</section>
		<hr class="opacity-80" />
		<footer class="card-footer pt-4">
			{#if question.response}
				{question.response}
				<br><i class="text-slate-600">{formatDate(question.responded_on || question.created_on)}</i>
			{:else}
				<p class="text-center text-xl">UNBEANTWORTET</p>
			{/if}
		</footer>
	</div>
	<hr class="opacity-100" />
	<footer class="px-8 py-4 flex flex-row gap-4">
		<span>An: {question.delegate_id}</span>
		<span>Von: {question.issuer_id}</span>
		<div class="flex flex-row gap-5 ml-auto">
			<span class="flex items-center gap-1">
				👍 {question.likes}
			</span>
			<span class="flex items-center gap-1">
				👎 {question.dislikes}
			</span>
			<span class="font-semibold {score > 0 ? 'text-green-600' : score < 0 ? 'text-red-600' : ''}">
				{score > 0 ? '+' : ''}{score}
			</span>
		</div>
	</footer>
</div>