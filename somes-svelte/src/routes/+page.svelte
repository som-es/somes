<script lang="ts">
	import { goto } from "$app/navigation";
	import { t } from "$lib/translations";
	import { loginDrawerSettings } from "$lib/constants";
	import { localStorageStore, type DrawerSettings, drawerStore } from "@skeletonlabs/skeleton";
	import { get, type Readable, type Writable } from "svelte/store";
	import somesWhite from "$lib/assets/somes_white.svg?raw";
	import somesAudio from "$lib/assets/somes.mp3";
	import { onMount } from "svelte";

	const noAccountStorage: Readable<boolean | null> = localStorageStore("noAccount", null);
	const isNoAccount = get(noAccountStorage);
	let somesSound: HTMLAudioElement;

	if (isNoAccount) {
		// goto("/home");
	}

	function redirectToHome() {
		console.log("redirecting to home");
		const noAccountStorage: Writable<boolean | null> = localStorageStore("noAccount", null);
		noAccountStorage.set(true);
		// localStorageStore('noAccount', true);
		goto("/home");
	}

	onMount(() => {
		somesSound = new Audio(somesAudio);

		const bg: Element = document.querySelector(".background");
		const main: Element = document.querySelector(".main");
		const windowWidth = window.innerWidth / 5;
		const windowHeight = window.innerHeight / 5 ;

		main.addEventListener('mousemove', (e): void => {
			const mouseX = e.clientX / windowWidth;
			const mouseY = e.clientY / windowHeight;
			
			bg.style.transform = `translate3d(-${mouseX}%, -${mouseY}%, 0)`;
		});
	})
</script>

<div class="h-full w-full background bg-surface-400-500-token bg-blend-multiply -z-10"></div>
<div class="h-full w-full main">
	<div class="trans-layer flex">
		<div class="flex flex-row center">
			<div class="flex flex-col w-1/2 gap-[5vh]">
				<div class="w-1/4 self-center">
					{@html somesWhite}
				</div>
				<p class="text-8xl font-bold text-center bg-gradient-to-br from-yellow-300 to-red-300 bg-clip-text text-transparent">
					{$t("common.tagline")}
				</p>
			</div>
			<div class="w-1/2">
				<div class="flex flex-col gap-7">
					<div class="flex flex-col gap-2">
						<div class="flex flex-row gap-2">
						<h1 class="h1">Somes</h1>
						<button 
							type="button"
							class="h3 h-min"
							title={$t("common.somes_button_title")}
							on:click={() => somesSound.play()}
							on:keypress={() => somesSound.play()}
						>{$t("common.somes_pronunciation")}</button>
					</div>
					<h4 class="h4"><b>{$t("common.somes_description")}</b></h4>
					</div>
					<div class="flex flex-col gap-3 w-1/3">
						<button
							type="button"
							on:click={(_) => drawerStore.open(loginDrawerSettings)}
							class="btn btn-lg bg-tertiary-500 rounded-lg h-12"
							>{$t("common.login")}</button
						>
						<button
							type="button"
							on:click={(_) => redirectToHome()}
							class="btn btn-lg bg-secondary-500 rounded-lg h-12"
							>{$t("common.no_account")}</button
						>
					</div>
					<div class="mt-2">
						<h5>
							<span class="font-semibold">{$t("common.sign_up_tagline")}</span>
							<a href="/register" class="font-bold"
								>{$t("common.sign_up_request")}</a
							>
						</h5>
					</div>
				</div>
			</div>
		</div>
	</div>
</div>

<style lang="scss">
	.background {
		position: absolute;
		width: 110%;
  		height: 110%;
		background: url("$lib/assets/parliament.png") no-repeat left center / cover;
	}

	.trans-layer {
		width: 100%;
		height: 100%;
		align-items: center;
  		justify-content: center;
	}

	a {
		text-decoration: none !important;
		color: inherit !important;
	}

	p { 
		word-spacing: 9999999px; 
	}

	.h1 {
		font-size: 5em;
	}

	.h4 {
		font-size: 1.5em;
	}
</style>
