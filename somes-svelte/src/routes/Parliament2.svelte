<script lang="ts">
	import { setDelOnBubble, type Bubble, generateHalfCircle, setupParliament } from "$lib/parliament";
	import type { Delegate } from "$lib/types"
	import DelegateCard from "./DelegateCard.svelte";

    export let seats: number[];
    export let dels: Delegate[];
        
    const width = 830;
    const height = 900;

    let selected: Bubble;


    function select(bubble: Bubble, event: MouseEvent) {
        if (bubble.del == null) {
            return;
        }
        if (selected != null) {
            selected.r = 6;
        }
        bubble.r = +10.9;
        circles = circles;
        event.stopPropagation();
        selected = bubble;
        console.log(selected);
	}

    let circles2d: Bubble[][] = setupParliament(seats, width, height, 6.9);

    dels.forEach((del) => {
        setDelOnBubble(del, circles2d);
    });
    
    let circles: Bubble[] = circles2d.flat(1);

    selected = circles[0];
</script>

<div class="container">
    <div class="element h-screen bg-slate-500" style="width: 30%;">
        {#if selected && selected.del}
            <div class="card">
                <header>
                    <img src={selected.del.image_url} class="bg-black/50 w-full aspect-[10/9]" alt="image of politician {selected.del.name}">
                </header>
                <section class="p-4"><h4>{selected.del.name}</h4><span>{selected.del.party}</span></section>
            </div>
            <!-- <DelegateCard name={selected.del.name} party={selected.del.party} image={selected.del.image_url} /> -->
        {/if}
    </div>
    <svg class="element" {height} width="70%"> 
        {#each circles as circle}
            <div class="box" >B</div>
            <circle type="button" cx={circle.x} cy={circle.y} r={circle.r}
                on:click="{event => select(circle, event)}"
                fill={circle.color}
                fill-opacity={circle.color == "rgb(196, 180, 189)" && circle.del == null ? 0.2 : 1}
            />
            <div class="overlay">Overlay</div>
        {/each}
        
    </svg>
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
.container {
  position: relative;
}
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

.container {
  background-color: red;
  position: relative;
  width: 80%;
  padding-top: 80%; /* 1:1 Aspect Ratio */
}

/* Create two equal columns that floats next to each other */
.column {
  float: left;
  width: 75%;
  padding: 10px;
}

.column-less {
  float: left;
  width: 25%;
  padding: 10px;

}

/* Clear floats after the columns */
.row:after {
  content: "";
  display: table;
  clear: both;
}

.element {
  position:  absolute;
  top: 0;
  left: 0;
  bottom: 0;
  right: 0;
}
</style>