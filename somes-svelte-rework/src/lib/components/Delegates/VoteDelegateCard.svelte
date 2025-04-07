<script lang="ts">
	import { gotoHistory } from '$lib/goto';
	import type { Bubble } from '$lib/parliament';
	import { partyToColor } from '$lib/partyColor';
	import { currentDelegateStore } from '$lib/stores/stores';
	import type { Delegate } from '$lib/types';
	import { popup, type PopupSettings } from '@skeletonlabs/skeleton';
	import SButton from '../UI/SButton.svelte';
	import DelegateCard from './DelegateCard.svelte';

	export let bubble: Bubble;
	export let date: Date;
	export let gp: string;

	let delegate: Delegate;
	if (bubble && bubble.del !== null) {
		delegate = bubble.del;
	}

	const onShowDetails = () => {
		currentDelegateStore.set(delegate);
		gotoHistory(`/delegates?gp=${gp}&date=${date}`, true);
	};

	const popupFeatured: PopupSettings = {
		event: 'hover',
		target: 'popupFeatured',
		placement: 'bottom'
	};

	$: infoText = '';

	$: opinionColor = '';
	$: opinion = '';

	$: if (bubble.speech) {
		opinionColor =
			bubble.speech.infavor != null
				? bubble.speech.infavor
					? 'bg-success-600'
					: 'bg-red-600'
				: 'bg-primary-500';
	} else {
		opinionColor = 'bg-primary-500'
	}
	$: if (bubble.namedVote) {
		infoText = `unsichere Zuteilung: "${bubble.namedVote.searched_with}" wurde ${bubble.namedVote.manually_matched ? 'manuell' : 'automatisch'} "${bubble.namedVote.matched_with}" zugeteilt`;
		opinionColor =
			bubble.namedVote.infavor != null
				? bubble.namedVote.infavor
					? 'bg-success-600'
					: 'bg-red-600'
				: 'bg-primary-500';
		opinion =
			bubble.namedVote.infavor != null
				? bubble.namedVote.infavor
					? 'Ja'
					: 'Nein'
				: 'Abwesend/keine Stimme abgegeben';
	}
	$: if (bubble && bubble.del != null) delegate = bubble.del;


	let clazz = '';
	export { clazz as class };
</script>

<DelegateCard {delegate} title={bubble.title} showMoreDetailsBtn onlyTop={true} showAI={false}>
	<span slot="title">
		{#if bubble.namedVote}
			<div class="text-lg font-bold badge {opinionColor} text-white max-w-fit">{opinion}</div>
		{:else}
			{#if bubble.title}
				<span class="badge text-white {opinionColor} font-bold text-lg">{bubble.title}</span>
			{/if}
		{/if}
	</span>

	<span slot="info">
		{#if bubble.namedVote && (bubble.namedVote.similiarity_score != 0 || bubble.namedVote.manually_matched)}
			<div class="!z-50 card p-4 w-72 shadow-xl" data-popup="popupFeatured">
				<div class="z-50 font-bold text-xl">Unsichere Zuteilung</div>
				<div>
					<span class="font-bold">"{bubble.namedVote.searched_with}"</span> wurde {bubble.namedVote
						.manually_matched
						? 'manuell'
						: 'automatisch'} <span class="font-bold">"{bubble.namedVote.matched_with}"</span>
					zugeordnet.
					<div>
						errechneter Unterschied: {bubble.namedVote.similiarity_score}
					</div>
				</div>

				<div class="!z-10 arrow bg-surface-100-800-token" />
			</div>

			<button class="text-2xl" use:popup={popupFeatured}>⚠</button>
		{/if}
	</span>
	<span slot="footerButtons">
		{#if bubble.speech}
			<SButton
				class="bg-secondary-500 text-black"
				on:click={() =>
					window.open(`https://www.parlament.gv.at${bubble.speech?.document_url}`, '_blank')}
				>Rede</SButton
			>
		{/if}
	</span>
</DelegateCard>
