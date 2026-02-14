<script lang="ts">
	import type { StanceTopicInfluences } from '$lib/types';
	import { Dialog, Popover } from 'bits-ui';
	import PoliticalSpectrumQuestionsModal from './PoliticalSpectrumQuestionsModal.svelte';
	import ExtendInfoDialog from '../ExtendInfoDialog.svelte';

	interface Props {
		stanceTopicInfluences: StanceTopicInfluences[];
	}

	let { stanceTopicInfluences }: Props = $props();

</script>

<div class="flex justify-between items-center">
	<h1 class="font-bold max-lg:text-lg text-2xl">Politische Haltung und Richtung</h1>
	<div class="flex self-center items-center">
		<Popover.Root>
			<Popover.Trigger openOnHover openDelay={100}>
				<span class="text-5xl mx-2" >⚠</span>
			</Popover.Trigger>
			<Popover.Portal>
				<Popover.Content class="z-40 p-4 w-72 bg-primary-100 dark:bg-primary-600 rounded-lg shadow-lg">
					Die Einordnung der politischen Position ist eine grobe Schätzung und muss nicht der Realität
					entsprechen. Verwendet wurden AI generierte Antworten zu Fragen, die nicht immer vollständig
					korrekt sind und unter 'Details' abrufbar sind.
				</Popover.Content>
			</Popover.Portal>
		</Popover.Root>
		<ExtendInfoDialog title="Details">
			<PoliticalSpectrumQuestionsModal stanceTopicInfluences={stanceTopicInfluences}  />
		</ExtendInfoDialog>
	</div>
</div>

<style>
</style>
