<script lang="ts">
	import { getParliament, plink, type Parliament } from '$lib/api/parliament';
	import { gotoHistory } from '$lib/goto';
	import type { Bubble } from '$lib/parliament';
	import { getPartyColors } from '$lib/partyColor';
	import { currentDelegateStore } from '$lib/stores/stores';
	import type { Delegate } from '$lib/types';
	import DelegateCard from './DelegateCard.svelte';
	import SpeechModal from './Speeches/SpeechModal.svelte';
	import SpeechDelegateHeader from './Speeches/SpeechDelegateHeader.svelte';
	import { Popover } from 'bits-ui';

	interface Props {
		bubble: Bubble;
		date: string;
		gp: string;
		class?: string;
		partyColors?: Map<string, string>;
		parliament?: Parliament;
	}

	let {
		bubble,
		date,
		gp,
		class: clazz = '',
		partyColors = getPartyColors(),
		parliament = getParliament()
	}: Props = $props();

	let delegate: Delegate | null = $derived(bubble.del);

	let speechModalOpen = $state(false);

	const onShowDetails = () => {
		currentDelegateStore.value = delegate;
		if (delegate) {
			const link = plink(`/delegates?gp=${gp}&date=${date}&delegate=${delegate.id}`);
			gotoHistory(link, true);
		}
	};

	// const popupFeatured: PopupSettings = {
	// 	event: 'hover',
	// 	target: 'popupFeatured',
	// 	placement: 'bottom'
	// };

	let infoText = $derived(
		bubble.namedVote
			? `unsichere Zuteilung: "${bubble.namedVote.searched_with}" wurde ${bubble.namedVote.manually_matched ? 'manuell' : 'automatisch'} "${bubble.namedVote.matched_with}" zugeteilt`
			: ''
	);
	let namedVoteText = $derived(
		bubble.namedVote
			? bubble.namedVote.infavor != null
				? bubble.namedVote.infavor
					? 'Ja'
					: 'Nein'
				: bubble.namedVote.was_abstention
					? 'Enthalten'
					: 'Abwesend/keine Stimme abgegeben'
			: ''
	);

	let speechText = $derived(
		bubble.speech?.speech.infavor != null
			? bubble.speech.speech.infavor
				? 'Pro'
				: 'Contra'
			: (bubble.speech?.speech.opinion ?? bubble.title)
	);

	let opinionColor = $derived.by(() => {
		let color = '#ccc';
		if (bubble.speech) {
			color =
				bubble.speech.speech.infavor != null
					? bubble.speech.speech.infavor
						? 'bg-success-600'
						: 'bg-red-600'
					: 'bg-primary-500';
		} else {
			color = 'bg-primary-500';
		}
		if (bubble.namedVote) {
			color =
				bubble.namedVote.infavor != null
					? bubble.namedVote.infavor
						? 'bg-success-600'
						: 'bg-red-600'
					: bubble.namedVote.was_abstention
						? 'bg-blue-400'
						: 'bg-primary-500';
		}

		return color;
	});
</script>

{#if delegate}
	<DelegateCard
		{delegate}
		title={bubble.title}
		showMoreDetailsBtn
		onlyTop
		showAI={false}
		{partyColors}
		{parliament}
		{onShowDetails}
	>
		{#snippet top()}
			<span class="mt-2">
				{#if bubble.namedVote}
					<div
						class="badge text-sm font-bold sm:text-base md:text-lg {opinionColor} max-w-fit text-white"
					>
						{namedVoteText}
					</div>
				{:else if bubble.title}
					<span class="badge text-white {opinionColor} text-sm font-bold sm:text-base md:text-lg"
						>{speechText}</span
					>
				{/if}
			</span>
		{/snippet}

		{#snippet info()}
			<span>
				{#if bubble.namedVote && (bubble.namedVote.similiarity_score != 0 || bubble.namedVote.manually_matched)}
					<Popover.Root>
						<Popover.Trigger>
							<button class="text-2xl">⚠</button>
						</Popover.Trigger>
						<Popover.Portal>
							<Popover.Content>
								<div class="z-50! w-72 card p-4 shadow-xl">
									<div class="z-50 font-bold md:text-xl">Unsichere Zuteilung</div>
									<div>
										<span class="font-bold">"{bubble.namedVote.searched_with}"</span> wurde {bubble
											.namedVote.manually_matched
											? 'manuell'
											: 'automatisch'}
										<span class="font-bold">"{bubble.namedVote.matched_with}"</span>
										zugeordnet.
										<div>
											errechneter Unterschied: {bubble.namedVote.similiarity_score}
										</div>
									</div>
									<div class="arrow z-10! bg-surface-100-900"></div>
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
					<button
						class="rounded-xl bg-primary-600 p-2 px-3 text-white"
						onclick={() => (speechModalOpen = true)}
					>
						<h4>Rede</h4>
					</button>
				{/if}
			</span>
		{/snippet}
	</DelegateCard>

	{#if bubble.speech}
		<SpeechModal speech={bubble.speech} bind:open={speechModalOpen}>
			{#snippet header()}
				<div class="mb-1.5">
					<SpeechDelegateHeader {delegate} {partyColors} />
				</div>
			{/snippet}
		</SpeechModal>
	{/if}
{/if}
