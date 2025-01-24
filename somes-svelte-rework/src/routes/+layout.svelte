<script lang="ts">
	import '../app.postcss';
	import {
		AppShell,
		Drawer,
		initializeStores,
		Modal,
		type ModalComponent
	} from '@skeletonlabs/skeleton';
	import CacheInvalidation from '$lib/components/CacheInvalidation/CacheInvalidation.svelte';
	import Sidebar from '$lib/components/Bars/Sidebar.svelte';
	import Bottombar from '$lib/components/Bars/Bottombar.svelte';
	import Navbar from '$lib/components/Bars/Navbar.svelte';

	import { computePosition, autoUpdate, offset, shift, flip, arrow } from '@floating-ui/dom';
	import { storePopup } from '@skeletonlabs/skeleton';
	import LoginDrawer from '$lib/components/Login/LoginDrawer.svelte';
	storePopup.set({ computePosition, autoUpdate, offset, shift, flip, arrow });
	initializeStores();

	import { getDrawerStore } from '@skeletonlabs/skeleton';
	import RenewToken from '$lib/components/Login/RenewToken.svelte';
	import DelegateQAModal from '$lib/components/Delegates/QA/DelegateQAModal.svelte';
	import AiChatModal from '$lib/components/Delegates/AIChat/AIChatModal.svelte';
	import AllProposalsModal from '$lib/components/Proposals/AllProposalsModal.svelte';
	import AllSpeechesModal from '$lib/components/Delegates/Speeches/AllSpeechesModal.svelte';
	import PoliticalSpectrumQuestionsModal from '$lib/components/Delegates/Spectrum/PoliticalSpectrumQuestionsModal.svelte';
	const drawerStore = getDrawerStore();

	// const drawerStore = getDrawerStore();
	const modalRegistry: Record<string, ModalComponent> = {
		delegateQA: { ref: DelegateQAModal },
		aiChat: { ref: AiChatModal },
		allGovProposals: { ref: AllProposalsModal },
		allSpeeches: { ref: AllSpeechesModal },
		politicalSpectrumQuestions: { ref: PoliticalSpectrumQuestionsModal}
		// imdying:{ ref: ModalExample }
	};
</script>

<RenewToken />
<CacheInvalidation />
<Modal components={modalRegistry} />
<Drawer>
	{#if $drawerStore.id === 'login-drawer'}
		<LoginDrawer />
	{/if}
</Drawer>

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
