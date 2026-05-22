<script lang="ts">
	import { Dialog } from 'bits-ui';
	import type { Delegate } from '$lib/types';
	import { partyColors } from '$lib/partyColor';
	import { groupPartyDelegates } from '$lib/parliaments/defaultParliament';
	import { mockDelegatesNoColor, mockVoteResult } from '$lib/parliaments/mock';
	import { getSeats } from '$lib/caching/seats';
	import ModalCloseButton from '$lib/components/UI/ModalCloseButton.svelte';
	import VoteParliament2 from './VoteParliament2.svelte';

	interface Props {
		delegates: Delegate[];
		delegate?: Delegate | null;
		syncDelegates?: Delegate[];
		allSeats: Map<string, number[]> | null;
		selectedPeriod: string;
		supplyDate: Date | null;
		hasSeatInfo: boolean;
	}

	let {
		delegates,
		delegate = $bindable(),
		syncDelegates = $bindable([]),
		allSeats,
		selectedPeriod,
		supplyDate,
		hasSeatInfo
	}: Props = $props();
</script>

<Dialog.Root>
	<Dialog.Trigger
		class="flex h-10 w-10 shrink-0 touch-manipulation items-center justify-center rounded-xl border-[2px] border-gray-400 bg-transparent p-1.5 transition-colors hover:bg-tertiary-400/30 focus-visible:ring-2 focus-visible:ring-gray-400 focus-visible:ring-offset-2 focus-visible:outline-none lg:hidden"
		aria-label="Sitzplan anzeigen"
		title="Sitzplan anzeigen"
	>
		<span class="block w-16 scale-150">
			<VoteParliament2
				againstOpacity={0.3}
				voteResult={mockVoteResult()}
				delegates={mockDelegatesNoColor()}
				allSeats={new Map([['XX', getSeats(new Map(), 'XX', true)]])}
				preview
				overrideDelegates
				noSeats
				useOffset={false}
				enforceSvg
				showGovs={false}
				forceColor="black"
			/>
		</span>
	</Dialog.Trigger>
	<Dialog.Portal>
		<Dialog.Overlay
			class="fixed inset-0 z-70 bg-black/70 data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:animate-in data-[state=open]:fade-in-0 lg:hidden"
		/>
		<Dialog.Content
			class="fixed top-[50%] left-[50%] z-70 flex max-h-[92dvh] w-[94vw] translate-x-[-50%] translate-y-[-50%] flex-col overflow-hidden rounded-xl bg-primary-100 p-4 shadow-xl outline-hidden data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95 data-[state=open]:animate-in data-[state=open]:fade-in-0 data-[state=open]:zoom-in-95 lg:hidden dark:bg-primary-600"
		>
			<div class="mb-3 flex items-center justify-between gap-3">
				<Dialog.Title class="text-lg font-semibold text-gray-900 dark:text-gray-100">
					Sitzplan
				</Dialog.Title>
				<Dialog.Close>
					<ModalCloseButton class="p-1" />
				</Dialog.Close>
			</div>
			<Dialog.Description class="sr-only">
				Interaktiver Sitzplan der aktuell gefilterten Abgeordneten.
			</Dialog.Description>

			{#if delegates && delegates.length > 0 && supplyDate}
				<div class="min-h-0 overflow-y-auto rounded-xl bg-primary-300 p-3 dark:bg-primary-200">
					<div
						class="mb-3 grid grid-cols-[min-content_auto_min-content] items-center gap-x-2 gap-y-0"
					>
						{#each [...groupPartyDelegates(structuredClone(delegates))].sort((a, b) => b[1].length - a[1].length) as [party, partyDelegates]}
							<div
								class="h-2.5 w-2.5 rounded-full"
								style="background-color: {partyColors.get(party) ?? '#ccc'};"
							></div>
							<span class="text-sm font-medium text-gray-800">{party}</span>
							<span class="text-right text-sm font-medium text-gray-800">
								({partyDelegates.length})
							</span>
						{/each}
					</div>
					<VoteParliament2
						againstOpacity={1}
						voteResult={null}
						bind:delegate
						bind:syncDelegates
						{delegates}
						{allSeats}
						gp={selectedPeriod}
						{supplyDate}
						orderingFactor={-1}
						showGovs={true}
						overrideDelegates
						noSeats={!hasSeatInfo}
						useOffset={hasSeatInfo}
					/>
				</div>
			{/if}
		</Dialog.Content>
	</Dialog.Portal>
</Dialog.Root>
