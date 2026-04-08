<script lang="ts">
	interface Props {
		title: string;
		description: string;
		checked: boolean;
		onchange: () => void;
	}

	let { title, description, checked = $bindable(), onchange }: Props = $props();

	function toggle() {
		checked = !checked;
		onchange();
	}
</script>

<button
	onclick={toggle}
	class="px-4 py-2 rounded-lg bg-gray-200 dark:bg-gray-500 w-full sm:w-55 text-left cursor-pointer transition-opacity hover:opacity-80 border-2 {checked ? 'border-secondary-400' : 'border-transparent'}"
>
	<div class="flex items-center justify-between">
		<p class="text-base font-semibold">{title}</p>
		<div class="ml-2 flex items-center justify-center w-4 h-4 rounded-full border-2 {checked ? 'bg-secondary-500 border-secondary-500' : 'bg-gray-200 dark:bg-gray-500 border-gray-400 dark:border-gray-300'}">
			{#if checked}
				<svg xmlns="http://www.w3.org/2000/svg" class="w-2.5 h-2.5 text-white" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round">
					<polyline points="20 6 9 17 4 12" />
				</svg>
			{/if}
		</div>
	</div>
	<p class="text-sm text-gray-600 dark:text-gray-300">{description}</p>
</button>
