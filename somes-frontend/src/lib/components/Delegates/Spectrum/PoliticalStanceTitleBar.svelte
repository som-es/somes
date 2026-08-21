<script lang="ts">
	import type { StanceTopicInfluences } from '$lib/types';
	import { Dialog, Popover } from 'bits-ui';
	import PoliticalSpectrumQuestionsModal from './PoliticalSpectrumQuestionsModal.svelte';
	import ExtendInfoDialog from '../ExtendInfoDialog.svelte';

	interface Props {
		stanceTopicInfluences: StanceTopicInfluences[];
		usefulInfoCount: number;
	}

	let { stanceTopicInfluences, usefulInfoCount }: Props = $props();
	console.log(usefulInfoCount);
</script>

<div class="flex w-full flex-col lg:flex-row lg:items-center lg:justify-between">
	<h1 class="text-2xl font-bold max-lg:text-lg">Politische Haltung und Richtung</h1>
	{#if usefulInfoCount < 5}
	    <span class="text-xs font-semibold text-orange-400">Achtung! Wenige Infos vorhanden</span>
	{/if}
	<div class="flex items-center justify-between lg:justify-end lg:gap-4">
		<Popover.Root>
			<Popover.Trigger openOnHover openDelay={100}>
				<div class="flex items-center">
					<span class="font-bold">Hinweis &#8594; </span>
					<span class="mx-2 text-3xl sm:text-4xl">⚠</span>
				</div>
			</Popover.Trigger>
			<Popover.Portal>
				<Popover.Content
					class="z-40 w-72 rounded-lg bg-primary-100 p-4 shadow-lg dark:bg-primary-600"
				>
					Die Einordnung der politischen Position ist eine grobe Schätzung und muss nicht der
					Realität entsprechen. Verwendet wurden AI generierte Antworten zu Fragen, die nicht immer
					vollständig korrekt und unter 'Details' abrufbar sind. <span class="font-bold"
						>Eine Beschreibung der Methodik und Verbesserungen folgen!</span
					>
				</Popover.Content>
			</Popover.Portal>
		</Popover.Root>
		<ExtendInfoDialog title="Details">
			<PoliticalSpectrumQuestionsModal {stanceTopicInfluences} />
		</ExtendInfoDialog>
	</div>
</div>

<style>
</style>
