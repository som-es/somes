<script lang="ts">
	import type { Question } from "$lib/types";
    import { FontAwesomeIcon } from '@fortawesome/svelte-fontawesome'
  	import { faThumbsUp } from '@fortawesome/free-solid-svg-icons'
    import { faThumbsDown } from '@fortawesome/free-solid-svg-icons'
	import { format } from "$lib/date";

    export let question: Question;

    $: score = question.likes - question.dislikes;
</script>

<div class="card w-full flex flex-col {question.response ? "border-2 border-emerald-500" : ""}">
    <div class="p-4">
        <header class="card-header font-bold text-xl">{question.title}</header>
        <section class="p-4">
            {question.body}
            <br><i class="black text-slate-600">{format(question.created_on)}</i>
        </section>
        <hr class="opacity-80" />
        <footer class="card-footer pt-4">
            {#if question.response}
                {question.response}
                <br><i class="black text-slate-600">{format(question.created_on)}</i>
            {:else}
                <p class="text-center text-xl">UNBEANTWORTET</p>
            {/if}
        </footer>
    </div>
    <hr class="opacity-100" />
    <footer class="px-8 py-4 flex flex-row gap-4">
		<span>An: {question.delegate_id}</span>
        <span>Von: {question.delegate_id}</span>
        <div class="flex flex-row gap-5 ml-auto">
            <span>
                <FontAwesomeIcon icon={faThumbsUp} class="" />
            </span>
            {(score > 0 ? "+" : "") + score}
            <span>
                <FontAwesomeIcon icon={faThumbsDown} class="" />
            </span>
        </div>
	</footer>
</div>