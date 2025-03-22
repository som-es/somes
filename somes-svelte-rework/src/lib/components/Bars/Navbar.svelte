<script lang="ts">
	import {
		AppBar,
		getDrawerStore,
		LightSwitch,
		modeCurrent,
		setModeCurrent,
		type DrawerSettings
	} from '@skeletonlabs/skeleton';
	import userIcon from '$lib/assets/icons/user.svg?raw';
	import somesTextIcon from '$lib/assets/somes_with_text2.svg?raw';
	import { jwtStore } from '$lib/caching/stores/stores';
	import { get } from 'svelte/store';
	import { isHasError } from '$lib/api/api';
	import { loginDrawerSettings } from '../Login/constants';
	import { gotoHistory } from '$lib/goto';
	import { renew_token } from '$lib/api/authed';
	import hamburgerIcon from '$lib/assets/misc_icons/hamburger-menu.svg?raw';

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

	// fixes weird color bug
	setModeCurrent($modeCurrent);

	function drawerOpen(): void {
		const s: DrawerSettings = {
			id: 'sidebar',
			width: 'min-w-fit'
		};
		drawerStore.open(s);
	}
</script>

<AppBar class="!bg-surface-100-800-token" slotTrail="!space-x-2">
	<svelte:fragment slot="lead">
		<button on:click={drawerOpen} class="btn-icon btn-icon-sm lg:!hidden">
			{@html hamburgerIcon}
		</button>
		<a class="mx-4 fill-current stroke-current w-32" href="/">
			{@html somesTextIcon}
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
			<LightSwitch />
		</div>
	</svelte:fragment>
</AppBar>
