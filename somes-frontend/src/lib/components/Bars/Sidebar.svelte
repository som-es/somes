<script lang="ts">
	import homeIcon from '$lib/assets/icons/home.svg?raw';
	import delegatesIcon from '$lib/assets/icons/delegates.svg?raw';
	import statisticsIcon from '$lib/assets/icons/statistics.svg?raw';
	import questionsIcon from '$lib/assets/icons/questions.svg?raw';
	import somesIcon from '$lib/assets/somes_icon.svg?raw';
	import somesEuIcon from '$lib/assets/somes_icon_eu.svg?raw';
	import { page } from '$app/state';

	import { resolve } from '$app/paths';
	import { getParliament, plink } from '$lib/api/parliament';
	import VoteParliament2 from '../Parliaments/VoteParliament2.svelte';
	import { mockDelegatesNoColor, mockVoteResult } from '$lib/parliaments/mock';
	import { getSeats } from '$lib/caching/seats';
	import AiViewToggle from '../UI/AiViewToggle.svelte';
	import SidebarUserMenu from './SidebarUserMenu.svelte';
	import austriaFlagIcon from '$lib/assets/parliament_switch/austria_map_flag.svg?raw';
	import euFlagIcon from '$lib/assets/parliament_switch/EU_map_flag.svg?raw';
	import { parliamentModalOpenStore } from '$lib/caching/stores/stores.svelte';
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
	import { t } from '$lib/i18n/i18n.svelte';

	let activeUrl = $derived(page.url.pathname);
	let activeSectionHash = $state('');
	let activeHash = $derived(activeSectionHash || page.url.hash);
	let statisticsObserver: IntersectionObserver | null = null;
	let parliament = $derived(getParliament());

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
				title: t('nav.speeches'),
				route: '/statistics',
				list: [
					{ href: `${plink('/statistics')}#speech-time`, label: t('nav.speechTime'), keywords: '' },
					{
						href: `${plink('/statistics')}#total-speeches`,
						label: t('nav.totalSpeeches'),
						keywords: ''
					}
				]
			},
			{
				title: t('nav.activities'),
				route: '/statistics',
				list: [
					{ href: `${plink('/statistics')}#absences`, label: t('nav.absences'), keywords: '' },
					{ href: `${plink('/statistics')}#activity`, label: t('nav.activity'), keywords: '' },
					{
						href: `${plink('/statistics')}#call-to-orders`,
						label: t('nav.callToOrders'),
						keywords: ''
					}
				]
			},
			{
				title: t('nav.delegates'),
				route: '/statistics',
				list: [
					{ href: `${plink('/statistics')}#age`, label: t('nav.age'), keywords: '' },
					{
						href: `${plink('/statistics')}#orientation`,
						label: t('nav.orientation'),
						keywords: ''
					}
				]
			},

			{
				title: parliament === 'eu' ? t('nav.euParliament') : t('nav.nationalCouncil'),
				route: '/history',
				list: [
					{ href: voteResultUrl.href, label: t('nav.votes'), keywords: '' },
					{ href: unfinishedVoteResultUrl.href, label: t('nav.toVote'), keywords: '' }
				]
			}
		];
		if (parliament == 'at') {
			menus.push({
				title: t('nav.government'),
				route: '/history',
				list: [
					{ href: govProposalUrl.href, label: t('nav.ministerialDrafts'), keywords: '' },
					{ href: decreeUrl.href, label: t('nav.decrees'), keywords: '' }
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
				{@html parliament === 'eu' ? somesEuIcon : somesIcon}
			</span>
		</a>
		<a
			href={plink('/home')}
			title={t('nav.news')}
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
			title={t('nav.history')}
			class="{activeUrl?.includes('/history')
				? 'bg-tertiary-500! stroke-black'
				: ' stroke-white'} flex h-10 w-10 items-center justify-center rounded-xl hover:cursor-pointer hover:bg-tertiary-400/60"
		>
			<span class="w-15">
				<VoteParliament2
					parliament="at"
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
			title={t('nav.delegates')}
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
			title={t('nav.statistics')}
			class="{activeUrl?.includes('/statistics')
				? 'bg-tertiary-500! fill-black'
				: ' fill-white'} flex h-10 w-10 items-center justify-center rounded-xl hover:cursor-pointer hover:bg-tertiary-400/60 hover:fill-black"
		>
			<span class="w-6">
				{@html statisticsIcon}
			</span>
		</a>
		<a
			href={plink('/questions')}
			title={t('nav.questions')}
			class="{activeUrl?.includes('/questions')
				? 'bg-tertiary-500! fill-black'
				: ' fill-white'} flex h-10 w-10 items-center justify-center rounded-xl hover:cursor-pointer hover:bg-tertiary-400/60 hover:fill-black"
		>
			<span class="w-5">
				{@html questionsIcon}
			</span>
		</a>

		<div class="mt-auto mb-4 flex flex-col gap-3">
			<button
				onclick={() => (parliamentModalOpenStore.value = true)}
				title={t('nav.menu.parliament')}
				class="flex h-10 w-10 items-center justify-center rounded-xl text-white hover:cursor-pointer hover:bg-tertiary-400/60 hover:text-black"
			>
				<span class="h-6 w-6 [&_svg]:h-full [&_svg]:w-full">
					{@html parliament === 'eu' ? euFlagIcon : austriaFlagIcon}
				</span>
			</button>
			<AiViewToggle />
			<SidebarUserMenu />
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
