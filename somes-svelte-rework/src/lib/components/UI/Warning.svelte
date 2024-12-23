<script lang="ts">
	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();

	function handleClick(event: any) {
		dispatch('click', event);
	}

	let clazz = '';
	export { clazz as class };
</script>

<button on:click={handleClick} style="font-size: 34px" class="w-5 unselectable {clazz}">
	⚠
	<!-- &#x2715 -->
</button>
