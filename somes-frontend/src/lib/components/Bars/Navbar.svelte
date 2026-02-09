<script lang="ts">
	import somesWithText from '$lib/assets/somes_with_text2.svg?raw';
	import hamburgerMenuIcon from '$lib/assets/misc_icons/hamburger-menu.svg?raw';
	import rightArrowIcon from '$lib/assets/misc_icons/right-arrow-small.svg?raw';
	import crossmarkIcon from '$lib/assets/misc_icons/crossmark_small.svg?raw';
	import { page } from '$app/state';
	import { resolve } from '$app/paths';
	import { slide } from 'svelte/transition';

	let isOpen = $state(false);
	let expandedItems = $state<Record<string, boolean>>({});

	type SubItem = { label: string; href: string };
	type SubItemGroup = { title: string; items: SubItem[] };

	type NavItem = {
		label: string;
		href?: string;
		subItems?: (SubItem | SubItemGroup)[];
	};

	const navItems: NavItem[] = [
		{ href: '/home', label: 'Neuigkeiten' },
		{
			label: 'Abstimmungen',
			subItems: [
				{
					title: 'Nationalrat',
					items: [
						{ href: '/history/votes', label: 'Abstimmungen' },
						{ href: '/history/unfinished_votes', label: 'Zur Abstimmung' }
					]
				},
				{
					title: 'Regierung',
					items: [
						{ href: '/history/proposals', label: 'Ministerialentwürfe' },
						{ href: '/history/decrees', label: 'Verordnungen' }
					]
				}
			]
		},
		{ href: '/delegates', label: 'Abgeordnete' },
		{ href: '/statistics', label: 'Statistiken' },
		{ href: '/user', label: 'Benutzerprofil' }
	];

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
			class="rounded fill-white stroke-white p-2 hover:bg-surface-400 touch-manipulation"
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
						class="flex w-full items-center justify-between p-4 touch-manipulation text-base font-medium text-white hover:bg-surface-400"
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
										class="pl-5 text-xs font-bold tracking-widest text-surface-300 uppercase {i > 0 ? 'mt-6' : 'mt-2'} mb-3"
									>
										{subItem.title}
									</div>
									<div class="ml-5 border-l-2 border-surface-400">
									{#each subItem.items as nestedItem}
										<a
											href={resolve(nestedItem.href as any)}
											class="flex w-full items-center py-2 pl-4 text-sm font-medium hover:bg-surface-500 {page.url.pathname.includes(
												nestedItem.href
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
										href={resolve(subItem.href as any)}
										class="flex w-full items-center py-2 pr-4 pl-5 text-sm font-medium hover:bg-surface-500 {page.url.pathname.includes(
											subItem.href
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
						href={resolve((item.href || '') as any)}
						class="flex w-full items-center p-4 touch-manipulation text-base font-medium hover:bg-surface-400 {page.url.pathname.includes(
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
		</nav>
	{/if}
</div>
