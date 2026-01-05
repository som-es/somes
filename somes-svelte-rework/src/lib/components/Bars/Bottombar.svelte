<script lang="ts">
	import { Tabs } from '@skeletonlabs/skeleton-svelte';
	import { page } from '$app/stores';
	import homeIcon from '$lib/assets/icons/home.svg?raw';
	import userIcon from '$lib/assets/icons/user.svg?raw';
	import statsIcon from '$lib/assets/icons/statistics.svg?raw';
	import delegatesIcon from '$lib/assets/icons/delegates.svg?raw';
	import { mockDelegates, mockVoteResult } from '$lib/parliaments/mock';
	import { base } from '$app/paths';
	import VoteParliament2 from '../Parliaments/VoteParliament2.svelte';

	$: isSelected = (href: string) => $page.url.pathname?.includes(href);
</script>

<Tabs
	justify="justify-center"
	active="preset-filled-primary-500"
	hover="hover:preset-tonal-primary"
	flex="flex-1 lg:flex-none"
	rounded=""
	border=""
	class="bg-surface-100-900 w-full"
>
	<Tabs.Control href="{base}/home" selected={isSelected('/home')}>
		<!-- <svelte:fragment slot="lead">
			{@html homeIcon}
		</svelte:fragment> -->
		<div class="flex justify-center flex-col items-center ">
			<div>
				{@html homeIcon}
			</div>
			<span class="text-sm">Neuigkeiten</span>
		</div>
	</Tabs.Control>
	<Tabs.Control href="{base}/delegates" selected={isSelected('/delegates')}>
		<div class="flex justify-center flex-col items-center">
			<div class="p-0 w-[36px] h-[34px]">
				{@html delegatesIcon}
			</div>
			<span class="text-sm">Abgeordnete</span>
		</div>
	</Tabs.Control>
	<!-- <TabAnchor href="{base}/wahlhelfer" selected={isSelected('/wahlhelfer')}>
		<div class="flex justify-center flex-col items-center">
			<div class="p-0 w-[28px] h-[42px]">
				{@html waloIcon}
			</div>
			<div>Wahlhelfer</div>
		</div>
	</TabAnchor> -->
	<!-- <TabAnchor href="/statistics" selected={isSelected('/statistics')}>
		<div class="flex justify-center flex-col items-center">
			<div class="w-8">
				{@html statsIcon}
			</div>
			<div>Statistiken</div>
		</div>
	</TabAnchor> -->
	<Tabs.Control href="{base}/history/votes" selected={isSelected('/history')}>
		<div class="flex justify-center flex-col items-center">
			<div>
				<VoteParliament2
					class="w-[60px]"
					againstOpacity={0.3}
					voteResult={mockVoteResult()}
					delegates={mockDelegates()}
					preview
					overrideDelegates
					noSeats
					useOffset={false}
				/>
			</div>
			<!-- <div>
				{@html userIcon}
			</div> -->
			<span class="text-sm">Abstimmungen</span>
		</div>
	</Tabs.Control>
</Tabs>
