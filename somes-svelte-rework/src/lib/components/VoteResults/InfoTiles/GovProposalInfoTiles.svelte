<script lang="ts">
	import Square from '$lib/components/UI/Square.svelte';
	import { dashDateToDotDate } from '$lib/date';
	import type { GovProposal } from '$lib/types';
	import { popup, type PopupSettings } from '@skeletonlabs/skeleton';

	export let govProposal: GovProposal;
    
    const popupFeatured: PopupSettings = {
		event: 'click',
		target: 'ressort',
		placement: 'bottom'
	};
</script>

<div class="!z-50 card p-4 w-72 shadow-xl" data-popup="ressort">
    <div class="z-50 font-bold text-xl">{govProposal.ministrial_proposal.ressort}</div>
</div>

<Square>
	<div class="cursor-pointer font-bold text-lg" use:popup={popupFeatured}>
		{govProposal.ministrial_proposal.ressort_shortform}
	</div>
	<div>Ressort</div>
</Square>
<Square>
	<div class="font-bold text-lg">
		{dashDateToDotDate(govProposal.ministrial_proposal.created_at.split('T')[0].toString())}
	</div>
	<div>Veröffentlicht am</div>
</Square>

<Square>
	<div class="font-bold text-lg">
		{dashDateToDotDate(govProposal.ministrial_proposal.due_to.toString())}
	</div>
	<div>Fällig bis</div>
</Square>

{#if govProposal.vote_result}
	<Square>
		<div class="font-bold text-lg">
			{dashDateToDotDate(govProposal.vote_result.legislative_initiative.created_at.toString())}
		</div>
		<div>Abgestimmt am</div>
		<div>(finale Version)</div>
	</Square>
{/if}

