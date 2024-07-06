<script lang="ts">
	import '../app.postcss';
	import { AppShell, AppBar, LightSwitch, initializeStores, type DrawerSettings } from '@skeletonlabs/skeleton';
	import userIcon from "$lib/assets/icons/user.svg?raw";
	import homeIcon from "$lib/assets/icons/home.svg?raw";
	import somesTextIcon from "$lib/assets/somes_with_text.svg?raw";
	import CacheInvalidation from '$lib/components/CacheInvalidation/CacheInvalidation.svelte';
	import Sidebar from '$lib/components/Bars/Sidebar.svelte';
	import Bottombar from '$lib/components/Bars/Bottombar.svelte';
	initializeStores();

	// const drawerStore = getDrawerStore();

</script>

<CacheInvalidation />

<AppShell>
	<svelte:fragment slot="header">
		<AppBar slotTrail="!space-x-2">
			<svelte:fragment slot="lead">
				<a class="mx-4 fill-current stroke-current w-32" href="/">
					{@html somesTextIcon}
				</a>
			</svelte:fragment>
			<svelte:fragment slot="trail">
				<LightSwitch />
			</svelte:fragment>
		</AppBar>
	</svelte:fragment>
	<svelte:fragment slot="sidebarLeft">
		<Sidebar class="hidden sm:grid " />
	</svelte:fragment>
	<svelte:fragment slot="footer">
		<div class="sm:!hidden">
			<Bottombar />	
		</div>
	</svelte:fragment>
	<slot />
</AppShell>
