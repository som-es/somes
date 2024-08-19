<script lang="ts">
	import '../app.postcss';
	import { AppShell, initializeStores } from '@skeletonlabs/skeleton';
	import CacheInvalidation from '$lib/components/CacheInvalidation/CacheInvalidation.svelte';
	import Sidebar from '$lib/components/Bars/Sidebar.svelte';
	import Bottombar from '$lib/components/Bars/Bottombar.svelte';
	import Navbar from '$lib/components/Bars/Navbar.svelte';

	import { computePosition, autoUpdate, offset, shift, flip, arrow } from '@floating-ui/dom';
	import { storePopup } from '@skeletonlabs/skeleton';
	storePopup.set({ computePosition, autoUpdate, offset, shift, flip, arrow });
	initializeStores();

	// const drawerStore = getDrawerStore();
</script>

<CacheInvalidation />

<!-- 
<div class="grid h-screen grid-rows-[auto_1fr_auto]">
	<header class="sticky top-0 z-[10000000]">
		<Navbar />
	</header>
	<div class="grid grid-cols-1 md:grid-cols-[auto_1fr]">
		<aside class="sticky top-[67.5px] z-50 col-span-1 h-[calc(100vh-67.5px)]">
			<Sidebar class="hidden sm:grid" />
		</aside>
		<main class="space-y-4">
			<slot></slot>
		</main>
	</div>
	<footer class="sm:!hidden">
		<Bottombar />
	</footer>
</div> -->

<AppShell>
	<svelte:fragment slot="header">
		<Navbar />
	</svelte:fragment>
	<svelte:fragment slot="sidebarLeft">
		<Sidebar class="hidden sm:grid" />
	</svelte:fragment>
	<svelte:fragment slot="footer">
		<div class="sm:!hidden">
			<Bottombar />
		</div>
	</svelte:fragment>
	<slot />
</AppShell>
