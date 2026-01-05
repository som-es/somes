<script lang="ts">
	import {
		type ModalComponent
	} from '@skeletonlabs/skeleton-svelte';
	import CacheInvalidation from '$lib/components/CacheInvalidation/CacheInvalidation.svelte';
	import Sidebar from '$lib/components/Bars/Sidebar.svelte';
	import Bottombar from '$lib/components/Bars/Bottombar.svelte';
	import Navbar from '$lib/components/Bars/Navbar.svelte';

	import { computePosition, autoUpdate, offset, shift, flip, arrow } from '@floating-ui/dom';
	import LoginDrawer from '$lib/components/Login/LoginDrawer.svelte';
	storePopup.set({ computePosition, autoUpdate, offset, shift, flip, arrow });
	initializeStores();
	import RenewToken from '$lib/components/Login/RenewToken.svelte';
	import DelegateQAModal from '$lib/components/Delegates/QA/DelegateQAModal.svelte';
	import AiChatModal from '$lib/components/Delegates/AIChat/AIChatModal.svelte';
	import AllProposalsModal from '$lib/components/Proposals/AllProposalsModal.svelte';
	import AllSpeechesModal from '$lib/components/Delegates/Speeches/AllSpeechesModal.svelte';
	import PoliticalSpectrumQuestionsModal from '$lib/components/Delegates/Spectrum/PoliticalSpectrumQuestionsModal.svelte';
	import AbsencesModal from '$lib/components/Delegates/Absences/AbsencesModal.svelte';
	import NamedVoteModal from '$lib/components/Delegates/NamedVote/NamedVoteModal.svelte';
	import DetailedInterestsModal from '$lib/components/Delegates/Interests/DetailedInterestsModal.svelte';
	import DecreesModal from '$lib/components/Delegates/Decrees/DecreesModal.svelte';
	import { afterNavigate } from '$app/navigation';
	import { partyColors, setPartyColors } from '$lib/partyColor';
	import { cachedPartyColors } from '$lib/caching/party_color';
	import { onMount } from 'svelte';
	const drawerStore = getDrawerStore();

	afterNavigate(() => {
		window.scrollTo({ top: 0, left: 0, behavior: 'auto' });
	});

	// const drawerStore = getDrawerStore();
	const modalRegistry: Record<string, ModalComponent> = {
		delegateQA: { ref: DelegateQAModal },
		aiChat: { ref: AiChatModal },
		allGovProposals: { ref: AllProposalsModal },
		allSpeeches: { ref: AllSpeechesModal },
		politicalSpectrumQuestions: { ref: PoliticalSpectrumQuestionsModal },
		allAbsences: { ref: AbsencesModal },
		allNamedVotes: { ref: NamedVoteModal },
		detailedInterests: { ref: DetailedInterestsModal },
		allDecrees: { ref: DecreesModal }
		// imdying:{ ref: ModalExample }
	};
</script>

<RenewToken />
<CacheInvalidation />
<Modal components={modalRegistry} />

<Drawer>
	{#if $drawerStore.id === 'sidebar'}
		<Sidebar embedded={true} />
	{:else if $drawerStore.id === 'login-drawer'}
		<LoginDrawer />
	{/if}
</Drawer>

<div class="grid h-screen grid-rows-[auto_1fr_auto]">
	<header class="lg:hidden sticky top-0 z-0">
		<Navbar />
	</header>
	<div class="grid grid-cols-[auto_1fr]">
    <!-- Left Sidebar. -->
		<aside class="sticky top-0 col-span-1 h-screen">
			<Sidebar class="hidden lg:grid" />
		</aside>
		<!-- Main Content -->
		<main>
			<slot />
		</main>
	</div>
  	<footer class="sticky bottom-0 z-50">
		<div class=" sm:hidden! max-h-18">
			<Bottombar />
		</div>
  </footer>
</div>
<style>
</style>
