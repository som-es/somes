<script lang="ts">
	import {
		getDrawerStore,
		LightSwitch,
		modeCurrent,
		setModeCurrent,
		type DrawerSettings
	} from '@skeletonlabs/skeleton';
	import somesIcon from '$lib/assets/somes_icon.svg?raw';
	import somesIconText from '$lib/assets/somes_with_text2.svg?raw';
	import { jwtStore } from '$lib/caching/stores/stores';
	import { get } from 'svelte/store';
	import { isHasError } from '$lib/api/api';
	import { loginDrawerSettings } from '../Login/constants';
	import { gotoHistory } from '$lib/goto';
	import { renew_token } from '$lib/api/authed';
	import hamburgerIcon from '$lib/assets/misc_icons/hamburger-menu.svg?raw';
	import { onMount } from 'svelte';

	const drawerStore = getDrawerStore();

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

	// detects browsers light/dark mode at load and on change and sets the theme accordingly
	onMount(() => {
		// const mode = window.matchMedia('(prefers-color-scheme: light)');
		setModeCurrent($modeCurrent);
	});

	function drawerOpen(): void {
		const s: DrawerSettings = {
			id: 'sidebar',
			width: 'min-w-fit',
			position: 'right'
		};
		drawerStore.open(s);
	}
</script>

<!--
<AppBar class="!bg-surface-100-800-token" slotTrail="space-x-2!">
	<svelte:fragment slot="lead">
		<button on:click={drawerOpen} class="btn-icon btn-icon-sm lg:hidden!">
			{@html hamburgerIcon}
		</button>
		<a class="mx-4 fill-current stroke-current w-32" href="/">
			{@html somesIcon}
		</a>
	</svelte:fragment>
	<svelte:fragment slot="trail">
		<div class="flex gap-9">
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
				class="flex flex-col items-center sm:hidden"
			>
				{@html userIcon}
				<span class="font-bold text-sm">Benutzer</span>
			</div>
			<LightSwitch class="hidden sm:block" />
		</div>
	</svelte:fragment>
</AppBar>
-->

<div class="w-full flex justify-between items-center pr-6 pl-8 !bg-surface-100-800-token">
	<!-- Logo -->
	<div class="w-24 flex items-center py-4">
		<a class="fill-current stroke-current block sm:hidden min-w-[38px] min-h-[38px]" href="/">
			{@html somesIcon}
		</a>
		<a class="fill-current stroke-current hidden sm:block min-w-[38px] min-h-[38px]" href="/">
			{@html somesIconText}
		</a>
	</div>
	<!-- Placeholder -->
	<div class="w-6"></div>
	<!-- Hamburger Icon -->
	<div class="flex flex-row gap-3">
		<LightSwitch class="mt-1" />
		<div class="flex items-center justify-center">
			<button on:click={drawerOpen} class="fill-current btn-icon btn-icon-sm lg:hidden!">
				{@html hamburgerIcon}
			</button>
		</div>
	</div>
</div>
