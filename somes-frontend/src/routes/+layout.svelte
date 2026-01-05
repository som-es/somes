<script lang="ts">
	import './layout.css';
	import favicon from '$lib/assets/somes_logo.png';
	import Sidebar from '$lib/components/Bars/Sidebar.svelte';

	let { children } = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<div class="grid h-screen grid-rows-[auto_1fr_auto]">
	<header class="lg:hidden sticky top-0 z-0">
		<!-- <Navbar /> -->
	</header>
	<div class="grid grid-cols-[auto_1fr]">
    <!-- Left Sidebar. -->
		<aside class="sticky top-0 col-span-1 h-screen">
			<Sidebar  />
		</aside>
		<!-- Main Content -->
		<main>
			{@render children()}
		</main>
	</div>
  	<footer class="sticky bottom-0 z-50">
		<div class=" sm:hidden! max-h-18">
			<!-- <Bottombar /> -->
		</div>
  </footer>
</div>
<style>
</style>
