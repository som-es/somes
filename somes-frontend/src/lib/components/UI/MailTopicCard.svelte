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
	class="w-full cursor-pointer rounded-lg border-2 bg-gray-200 px-4 py-2 text-left transition-opacity hover:opacity-80 sm:w-55 dark:bg-gray-500 {checked
		? 'border-secondary-400'
		: 'border-transparent'}"
>
	<div class="flex items-center justify-between">
		<p class="text-base font-semibold">{title}</p>
		<div
			class="ml-2 flex h-4 w-4 items-center justify-center rounded-full border-2 {checked
				? 'border-secondary-500 bg-secondary-500'
				: 'border-gray-400 bg-gray-200 dark:border-gray-300 dark:bg-gray-500'}"
		>
			{#if checked}
				<svg
					xmlns="http://www.w3.org/2000/svg"
					class="h-2.5 w-2.5 text-white"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="3"
					stroke-linecap="round"
					stroke-linejoin="round"
				>
					<polyline points="20 6 9 17 4 12" />
				</svg>
			{/if}
		</div>
	</div>
	<p class="text-sm text-gray-600 dark:text-gray-300">{description}</p>
</button>
