<script lang="ts">
	import { Popover } from 'bits-ui';
	import SortIcon from './SortIcon.svelte';

	interface Props {
		sortOrder: 'relevance' | 'Desc' | 'Asc';
	}

	let { sortOrder = $bindable() }: Props = $props();
	let isOpen = $state(false);
</script>

<Popover.Root bind:open={isOpen}>
	<Popover.Trigger
		class="flex h-full w-9 shrink-0 items-center justify-center transition-colors {sortOrder !==
		'relevance'
			? 'text-primary-600 dark:text-primary-400'
			: 'text-gray-500 dark:text-gray-400'} hover:text-gray-800 dark:hover:text-gray-200"
	>
		<SortIcon {sortOrder} />
	</Popover.Trigger>
	<Popover.Content sideOffset={8} class="z-[30]">
		<div
			class="touch-manipulation rounded-xl border border-gray-300 bg-surface-50 px-4 py-3 shadow-lg dark:bg-surface-600"
		>
			<span class="text-sm font-semibold text-gray-800 dark:text-gray-50">Sortierung</span>
			<div class="mt-2 flex gap-1 rounded-lg border border-primary-300 dark:border-primary-400">
				<button
					class="cursor-pointer rounded-lg px-2 py-1 text-xs md:text-sm {sortOrder === 'relevance'
						? 'bg-primary-300 dark:bg-primary-400'
						: ''}"
					onclick={() => {
						sortOrder = 'relevance';
						isOpen = false;
					}}
				>
					Relevanz
				</button>
				<button
					class="cursor-pointer rounded-lg px-2 py-1 text-xs md:text-sm {sortOrder === 'Desc'
						? 'bg-primary-300 dark:bg-primary-400'
						: ''}"
					onclick={() => {
						sortOrder = 'Desc';
						isOpen = false;
					}}
				>
					Neueste
				</button>
				<button
					class="cursor-pointer rounded-lg px-2 py-1 text-xs md:text-sm {sortOrder === 'Asc'
						? 'bg-primary-300 dark:bg-primary-400'
						: ''}"
					onclick={() => {
						sortOrder = 'Asc';
						isOpen = false;
					}}
				>
					Älteste
				</button>
			</div>
			<Popover.Arrow class="rounded-sm fill-current stroke-gray-300 text-gray-300" />
		</div>
	</Popover.Content>
</Popover.Root>
