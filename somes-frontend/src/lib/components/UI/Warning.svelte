<script lang="ts">
	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();

	function handleClick(event: any) {
		dispatch('click', event);
	}

	let clazz = '';
	export { clazz as class };
</script>

<button on:click={handleClick} style="font-size: 34px" class="unselectable w-5 {clazz}">
	⚠
	<!-- &#x2715 -->
</button>
