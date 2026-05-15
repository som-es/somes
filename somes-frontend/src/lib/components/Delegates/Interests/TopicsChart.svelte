<script lang="ts">
	import { topicColors } from '$lib/interestColors';
	import type { InterestShare } from '$lib/types';
	import { Chart, Svg, Group, Rect } from 'layerchart';
	import { hierarchy } from 'd3-hierarchy';
	import ExtendInfoDialog from '../ExtendInfoDialog.svelte';
	import DetailedInterestsModal from './DetailedInterestsModal.svelte';
	import { Treemap } from 'layerchart/hierarchy';

	interface Props {
		interests: InterestShare[];
		detailedInterests: InterestShare[];
	}

	let { interests, detailedInterests }: Props = $props();

	let showRemaining = $state(false);
	const MAX_TOP_TILES = 8;

	function getTileOpacity(value: number, maxValue: number): number {
		const MIN_OPACITY = 0.35;
		const normalized = value / maxValue;
		return MIN_OPACITY + normalized * (1 - MIN_OPACITY);
	}

	function getLabelSize(w: number, h: number): number {
		const area = w * h;
		const base = Math.sqrt(area) / 10;
		return Math.max(7, Math.min(11, base));
	}

	let processedData = $derived.by(() => {
		const sorted = [...interests].sort((a, b) => b.occurences - a.occurences);

		const mapWithLocalMax = (items: InterestShare[]) => {
			const localMax = items[0]?.occurences ?? 1;
			const minVisualValue = localMax * 0.25;
			return items.map((i) => ({
				name: i.topic,
				value: Math.max(i.occurences, minVisualValue),
				actualValue: i.occurences,
				color: topicColors.get(i.topic) ?? '#94a3b8',
				opacity: getTileOpacity(i.occurences, localMax),
				isOther: false
			}));
		};

		if (sorted.length <= MAX_TOP_TILES + 1) {
			return { name: 'Root', children: mapWithLocalMax(sorted) };
		}

		if (showRemaining) {
			return { name: 'Root', children: mapWithLocalMax(sorted.slice(MAX_TOP_TILES)) };
		}

		const topItems = sorted.slice(0, MAX_TOP_TILES);
		const remainingItems = sorted.slice(MAX_TOP_TILES);
		const remainingSum = remainingItems.reduce((sum, item) => sum + item.occurences, 0);

		return {
			name: 'Root',
			children: [
				...mapWithLocalMax(topItems),
				{
					name: 'Weitere Themen → ',
					value: remainingSum,
					color: '#64748b',
					opacity: 1,
					isOther: true
				}
			]
		};
	});

	let hierarchyRoot = $derived(hierarchy(processedData).sum((d: any) => d.value));
</script>

<div class="title-item w-full gap-1 rounded-xl bg-primary-300 p-5 dark:bg-primary-500">
	<div class="flex items-center justify-between">
		<div class="flex flex-col">
			<span class="text-lg font-bold text-black xl:text-xl dark:text-white">
				Meist behandelte Themen
			</span>
		</div>

		<div class="flex items-center gap-2">
			{#if showRemaining}
				<button
					class="rounded-lg px-3 py-1.5 text-xs font-bold text-black transition-all dark:text-white"
					onclick={() => (showRemaining = false)}
				>
					← Zurück
				</button>
			{/if}

			{#if detailedInterests.length > 0}
				<ExtendInfoDialog title="Details">
					<DetailedInterestsModal {detailedInterests} />
				</ExtendInfoDialog>
			{/if}
		</div>
	</div>

	<div class="mt-3 h-[450px] w-full overflow-hidden rounded-xl bg-white/5 md:h-[320px]">
		<Chart data={hierarchyRoot} flatData={hierarchyRoot.leaves()}>
			<Svg>
				<Treemap hierarchy={hierarchyRoot}>
					{#snippet children({ nodes })}
						{#each nodes as node}
							{@const w = node.x1 - node.x0}
							{@const h = node.y1 - node.y0}
							{@const area = w * h}
							{@const labelSize = getLabelSize(w, h)}
							{@const valueSize = getLabelSize(w, h)}
							{#if !node.children}
								<g
									role="button"
									tabindex="0"
									class="outline-none {node.data.isOther ? 'cursor-pointer' : ''}"
									onclick={() => {
										if (node.data.isOther) showRemaining = true;
									}}
									onkeydown={(e) => {
										if (e.key === 'Enter' && node.data.isOther) showRemaining = true;
									}}
								>
									<Rect
										x={node.x0}
										y={node.y0}
										width={w}
										height={h}
										fill={node.data.color}
										fill-opacity={node.data.opacity}
										stroke="rgba(255,255,255,0.2)"
										strokeWidth={1}
										rx={4}
										class={node.data.isOther ? 'transition-opacity hover:opacity-80' : ''}
									/>

									<foreignObject
										x={node.x0}
										y={node.y0}
										width={w}
										height={h}
										class="pointer-events-none"
									>
										<div
											xmlns="http://www.w3.org/1999/xhtml"
											class="flex flex-col items-center justify-center overflow-hidden p-1 text-center {node
												.data.isOther
												? 'transition-opacity hover:opacity-80'
												: ''}"
											style="width: {w}px; height: {h}px;"
										>
											<p
												class="w-full font-bold tracking-tighter text-white uppercase"
												lang="de"
												style="text-shadow: 0 1px 2px rgba(0,0,0,0.5); font-size: {labelSize}px; line-height: 1.2; display: -webkit-box; -webkit-line-clamp: {Math.max(
													2,
													Math.floor(h / 14)
												)}; -webkit-box-orient: vertical; overflow: hidden; hyphens: auto; word-break: break-word; overflow-wrap: anywhere;"
											>
												{node.data.name}
											</p>

											<p
												class="mt-0.5 font-mono text-white/80"
												style="text-shadow: 0 1px 3px rgba(0,0,0,0.6); font-size: {valueSize}px;"
											>
												{node.data.actualValue ?? node.value}
											</p>
										</div>
									</foreignObject>
								</g>
							{/if}
						{/each}
					{/snippet}
				</Treemap>
			</Svg>
		</Chart>
	</div>
</div>
