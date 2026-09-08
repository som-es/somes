<script lang="ts">
	let clazz = '';
	export { clazz as class };
</script>

<div class="container mx-auto md:mt-2 {clazz}">
	<div class="px-3 md:px-8 md:pt-3 lg:px-9">
		<slot />
	</div>
</div>
