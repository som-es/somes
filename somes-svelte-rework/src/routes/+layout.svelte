<script lang="ts">
	import '../app.postcss';
	import { AppShell, AppBar, LightSwitch, initializeStores, type DrawerSettings, getDrawerStore } from '@skeletonlabs/skeleton';
	import userIcon from "$lib/assets/icons/user.svg?raw";
	import homeIcon from "$lib/assets/icons/home.svg?raw";
	import somesTextIcon from "$lib/assets/somes_with_text.svg?raw";
	import Sidebar from '$lib/components/Sidebar.svelte';
	import CacheInvalidation from '$lib/components/CacheInvalidation/CacheInvalidation.svelte';
	initializeStores();

	// const drawerStore = getDrawerStore();

</script>

<CacheInvalidation />
<!-- App Shell -->
<AppShell>
	<svelte:fragment slot="header">
		<!-- App Bar -->
		<AppBar slotTrail="!space-x-2">
			<svelte:fragment slot="lead">
				<a class="mx-4 fill-current stroke-current w-32" href="/">
					{@html somesTextIcon}
				</a>
				<!-- <strong class="text-xl uppercase">somes</strong> -->
			</svelte:fragment>
			<svelte:fragment slot="trail">
				<LightSwitch />
			</svelte:fragment>
		</AppBar>
	</svelte:fragment>
	<svelte:fragment slot="sidebarLeft">
		<Sidebar class="hidden sm:grid overflow-hidden" />
	</svelte:fragment>
	<svelte:fragment slot="footer">
		<AppBar class="sm:!hidden" gridColumns="grid-cols-3" slotDefault="place-self-center" slotTrail="place-content-end">
			<svelte:fragment slot="lead">Icon</svelte:fragment>
			Icon	
			<svelte:fragment slot="trail">Icon</svelte:fragment>
		</AppBar>	
		<!-- <Sidebar class="hidden sm:grid w-[360px] overflow-hidden" /> -->
	</svelte:fragment>
	<!-- Page Route Content -->
	<slot />
</AppShell>
