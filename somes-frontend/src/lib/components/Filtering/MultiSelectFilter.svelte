<script lang="ts" generics="T extends { value: string; label: string }">
	import { Select } from 'bits-ui';
	import type { Snippet } from 'svelte';
	import upDownArrowIcon from '$lib/assets/misc_icons/up-down-arrow.svg?raw';
	import checkmarkIcon from '$lib/assets/misc_icons/checkmark_small.svg?raw';

	interface Props {
		items: T[];
		value: string[];
		allLabel: string;
		itemLabel?: Snippet<[T]>;
		onValueChange?: (value: string[]) => void;
	}

	let { items, value = $bindable([]), allLabel, itemLabel, onValueChange }: Props = $props();

	let open = $state(false);

	// sort selected items to the top
	let displayItems = $state<T[]>([]);
	$effect(() => {
		if (open) return;
		const selected = new Set(value);
		displayItems = [...items].sort(
			(a, b) => Number(selected.has(b.value)) - Number(selected.has(a.value))
		);
	});

	let selectedItems = $derived(items.filter((item) => value.includes(item.value)));
</script>

{#snippet labelOf(item: T)}
	{#if itemLabel}
		{@render itemLabel(item)}
	{:else}
		<span class="truncate">{item.label}</span>
	{/if}
{/snippet}

<Select.Root
	type="multiple"
	bind:value
	bind:open
	{onValueChange}
	items={items.map((item) => ({ value: item.value, label: item.label }))}
>
	<Select.Trigger
		class="flex h-full w-full touch-manipulation items-center justify-center gap-1 rounded-xl bg-secondary-500 px-2 text-white transition-colors placeholder:text-gray-600 focus:ring-2 focus:ring-gray-400 focus:ring-offset-2 focus:outline-none lg:w-auto lg:px-3"
	>
		<div class="flex min-w-0 items-center gap-2">
			{#each selectedItems.slice(0, 1) as item (item.value)}
				{@render labelOf(item)}
			{/each}
			{#if selectedItems.length > 1}
				<span class="truncate">+{selectedItems.length - 1}</span>
			{/if}
			{#if selectedItems.length === 0}
				<span class="truncate">{allLabel}</span>
			{/if}
		</div>
		{@html upDownArrowIcon}
	</Select.Trigger>
	<Select.Portal>
		<Select.Content
			class="z-500 max-h-60 w-[calc(100vw-2rem)] min-w-[var(--bits-select-anchor-width)] overflow-hidden rounded-xl border border-gray-200 bg-surface-100 shadow-lg md:w-[200px] dark:bg-surface-500"
			sideOffset={8}
		>
			<Select.Viewport class="p-1">
				{#each displayItems as item (item.value)}
					<Select.Item
						class="flex h-10 w-full cursor-pointer justify-between rounded-lg py-3 pr-1.5 pl-3 text-sm transition-all duration-75 outline-none select-none data-highlighted:bg-gray-100 dark:data-highlighted:bg-gray-400"
						value={item.value}
						label={item.label}
					>
						{#snippet children({ selected })}
							<div class="flex min-w-0 items-center gap-2">
								{@render labelOf(item)}
							</div>
							{#if selected}
								<div class="ml-auto h-4 shrink-0 stroke-black dark:stroke-white">
									{@html checkmarkIcon}
								</div>
							{/if}
						{/snippet}
					</Select.Item>
				{/each}
			</Select.Viewport>
		</Select.Content>
	</Select.Portal>
</Select.Root>
