<script lang="ts">
	import { getModalStore } from '@skeletonlabs/skeleton';
	
    const modalStore = getModalStore();
</script>
<div class="card p-8 max-w-7xl">
	<button
		on:click={() => {
			modalStore.close();
		}}
		style="font-size: 34px"
		class="w-5 unselectable float-right">&#x2715</button
	>
</div>

