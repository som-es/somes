<script lang="ts">
    import { topicColors } from '$lib/interestColors';
    import type { InterestShare } from '$lib/types';
    import { Chart, Svg, Treemap, Group, Rect } from 'layerchart';
    import { hierarchy } from 'd3-hierarchy';
    import ExtendInfoDialog from '../ExtendInfoDialog.svelte';
    import DetailedInterestsModal from './DetailedInterestsModal.svelte';

    interface Props {
        interests: InterestShare[];
        detailedInterests: InterestShare[];
    }

    let { interests, detailedInterests }: Props = $props();

    let showRemaining = $state(false);
    const MAX_TOP_TILES = 6;

    function getTileOpacity(value: number, maxValue: number): number {
        const MIN_OPACITY = 0.35;
        const normalized = value / maxValue;
        return MIN_OPACITY + normalized * (1 - MIN_OPACITY);
    }

    let processedData = $derived.by(() => {
    const sorted = [...interests].sort((a, b) => b.occurences - a.occurences);

    const mapWithLocalMax = (items: InterestShare[]) => {
        const localMax = items[0]?.occurences ?? 1;  // ← max of this slice only
        return items.map(i => ({
            name: i.topic,
            value: i.occurences,
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
            { name: 'Weitere Themen...', value: remainingSum, color: '#64748b', opacity: 1, isOther: true }
        ]
    };
});

    let hierarchyRoot = $derived(hierarchy(processedData).sum((d: any) => d.value));
</script>

<div class="title-item rounded-xl bg-primary-300 dark:bg-primary-500 p-5 gap-1 w-full">
    <div class="flex justify-between items-center">
        <div class="flex flex-col">
            <span class="font-bold xl:text-xl text-lg text-black dark:text-white">
                Meist behandelte Themen
            </span>
        </div>

        <div class="flex gap-2 items-center">
            {#if showRemaining}
                <button 
                    class="text-xs px-3 py-1.5 rounded-lg font-bold transition-all text-black dark:text-white"
                    onclick={() => showRemaining = false}
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

    <div class="h-[350px] w-full rounded-xl mt-3 bg-white/5 overflow-hidden">
        <Chart 
            data={hierarchyRoot} 
            flatData={hierarchyRoot.leaves()}
        >
            <Svg>
                <Treemap hierarchy={hierarchyRoot}>
                    {#snippet children({ nodes })}
                        {#each nodes as node}
                            {#if !node.children}
                                <Group x={node.x0} y={node.y0}>
                                    <g 
                                        role="button"
                                        tabindex="0"
                                        class="outline-none {node.data.isOther ? 'cursor-pointer hover:opacity-80 transition-opacity' : ''}"
                                        onclick={() => { if (node.data.isOther) showRemaining = true; }}
                                        onkeydown={(e) => { if (e.key === 'Enter' && node.data.isOther) showRemaining = true; }}
                                    >
                                        <Rect 
                                            width={node.x1 - node.x0} 
                                            height={node.y1 - node.y0} 
                                            fill={node.data.color}
                                            fill-opacity={node.data.opacity}
                                            stroke="rgba(255,255,255,0.2)"
                                            strokeWidth={1}
                                            rx={4}
                                        />

                                        {#if (node.x1 - node.x0) > 40 && (node.y1 - node.y0) > 30}
                                            <foreignObject
                                                x={0}
                                                y={0}
                                                width={node.x1 - node.x0}
                                                height={node.y1 - node.y0}
                                                class="pointer-events-none p-2"
                                            >
                                                <div class="flex flex-col h-full justify-center items-center text-center overflow-hidden">
                                                    <p class="text-[11px] font-bold text-white leading-tight line-clamp-2 w-full uppercase tracking-tighter">
                                                        {node.data.name}
                                                    </p>
                                                    <p class="text-[10px] text-white/70 font-mono mt-1">
                                                        {node.value}
                                                    </p>
                                                </div>
                                            </foreignObject>
                                        {/if}
                                    </g>
                                </Group>
                            {/if}
                        {/each}
                    {/snippet}
                </Treemap>
            </Svg>
        </Chart>
    </div>
</div>