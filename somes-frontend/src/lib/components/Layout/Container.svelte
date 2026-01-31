<script lang="ts">
	let clazz = '';
	export { clazz as class };
</script>

<div class="container mx-auto md:mt-2 {clazz}">
	<div class="px-3 md:px-8 lg:px-9 md:pt-3">
		<slot />
	</div>
</div>
