<script lang="ts">
    import { setDelOnBubble, type Bubble, setupParliament } from "$lib/parliament";
	import type { Delegate } from "$lib/types";
	import { onDestroy, onMount } from "svelte";
	
    export let seats: number[];
    export let dels: Delegate[];

    const width = 830;
    const height = 900;
    
    let circles2d: Bubble[][] = setupParliament(seats, width, height, 6.9);

    dels.forEach((del) => {
        setDelOnBubble(del, circles2d);
    });

    let circles: Bubble[] = circles2d.flat(1);

    let interval: NodeJS.Timer;

    const possibleSelectionUnchanged = circles.filter((circle) => circle.del != null);
    let possibleSelection: Bubble[] = [...possibleSelectionUnchanged];
    //possibleSelectionUnchanged.forEach(bubble => possibleSelection.push(bubble));

    let activeSelection: Bubble;

    function select(bubble: Bubble) {
        if (bubble.del == null) {
            return;
        }
        if (activeSelection != null) {
            activeSelection.r = 6;
        }
        bubble.r = +10.9;
        circles = circles;
        activeSelection = bubble;
	}

    function updateSelection() {
        if (possibleSelection.length == 0) {
            possibleSelection = [...possibleSelectionUnchanged]
        }
        const idx = Math.floor(Math.random() * possibleSelection.length);
        select(possibleSelection[idx]);
        possibleSelection.splice(idx, 1);
    }
    updateSelection();
    onMount(() => {
        interval = setInterval(updateSelection, 1000 * 5);
    });

    onDestroy(() => {
        if (interval) {
            clearInterval(interval);
        }
    })
    
</script>

<!---->

<div class="flex border max-w-min">
    <svg width={width * 0.3} height={height * 0.15 + 10}> 
        {#each circles as circle}
            <circle type="button" cx={circle.x} cy={circle.y} r={circle.r}
                fill={circle.color}
                fill-opacity={circle.color == "rgb(196, 180, 189)" && circle.del == null ? 0.2 : 1}
                transform="scale(0.3)"
            />
        {/each}   
    </svg>
    <div class="self-center">
        {#if activeSelection && activeSelection.del}
            <div class="card w-20">
                <header>
                    <img src={activeSelection.del.image_url} class="bg-black/50 w-full aspect-[10/9] rounded" alt="image of politician {activeSelection.del.name}">
                </header>
                <section class="p-0.5" style="font-size: 9px;">{activeSelection.del.name}</section>
            </div>
        {/if}
    </div>
</div>