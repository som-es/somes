<!-- A button component -->
<script lang="ts">
	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();

	function handleClick(event: any) {
		dispatch('click', event);
	}
</script>

<button title="{$$props.title}" class="button {$$props.class}" on:click={handleClick}>
	<slot />
</button>

<style>
	.button {
		text-align: center;
		padding: 0.5rem 1rem;
		border: 1px solid #000;
		border-radius: 5px;
		cursor: pointer;
		transition:
			background-color 0.2s,
			filter 0.2s;
	}

	.button:hover {
		filter: brightness(0.8);
	}

	.button:disabled {
		cursor: not-allowed;
		opacity: 0.5;
	}

	.offset-button {
		margin-top: 1rem;
		margin-bottom: 1rem;
	}
</style>
