<script lang="ts">
	let clazz = '';
	export { clazz as class };
</script>

<div class="badge {clazz} bg-primary-600 dark:bg-primary-800 text-white dark:text-white">
	<slot />
</div>

<style>
</style>
