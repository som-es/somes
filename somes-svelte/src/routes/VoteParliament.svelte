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


    let partyToColorMap = new Map<string, {color: string, infavor: boolean}>();
    partyToColorMap.set("SPÖ", isPartyInFavor("SPÖ") ?     {color:"#E31E2D", infavor:true} : {color:"#f48992", infavor:false});
    partyToColorMap.set("ÖVP", isPartyInFavor("ÖVP") ?     {color:"#62C3D0", infavor:true} : {color:"#567a7f", infavor:false});
    partyToColorMap.set("FPÖ", isPartyInFavor("FPÖ") ?     {color:"#0052FB", infavor:true} : {color:"#354776", infavor:false});
    partyToColorMap.set("GRÜNE", isPartyInFavor("GRÜNE") ? {color:"#69B12E", infavor:true} : {color:"#537239", infavor:false});
    partyToColorMap.set("NEOS", isPartyInFavor("NEOS") ?   {color:"#E3257B", infavor:true} : {color:"#934b6c", infavor:false});
    

    let circles2d: Bubble[][] = setupParliament(seats, width, height, 6.9);

    function partyToColor(party: string): string { 
        if (partyToColorMap.has(party)) {
            return partyToColorMap.get(party)?.color as string;
        }
        return "rgb(196, 180, 189)";
    }

    function opacity(bubble: Bubble): number {
        if (bubble.del == null) {
            return 0.2;
        }
        if (partyToColorMap.has(bubble.del.party)) {
            return partyToColorMap.get(bubble.del.party)?.infavor as boolean ? 1 : 0.2;
        }
        return 1;

    }

    dels.forEach((del) => {
        setDelOnBubble(del, circles2d, partyToColor);
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
    <svg viewBox="0 0 {width} {height * 0.5+60}" style="width: 500px; max-width: 100%;">
        {#each circles2d.flat(1) as circle}
            <div class="box">B</div>
            <circle class="translated-circle" style="{getTransform(circle)}" type="button" cx={circle.x} cy={circle.y} r={circle.r}
                fill={circle.color}
                fill-opacity={opacity(circle)}
            />
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