<script lang="ts">
	let clazz = '';
	export { clazz as class };
</script>

<div class="container mx-auto mt-2 {clazz}">
	<div class="px-2 md:px-8 lg:px-9 pt-3">
		<slot />
	</div>
</div>
