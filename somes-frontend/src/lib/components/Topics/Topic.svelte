<script lang="ts">
	let clazz = '';
	export { clazz as class };
</script>

<div class="badge {clazz} bg-primary-600 text-white dark:bg-primary-800 dark:text-white">
	<slot />
</div>

<style>
</style>
