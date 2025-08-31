<script lang="ts">
	let clazz = '';
	export { clazz as class };
</script>

<div class="container mx-auto mt-0 {clazz}">
	<div class="mx-0 max-md:mx-3">
		<slot />
	</div>
</div>
