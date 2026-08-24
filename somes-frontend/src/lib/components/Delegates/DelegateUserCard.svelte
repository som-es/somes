<script lang="ts">
	import { t } from '$lib/i18n/i18n.svelte';
	import upDownArrowIcon from '$lib/assets/misc_icons/up-down-arrow.svg?raw';
	import checkmark_small from '$lib/assets/misc_icons/checkmark_small.svg?raw';
	import type { Delegate } from '$lib/types';
	import { Select } from 'bits-ui';
	import DelegateCard from './DelegateCard.svelte';
	import { updateDelegateFavo } from '$lib/api/authed';
	import { isHasError } from '$lib/api/api';

	interface Props {
		delegate: Delegate;
		currentNotifyInfoDays: number;
	}

	const { delegate, currentNotifyInfoDays }: Props = $props();

	let notifyInfoDays = $derived(currentNotifyInfoDays.toString());
	const notifyInfoDaysOptions = [
		{ value: '7', label: t('userCard.everyWeek') },
		{ value: '14', label: t('userCard.every2Weeks') },
		{ value: '30', label: t('userCard.everyMonth') },
		{ value: '60', label: t('userCard.every2Months') }
	];
</script>

<DelegateCard {delegate} showMoreDetailsBtn onlyTop={true}>
	{#snippet footerButtons()}
		<Select.Root
			type="single"
			bind:value={notifyInfoDays}
			onValueChange={async () => {
				const res = await updateDelegateFavo({
					delegate_id: delegate.id,
					user_info_days: +notifyInfoDays
				});
				if (isHasError(res)) {
					console.error(res);
				}
			}}
			items={notifyInfoDaysOptions}
		>
			<Select.Trigger
				class="flex touch-manipulation items-center rounded-xl
								    bg-primary-600 p-2 px-3 text-white focus:ring-2 focus:ring-gray-400 focus:ring-offset-2 focus:outline-none
				"
			>
				<span>
					<h4>Infoabstand</h4>
				</span>
				{@html upDownArrowIcon}
			</Select.Trigger>
			<Select.Portal>
				<Select.Content
					class="z-500 max-h-60 w-[200px] min-w-[var(--bits-select-anchor-width)] overflow-hidden rounded-xl border border-gray-200 bg-surface-100 shadow-lg dark:bg-surface-500"
					sideOffset={8}
				>
					<Select.Viewport class="p-1">
						{#each notifyInfoDaysOptions as option (option.value)}
							<Select.Item
								class="flex h-10 w-full cursor-pointer items-center rounded-lg py-3 pr-1.5 pl-3 text-sm capitalize transition-all duration-75 outline-none select-none data-highlighted:bg-gray-100 dark:data-highlighted:bg-gray-400"
								value={option.value}
								label={option.label}
							>
								{#snippet children({ selected })}
									<div class="flex items-center gap-2">
										{option.label}
									</div>
									{#if selected}
										<div class="ml-auto h-4 stroke-black dark:stroke-white">
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
	{/snippet}
</DelegateCard>
