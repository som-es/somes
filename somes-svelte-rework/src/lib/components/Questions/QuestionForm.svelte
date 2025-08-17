<script lang="ts">
	export let question: string;
	export let details: string;
</script>

<!-- TODO: add form validation like min length and word filters -->
<!-- TODO: add markdown support (maybe) -->
<div class="flex flex-col w-full h-max gap-[2vh]">
	<label class="label">
		<span>Frage</span>
		<input
			class="input h-[2rem] rounded-lg p-2"
			type="text"
			placeholder="Titel"
			bind:value={question}
		/>
	</label>
	<label class="label">
		<span>Details</span>
		<textarea
			class="textarea p-2 resize-none"
			rows="6"
			placeholder="Genauere Details zur Frage im Titel"
			bind:value={details}
		/>
	</label>
</div>