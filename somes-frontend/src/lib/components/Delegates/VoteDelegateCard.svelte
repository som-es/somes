<script lang="ts">
	import { run } from 'svelte/legacy';

	import { gotoHistory } from '$lib/goto';
	import type { Bubble } from '$lib/parliament';
	import { currentDelegateStore } from '$lib/stores/stores';
	import type { Delegate } from '$lib/types';
	import SButton from '../UI/SButton.svelte';
	import DelegateCard from './DelegateCard.svelte';
	import { Popover } from 'bits-ui';

	interface Props {
		bubble: Bubble;
		date: string;
		gp: string;
		class?: string;
	}

	let {
		bubble,
		date,
		gp,
		class: clazz = ''
	}: Props = $props();

	let delegate: Delegate | null = $derived(bubble.del);

	const onShowDetails = () => {
		currentDelegateStore.value = delegate;
		gotoHistory(`/delegates?gp=${gp}&date=${date}`, true);
	};
	
	// const popupFeatured: PopupSettings = {
	// 	event: 'hover',
	// 	target: 'popupFeatured',
	// 	placement: 'bottom'
	// };

	let infoText = $derived(bubble.namedVote ? `unsichere Zuteilung: "${bubble.namedVote.searched_with}" wurde ${bubble.namedVote.manually_matched ? 'manuell' : 'automatisch'} "${bubble.namedVote.matched_with}" zugeteilt` : "");
	let opinion = $derived(bubble.namedVote ? (bubble.namedVote.infavor != null
					? bubble.namedVote.infavor
						? 'Ja'
						: 'Nein'
					: 'Abwesend/keine Stimme abgegeben') : "");
	
	let opinionColor = $derived.by(() => {
		let color = "#ccc";
		if (bubble.speech) {
			color =
				bubble.speech.infavor != null
					? bubble.speech.infavor
						? 'bg-success-600'
						: 'bg-red-600'
					: 'bg-primary-500';
		} else {
			color = 'bg-primary-500'
		}
		if (bubble.namedVote) {
			color =
				bubble.namedVote.infavor != null
					? bubble.namedVote.infavor
						? 'bg-success-600'
						: 'bg-red-600'
					: 'bg-primary-500';
		}

		return color
	})

	
</script>

{#if delegate}
<DelegateCard {delegate} title={bubble.title} showMoreDetailsBtn onlyTop={true} showAI={false}>
	{#snippet top()}
		<span>
			{#if bubble.namedVote}
				<div class="text-sm sm:text-base md:text-lg font-bold badge {opinionColor} text-white max-w-fit">{opinion}</div>
			{:else}
				{#if bubble.title}
					<span class="badge text-white {opinionColor} font-bold text-sm sm:text-base md:text-lg">{bubble.title}</span>
				{/if}
			{/if}
		</span>
	{/snippet}

	{#snippet info()}
		<span >
			{#if bubble.namedVote && (bubble.namedVote.similiarity_score != 0 || bubble.namedVote.manually_matched)}
								<Popover.Root>
					<Popover.Trigger>
						<button class="text-2xl">⚠</button>
					</Popover.Trigger>
					<Popover.Portal>
						<Popover.Content>
							<div class="z-50! card p-4 w-72 shadow-xl" >
								<div class="z-50 font-bold md:text-xl">Unsichere Zuteilung</div>
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

								<div class="z-10! arrow bg-surface-100-900"></div>
							</div>

						</Popover.Content>
					</Popover.Portal>
				</Popover.Root>

			{/if}
		</span>
	{/snippet}
	{#snippet footerButtons()}
		<span>
			{#if bubble.speech}
			<button class="bg-primary-600 p-2 px-3 rounded-xl text-white" on:click={() =>
						window.open(`https://www.parlament.gv.at${bubble.speech?.document_url}`, '_blank')}>
				<h4>Rede</h4>
			</button>
			{/if}
		</span>
	{/snippet}
</DelegateCard>

{/if}