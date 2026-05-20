<script lang="ts">
	interface Props {
		asButton?: boolean;
		class?: string;
		'aria-label'?: string;
		onclick?: (event: MouseEvent) => void;
	}

	let {
		asButton = false,
		class: className = '',
		'aria-label': ariaLabel = 'Schließen',
		...props
	}: Props = $props();
</script>

{#if asButton}
	<button
		type="button"
		aria-label={ariaLabel}
		class="unselectable text-2xl text-gray-500 transition-colors hover:text-gray-700 md:text-3xl lg:text-4xl {className}"
		{...props}
	>
		<span aria-hidden="true">✕</span>
	</button>
{:else}
	<span
		class="unselectable text-2xl text-gray-500 transition-colors hover:text-gray-700 md:text-3xl lg:text-4xl {className}"
	>
		<span class="sr-only">{ariaLabel}</span>
		<span aria-hidden="true">✕</span>
	</span>
{/if}
