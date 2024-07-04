<script lang="ts">
	import { page } from '$app/stores';
	import userIcon from "$lib/assets/icons/user.svg?raw";
	import homeIcon from "$lib/assets/icons/home.svg?raw";
	import statsIcon from "$lib/assets/icons/statistics.svg?raw";
	import { AppRail, AppRailAnchor, AppRailTile, getDrawerStore } from '@skeletonlabs/skeleton';
	
	let currentTile: number = 0;
	$: activeAnchorColor = (href: string) => ($page.url.pathname?.includes(href) ? 'flex flex-col justify-center items-stretch bg-primary-active-token' : 'bg-primary-hover-token');

</script>

<div class="grid grid-cols-[auto_1fr] h-full bg-surface-50-900-token {$$props.class ?? ''}">
	<AppRail>
		<!-- <svelte:fragment slot="lead">
			<AppRailAnchor href="/" >(icon)</AppRailAnchor>
		</svelte:fragment> -->
		<!-- --- -->
		<AppRailAnchor class={activeAnchorColor("/home")} href="/home" bind:group={currentTile} name="tile-1" value={0} title="tile-1">
			<svelte:fragment slot="lead"><div class="fill-current stroke-current">{@html homeIcon}</div></svelte:fragment>
			<span style="font-size: smaller;">Neuigkeiten</span>
		</AppRailAnchor>
		<hr>
		<AppRailAnchor class={activeAnchorColor("/delegates")} href="/delegates" bind:group={currentTile} name="tile-2" value={1} title="tile-2">
			<svelte:fragment slot="lead">{@html userIcon}</svelte:fragment>
			<span style="font-size: smaller;">Abgeordnete</span>
		</AppRailAnchor>
		<hr>
		<AppRailAnchor class={activeAnchorColor("/statistics")} href="/statistics" bind:group={currentTile} name="tile-3" value={2} title="tile-3">
			<svelte:fragment slot="lead">{@html statsIcon}</svelte:fragment>
			<span style="font-size: smaller;">Statistiken</span>
		</AppRailAnchor>
		<hr>
		<!-- --- -->
		<svelte:fragment slot="trail">
			<AppRailAnchor href="/" target="_blank" title="Account">(icon)</AppRailAnchor>
		</svelte:fragment>
	</AppRail>
		<!-- Mobile Only -->
		<!-- prettier-ignore -->
		<!-- <AppRailAnchor href="/" class="lg:hidden" on:click={() => { onClickAnchor() }}> -->
			<!-- <svelte:fragment slot="lead"><i class="fa-solid fa-home text-2xl"></i></svelte:fragment> -->
			<!-- <span>Home</span> -->
		<!-- </AppRailAnchor> -->
</div>