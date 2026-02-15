<script lang="ts">
	import './layout.css';
	import favicon from '$lib/assets/somes_logo.png';
	import somesWithText from '$lib/assets/somes_with_text2.svg?raw';
	import Sidebar from '$lib/components/Bars/Sidebar.svelte';
	import Navbar from '$lib/components/Bars/Navbar.svelte';
	import { onMount } from 'svelte';
	import { resolve } from '$app/paths';
	import { lightModeStore } from '$lib/lightmode.svelte';
	import RenewToken from '$lib/components/Login/RenewToken.svelte';
	import LoginDrawer from '$lib/components/Login/LoginDrawer.svelte';
	import { loginDrawerOpenStore } from '$lib/caching/stores/stores.svelte';

	let { children } = $props();

	onMount(() => {
		document.documentElement.classList.toggle(
			'dark',
			lightModeStore.value === 'dark' ||
				(!('theme' in localStorage) && window.matchMedia('(prefers-color-scheme: dark)').matches)
		);

		if (!localStorage.getItem('cleared_feb_15')) {
			localStorage.clear();
			localStorage.setItem('cleared_feb_15', 'true');
			window.location.reload(); 
		}
	});
</script>

<RenewToken />
<LoginDrawer bind:open={loginDrawerOpenStore.value} />

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<div class="grid h-screen grid-rows-[auto_1fr_auto]">
	<header class="sticky top-0 z-50 lg:hidden">
		<Navbar />
	</header>

	<div class="grid grid-cols-1 lg:grid-cols-[auto_1fr]">
		<!-- Left Sidebar. -->
		<aside class="sticky top-0 col-span-1 hidden h-screen lg:block">
			<Sidebar />
		</aside>
		<!-- Main Content -->
		<main class="mb-35 w-full min-w-0">
			{@render children()}
		</main>
	</div>
	<footer class="sticky bottom-0 z-50">
		<div class=" max-h-18 sm:hidden!">
			<!-- <Bottombar /> -->
		</div>
	</footer>
	<footer class="border-t border-surface-800 bg-surface-900 py-12 text-surface-400">
		<div class="mx-auto grid max-w-7xl grid-cols-1 px-4 md:grid-cols-5">
			<div class="col-span-1 md:col-span-2">
				<!-- <span class="text-2xl font-bold text-white block mb-4">somes.at</span> -->
				<div class="w-72 fill-white stroke-white">
					{@html somesWithText}
				</div>
				<p class="mt-2 max-w-xs text-sm">
					Parteiübergreifend machen wir Demokratie transparent, verständlich und zugänglich.
				</p>
				<p class="mt-2 max-w-xs text-sm font-thin">
					Das Entwicklungsteam wird seit 03.11.2025 von <a
						href="https://www.netidee.at"
						target="_blank"
						rel="noopener noreferrer"
						class="hover:text-secondary-400">Netidee</a
					> gefördert.
				</p>
			</div>
			<div class="mb-4">
				<h4 class="mb-4 font-bold text-white">Entwicklung</h4>
				<ul class="space-y-2 text-sm">
					<li>
						<a
							href="https://github.com/som-es"
							class="hover:text-secondary-400"
							target="_blank"
							rel="noopener noreferrer">Github</a
						>
					</li>
					<li>
						<a
							href="https://www.netidee.at/somes"
							class="hover:text-secondary-400"
							target="_blank"
							rel="noopener noreferrer">Netidee</a
						>
					</li>
				</ul>
			</div>
			<div class="mb-4">
				<h4 class="mb-4 font-bold text-white">Links</h4>
				<ul class="space-y-2 text-sm">
					<li><a href={resolve('/')} class="hover:text-secondary-400">Verein</a></li>
					<li><a href="{resolve('/')}#events" class="hover:text-secondary-400">Events</a></li>
					<li><a href={resolve('/impressum')} class="hover:text-secondary-400">Impressum</a></li>
					<li>
						<a href={resolve('/datenschutz')} class="hover:text-secondary-400">Datenschutz</a>
					</li>
				</ul>
			</div>
			<div>
				<h4 class="mb-4 font-bold text-white">Socials</h4>
				<div class="flex gap-4">
					<a
						href="https://www.instagram.com/somes.at"
						title="Somes Instagram"
						target="_blank"
						rel="noopener noreferrer"
						class="flex h-10 w-10 items-center justify-center rounded-full bg-surface-800 transition-colors hover:bg-primary-600 hover:text-white"
					>
						<svg
							xmlns="http://www.w3.org/2000/svg"
							viewBox="0 0 24 24"
							width="24"
							height="24"
							fill="#FFFFFF"
							style="opacity:1;"
							><path
								d="M12.001 9a3 3 0 1 0 0 6a3 3 0 0 0 0-6m0-2a5 5 0 1 1 0 10a5 5 0 0 1 0-10m6.5-.25a1.25 1.25 0 0 1-2.5 0a1.25 1.25 0 0 1 2.5 0M12.001 4c-2.474 0-2.878.007-4.029.058c-.784.037-1.31.142-1.798.332a2.9 2.9 0 0 0-1.08.703a2.9 2.9 0 0 0-.704 1.08c-.19.49-.295 1.015-.331 1.798C4.007 9.075 4 9.461 4 12c0 2.475.007 2.878.058 4.029c.037.783.142 1.31.331 1.797c.17.435.37.748.702 1.08c.337.336.65.537 1.08.703c.494.191 1.02.297 1.8.333C9.075 19.994 9.461 20 12 20c2.475 0 2.878-.007 4.029-.058c.782-.037 1.308-.142 1.797-.331a2.9 2.9 0 0 0 1.08-.703c.337-.336.538-.649.704-1.08c.19-.492.296-1.018.332-1.8c.052-1.103.058-1.49.058-4.028c0-2.474-.007-2.878-.058-4.029c-.037-.782-.143-1.31-.332-1.798a2.9 2.9 0 0 0-.703-1.08a2.9 2.9 0 0 0-1.08-.704c-.49-.19-1.016-.295-1.798-.331C14.926 4.006 14.54 4 12 4m0-2c2.717 0 3.056.01 4.123.06c1.064.05 1.79.217 2.427.465c.66.254 1.216.598 1.772 1.153a4.9 4.9 0 0 1 1.153 1.772c.247.637.415 1.363.465 2.428c.047 1.066.06 1.405.06 4.122s-.01 3.056-.06 4.122s-.218 1.79-.465 2.428a4.9 4.9 0 0 1-1.153 1.772a4.9 4.9 0 0 1-1.772 1.153c-.637.247-1.363.415-2.427.465c-1.067.047-1.406.06-4.123.06s-3.056-.01-4.123-.06c-1.064-.05-1.789-.218-2.427-.465a4.9 4.9 0 0 1-1.772-1.153a4.9 4.9 0 0 1-1.153-1.772c-.248-.637-.415-1.363-.465-2.428C2.012 15.056 2 14.717 2 12s.01-3.056.06-4.122s.217-1.79.465-2.428a4.9 4.9 0 0 1 1.153-1.772A4.9 4.9 0 0 1 5.45 2.525c.637-.248 1.362-.415 2.427-.465C8.945 2.013 9.284 2 12.001 2"
							/></svg
						>
					</a>
					<a
						href="https://www.linkedin.com/company/somes-at"
						target="_blank"
						title="Somes Linked In"
						class="flex h-10 w-10 items-center justify-center rounded-full bg-surface-800 transition-colors hover:bg-primary-600 hover:text-white"
					>
						<svg
							xmlns="http://www.w3.org/2000/svg"
							viewBox="0 0 24 24"
							width="24"
							height="24"
							fill="#FFFFFF"
							style="opacity:1;"
							><path
								d="M6.94 5a2 2 0 1 1-4-.002a2 2 0 0 1 4 .002M7 8.48H3V21h4zm6.32 0H9.34V21h3.94v-6.57c0-3.66 4.77-4 4.77 0V21H22v-7.93c0-6.17-7.06-5.94-8.72-2.91z"
							>
							</path></svg
						>
					</a>
				</div>
			</div>
		</div>
		<div class="mx-auto mt-1 max-w-7xl border-t border-surface-800 px-4 pt-8 text-center text-xs">
			&copy; {new Date().getFullYear()} somes - Verein für Demokratie und politische Transparenz.
		</div>
	</footer>
	<!-- <img src="https://www.netidee.at/sites/default/files/2016-12/netidee-Logo-HiRes300dpi-Projekte-Standard.jpg" alt="Netidee Logo" class="h-8" /> -->
</div>

<style>
</style>
