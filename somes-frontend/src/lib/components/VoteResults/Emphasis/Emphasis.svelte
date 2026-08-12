<script lang="ts">
	import type { Glossary, Keypoint } from '$lib/ai_summary_types';
	import type { FullSpeech } from '$lib/speechTypes';
	import type { Delegate } from '$lib/types';
	import GlossaryText from '$lib/components/UI/GlossaryText.svelte';
	import KeypointSpeakers, { type KeypointSpeaker } from './KeypointSpeakers.svelte';
	import { slide } from 'svelte/transition';

	interface Props {
		emphasis: Keypoint[] | null;
		glossary?: Glossary | null;
		speeches?: FullSpeech[];
		delegates?: Delegate[];
		legisInitId?: number;
	}

	let { emphasis, glossary = null, speeches = [], delegates = [], legisInitId }: Props = $props();

	const VISIBLE_COUNT = 4;

	let open = $state(false);
	let firstPoints: Keypoint[] = $derived((emphasis ?? []).slice(0, VISIBLE_COUNT));
	let restPoints: Keypoint[] = $derived((emphasis ?? []).slice(VISIBLE_COUNT));

	let speakersByPoint = $derived.by(() => {
		const map = new Map<number, KeypointSpeaker[]>();
		if (legisInitId == null) return map;

		for (const speech of speeches) {
			const delegate = delegates.find((d) => d.id === speech.delegate_id);
			if (!delegate) continue;

			const pointsOfSpeech = new Map<number, number[]>();
			for (const relation of speech.relations) {
				if (relation.legis_init_id !== legisInitId) continue;
				for (const rel of relation.full_speech_relations.propsal_keypoint_relations ?? []) {
					for (const pointId of rel.referenced_proposal_key_point_ids) {
						const indexes = pointsOfSpeech.get(pointId) ?? [];
						if (!indexes.includes(rel.speech_key_point)) indexes.push(rel.speech_key_point);
						pointsOfSpeech.set(pointId, indexes);
					}
				}
			}

			for (const [pointId, pointIndexes] of pointsOfSpeech) {
				map.set(pointId, [...(map.get(pointId) ?? []), { delegate, speech, pointIndexes }]);
			}
		}

		return map;
	});
</script>

{#if emphasis}
	{#if emphasis.length > 0}
		<div class="emphasis-item rounded-xl bg-primary-300 px-5 pt-3 pb-3 dark:bg-primary-500">
			<div class="flex justify-between">
				<h1 class="text-lg font-semibold md:text-xl">Schwerpunkte</h1>
			</div>

			<ul class="list mt-2 fill-primary-400 px-3">
				{#each firstPoints as emph, i}
					<li class="mb-3 md:flex md:items-start md:justify-between md:gap-3">
						<span class="md:min-w-0 md:flex-1">
							<span class="badge bg-primary-500 dark:bg-primary-300"></span>
							{#if glossary}
								<span class="text-base text-gray-800 lg:text-base dark:text-gray-200">
									<GlossaryText text={emph.point} {glossary} />
								</span>
							{:else}
								<span>{emph.point}</span>
							{/if}
						</span>
						<KeypointSpeakers speakers={speakersByPoint.get(i) ?? []} pointText={emph.point} />
					</li>
				{/each}

				{#if open}
					<div transition:slide={{ duration: 240 }}>
						{#each restPoints as emph, i}
							<li class="my-3 md:flex md:items-start md:justify-between md:gap-3">
								<span class="md:min-w-0 md:flex-1">
									<span class="badge bg-primary-500 dark:bg-primary-300"></span>
									{#if glossary}
										<span class="text-base text-gray-800 lg:text-base dark:text-gray-200">
											<GlossaryText text={emph.point} {glossary} />
										</span>
									{:else}
										<span>{emph.point}</span>
									{/if}
								</span>
								<KeypointSpeakers
									speakers={speakersByPoint.get(i + VISIBLE_COUNT) ?? []}
									pointText={emph.point}
								/>
							</li>
						{/each}
					</div>
				{/if}

				{#if restPoints.length > 0}
					<button class="text-md font-semibold" onclick={() => (open = !open)}>
						<span>{open ? 'Weniger' : 'Mehr'} anzeigen</span>
					</button>
				{/if}
			</ul>
		</div>
	{:else}
		<div class="emphasis-item"></div>
	{/if}
{/if}

<style>
	.emphasis-item {
		grid-area: e;
	}
</style>
