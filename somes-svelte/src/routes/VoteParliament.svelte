<script lang="ts">
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

    let partyToColorMap = new Map<string, string>();
    partyToColorMap.set("SPÖ", isPartyInFavor("SPÖ") ? "#E31E2D" : "#834249");
    partyToColorMap.set("ÖVP", isPartyInFavor("ÖVP") ? "#62C3D0" : "#567a7f");
    partyToColorMap.set("FPÖ", isPartyInFavor("FPÖ") ? "#0052FB" : "#354776");
    partyToColorMap.set("GRÜNE", isPartyInFavor("GRÜNE") ? "#69B12E" : "#537239");
    partyToColorMap.set("NEOS", isPartyInFavor("NEOS") ? "#E3257B" : "#934b6c");
    

    let circles2d: Bubble[][] = setupParliament(seats, width, height, 6.9);

    function partyToColor(party: string): string { 
        if (partyToColorMap.has(party)) {
            return partyToColorMap.get(party) as string;
        }
        return "rgb(196, 180, 189)";
        
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


<svg viewBox="0 0 {width} {height * 0.5+60}" style="width: 500px; max-width: 100%;">
    {#each circles2d.flat(1) as circle}
        <div class="box">B</div>
        <circle class="translated-circle" style="{getTransform(circle)}" type="button" cx={circle.x} cy={circle.y} r={circle.r}
            fill={circle.color}
            fill-opacity={circle.color == "rgb(196, 180, 189)" && circle.del == null ? 0.2 : 1}
        />
        <div class="overlay">Overlay</div>
    {/each}
</svg>

<style>
.translated-circle {
  /* transform: translateZ(50px);  move closer to viewer */
  /* transform: rotate3d(0, 2, 1, 31deg); */
}

</style>