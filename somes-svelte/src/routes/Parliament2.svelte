<script lang="ts">
	import type { Delegate } from "$lib/types";
	import { SvelteComponentDev } from "svelte/internal";
	import DelegateCard from "./DelegateCard.svelte";

    export let seats: number[];
    export let dels: Delegate[];
        
    const width = 830;
    const height = 900;

    let selected: Bubble;

    interface Bubble {
        r: number,
        x: number;
        y: number;
        del: Delegate | null;
        color: string | null;
    }

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

    function generateHalfCircle(n: number, r: number) {
        let smaller_n = n - 2;

        let scaled_max_angle = 18000.0;
        let modulo = scaled_max_angle / smaller_n;
        let count_to = scaled_max_angle + modulo + 1;

        let normalize = 100. * count_to / scaled_max_angle;
        let circles: { x: number, y: number }[] = [];

        for (let angle_deg = count_to; angle_deg > 0; angle_deg -= modulo) {
            let angle_rad = -(angle_deg / normalize) * Math.PI / 180.;
            
            const x = 2.0 * r * Math.cos(angle_rad) + width / 2;
            const y = 2.0 * r * Math.sin(angle_rad) + height / 2;

            const circle = {
                x, y
            };
            circles = circles.concat(circle);
            // circles.push(circle);
        }
        return circles
    }
    let circles2d: Bubble[][] = [];
    seats.forEach((seat, idx) => {
        circles2d.push(generateHalfCircle(seat, 70 + idx * (idx == 1 ? 30 : 20) + (idx >= 2 ? 30: 0)).map((circle) => {
            return {
                r: 6.9,
                x: circle.x,
                y: circle.y,
                del: null,
                color: "rgb(196, 180, 189)"
            }
        }));
        // circles = circles.concat(generateHalfCircle(seat, 70 + idx * (idx == 1 ? 30 : 20) + (idx >= 2 ? 30: 0)));
    });

    dels.forEach((del) => {
        if (del.seat_row == null || del.seat_col == null) {
            return
        }
        circles2d[del.seat_row-1][del.seat_col-1].del = del;
        console.log(del);
        
        switch (del.party) {
            case "SPÖ":
                circles2d[del.seat_row-1][del.seat_col-1].color = "#E31E2D";
                break;
            case "ÖVP":
                circles2d[del.seat_row-1][del.seat_col-1].color = "#62C3D0";
                break;
            case "FPÖ":
                circles2d[del.seat_row-1][del.seat_col-1].color = "#0052FB";
                break;
            case "GRÜNE":
                circles2d[del.seat_row-1][del.seat_col-1].color = "#69B12E";
                break;
            case "NEOS":
                circles2d[del.seat_row-1][del.seat_col-1].color = "#E3257B";
                break;
            default:
                circles2d[del.seat_row-1][del.seat_col-1].color = "rgb(196, 180, 189)";
                break;
        }
    });
    
    let circles: Bubble[]= circles2d.flat(1);
</script>

<!--<div>
    {#if selected && selected.del}
    
        <DelegateCard name={selected.del.name} party={selected.del.party} image={selected.del.image_url} />
    {/if}

</div>-->
<!-- <div class="w-screen h-screen bg-slate-500"></div> -->
<!-- <div> -->
    <!--<div class="container">
        <div class="box">HALLO</div>
        <div class="overlay">OVERLAY</div>
      </div>-->
    
    <!-- <svg {width} {height} style="margin: auto; float: right;"> -->
    <svg height="100%" width="100%"> 
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
</style>