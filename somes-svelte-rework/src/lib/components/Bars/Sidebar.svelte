<script lang="ts">
	import { page } from '$app/stores';
	import userIcon from '$lib/assets/icons/user.svg?raw';
	import delegatesIcon from '$lib/assets/icons/delegates.svg?raw';
	import homeIcon from '$lib/assets/icons/home.svg?raw';
	import statsIcon from '$lib/assets/icons/statistics.svg?raw';
	import waloIcon from '$lib/assets/icons/walo.svg?raw';
	import { AppRail, AppRailAnchor, AppRailTile, getDrawerStore } from '@skeletonlabs/skeleton';
	import { mockDelegates, mockVoteResult } from '$lib/parliaments/mock';
	import { base } from '$app/paths';
	import VoteParliament2 from '../Parliaments/VoteParliament2.svelte';
	import { loginDrawerSettings } from '../Login/constants';
	import { get } from 'svelte/store';
	import { jwtStore } from '$lib/caching/stores/stores';
	import { isHasError, renew_token } from '$lib/api';
	import { gotoHistory } from '$lib/goto';

	const drawerStore = getDrawerStore();

	let currentTile: number = 0;
	$: isSelected = (href: string) => {
		return $page.url.pathname?.includes(href);
	};

	$: accountOrLogin = async () => {
		const jwt = get(jwtStore);
		if (jwt) {
			if (isHasError(await renew_token())) {
				drawerStore.open(loginDrawerSettings);
			} else {
				gotoHistory('/user');
			}
		} else {
			drawerStore.open(loginDrawerSettings);
		}
	};

	// $: activeAnchorColor = (href: string) => ($page.url.pathname?.includes(href) ? 'flex flex-col justify-center items-stretch bg-primary-active-token' : 'bg-primary-hover-token');
</script>

<div class="h-full bg-surface-50-900-token {$$props.class ?? ''}">
	<AppRail width="w-20 2xl:w-32">
		<!-- <svelte:fragment slot="lead">
			<AppRailAnchor href="/" >(icon)</AppRailAnchor>
		</svelte:fragment> -->
		<!-- --- -->
		<AppRailAnchor
			selected={isSelected('/home')}
			href="{base}/home"
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
			href="{base}/delegates"
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
			href="{base}/statistics"
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
			href="{base}/vote_history"
			bind:group={currentTile}
			name="Abstimmungen"
			value={3}
			title="Abstimmungen"
		>
			<svelte:fragment slot="lead">
				<VoteParliament2
					againstOpacity={0.3}
					voteResult={mockVoteResult()}
					delegates={mockDelegates()}
					preview
					overrideDelegates
					noSeats
					useOffset={false}
				/>
			</svelte:fragment>
			<span style="font-size: x-small;">Abstimmungen</span>
		</AppRailAnchor>
		<hr />

		<AppRailAnchor
			selected={isSelected('/wahlhelfer')}
			href="{base}/wahlhelfer"
			bind:group={currentTile}
			name="Wahlhelfer"
			value={4}
			title="Wahlhelfer"
		>
			<svelte:fragment slot="lead"><div class="w-10">{@html waloIcon}</div></svelte:fragment>
			<span style="font-size: x-small;">Wahlhelfer</span>
			<br />
			<span style="font-size: x-small;">NRWAHL 2024</span>
		</AppRailAnchor>

		<!-- <AppRailAnchor class={activeAnchorColor("/statistics")} href="/statistics" bind:group={currentTile} name="Statistiken" value={2} title="Statistiken">
			<svelte:fragment slot="lead">{@html statsIcon}</svelte:fragment>
			<span style="font-size: x-small;">Statistiken</span>
		</AppRailAnchor> -->
		<hr />
		<!-- --- -->
		<svelte:fragment slot="trail">
			<div
				on:click={async () => {
					await accountOrLogin();
				}}
				tabindex="0"
				role="button"
				on:keydown={async (event) => {
					if (event.key === 'Enter' || event.key === ' ') {
						await accountOrLogin();
					}
				}}
			>
				<AppRailAnchor
					selected={isSelected('/user')}
					bind:group={currentTile}
					name="Account"
					value={5}
					title="Account"
					on:click={() => {}}
				>
					<svelte:fragment slot="lead">{@html userIcon}</svelte:fragment>

					<span style="font-size: smaller;">Benutzer</span>
				</AppRailAnchor>
			</div>
		</svelte:fragment>
	</AppRail>
	<!-- Mobile Only -->
	<!-- prettier-ignore -->
	<!-- <AppRailAnchor href="/" class="lg:hidden" on:click={() => { onClickAnchor() }}> -->
	<!-- <svelte:fragment slot="lead"><i class="fa-solid fa-home text-2xl"></i></svelte:fragment> -->
	<!-- <span>Home</span> -->
	<!-- </AppRailAnchor> -->
</div>
