<script lang="ts">
	import { gotoHistory } from '$lib/goto';
	import type { Bubble } from '$lib/parliament';
	import { partyToColor } from '$lib/partyColor';
	import { currentDelegateStore } from '$lib/stores/stores';
	import type { Delegate } from '$lib/types';
	import { popup, type PopupSettings } from '@skeletonlabs/skeleton';
	import SButton from '../UI/SButton.svelte';

	export let bubble: Bubble;
	let delegate: Delegate;
	if (bubble && bubble.del !== null) {
		delegate = bubble.del;
	}

	const onShowDetails = () => {
		currentDelegateStore.set(delegate);
		gotoHistory('/delegates', true);
	};

	const popupFeatured: PopupSettings = {
		event: 'hover',
		target: 'popupFeatured',
		placement: 'bottom'
	};

	let infoText = '';
	$: if (bubble.namedVote !== null) {
		infoText = `unsichere Zuteilung: "${bubble.namedVote.searched_with}" wurde ${bubble.namedVote.manually_matched ? 'manuell' : 'automatisch'} "${bubble.namedVote.matched_with}" zugeteilt`;
	}
	$: if (bubble && bubble.del != null) delegate = bubble.del;

	let clazz = '';
	export { clazz as class };
</script>

{#if delegate}
	<div class="!z-0 card card-hover min-h-full mx-4 drop-shadow-lg flex flex-col {clazz}">
		<header class="flex justify-center">
			<img
				src={delegate.image_url}
				style="width: 200px;"
				alt="Image of politician {delegate.name}"
			/>
		</header>
		<section class="p-4 flex-grow">
			<h4 class="font-bold text-lg">
				{delegate.name}
			</h4>
			<h5 style="color: {partyToColor(delegate.party)}">
				{#if delegate.party == 'OK'}
					Ohne Klub
				{:else}
					<span>{delegate.party}</span>
				{/if}
			</h5>
			<!-- <h6>{delegate.constituency}</h6> -->
			<!-- <hr class="!border-t-2 my-1" /> -->
			<!-- <h6>{delegate.divisions?.join(', ')}</h6> -->
			<hr class="!border-t-2 my-1" />
			{bubble.title}
			{#if bubble.namedVote && (bubble.namedVote.similiarity_score != 0 || bubble.namedVote.manually_matched)}
				<div class="!z-10 card p-4 w-72 shadow-xl" data-popup="popupFeatured">
					<div class="font-bold text-xl">Unsichere Zuteilung</div>
					<div>
						<span class="font-bold">"{bubble.namedVote.searched_with}"</span> wurde {bubble
							.namedVote.manually_matched
							? 'manuell'
							: 'automatisch'} <span class="font-bold">"{bubble.namedVote.matched_with}"</span>
						zugeordnet.
						<div>
							errechneter Unterschied: {bubble.namedVote.similiarity_score}
						</div>
					</div>

					<div class="!z-10 arrow bg-surface-100-800-token" />
				</div>

				<button class="text-2xl" use:popup={popupFeatured}>&#9432;</button>
			{/if}
		</section>
		<hr class="!border-t-2 my-1" />
		<footer class="card-footer flex justify-end items-end mt-3">
			<SButton class="bg-tertiary-500 text-black" on:click={onShowDetails}>Details anzeigen</SButton
			>
		</footer>
	</div>
{/if}
