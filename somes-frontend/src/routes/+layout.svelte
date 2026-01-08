<script lang="ts">
	import './layout.css';
	import favicon from '$lib/assets/somes_logo.png';
	import somesWithText from '$lib/assets/somes_with_text2.svg?raw';
	import Sidebar from '$lib/components/Bars/Sidebar.svelte';
	import { onMount } from 'svelte';

	let { children } = $props();
	

	onMount(() => {
		document.documentElement.classList.toggle(
		"dark",
		localStorage.theme === "dark" ||
			(!("theme" in localStorage) && window.matchMedia("(prefers-color-scheme: dark)").matches),
	);
	});
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
		<main class="mb-35">
			{@render children()}
		</main>
	</div>
  	<footer class="sticky bottom-0 z-50">
		<div class=" sm:hidden! max-h-18">
			<!-- <Bottombar /> -->
		</div>
  	</footer>
	<footer class="bg-primary-600 text-white p-7 flex items-center justify-between flex-wrap">
		<div class="flex gap-10">
			<span class="stroke-white fill-white">
				{@html somesWithText}
			</span>
			<!-- <img src="https://www.netidee.at/sites/default/files/2016-12/netidee-Logo-HiRes300dpi-Projekte-Standard.jpg" alt="Netidee Logo" class="h-8" /> -->
		</div>
		<div class="flex flex-wrap gap-4">
			<span>Verein</span>
			<span>Netidee</span>
			<span>Impressum</span>
			<span>Datenschutz</span>
		</div>
	</footer>
</div>
<style>
</style>
