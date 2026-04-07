<script lang="ts">
	import DecreeBar from './DecreeBar.svelte';
	import type { Decree, DecreeDelegate } from './types';
	import type { Delegate } from '$lib/types';
	import ExtendInfoDialog from '../ExtendInfoDialog.svelte';
	import DecreesModal from './DecreesModal.svelte';

	interface Props {
		decrees: Decree[];
		delegate: Delegate;
	}

	let { decrees, delegate }: Props = $props();

	// $: allDecrees = {
	// 	type: 'component',
	// 	component: 'allDecrees',
	// 	meta: { delegateId: delegate.id, decrees }
	// } as ModalSettings;

	// const modalStore = getModalStore();

	let delegateDecrees: DecreeDelegate[] = $derived(
		decrees.map((decree) => {
			return { decree, delegate };
		})
	);

	let previewDecrees = $derived(delegateDecrees.slice(0, 2));
</script>

<div>
	<div class="flex items-start justify-between">
		<div>
			<h1 class="text-lg font-bold text-black xl:text-xl dark:text-white">Letzte Verordnungen</h1>

			<h2 class="text-sm text-primary-600 dark:text-primary-300">
				{decrees.length}
				{decrees.length == 1 ? 'Verordnung' : 'Verordnungen'} insgesamt
			</h2>
		</div>
		<ExtendInfoDialog title="Alle anzeigen">
			<DecreesModal decrees={delegateDecrees} />
		</ExtendInfoDialog>
	</div>
</div>
<div>
	{#each previewDecrees as decree}
		<!-- <div class="gap-3 rounded-sm variant-filled my-1">{speech.legislative_initiatives_id} {speech.opinion}</div> -->
		<DecreeBar {decree}></DecreeBar>
		<!-- <GovProposalExpandableBar {govProposal} /> -->
	{/each}
</div>
