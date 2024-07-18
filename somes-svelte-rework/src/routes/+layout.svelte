<script lang="ts">
	import '../app.postcss';
	import { AppShell, initializeStores } from '@skeletonlabs/skeleton';
	import CacheInvalidation from '$lib/components/CacheInvalidation/CacheInvalidation.svelte';
	import Sidebar from '$lib/components/Bars/Sidebar.svelte';
	import Bottombar from '$lib/components/Bars/Bottombar.svelte';
	import Navbar from '$lib/components/Bars/Navbar.svelte';
	initializeStores();

	// const drawerStore = getDrawerStore();
</script>

<CacheInvalidation />

<AppShell>
	<svelte:fragment slot="header">
		<Navbar />
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
