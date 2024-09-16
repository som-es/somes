<script lang="ts">
	import { TabAnchor, TabGroup } from '@skeletonlabs/skeleton';
	import { page } from '$app/stores';
	import homeIcon from '$lib/assets/icons/home.svg?raw';
	import userIcon from '$lib/assets/icons/user.svg?raw';
	import statsIcon from '$lib/assets/icons/statistics.svg?raw';
	import waloIcon from '$lib/assets/icons/walo.svg?raw';
	import VoteParliament from '../Parliaments/VoteParliament.svelte';
	import { mockDelegates, mockVoteResult } from '$lib/parliaments/mock';
	import { base } from '$app/paths';

	$: isSelected = (href: string) => $page.url.pathname?.includes(href);
</script>

<TabGroup
	justify="justify-center"
	active="variant-filled-primary"
	hover="hover:variant-soft-primary"
	flex="flex-1 lg:flex-none"
	rounded=""
	border=""
	class="bg-surface-100-800-token w-full"
>
	<TabAnchor href="{base}/home" selected={isSelected('/home')}>
		<div class="flex justify-center flex-col items-center">
			<div>
				{@html homeIcon}
			</div>
			<div>Neuigkeiten</div>
		</div>
	</TabAnchor>
	<TabAnchor href="{base}/wahlhelfer" selected={isSelected('/wahlhelfer')}>
		<div class="flex justify-center flex-col items-center">

			<div class="p-0 w-[28px] h-[42px]">
				{@html waloIcon}
			</div>
			<div>Wahlhelfer</div>

			<!-- <span class="" style="font-size: x-small;">Wahlhelfer</span> -->
			<!-- <span style="font-size: x-small;">NRWAHL 2024</span> -->
		</div>
	</TabAnchor>
	<!-- <TabAnchor href="/statistics" selected={isSelected('/statistics')}>
		<div class="flex justify-center flex-col items-center">
			<div class="w-8">
				{@html statsIcon}
			</div>
			<div>Statistiken</div>
		</div>
	</TabAnchor> -->
	<TabAnchor href="{base}/vote_history" selected={isSelected('/vote_history')}>
		<div class="flex justify-center flex-col items-center">
			<div>
				<VoteParliament
					class="w-[60px]"
					againstOpacity={0.3}
					voteResult={mockVoteResult()}
					dels={[]}
					delsAtDate={mockDelegates()}
					preview
				/>
			</div>
			<!-- <div>
				{@html userIcon}
			</div> -->
			<div>Abstimmungen</div>
		</div>
	</TabAnchor>
</TabGroup>
