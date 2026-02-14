<script lang="ts">
		import Pagination from '$lib/components/Pagination.svelte';
	import AbsenceBar from './AbsenceBar.svelte';
	import type { Absence } from '$lib/types';
	import { Dialog } from 'bits-ui';
	import ModalCloseButton from '$lib/components/UI/ModalCloseButton.svelte';

	interface Props {
		absences: Absence[];
	}

	let { absences = [] }: Props = $props();
	const ENTRIES = 15;
	let page = $state(1);

	let currentPageAbsences: Absence[] = $derived(absences.slice((page - 1) * ENTRIES, page * ENTRIES));

</script>

<div class="card p-8 max-w-7xl w-7xl">
	<div class="flex justify-between items-center">
		<h1 class="font-bold text-2xl">Letzte Abwesenheiten</h1>
		<Dialog.Close>
			<ModalCloseButton />	
		</Dialog.Close>
	</div>
	{#each currentPageAbsences as absence}
		<AbsenceBar {absence} {page} />
	{/each}
	
	<div class="float-right">
		<Pagination bind:dynPage={page} maxPage={Math.ceil(absences.length / ENTRIES)} />
	</div>
</div>
