<script lang="ts">
	import type { Delegate } from "$lib/types";

    export let seats: number[];
    export let dels: Delegate[];
        
    const width = 830;
    const height = 900;

    function generateHalfCircle(n: number, r: number) {
        let smaller_n = n - 2;

        let scaled_max_angle = 18000.0;
        let modulo = scaled_max_angle / smaller_n;
        let count_to = scaled_max_angle + modulo + 1;

        let normalize = 100. * count_to / scaled_max_angle;
        let circles: { x: number, y: number }[] = [];

        for (let angle_deg = 0; angle_deg < count_to; angle_deg += modulo) {
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
    let circles2d: { x: number, y: number, party: string | null, color: string | null }[][] = [];
    seats.forEach((seat, idx) => {
        circles2d.push(generateHalfCircle(seat, 70 + idx * (idx == 1 ? 30 : 20) + (idx >= 2 ? 30: 0)).map((circle) => {
            return {
                x: circle.x,
                y: circle.y,
                party: null,
                color: "rgb(196, 180, 189)"
            }
        }));
        // circles = circles.concat(generateHalfCircle(seat, 70 + idx * (idx == 1 ? 30 : 20) + (idx >= 2 ? 30: 0)));
    });

    dels.forEach((del, idx) => {
        if (del.seat_row == null || del.seat_col == null) {
            return
        }
        console.log(del);
        circles2d[del.seat_row-1][del.seat_col-1].party = del.party;
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
    
    let circles: { x: number, y: number, party: string | null, color: string | null}[]= circles2d.flat(1);
</script>

<svg {width} {height} style="margin: auto;">
    {#each circles as circle}

        <circle cx={circle.x} cy={circle.y} r=6
            fill={circle.color}
        />
    {/each}
</svg>