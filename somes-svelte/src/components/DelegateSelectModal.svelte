<script lang="ts">
    import DelegateSelect from "@/components/DelegateSelect.svelte";
	import DelegateCard from "./DelegateCard.svelte";
	import type { Delegate } from "$lib/types";

    export let delegate: Delegate | undefined;
</script>

<div class="card relative flex flex-row gap-[3vw] p-8 w-2/3">
    <DelegateCard {delegate} />
    <DelegateSelect bind:selectedDel={delegate} />
    <div class="absolute bottom-0 right-0 m-8">
        <button type="button" class="btn btn-lg variant-filled">
            Select Delegate
        </button>
    </div>
</div>