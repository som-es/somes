<script lang="ts">
	import Pagination from '$lib/components/Pagination.svelte';
	import AbsenceBar from './AbsenceBar.svelte';
	import type { Absence, Delegate, PlenarySession } from '$lib/types';
	import { Dialog } from 'bits-ui';
	import ModalCloseButton from '$lib/components/UI/ModalCloseButton.svelte';
	import { onMount } from 'svelte';
	import { cachedPlenarySessions } from '$lib/caching/plenarySessions';
	import { activePlenarySessionsForDelegate } from '$lib/activePlenarySessions';

	interface Props {
		absences: Absence[];
		title?: string;
		showDetails?: boolean;
		delegate: Delegate;
	}

	let {
		absences = [],
		title = 'Letzte Abwesenheiten',
		showDetails = true,
		delegate
	}: Props = $props();
	const ENTRIES = 15;
	let page = $state(1);

	let activePlenarySessionsPerGp: Record<string, PlenarySession[]> | null = $state(null);

	onMount(async () => {
		const plenarySessions = await cachedPlenarySessions();
		if (plenarySessions) {
			activePlenarySessionsPerGp = activePlenarySessionsForDelegate(
				delegate.mandates ?? [],
				plenarySessions
			);
		}
	});

	let currentPageAbsences: Absence[] = $derived(
		absences.slice((page - 1) * ENTRIES, page * ENTRIES)
	);
</script>

<div class="card p-8">
	<div class="flex items-center justify-between">
		<h1 class="text-2xl font-bold">{title}</h1>
		<Dialog.Close>
			<ModalCloseButton />
		</Dialog.Close>
	</div>
	{#each currentPageAbsences as absence}
		<AbsenceBar {absence} {page} {showDetails} />
	{/each}

	<div class="float-right">
		<Pagination bind:dynPage={page} maxPage={Math.ceil(absences.length / ENTRIES)} />
	</div>
</div>
