<script lang="ts">
	import type { Question } from "$lib/types";

    export let question: Question;
</script>

<div class="card p-4">
    <header class="card-header font-bold text-xl">{question.title}</header>
    <section class="p-4">{question.body}</section>
    <hr class="opacity-80" />
    <footer class="card-footer pt-4">{question.response}</footer>
</div>