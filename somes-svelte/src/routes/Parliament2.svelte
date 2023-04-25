<script lang="ts">
	import { setDelOnBubble, type Bubble, generateHalfCircle, setupParliament, partyToColor } from "$lib/parliament";
	import type { Delegate } from "$lib/types"
	import DelegateCard from "./DelegateCard.svelte";
    import { get, type Readable } from 'svelte/store';
    import { localStorageStore } from '@skeletonlabs/skeleton';

    export let seats: number[];
    export let dels: Delegate[];
        
    const width = 830;
    const height = 900;

    let selected: Bubble;

    function fromLocalStorage() {
        const currentDelegateStorage: Readable<Bubble | null> = localStorageStore('currentDelegate', null);
        const bubble = get(currentDelegateStorage);
        if (bubble == null) {
            return;
        }
        if (bubble.del == null) {
            return;
        }
        if (bubble.del.seat_row == null || bubble.del.seat_col == null) {
            return;
        }
        select(circles2d[bubble.del.seat_row-1][bubble.del.seat_col-1], null);
    }

    function select(bubble: Bubble, event: MouseEvent | null) {
        if (bubble.del == null) {
            return;
        }
        if (selected != null) {
            selected.r = 6;
        }
        bubble.r = +10.9;
        circles2d = circles2d;
        if (event != null) {
            event.stopPropagation();
        }
        selected = bubble;
        console.log(selected);
	}

    let circles2d: Bubble[][] = setupParliament(seats, width, height, 6.9);

    dels.forEach((del) => {
        setDelOnBubble(del, circles2d, partyToColor);
    });

    fromLocalStorage();


</script>

<div class="flex flex-wrap-reverse">
    <div class="h-screen w-60 bg-slate-500">
        {#if selected && selected.del}
            <div class="card">
                <header>
                    <img src={selected.del.image_url} class="bg-black/50  aspect-[10/9]" alt="image of politician {selected.del.name}">
                </header>
                <section class="p-4"><h4>{selected.del.name}</h4><span>{selected.del.party}</span></section>
            </div>
            <!-- <DelegateCard name={selected.del.name} party={selected.del.party} image={selected.del.image_url} /> -->
        {/if}
    </div>
    <div class="self-center">
        <!-- <svg {width} height={height * 0.5 +10}> -->
        <svg viewBox="0 0 {width} {height * 0.5+10}" style="width: 54rem; max-width: 100%;">
            {#each circles2d.flat(1) as circle}
                <div class="box" >B</div>
                <circle type="button" cx={circle.x} cy={circle.y} r={circle.r}
                    on:click="{event => select(circle, event)}"
                    fill={circle.color}
                    fill-opacity={circle.color == "rgb(196, 180, 189)" && circle.del == null ? 0.2 : 1}
                    transform=""
                />
                <div class="overlay">Overlay</div>
            {/each}
        </svg>
    </div>
</div>
<!-- <div class="w-screen h-screen bg-slate-500"></div> -->
<!-- <div> -->
    <!--<div class="container">
        <div class="box">HALLO</div>
        <div class="overlay">OVERLAY</div>
      </div>-->
    
    <!-- <svg {width} {height} style="margin: auto; float: right;"> -->
    
    
<!-- </div> -->

<style>

.box {
  position: absolute;
  background: #0057e3;
}
.overlay {
  position: absolute;
  top: 0;
  left: 0;
  margin: 30px;
  background: #009938;
  z-index: 9;
  visibility: hidden;
}
.box:hover + .overlay {
  visibility: visible;
}

* {
  box-sizing: border-box;
}

</style>