<script lang="ts">
	import { page } from '$app/stores';
	import userIcon from '$lib/assets/icons/user.svg?raw';
	import delegatesIcon from '$lib/assets/icons/delegates.svg?raw';
	import homeIcon from '$lib/assets/icons/home.svg?raw';
	import statsIcon from '$lib/assets/icons/statistics.svg?raw';
	import { AppRail, AppRailAnchor, AppRailTile, getDrawerStore } from '@skeletonlabs/skeleton';
	import VoteParliament from '../Parliaments/VoteParliament.svelte';
	import { mockDelegates, mockVoteResult } from '$lib/parliaments/mock';

	let currentTile: number = 0;
	$: isSelected = (href: string) => {
		return $page.url.pathname?.includes(href);
	};

	// $: activeAnchorColor = (href: string) => ($page.url.pathname?.includes(href) ? 'flex flex-col justify-center items-stretch bg-primary-active-token' : 'bg-primary-hover-token');
</script>

<div class="grid grid-cols-[auto_1fr] h-full bg-surface-50-900-token {$$props.class ?? ''}">
	<AppRail>
		<!-- <svelte:fragment slot="lead">
			<AppRailAnchor href="/" >(icon)</AppRailAnchor>
		</svelte:fragment> -->
		<!-- --- -->
		<AppRailAnchor
			selected={isSelected('/home')}
			href="/home"
			bind:group={currentTile}
			name="Neuigkeiten"
			value={0}
			title="Neuigkeiten"
		>
			<svelte:fragment slot="lead"
				><div class="fill-current stroke-current">{@html homeIcon}</div></svelte:fragment
			>
			<span style="font-size: x-small;">Neuigkeiten</span>
		</AppRailAnchor>
		<hr />
		<AppRailAnchor
			selected={isSelected('/delegates')}
			href="/delegates"
			bind:group={currentTile}
			name="Abgeordnete"
			value={1}
			title="Abgeordnete"
		>
			<svelte:fragment slot="lead">{@html delegatesIcon}</svelte:fragment>
			<span style="font-size: x-small;">Abgeordnete</span>
		</AppRailAnchor>
		<hr />
	
		<AppRailAnchor
			selected={isSelected('/statistics')}
			href="/statistics"
			bind:group={currentTile}
			name="Statistiken"
			value={2}
			title="Statistiken"
		>
			<svelte:fragment slot="lead"><div class="w-10">{@html statsIcon}</div></svelte:fragment>
			<span style="font-size: x-small;">Statistiken</span>
		</AppRailAnchor>
		
		<hr />
		<AppRailAnchor
			selected={isSelected('/vote_history')}
			href="/vote_history"
			bind:group={currentTile}
			name="Abstimmungen"
			value={1}
			title="Abstimmungen"
		>
			<svelte:fragment slot="lead">
				<VoteParliament againstOpacity={0.3} voteResult={mockVoteResult()} dels={[]} delsAtDate={mockDelegates()} preview />
			</svelte:fragment>
			<span style="font-size: x-small;">Abstimmungen</span>
		</AppRailAnchor>
		<!-- <AppRailAnchor class={activeAnchorColor("/statistics")} href="/statistics" bind:group={currentTile} name="Statistiken" value={2} title="Statistiken">
			<svelte:fragment slot="lead">{@html statsIcon}</svelte:fragment>
			<span style="font-size: x-small;">Statistiken</span>
		</AppRailAnchor> -->
		<hr />
		<!-- --- -->
		<svelte:fragment slot="trail">
			<AppRailAnchor href="/" target="_blank" title="Account">
				<svelte:fragment slot="lead">{@html userIcon}</svelte:fragment>
				<span style="font-size: smaller;">Benutzer</span>
			</AppRailAnchor>
		</svelte:fragment>
	</AppRail>
	<!-- Mobile Only -->
	<!-- prettier-ignore -->
	<!-- <AppRailAnchor href="/" class="lg:hidden" on:click={() => { onClickAnchor() }}> -->
	<!-- <svelte:fragment slot="lead"><i class="fa-solid fa-home text-2xl"></i></svelte:fragment> -->
	<!-- <span>Home</span> -->
	<!-- </AppRailAnchor> -->
</div>
