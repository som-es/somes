<script lang="ts">
    export let n: number;
    export let r: number;

    let smaller_n = n - 2;

    let scaled_max_angle = 18000.0;
    let modulo = scaled_max_angle / smaller_n;
    let count_to = scaled_max_angle + modulo + 1;

    let normalize = 100. * count_to / scaled_max_angle;
    let circles: { x: number, y: number }[] = [];

    for (let angle_deg = 0; angle_deg < count_to; angle_deg += modulo) {
        let angle_rad = -(angle_deg / normalize) * Math.PI / 180.;
        
        const x = 2.0* r * Math.cos(angle_rad) + 700;
        const y = 2.0*r * Math.sin(angle_rad) + 700;

        const circle = {
            x, y
        };
        circles = circles.concat(circle);
        // circles.push(circle);
    }
</script>

<div>
    <!--<svg>
        {#each circles as circle}
            <circle cx={circle.x} cy={circle.y} r=10
                fill="rgb(196, 180, 189)"
            />
        {/each}
    </svg>-->
    {#each circles as circle}
        <div class="circle" style="left: {circle.x}px; top: {circle.y}px;">

        </div>
    {/each}
</div>

<style>
svg {
    width: auto;
    height: auto;
}

.circle {
  position: absolute;
  width: 10px;
  height: 10px;
  background-color: rgb(196, 180, 189);
  border-radius: 50%;
}
</style>