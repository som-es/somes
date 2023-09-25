<script lang="ts">
    import DelegateSelect from "@/components/DelegateSelect.svelte";
	import DelegateCard from "./DelegateCard.svelte";
	import type { Delegate } from "$lib/types";
	import { modalDelegateStore } from "../stores/stores";
	import { modalStore } from "@skeletonlabs/skeleton";

    export let delegate: Delegate | undefined;

    function close() {
        modalDelegateStore.set(delegate);
        modalStore.close();
    }
</script>

<div class="card relative flex flex-row gap-[3vw] p-8 w-2/3">
    <DelegateCard {delegate} />
    <DelegateSelect bind:selectedDel={delegate} />
    <div class="absolute bottom-0 right-0 m-8">
        <button type="button" class="btn btn-lg variant-filled" on:click={close} disabled={!delegate}>
            Select Delegate
        </button>
    </div>
</div>