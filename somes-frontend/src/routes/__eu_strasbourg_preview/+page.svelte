<script lang="ts">
	import { getWithRoute } from "$lib/api/api";
	import { cachedDelegates } from "$lib/caching/delegates.svelte";
	import BaseParliament from "$lib/components/Parliaments/BaseParliament.svelte";
	import { generateHalfCircle, setupParliament, type Bubble } from "$lib/parliament";
	import { partyToColor } from "$lib/partyColor";
	import type { Delegate } from "$lib/types";
	import { onMount } from "svelte";

	let delegates: Delegate[] = $state([]);
	let seatsCount: number[] = $state([]);
	let seats: any[] = $state([]);

    onMount(async () => {
        delegates = await cachedDelegates(true) ?? [];
        const del = delegates.find(delegate => {
          if (delegate.name.includes("AUBRY"))
          return delegate
        });
        // console.log(delegates);
        console.log(del);
        const seatMapping = await (await fetch(`http://localhost:5173/seat_to_circle.json`, {
          method: 'GET',
          headers: {
            'Content-Type': 'application/json',
            'Accept-Encoding': 'gzip'
          }
        })).json();

        seatsCount = seatMapping.circles.map((circle) => {
            return circle.slots_including_gaps as number
        });

        seats = seatMapping.seats;
    })

    const width = 830;
	const height = 1030;
	const r = 6;

	const viewPortWidth = 1800;
	const viewPortHeight = 1250;
	let id = 0;
	// [2, (2 skip), 17,  (2 skip), 2]
	// [2, (2 skip), 3, (2 skip), 4, (2 skip)]
	let circles2d: Bubble[][] = $derived.by(() => {
	const innerCircles2d: Bubble[][] = [];
  	seatsCount.forEach((seat, idx) => {
  		innerCircles2d.push(
  			generateHalfCircle(
  				seat,
  				70 + idx * 10,
  				width,
  				height,
  260
  			).map((circle) => {
  				id += 1;
  				return {
  					r,
  					x: circle.x,
  					y: circle.y - 95,
  					angle_rad: circle.angle_rad,
  					del: null,
  					color: 'rgb(196, 180, 189)',
  					opacity: 0,
  					title: null,
  					namedVote: null,
  					speech: null,
  					texture: null,
  					material: null,
  					id
  				};
  			})
  		);
  	});

   delegates.forEach(delegate => {
     if (delegate.seat_row && seats.length > 0) {
       const seat = seats[delegate.seat_row - 1]

       if (delegate.seat_row == 533) {
         console.log($state.snapshot(seat))
         innerCircles2d[seat.circle - 1][seat.slot_in_circle_with_gaps - 1].color = "rgb(255, 0, 0)";
       }
       innerCircles2d[seat.circle - 1][seat.slot_in_circle_with_gaps - 1].opacity = 1;
       innerCircles2d[seat.circle - 1][seat.slot_in_circle_with_gaps - 1].color = partyToColor(delegate.party);
     }
   })
   return innerCircles2d;

	});
</script>

<BaseParliament {circles2d} select={() => {}}  width={viewPortWidth} height={viewPortHeight} />
