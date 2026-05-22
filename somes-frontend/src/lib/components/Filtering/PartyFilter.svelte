<script lang="ts">
	import { Select } from 'bits-ui';
	import upDownArrowIcon from '$lib/assets/misc_icons/up-down-arrow.svg?raw';
	import checkmark_small from '$lib/assets/misc_icons/checkmark_small.svg?raw';

	export type PartyFilterOption = {
		name: string;
		color: string;
	};

	interface Props {
		parties: PartyFilterOption[];
		selectedNames: string[];
		onSelectionChange?: (selectedParties: PartyFilterOption[], selectedNames: string[]) => void;
	}

	let { parties, selectedNames = $bindable(), onSelectionChange }: Props = $props();

	let selectedParties = $derived(parties.filter((party) => selectedNames.includes(party.name)));
</script>

<Select.Root
	type="multiple"
	bind:value={selectedNames}
	onValueChange={(value) => {
		onSelectionChange?.(
			parties.filter((party) => value.includes(party.name)),
			value
		);
	}}
	items={parties.map((party) => ({ value: party.name, label: party.name }))}
>
	<Select.Trigger
		class="flex h-full grow touch-manipulation items-center justify-center gap-1 rounded-xl bg-secondary-500 px-2 text-white transition-colors placeholder:text-gray-600 focus:ring-2 focus:ring-gray-400 focus:ring-offset-2 focus:outline-none md:grow-0"
	>
		<div class="flex min-w-0 items-center gap-2">
			{#each selectedParties.slice(0, 1) as party}
				<div class="h-3 w-3 shrink-0 rounded-full" style="background-color: {party.color};"></div>
				<span class="truncate">{party.name}</span>
			{/each}
			{#if selectedParties.length > 1}
				<span class="truncate">+{selectedParties.length - 1} weitere</span>
			{/if}
			{#if selectedParties.length === 0}
				<span class="truncate">Alle Parteien</span>
			{/if}
		</div>
		<span class="shrink-0">
			{@html upDownArrowIcon}
		</span>
	</Select.Trigger>
	<Select.Portal>
		<Select.Content
			class="z-500 max-h-60 w-[200px] min-w-[var(--bits-select-anchor-width)] overflow-hidden rounded-xl border border-gray-200 bg-surface-100 shadow-lg dark:bg-surface-500"
			sideOffset={8}
		>
			<Select.Viewport class="p-1">
				{#each parties as party}
					<Select.Item
						class="flex h-10 w-full cursor-pointer justify-between rounded-lg py-3 pr-1.5 pl-3 text-sm capitalize transition-all duration-75 outline-none select-none data-highlighted:bg-gray-100 dark:data-highlighted:bg-gray-400"
						value={party.name}
						label={party.name}
					>
						{#snippet children({ selected })}
							<div class="flex min-w-0 items-center gap-2">
								<div
									class="h-3 w-3 shrink-0 rounded-full"
									style="background-color: {party.color};"
								></div>
								<span class="truncate">{party.name}</span>
							</div>
							{#if selected}
								<div class="ml-auto h-4 shrink-0 stroke-black dark:stroke-white">
									{@html checkmark_small}
								</div>
							{/if}
						{/snippet}
					</Select.Item>
				{/each}
			</Select.Viewport>
		</Select.Content>
	</Select.Portal>
</Select.Root>
