<script lang="ts">
	let clazz = '';
	export { clazz as class };
</script>

<div class="badge h-8 {clazz} bg-primary-600 text-white dark:text-white">
	<slot />
</div>

<style>
</style>
