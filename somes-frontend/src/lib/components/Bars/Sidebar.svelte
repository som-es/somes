<script lang="ts">
	import homeIcon from '$lib/assets/icons/home.svg?raw';
	import delegatesIcon from '$lib/assets/icons/delegates.svg?raw';
	import statisticsIcon from '$lib/assets/icons/statistics.svg?raw';
	import somesIcon from '$lib/assets/somes_icon.svg?raw';
	import userIcon from '$lib/assets/icons/user.svg?raw';
	import { page } from '$app/state';

	import { resolve } from '$app/paths';
	import { getParliament, plink } from '$lib/api/parliament';
	import VoteParliament2 from '../Parliaments/VoteParliament2.svelte';
	import { mockDelegatesNoColor, mockVoteResult } from '$lib/parliaments/mock';
	import { getSeats } from '$lib/caching/seats';
	import LightSwitch from '../UI/LightSwitch.svelte';
	import AiViewToggle from '../UI/AiViewToggle.svelte';
	import { jwtStore, loginDrawerOpenStore } from '$lib/caching/stores/stores.svelte';
	import { isHasError } from '$lib/api/api';
	import { renew_token } from '$lib/api/authed';
	import { goto } from '$app/navigation';
	import { convertVoteResultFilterToUrl } from '../VoteResults/Expandable/urlConversion';
	import {
		currentDecreeFilterStore,
		currentDelegateFilterStore,
		currentGovProposalFilterStore,
		currentUnfinshedVoteResultFilterStore,
		currentVoteResultFilterStore
	} from '$lib/stores/stores';
	import { convertGovPropFilterToUrl } from '../Proposals/urlConversion';
	import { convertDecreeFilterToUrl } from '../Decrees/urlConversion';
	import { accountOrLogin } from './user';

	let activeUrl = $derived(page.url.pathname);
	let activeSectionHash = $state('');
	let activeHash = $derived(activeSectionHash || page.url.hash);
	let statisticsObserver: IntersectionObserver | null = null;
	let parliament = $state(getParliament());

	function hrefPath(href: string) {
		return new URL(href, page.url.origin).pathname;
	}

	function hrefHash(href: string) {
		return new URL(href, page.url.origin).hash;
	}

	function isActiveHref(href: string) {
		const hash = hrefHash(href);
		return (
			hrefPath(href) === activeUrl &&
			(!hash || hash === activeHash || (!activeHash && hash === firstStatisticsHash()))
		);
	}

	function onSubmenuClick(href: string) {
		const hash = hrefHash(href);
		if (hrefPath(href).endsWith('/statistics') && hash) {
			activeSectionHash = hash;
		}
	}

	function statisticsSubmenuHashes() {
		return submenu
			.filter((segment) => segment.route === '/statistics')
			.flatMap((segment) => segment.list.map((item) => hrefHash(item.href)))
			.filter((hash) => hash.length > 0);
	}

	function firstStatisticsHash() {
		return statisticsSubmenuHashes()[0] ?? '#speech-time';
	}

	const voteResultUrl = $derived(
		convertVoteResultFilterToUrl(currentVoteResultFilterStore.value, '', undefined, true)
	);
	const unfinishedVoteResultUrl = $derived(
		convertVoteResultFilterToUrl(currentUnfinshedVoteResultFilterStore.value, '', undefined, false)
	);
	const govProposalUrl = $derived(
		convertGovPropFilterToUrl(currentGovProposalFilterStore.value, '', undefined)
	);
	const decreeUrl = $derived(
		convertDecreeFilterToUrl(currentDecreeFilterStore.value, '', undefined)
	);

	const submenu = $derived.by(() => {
		const menus = [
			{
				title: 'Reden',
				route: '/statistics',
				list: [
					{ href: `${plink('/statistics')}#speech-time`, label: 'Redezeit', keywords: '' },
					{ href: `${plink('/statistics')}#total-speeches`, label: 'Gehaltene Reden', keywords: '' }
				]
			},
			{
				title: 'Aktivitäten',
				route: '/statistics',
				list: [
					{ href: `${plink('/statistics')}#absences`, label: 'Abwesenheiten', keywords: '' },
					{ href: `${plink('/statistics')}#activity`, label: 'Aktivität', keywords: '' },
					{ href: `${plink('/statistics')}#call-to-orders`, label: 'Ordnungsrufe', keywords: '' }
				]
			},
			{
				title: 'Abgeordnete',
				route: '/statistics',
				list: [
					{ href: `${plink('/statistics')}#age`, label: 'Alter', keywords: '' },
					{
						href: `${plink('/statistics')}#orientation`,
						label: 'Politische Positionen',
						keywords: ''
					}
				]
			},

			{
				title: 'Nationalrat',
				route: '/history',
				list: [
					{ href: voteResultUrl.href, label: 'Abstimmungen', keywords: '' },
					{ href: unfinishedVoteResultUrl.href, label: 'Zur Abstimmung', keywords: '' }
				]
			}
		];
		if (parliament == 'at') {
			menus.push({
				title: 'Regierung',
				route: '/history',
				list: [
					{ href: govProposalUrl.href, label: 'Ministerialentwürfe', keywords: '' },
					{ href: decreeUrl.href, label: 'Verordnungen', keywords: '' }
				]
			});
		}
		return menus;
	});

	function syncStatisticsObserver() {
		statisticsObserver?.disconnect();
		const observedElements = statisticsSubmenuHashes()
			.map((hash) => document.getElementById(hash.slice(1)))
			.filter((element): element is HTMLElement => element !== null);

		if (observedElements.length === 0) return;

		statisticsObserver = new IntersectionObserver(
			(entries) => {
				const visibleEntries = entries
					.filter((entry) => entry.isIntersecting)
					.sort((a, b) => a.boundingClientRect.top - b.boundingClientRect.top);
				const activeEntry = visibleEntries[0];

				if (activeEntry?.target.id) {
					activeSectionHash = `#${activeEntry.target.id}`;
				}
			},
			{
				root: null,
				rootMargin: '-20% 0px -55% 0px',
				threshold: [0, 0.2, 0.6]
			}
		);

		for (const element of observedElements) {
			statisticsObserver.observe(element);
		}

		if (!activeSectionHash) {
			activeSectionHash = page.url.hash || firstStatisticsHash();
		}
	}

	$effect(() => {
		if (!activeUrl.endsWith('/statistics') || typeof IntersectionObserver === 'undefined') {
			statisticsObserver?.disconnect();
			activeSectionHash = '';
			return;
		}

		const timeout = window.setTimeout(syncStatisticsObserver);

		return () => {
			window.clearTimeout(timeout);
			statisticsObserver?.disconnect();
		};
	});
</script>

<div class="flex h-full grid-cols-[auto_1fr] bg-surface-50 lg:grid">
	<div class="flex h-screen w-21 flex-col items-center justify-center gap-4 bg-surface-500">
		<a
			href={resolve('/')}
			title="Somes"
			class="mt-4 flex items-center justify-center rounded-xl fill-white stroke-white hover:cursor-pointer"
		>
			<span class="mt-3 flex w-10 items-center justify-center fill-white stroke-white!">
				{@html somesIcon}
			</span>
		</a>
		<a
			href={plink('/home')}
			title="Neuigkeiten"
			class="{activeUrl?.includes('/home')
				? 'bg-tertiary-500! stroke-black'
				: ' stroke-white'} mt-5 flex h-10 w-10 items-center justify-center rounded-xl hover:cursor-pointer hover:bg-tertiary-400/60 hover:stroke-black"
		>
			<span class="w-8">
				{@html homeIcon}
			</span>
		</a>
		<a
			href={voteResultUrl.href}
			title="Abstimmungshistorie"
			class="{activeUrl?.includes('/history')
				? 'bg-tertiary-500! stroke-black'
				: ' stroke-white'} flex h-10 w-10 items-center justify-center rounded-xl hover:cursor-pointer hover:bg-tertiary-400/60"
		>
			<span class="w-15">
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
					forceColor={activeUrl?.includes('/history') ? 'black' : 'white'}
				/>
			</span>
		</a>
		<a
			href={plink('/delegates')}
			title="Abgeordnete"
			class="{activeUrl?.includes('/delegates')
				? 'bg-tertiary-500! fill-black'
				: ' fill-white'} flex h-10 w-10 items-center justify-center rounded-xl hover:cursor-pointer hover:bg-tertiary-400/60 hover:fill-black"
		>
			<span class="w-6">
				{@html delegatesIcon}
			</span>
		</a>
		<a
			href={plink('/statistics')}
			title="Statistiken"
			class="{activeUrl?.includes('/statistics')
				? 'bg-tertiary-500! fill-black'
				: ' fill-white'} flex h-10 w-10 items-center justify-center rounded-xl hover:cursor-pointer hover:bg-tertiary-400/60 hover:fill-black"
		>
			<span class="w-6">
				{@html statisticsIcon}
			</span>
		</a>

		<div class="mt-auto mb-4 flex flex-col gap-3">
			<LightSwitch />
			<AiViewToggle />
			<!-- <DarkMode class="text-primary-500 dark:text-primary-600 border dark:border-gray-800 hover:bg-primary-800" /> -->
			<button
				onclick={async () => {
					await accountOrLogin();
				}}
				title="Benutzerprofil"
				class="{activeUrl?.includes('/user')
					? 'bg-tertiary-500! fill-black'
					: ' fill-white'} flex h-10 w-10 items-center justify-center rounded-xl hover:cursor-pointer hover:bg-tertiary-400/60 hover:fill-black"
			>
				<span class="h-5 w-5">
					{@html userIcon}
				</span>
			</button>
		</div>
	</div>

	{#if activeUrl?.includes('/statistics') || activeUrl?.includes('/history')}
		<section class="max-w-60 space-y-4 overflow-y-auto bg-gray-300 p-3 pb-15 dark:bg-surface-600">
			{#each submenu as segment, i}
				{#if activeUrl?.includes(segment.route)}
					<!-- Title -->
					<p class="mt-3 mb-0 pb-0 pl-4 text-2xl font-bold">{segment.title}</p>
					<!-- Nav List -->
					<nav class="list-nav">
						<ul class="mb-2">
							{#each segment.list as { href, label }}
								<li class="px-2 py-1">
									<a
										{href}
										onclick={() => onSubmenuClick(href)}
										class="flex w-fit rounded-3xl p-2 px-4 {isActiveHref(href)
											? 'bg-primary-600'
											: 'hover:bg-primary-300'}"
										data-sveltekit-preload-data="hover"
									>
										<span class="flex-auto">{@html label}</span>
										<!-- {#if badge}<span class="badge variant-filled-secondary">{badge}</span>{/if} -->
									</a>
								</li>
							{/each}
						</ul>
					</nav>
					<!-- Divider -->
					{#if i + 1 < submenu.length}<hr class="my-6! opacity-50" />{/if}
				{/if}
			{/each}
		</section>
	{/if}
</div>
