<script lang="ts">
	import { setupParliament, type Bubble, setDelOnBubble } from "$lib/parliament";
	import type { Delegate } from "$lib/types";

    export let seats: number[];
    export let dels: Delegate[];

    const width = 830;
    const height = 900;
    
    let circles2d: Bubble[][] = setupParliament(seats, width, height, 6.9);

    dels.forEach((del) => {
        setDelOnBubble(del, circles2d);
    });

</script>


<svg viewBox="0 0 {width} {height * 0.5+10}" style="width: 500px; max-width: 100%;">
    {#each circles2d.flat(1) as circle}
        <div class="box" >B</div>
        <circle type="button" cx={circle.x} cy={circle.y} r={circle.r}
            fill={circle.color}
            fill-opacity={circle.color == "rgb(196, 180, 189)" && circle.del == null ? 0.2 : 1}
            transform=""
        />
        <div class="overlay">Overlay</div>
    {/each}
</svg>