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
	import { isHasError } from '$lib/api/api';
	import { gotoHistory } from '$lib/goto';
	import { renew_token } from '$lib/api/authed';

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


	const submenu =  [
		{
			title: 'Statistiken',
			route: "/statistics",
			list: [
				{ href: `${base}/statistics/overview`, label: 'Übersicht', keywords: '' },
				{ href: `${base}/statistics/speechtime`, label: 'Redezeit', keywords: '' },
				{ href: `${base}/statistics/total_speeches`, label: 'Gehaltene Reden', keywords: '' },
				{ href: `${base}/statistics/speechcomplexity`, label: 'Sprach Komplexität', keywords: '' },
				{ href: `${base}/statistics/absences`, label: 'Abwesenheiten', keywords: '' },
				{ href: `${base}/statistics/age`, label: 'Alter', keywords: '' },
				{ href: `${base}/statistics/division_accuracy`, label: 'Bereichssprechergenauigkeit', keywords: '' },
				{ href: `${base}/statistics/call_to_orders`, label: 'Ordnungsrufe', keywords: '' },
				{ href: `${base}/statistics/activity`, label: 'Aktivität', keywords: '' },
				{ href: `${base}/statistics/political_positions`, label: 'Politische Positionen', keywords: '' },
				{ href: `${base}/statistics/absolute_majority_initiatives`, label: '2/3 Abstimmugnen', keywords: '' },
				{ href: `${base}/statistics/votes_together`, label: 'Zusammenabstimmen der Parteien', keywords: '' }
			]
		},
		{	
			title: 'Gegenstände',
			route: "/history",
			list: [
				{ href: `${base}/history/votes`, label: 'Abstimmungen', keywords: '' },
				{ href: `${base}/history/proposals`, label: 'Entwürfe', keywords: '' }
			]
		},
	];
	
	$: listboxItemActive = (href: string) => (isSelected(href) ? 'bg-primary-active-token' : '');
	// $: activeAnchorColor = (href: string) => ($page.url.pathname?.includes(href) ? 'flex flex-col justify-center items-stretch bg-primary-active-token' : 'bg-primary-hover-token');
</script>

<div class="h-full bg-surface-50-900-token grid-cols-[auto_1fr] flex  {$$props.class ?? ''}">
	<AppRail width="w-20 2xl:w-32 border-r border-surface-500/30">
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
			selected={isSelected('/history')}
			href="{base}/history/votes"
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
			href="{base}/statistics/overview"
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
	{#if isSelected("/statistics") || isSelected("/history")}
		<section class="p-4 pb-20 space-y-4 overflow-y-auto !bg-surface-200-700-token max-w-60">
			{#each submenu as segment, i}
				{#if isSelected(segment.route)}
					<!-- Title -->
					<p class="font-bold pl-4 text-2xl">{segment.title}</p>
					<!-- Nav List -->
					<nav class="list-nav ">
						<ul>
							{#each segment.list as { href, label }}
								<li>
									<a {href} class={listboxItemActive(href)} data-sveltekit-preload-data="hover" on:keypress on:click={drawerStore.close}>
										<span class="flex-auto">{@html label}</span>
										<!-- {#if badge}<span class="badge variant-filled-secondary">{badge}</span>{/if} -->
									</a>
								</li>
							{/each}
						</ul>
					</nav>
					<!-- Divider -->
					{#if i + 1 < submenu.length}<hr class="!my-6 opacity-50" />{/if}

				{/if}
			{/each}
		</section>
	{/if}
	<!-- Mobile Only -->
	<!-- prettier-ignore -->
	<!-- <AppRailAnchor href="/" class="lg:hidden" on:click={() => { onClickAnchor() }}> -->
	<!-- <svelte:fragment slot="lead"><i class="fa-solid fa-home text-2xl"></i></svelte:fragment> -->
	<!-- <span>Home</span> -->
	<!-- </AppRailAnchor> -->
</div>
