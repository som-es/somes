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
    const possibleSelection: Bubble[] = [];
    possibleSelectionUnchanged.forEach(bubble => possibleSelection.push(Object.assign({}, bubble)));

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

    onMount(() => {
        interval = setInterval(() => {            
            if (possibleSelection.length == 0) {
                possibleSelectionUnchanged.forEach(bubble => possibleSelection.push(Object.assign({}, bubble)));
            }
            const idx = Math.floor(Math.random() * possibleSelection.length);
            select(possibleSelection[idx]);
            possibleSelection.splice(idx, 1);
        }, 1000 * 5);
    });

    onDestroy(() => {
        if (interval) {
            clearInterval(interval);
        }
    })
    
</script>

{#if activeSelection && activeSelection.del}
    <div class="card w-60">
        <header>
            <img src={activeSelection.del.image_url} class="bg-black/50 w-full aspect-[10/9]" alt="image of politician {activeSelection.del.name}">
        </header>
        <section class="p-4"><h4>{activeSelection.del.name}</h4><span>{activeSelection.del.party}</span></section>
    </div>
    <!-- <DelegateCard name={selected.del.name} party={selected.del.party} image={selected.del.image_url} /> -->
{/if}

<svg {width} {height}> 
    {#each circles as circle}
        <div class="box" >B</div>
        <circle type="button" cx={circle.x} cy={circle.y} r={circle.r}
            fill={circle.color}
            fill-opacity={circle.color == "rgb(196, 180, 189)" && circle.del == null ? 0.2 : 1}
        />
        <div class="overlay">Overlay</div>
    {/each}
    
</svg>