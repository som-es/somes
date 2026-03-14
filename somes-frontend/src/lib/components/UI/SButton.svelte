<!-- A button component -->
<script lang="ts">
	import type { Snippet } from 'svelte';
	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();

	interface Props {
		onclick?: (event: MouseEvent) => void;
		title?: string;
		class?: string;
		disabled?: boolean;
		children?: Snippet;
	}

	let { onclick, title, class: className = '', disabled = false, children }: Props = $props();

	function handleClick(event: MouseEvent) {
		dispatch('click', event);
		onclick?.(event);
	}
</script>

<button {title} {disabled} class="button {className}" onclick={handleClick}>
	{@render children?.()}
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
		filter: brightness(1.1);
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
