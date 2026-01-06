<script lang="ts">
	import type { Delegate, StanceTopicInfluences } from '$lib/types';
	export let delegate: Delegate;
	export let stanceTopicInfluences: StanceTopicInfluences[];

	// $: questionDetails = {
	// 	type: 'component',
	// 	component: 'politicalSpectrumQuestions',
	// 	meta: { delegate, stanceTopicInfluences }
	// } as ModalSettings;

	// const popupFeatured: PopupSettings = {
	// 	event: 'hover',
	// 	target: 'politicalPositionHint',
	// 	placement: 'bottom'
	// };

	// const modalStore = getModalStore();
</script>

<div class="flex justify-between items-center">
	<h1 class="font-bold max-lg:text-lg text-2xl">Politische Haltung und Richtung</h1>
	<div class="flex self-center items-center">
		<button class="text-5xl mx-2" >⚠</button>
		<!-- <Popover placement="bottom" trigger="hover"  transitionParams={{ duration: 200 }} class="z-40">
			<div class="font-bold text-xl p-4 w-72 card">
				Die Einordnung der politschen Position ist eine grobe Schätzung und muss nicht der Realität
				entsprechen. Verwendet wurden AI generierte Antworten zu Fragen, die nicht immer vollständig
				korrekt sind und unter 'Details' abrufbar sind.
			</div>
		</Popover> -->
		<button
			class="mr-auto btn sm:btn-lg preset-filled"
			on:click={() => {
			// modalStore.trigger(questionDetails)
		}}>Details
		</button>
		
	</div>
</div>

<style>
</style>
