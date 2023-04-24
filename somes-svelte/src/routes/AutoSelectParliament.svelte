<script lang="ts">
    import { setDelOnBubble, type Bubble, setupParliament } from "$lib/parliament";
	import type { Delegate } from "$lib/types";

    export let seats: number[];
    export let dels: Delegate[];

    const width = 830;
    const height = 900;
    
    let circles2d: Bubble[][] = setupParliament(seats, width, height, 6.9);

    dels.forEach((del) => {
        setDelOnBubble(del, circles2d);
    });

    let circles: Bubble[] = circles2d.flat(1);

    let possibleSelection = circles.filter((circle) => circle.del != null);
    
    
    
    let selectionSet: Set<Bubble> = new Set(possibleSelection);
    const idx = Math.floor(Math.random() * selectionSet.size);
    selectionSet.keys()[idx];
    const selection = possibleSelection[idx];

    selectionSet.delete(selection);
    
</script>

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