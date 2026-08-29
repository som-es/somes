<script lang="ts">
	import somesWithText from '$lib/assets/somes_with_text2.svg?raw';
	import hamburgerMenuIcon from '$lib/assets/misc_icons/hamburger-menu.svg?raw';
	import rightArrowIcon from '$lib/assets/misc_icons/right-arrow-small.svg?raw';
	import crossmarkIcon from '$lib/assets/misc_icons/crossmark_small.svg?raw';
	import austriaMapIcon from '$lib/assets/misc_icons/austria-map.svg?raw';
	import euMapIcon from '$lib/assets/misc_icons/eu-map.svg?raw';
	import { page } from '$app/state';
	import { resolve } from '$app/paths';
	import { getParliament, plink } from '$lib/api/parliament';
	import { parliamentModalOpenStore } from '$lib/caching/stores/stores.svelte';
	import { slide } from 'svelte/transition';
	import { convertVoteResultFilterToUrl } from '../VoteResults/Expandable/urlConversion';
	import {
		currentDecreeFilterStore,
		currentGovProposalFilterStore,
		currentUnfinshedVoteResultFilterStore,
		currentVoteResultFilterStore
	} from '$lib/stores/stores';
	import { convertDecreeFilterToUrl } from '../Decrees/urlConversion';
	import { convertGovPropFilterToUrl } from '../Proposals/urlConversion';
	import { accountOrLogin } from './user';
	import { t } from '$lib/i18n/i18n.svelte';

	let isOpen = $state(false);
	let expandedItems = $state<Record<string, boolean>>({});
	let parliament = $derived(getParliament());

	type SubItem = { label: string; href: string; pathname: string };
	type SubItemGroup = { title: string; items: SubItem[] };

	type NavItem = {
		label: string;
		href?: string;
		subItems?: (SubItem | SubItemGroup)[];
	};

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

	const navItems: NavItem[] = $derived.by(() => {
		const voteSubItems: (SubItem | SubItemGroup)[] = [
			{
				title: t('nav.nationalCouncil'),
				items: [
					{ href: voteResultUrl.href, pathname: voteResultUrl.pathname, label: t('nav.votes') },
					{
						href: unfinishedVoteResultUrl.href,
						pathname: unfinishedVoteResultUrl.pathname,
						label: t('nav.toVote')
					}
				]
			}
		];
		if (parliament == 'at') {
			voteSubItems.push({
				title: t('nav.government'),
				items: [
					{
						href: govProposalUrl.href,
						pathname: govProposalUrl.pathname,
						label: t('nav.ministerialDrafts')
					},
					{ href: decreeUrl.href, pathname: decreeUrl.pathname, label: t('nav.decrees') }
				]
			});
		}
		return [
			{ href: plink('/home'), label: t('nav.news') },
			{ label: t('nav.votes'), subItems: voteSubItems },
			{ href: plink('/delegates'), label: t('nav.delegates') },
			{ href: plink('/statistics'), label: t('nav.statistics') },
			{ href: plink('/questions'), label: t('nav.questions') }
		];
	});

	function toggleMenu() {
		isOpen = !isOpen;
	}

	function closeMenu() {
		isOpen = false;
	}

	function toggleSubmenu(label: string) {
		expandedItems[label] = !expandedItems[label];
	}

	// Close menu when route changes
	$effect(() => {
		const _ = page.url.pathname;
		isOpen = false;
	});
</script>

<div class="relative bg-surface-500 text-white shadow-md">
	<div class="flex items-center justify-between p-4">
		<a href={resolve('/')} class="w-32 fill-white stroke-white" onclick={closeMenu}>
			{@html somesWithText}
		</a>
		<button
			onclick={toggleMenu}
			class="touch-manipulation rounded fill-white stroke-white p-2 hover:bg-surface-400"
			aria-label="Menu"
		>
			<!-- Hamburger Icon / Close Icon -->
			{#if isOpen}
				<div class="h-8 w-8 [&_line]:stroke-white [&>svg]:h-full [&>svg]:w-full">
					{@html crossmarkIcon}
				</div>
			{:else}
				<div class="h-8 w-8 [&>svg]:h-full [&>svg]:w-full [&>svg]:fill-white">
					{@html hamburgerMenuIcon}
				</div>
			{/if}
		</button>
	</div>

	{#if isOpen}
		<nav
			class="absolute top-full right-0 left-0 z-60 flex flex-col border-t border-surface-400 bg-surface-500 shadow-xl"
			transition:slide={{ duration: 200 }}
		>
			{#each navItems as item}
				{#if item.subItems}
					<button
						class="flex w-full touch-manipulation items-center justify-between p-4 text-base font-medium text-white hover:bg-surface-400"
						onclick={() => toggleSubmenu(item.label)}
					>
						<span>{item.label}</span>
						<div
							class="h-7 w-7 transition-transform duration-200 {expandedItems[item.label]
								? 'rotate-90'
								: ''} [&>svg]:h-full [&>svg]:w-full [&>svg]:fill-white [&>svg]:stroke-white"
						>
							{@html rightArrowIcon}
						</div>
					</button>
					{#if expandedItems[item.label]}
						<div transition:slide={{ duration: 200 }} class="bg-surface-600/30 py-2">
							{#each item.subItems as subItem, i}
								{#if 'title' in subItem}
									<div
										class="pl-5 text-xs font-bold tracking-widest text-surface-300 uppercase {i > 0
											? 'mt-6'
											: 'mt-2'} mb-3"
									>
										{subItem.title}
									</div>
									<div class="ml-5 border-l-2 border-surface-400">
										{#each subItem.items as nestedItem}
											<a
												href={nestedItem.href}
												class="flex w-full items-center py-2 pl-4 text-sm font-medium hover:bg-surface-500 {page.url.pathname.includes(
													nestedItem.pathname
												)
													? 'text-tertiary-500'
													: 'text-white/90'}"
												onclick={closeMenu}
											>
												{nestedItem.label}
											</a>
										{/each}
									</div>
								{:else}
									<a
										href={subItem.href}
										class="flex w-full items-center py-2 pr-4 pl-5 text-sm font-medium hover:bg-surface-500 {page.url.pathname.includes(
											subItem.pathname
										)
											? 'text-tertiary-500'
											: 'text-white/90'}"
										onclick={closeMenu}
									>
										{subItem.label}
									</a>
								{/if}
							{/each}
						</div>
					{/if}
				{:else}
					<a
						href={item.href || ''}
						class="flex w-full touch-manipulation items-center p-4 text-base font-medium hover:bg-surface-400 {page.url.pathname.includes(
							item.href || ''
						)
							? 'text-tertiary-500'
							: 'text-white'}"
						onclick={closeMenu}
					>
						{item.label}
					</a>
				{/if}
			{/each}
			<button
				class="flex w-full touch-manipulation items-center p-4 text-base font-medium hover:bg-surface-400 {page.url.pathname.includes(
					'user'
				)
					? 'text-tertiary-500'
					: 'text-white'}"
				onclick={async () => {
					await accountOrLogin();
					closeMenu();
				}}
			>
				{t('nav.profile')}
			</button>
			<button
				class="flex w-full touch-manipulation items-center justify-between p-4 text-base font-medium text-white hover:bg-surface-400"
				onclick={() => {
					closeMenu();
					parliamentModalOpenStore.value = true;
				}}
			>
				<span>{t('nav.menu.parliament')}</span>
				<div class="h-6 w-6 text-white [&_svg]:h-full [&_svg]:w-full">
					{@html parliament === 'eu' ? euMapIcon : austriaMapIcon}
				</div>
			</button>
		</nav>
	{/if}
</div>
