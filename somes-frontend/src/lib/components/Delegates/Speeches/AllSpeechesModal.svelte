<script lang="ts">
	import type { Speech } from '$lib/types';
	import Pagination from '$lib/components/Pagination.svelte';
	import SpeechBar from './SpeechBar.svelte';
	import { errorToNull, speeches_by_delegate_per_page } from '$lib/api/api';
	import { Dialog } from 'bits-ui';
	import ModalCloseButton from '$lib/components/UI/ModalCloseButton.svelte';

	// const modalStore = getModalStore();
	

	interface Props {
		// export let parent;
		delegateId: number;
		speechesPage0: { speeches: Speech[]; max_page: number };
	}

	let { delegateId, speechesPage0 }: Props = $props();

	let filteredSpeeches: Speech[] = [];
	let currentPageSpeeches: Speech[] = $derived(speechesPage0.speeches);

	let page = $state(1);

	$effect(() => {
		console.log("hit effect", page);
		speeches_by_delegate_per_page(delegateId, page - 1).then((res) => {
			currentPageSpeeches = errorToNull(res)?.speeches ?? [];
		});
	});

</script>

<div class="card p-8 ">
	<div class="flex justify-between">
		<h1 class="font-bold text-2xl">Letzte Reden</h1>
		<Dialog.Close>
			<ModalCloseButton />
		</Dialog.Close>	
	</div>

	{#each currentPageSpeeches as speech}
		<SpeechBar {speech} />
	{/each}

	<div class="float-right">
		<Pagination bind:dynPage={page} maxPage={speechesPage0.max_page} />
	</div>
</div>
