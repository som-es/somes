<script lang="ts">
	import { getModalStore } from "@skeletonlabs/skeleton";

	const modalStore = getModalStore();

</script>


{#if $modalStore.length > 0 && $modalStore[0].meta }
    <div class="card p-8 max-w-7xl w-[80rem]">
        HIHIHI
    </div>
{/if}