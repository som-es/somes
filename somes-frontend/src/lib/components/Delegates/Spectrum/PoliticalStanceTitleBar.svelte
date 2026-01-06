<script lang="ts">
	import type { StanceTopicInfluences } from '$lib/types';
	import { Dialog, Popover } from 'bits-ui';
	import PoliticalSpectrumQuestionsModal from './PoliticalSpectrumQuestionsModal.svelte';

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
				<button class="text-5xl mx-2" >⚠</button>
			</Popover.Trigger>
			<Popover.Portal>
				<Popover.Content class="z-40 p-4 w-72 bg-primary-100 dark:bg-primary-600 rounded-lg shadow-lg">
					Die Einordnung der politischen Position ist eine grobe Schätzung und muss nicht der Realität
					entsprechen. Verwendet wurden AI generierte Antworten zu Fragen, die nicht immer vollständig
					korrekt sind und unter 'Details' abrufbar sind.
				</Popover.Content>
			</Popover.Portal>
		</Popover.Root>
		<Dialog.Root>
			<Dialog.Trigger>
				<button
					class="mr-auto btn sm:btn-lg bg-secondary-500 text-white"
				>Details
				</button>
			</Dialog.Trigger>	
			<Dialog.Portal>
				<Dialog.Overlay
					class="data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 fixed inset-0 z-30 bg-black/80"
				/>
				<Dialog.Content class="
					overflow-y-scroll
					data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 
					data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 
					outline-hidden fixed left-[50%] top-[50%] z-30 translate-x-[-50%] translate-y-[-50%] w-full max-w-7xl h-[90vh] bg-primary-100 dark:bg-gray-800 shadow-lg rounded-lg overflow-hidden"
				>
					<PoliticalSpectrumQuestionsModal stanceTopicInfluences={stanceTopicInfluences}  />
				</Dialog.Content>
			</Dialog.Portal>
		</Dialog.Root>
	
	</div>
</div>

<style>
</style>
