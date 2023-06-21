<script lang="ts">
	import { getPartyToColor } from "$lib/getPartyToColor";
	import { setupParliament, type Bubble, setDelOnBubble } from "$lib/parliament";
	import type { Delegate, VoteResult } from "$lib/types";

    export let seats: number[];
    export let dels: Delegate[];
    export let voteResult: VoteResult;

    const width = 830;
    const height = 900;

    function isPartyInFavor(party: string): boolean {
        return voteResult.votes.find(vote => vote.party === party)?.infavor ?? false;
    }

    function findDelegateFromId(id: number): Delegate | undefined {
        return dels.find(del => del.id === id);
    }
 
    let circles2d: Bubble[][] = setupParliament(seats, width, height, 6.9);
    let selected: Bubble;

    function select(bubble: Bubble, event: MouseEvent | null) {
        circles2d = circles2d;
        if (event != null) {
            event.stopPropagation();
        }
        selected = bubble;
        console.log(selected);
	}

    const partyToColorMap = getPartyToColor();

    function partyToColor(party: string): string {
        const color = partyToColorMap.get(party)
        if (color == null) {
            return "#B8B8B8";
        }
        return color;
    }

    let partyInfavorMap = new Map<string, boolean>();
    partyToColorMap.forEach((_v, party) => {
        partyInfavorMap.set(party, isPartyInFavor(party));
    });

    function setOpacity(bubble: Bubble) {
        if (bubble.del == null) {
            bubble.opacity = 0.2;
            return;
        }

        if (partyInfavorMap.has(bubble.del.party)) {
            bubble.opacity = partyInfavorMap.get(bubble.del.party) ? 1 : 0.16;
            return
        }
        bubble.opacity = 1;
    }

    dels.forEach((del) => {
        setDelOnBubble(del, circles2d, partyToColor);

        if (del.seat_col != null && del.seat_row != null) {
            setOpacity(circles2d[del.seat_row-1][del.seat_col-1]);
        }

    });

    voteResult.speeches.forEach((speech) => {
        let del = findDelegateFromId(speech.delegate_id);
        if (del == null) {
            return;
        }

        if (del.seat_col == null || del.seat_row == null) {
            return;
        }

        if (speech.infavor) {
            circles2d[del.seat_row-1][del.seat_col-1].opacity = 1.0;            
        } else {
            circles2d[del.seat_row-1][del.seat_col-1].opacity = 0.2;
        }

        circles2d[del.seat_row-1][del.seat_col-1].r = +10.9;
    });

    function getTransform(bubble: Bubble): string {
        if (bubble.del == null) {
            return "";
        }

        if (isPartyInFavor(bubble.del.party)) {
            // return "rotate3d(0, 2, 1, 10deg)";

            // decrease r for circle creation?
            return "transform: skewY(3.3deg);";
        }
        return "";
    }
</script>

<div>
    <svg viewBox="0 0 {width} {height * 0.5+60}" style="max-width: 100%;">
        {#each circles2d.flat(1) as circle}
            {#if circle == selected}
                <circle class="translated-circle" style="{getTransform(circle)}" type="button" cx={circle.x} cy={circle.y} r={circle.r}
                    on:click={event => select(circle, event)}
                    fill={circle.color}
                    fill-opacity={circle.opacity}
                    stroke="orange"
                    stroke-width="4"
                />
            {:else}
                <circle class="translated-circle" style="{getTransform(circle)}" type="button" cx={circle.x} cy={circle.y} r={circle.r}
                    on:click={event => select(circle, event)}
                    fill={circle.color}
                    fill-opacity={circle.opacity}
                />
            {/if}
            <div class="box">B</div>
            
            <div class="overlay">Overlay</div>
        {/each}
    </svg>
</div>

<style>
.translated-circle {
  /* transform: translateZ(50px);  move closer to viewer */
  /* transform: rotate3d(0, 2, 1, 31deg); */
}

</style>