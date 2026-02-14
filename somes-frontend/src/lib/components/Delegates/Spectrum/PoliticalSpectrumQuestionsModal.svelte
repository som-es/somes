<script lang="ts">
	import ModalCloseButton from '$lib/components/UI/ModalCloseButton.svelte';
	import type { StanceTopicInfluences } from '$lib/types';
	import QaDelegateStanceInfluences from './Stance/QADelegateStanceInfluences.svelte';
	import { Dialog } from 'bits-ui';

	interface Props {
		stanceTopicInfluences?: StanceTopicInfluences[];
	}

	let { stanceTopicInfluences = [] }: Props = $props();

</script>

<div class="card p-8 w-full min-w-7xl max-w-7xl">
	<Dialog.Close class="float-right">
		<ModalCloseButton />	
	</Dialog.Close>
	

	{#if stanceTopicInfluences.length > 0}
		{#each stanceTopicInfluences as qa}
			<QaDelegateStanceInfluences class="mt-3" stanceTopicInfluences={qa} />
		{/each}
	{:else}
		<p class="text-center">Keine Fragen und Antworten verfügbar.</p>	
	{/if}
</div>

<style>
</style>
