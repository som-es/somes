<script lang="ts">
	import '../app.postcss';
	import { AppShell, AppBar, initializeStores, type DrawerSettings, getDrawerStore } from '@skeletonlabs/skeleton';
	import Sidebar from '$lib/components/Sidebar.svelte';
	initializeStores();

	const drawerStore = getDrawerStore();

	function drawerOpen(): void {
		const s: DrawerSettings = { id: 'sidenav' };
		drawerStore.open(s);
	}
</script>

<!-- App Shell -->
<AppShell>
	<svelte:fragment slot="header">
		<!-- App Bar -->
		<AppBar shadow="shadow-2xl" slotTrail="!space-x-2">
			<svelte:fragment slot="lead">
				<strong class="text-xl uppercase">somes</strong>
			</svelte:fragment>
			<svelte:fragment slot="trail">
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
