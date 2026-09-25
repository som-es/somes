<script lang="ts" generics="T extends { value: string; label: string }">
	import { Select } from 'bits-ui';
	import upDownArrowIcon from '$lib/assets/misc_icons/up-down-arrow.svg?raw';
	import checkmarkIcon from '$lib/assets/misc_icons/checkmark_small.svg?raw';

	let {
		items,
		value = $bindable(''),
		label
	}: {
		items: T[];
		value: string;
		label: string;
	} = $props();

	let open = $state(false);
	let selectedItem = $derived(items.find((item) => item.value === value));
</script>

<Select.Root
	type="single"
	bind:value
	bind:open
	items={items.map((item) => ({ value: item.value, label: item.label }))}
>
	<Select.Trigger
		class="flex h-10 touch-manipulation items-center justify-center gap-2 rounded-xl bg-secondary-500 px-3 text-white transition-colors hover:bg-secondary-600 focus:ring-2 focus:ring-gray-400 focus:ring-offset-2 focus:outline-none"
	>
		<span class="text-xs text-white/80">{label}</span>
		<span class="font-semibold">{selectedItem?.label ?? ''}</span>
		<span class="w-4 shrink-0">{@html upDownArrowIcon}</span>
	</Select.Trigger>
	<Select.Portal>
		<Select.Content
			class="z-500 max-h-60 w-[calc(100vw-2rem)] min-w-[var(--bits-select-anchor-width)] overflow-hidden rounded-xl border border-gray-200 bg-surface-100 shadow-lg md:w-[220px] dark:bg-surface-500"
			sideOffset={8}
		>
			<Select.Viewport class="p-1">
				{#each items as item (item.value)}
					<Select.Item
						class="flex h-10 w-full cursor-pointer items-center justify-between rounded-lg px-3 text-sm outline-none select-none data-highlighted:bg-gray-100 dark:data-highlighted:bg-gray-400"
						value={item.value}
						label={item.label}
					>
						{#snippet children({ selected })}
							<span>{item.label}</span>
							{#if selected}
								<span class="ml-auto h-4 shrink-0 stroke-black dark:stroke-white">
									{@html checkmarkIcon}
								</span>
							{/if}
						{/snippet}
					</Select.Item>
				{/each}
			</Select.Viewport>
		</Select.Content>
	</Select.Portal>
</Select.Root>
