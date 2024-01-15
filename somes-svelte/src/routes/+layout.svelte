<script lang="ts">
	// Global CSS imports
	import "../theme.postcss";
	import "@skeletonlabs/skeleton/styles/all.css";
	import "../app.postcss";
	import '@fortawesome/fontawesome-svg-core/styles.css'

	// Popup imports
	import { computePosition, autoUpdate, offset, shift, flip, arrow } from "@floating-ui/dom";
	import { Modal, storePopup, type ModalComponent } from "@skeletonlabs/skeleton";
	storePopup.set({ computePosition, autoUpdate, offset, shift, flip, arrow });

	import { AppBar, AppShell, Drawer, LightSwitch, drawerStore } from "@skeletonlabs/skeleton";
	import LoginDrawer from "@/components/LoginDrawer.svelte";
	import { userStore } from "../stores/stores";
	import { get } from "svelte/store";
	import { config } from '@fortawesome/fontawesome-svg-core'
	import QuestionModal from "@/components/QuestionModal.svelte";
	import LangSwitch from "@/components/LangSwitch.svelte";
	import somesTextIcon from "$lib/assets/somes_with_text.svg?raw";
	import userIcon from "$lib/assets/icons/user.svg?raw";
	import homeIcon from "$lib/assets/icons/home.svg?raw";
	import questionsIcon from "$lib/assets/icons/questions.svg?raw";
	import { base } from "$app/paths";

  config.autoAddCss = false;

	const parliamentUrl = new URL("$lib/assets/somes_with_text.svg", import.meta.url).href;
	drawerStore.close();

	// $: user = get(userStore);

	const profileLink = "{base}/profile";
	// $: if (user == null) {
	// profileLink = "/";
	// }

	const modalComponentRegistry: Record<string, ModalComponent> = {
		QuestionModal: {
			ref: QuestionModal,
			slot: "<p>Skeleton</p>",
		},
	};
</script>

<Drawer>
	{#if $drawerStore.id === "login-drawer"}
		<LoginDrawer />
	{/if}
</Drawer>

<Modal components={modalComponentRegistry} />

<AppShell>
	<!-- TODO: show the second header below the the first while also making it sticky (without the first one being sticky) -->
	<svelte:fragment slot="header">
		<AppBar background="bg-surface-50-900-token" padding="p-2">
			<svelte:fragment slot="lead">
				<a class="mx-4 fill-current stroke-current" href="/">
					{@html somesTextIcon}
				</a>
			</svelte:fragment>
			<svelte:fragment slot="trail">
				<LangSwitch />
				<LightSwitch width="w-12" />
			</svelte:fragment>
		</AppBar>
		<AppBar
			gridColumns="grid-cols-3"
			slotDefault="place-self-center"
			slotTrail="place-content-end"
			padding="p-2"
		>
			<svelte:fragment slot="lead">
				<a href="{base}/questions" class="pl-[25vw]">
					<div>
						{@html questionsIcon}
					</div>
				</a>
			</svelte:fragment>
			<a href="{base}/home" class="fill-current stroke-current">
				<div>
					{@html homeIcon}
				</div>
			</a>
			<svelte:fragment slot="trail">
				<a href="{base}/profile" class="pr-[25vw]">
					{@html userIcon}
				</a>
			</svelte:fragment>
		</AppBar>
	</svelte:fragment>
	<slot />
</AppShell>
