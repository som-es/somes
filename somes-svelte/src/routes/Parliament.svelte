<script lang="ts">
	import { setDelOnBubble, type Bubble, setupParliament } from "$lib/parliament";
	import type { Delegate } from "$lib/types"
    import { get, type Readable } from 'svelte/store';
    import { localStorageStore } from '@skeletonlabs/skeleton';
	import { partyToColorFn } from "$lib/getPartyToColor";
    import DelegateCard from "./DelegateCard.svelte";

    export let seats: number[];
    export let dels: Delegate[];
        
    const width = 830;
    const height = 900;

    let selected: Bubble;
    let circles2d: Bubble[][] = setupParliament(seats, width, height, 6.9);

    function fromLocalStorage() {
        const currentDelegateStorage: Readable<Bubble | null> = localStorageStore('currentDelegate', null);
        const bubble = get(currentDelegateStorage);

        if (bubble == null 
            || bubble.del == null 
            || bubble.del.seat_row == null 
            || bubble.del.seat_col == null) return;

        select(circles2d[bubble.del.seat_row-1][bubble.del.seat_col-1], null);
    }

    function select(bubble: Bubble, event: MouseEvent | KeyboardEvent | null) {
        if (bubble.del == null) return;

        if (selected != null) {
            selected.r = 6;
        }
        bubble.r = +10.9;
        circles2d = circles2d; // TODO: remove this evil pointer hack without breaking everything
        
        if (event != null) {
            event.stopPropagation();
        }

        selected = bubble;
	}

    dels.forEach((del) => {
        setDelOnBubble(del, circles2d, partyToColorFn);
    });

    fromLocalStorage();
</script>

<div class="flex">
    <div class="self-center m-auto">
        {#if selected && selected.del}
            <DelegateCard name={selected.del.name} party={selected.del.party} image={selected.del.image_url} />  
        {/if}
    </div>
    <div class="self-center m-auto w-8/12">
        <svg viewBox="0 0 {width} {height * 0.5+60}">
            {#each circles2d.flat(1) as circle, i}
                <circle 
                    type="button" 
                    cx={circle.x} 
                    cy={circle.y} 
                    r={circle.r}
                    fill={circle.color}
                    fill-opacity={circle.color == "rgb(196, 180, 189)" && circle.del == null ? 0.2 : 1}
                    on:click={event => select(circle, event)}
                    on:keypress={event => select(circle, event)}
                    role="button"
                    tabindex={100+i}
                />
            {/each}
        </svg>
    </div>
</div>

<style>
* {
  box-sizing: border-box;
}
</style>