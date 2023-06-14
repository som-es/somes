<script lang="ts">
    import { setDelOnBubble, type Bubble, setupParliament, partyToColor } from "$lib/parliament";
	import type { Delegate } from "$lib/types";
	import { onDestroy, onMount } from "svelte";
    import { localStorageStore } from '@skeletonlabs/skeleton';
    import type { Writable } from 'svelte/store';
	
    export let seats: number[];
    export let dels: Delegate[];

    const width = 830;
    const height = 900;
    
    let circles2d: Bubble[][] = setupParliament(seats, width, height, 6.9);

    dels.forEach((del) => {
        setDelOnBubble(del, circles2d, partyToColor);
    });

    let circles: Bubble[] = circles2d.flat(1);

    let interval: NodeJS.Timer;

    const possibleSelectionUnchanged = circles.filter((circle) => circle.del != null);
    let possibleSelection: Bubble[] = [...possibleSelectionUnchanged];
    //possibleSelectionUnchanged.forEach(bubble => possibleSelection.push(bubble));

    let activeSelection: Bubble;

    const currentDelegateStorage: Writable<Bubble | null> = localStorageStore('currentDelegate', null);

    function select(bubble: Bubble) {
        if (bubble.del == null) {
            return;
        }
        if (activeSelection != null) {
            activeSelection.r = 6;
        }
        bubble.r = +10.9;
        circles = circles;
        bubblesWithImage = bubblesWithImage;
        activeSelection = bubble;
        currentDelegateStorage.set(bubble);
	}

    function randomBubble() {
        const idx = Math.floor(Math.random() * possibleSelection.length);;
        const bubble = possibleSelection[idx];
        possibleSelection.splice(idx, 1);
        return bubble;
    }

    let bubblesWithImage: {bubble: Bubble, image: HTMLImageElement}[] = [];
    let currentImg: null | HTMLImageElement = null;

    function prepareBubble() {
        const bubble = randomBubble();
        if (bubble.del != null) {
            currentImg = preloadImage(bubble.del.image_url);
            bubblesWithImage.push({bubble, image: currentImg});
        }    
        return bubble;
    }

    prepareBubble();

    function updateSelection() {
        if (possibleSelection.length == 0) {
            possibleSelection = [...possibleSelectionUnchanged]
        }

        if (possibleSelection.length == 0) {
            return;
        }
        
        prepareBubble();
        currentImg = bubblesWithImage[0].image;

        select(bubblesWithImage[0].bubble);
        bubblesWithImage.splice(0, 1);
    }   
    
    function preloadImage(url: string) {
        console.log(url);
        const img = new Image();
        img.src = url;
        return img;
    }

    updateSelection()
    
    onMount(() => {
        interval = setInterval(() => {
            updateSelection();
        }, 1000 * 4); // 1000 * 4
    });
    
    onDestroy(() => {
        if (interval) {
            clearInterval(interval);
        }
    })
    
</script>

<!---->

<a href="/delegate" class="flex flex-wrap border max-w-[20.2rem]">
    <div>
        <svg viewBox="0 0 {width} {height * 0.5+13}" style="width: 240px; max-width: 100%;">
        <!-- <svg width={width * 0.3} height={height * 0.15 + 10}>  -->
            {#each circles as circle}
                <circle type="button" cx={circle.x} cy={circle.y} r={circle.r}
                    fill={circle.color}
                    fill-opacity={circle.color == "rgb(196, 180, 189)" && circle.del == null ? 0.2 : 1}
                />
            {/each}   
        </svg>
    </div>
    <div class="self-center">
        <!-- {selectedBubbles.shift()} -->
        {#if activeSelection && activeSelection.del}
            <div class="card w-20 border" style="border-color: {activeSelection.color};">
                <header>     
                    {#if currentImg}
                        <img src={currentImg.src} class="bg-black/50 w-full aspect-[10/9] rounded" alt="image of politician {activeSelection.del.name}">
                    {/if}
                    <!-- <img src={activeSelection.del.image_url} class="bg-black/50 w-full aspect-[10/9] rounded" alt="image of politician {activeSelection.del.name}"> -->
                </header>
                <section class="p-0.5 dark:text-white" style="font-size: 9px;">{activeSelection.del.name}</section>
            </div>
        {/if}
    </div>
</a>

<style>
a:link{
  text-decoration: none!important;
}
</style>