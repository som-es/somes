<script lang="ts">
	import type { VoteResult } from "$lib/types";

    export let voteResult: VoteResult;

    let badgeText = ""
    $: {
        if (voteResult.legislative_initiative.voting) {
            switch (voteResult.legislative_initiative.voting) {
                case "Law":
                    badgeText = "Gesetz"
                    break;
                case "Amendment":
                    badgeText = "Abänderung"
                    break;
                case "Resolution":
                    badgeText = "Entschließung"
                    break;
                case "Report":
                    badgeText = "Bericht"
                    break;
                default:
                    badgeText = ""
            }
        } else {
            badgeText = "Andere"
        }
    }
</script>
<div class="badge bg-tertiary-400 text-black">{badgeText}</div>
