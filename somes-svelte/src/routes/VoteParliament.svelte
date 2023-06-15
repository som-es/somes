<script lang="ts">
	import { setupParliament, type Bubble, setDelOnBubble } from "$lib/parliament";
	import type { Delegate, VoteResult } from "$lib/types";
	import { localStorageStore } from "@skeletonlabs/skeleton";

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

    let otherPartyToColor = new Map<string, {color: string}[]>();
    otherPartyToColor.set("SPÖ", [{color:"#f48992"}, {color:"#E31E2D"}]);
    otherPartyToColor.set("ÖVP", [{color:"#567a7f"}, {color:"#62C3D0"}]);
    otherPartyToColor.set("FPÖ", [{color:"#354776"}, {color:"#0052FB"}]);
    otherPartyToColor.set("GRÜNE", [{color:"#537239"}, {color:"#69B12E"}]);
    otherPartyToColor.set("NEOS", [{color:"#934b6c"}, {color:"#E3257B"}]);


    let partyToColorMap = new Map<string, {color: string, infavor: boolean}>();
    // better
    // partyToColorMap.set("SPÖ", otherPartyToColor.get("SPÖ")[isPartyInFavor("SPÖ") ? 0 : 1]);

    partyToColorMap.set("SPÖ", isPartyInFavor("SPÖ") ?     {color:"#E31E2D", infavor:true} : {color:"#f48992", infavor:false});
    partyToColorMap.set("ÖVP", isPartyInFavor("ÖVP") ?     {color:"#62C3D0", infavor:true} : {color:"#567a7f", infavor:false});
    partyToColorMap.set("FPÖ", isPartyInFavor("FPÖ") ?     {color:"#0052FB", infavor:true} : {color:"#354776", infavor:false});
    partyToColorMap.set("GRÜNE", isPartyInFavor("GRÜNE") ? {color:"#69B12E", infavor:true} : {color:"#537239", infavor:false});
    partyToColorMap.set("NEOS", isPartyInFavor("NEOS") ?   {color:"#E3257B", infavor:true} : {color:"#934b6c", infavor:false});
    
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

    function partyToColor(party: string): string { 
        if (partyToColorMap.has(party)) {
            return partyToColorMap.get(party)?.color as string;
        }
        return "rgb(196, 180, 189)";
    }

    function setOpacity(bubble: Bubble) {
        if (bubble.del == null) {
            bubble.opacity = 0.2;
            return;
        }

        if (partyToColorMap.has(bubble.del.party)) {
            bubble.opacity = partyToColorMap.get(bubble.del.party)?.infavor as boolean ? 1 : 0.2;
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

        const biColors = otherPartyToColor.get(del.party);

        if (biColors == null) {
            return;
        }

        // select delegates.name,delegates.party,speeches.infavor from speeches inner join delegates on speeches.delegate_id=delegates.id where legislative_initiatives_id=14;


        // if speaker is against partei linie
        circles2d[del.seat_row-1][del.seat_col-1].color = biColors[speech.infavor ? 1 : 0].color;

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