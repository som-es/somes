<script lang="ts">
	import ModalCloseButton from '$lib/components/UI/ModalCloseButton.svelte';
	import { topicColors } from '$lib/interestColors';
	import type { InterestShare, StanceTopicScore } from '$lib/types';
	import { Dialog } from 'bits-ui';
	import LeftRightSvg from './LeftRightSvg.svelte';

	interface Props {
		stances: StanceTopicScore[];
	}

	let { stances }: Props = $props();
</script>

<div class="card p-4 sm:p-8">
	<div class="flex items-start justify-between gap-2">
		<span class="min-w-0 flex-1 text-xl font-bold sm:text-2xl">Richtung</span>
		<Dialog.Close>
			<ModalCloseButton />
		</Dialog.Close>
	</div>

	<span class="text-base sm:text-lg">Politische Ausrichtung nach Thema</span>

	<div class="mt-4 w-full overflow-x-auto">
		<LeftRightSvg {stances} />
	</div>
</div>
