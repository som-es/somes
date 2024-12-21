<script lang="ts">

	export let squareSize = "140px";
	export let flexDirection = "flex-col";

	$: squareSize;
	console.log(squareSize);

	let clazz = '';
	export { clazz as class };
</script>

<div class="{clazz} rounded-xl square min-w-[{squareSize}] min-h-[{squareSize}] max-w-[{squareSize}] max-h-[{squareSize}] h-[{squareSize}] w-[{squareSize}] bg-primary-300 dark:bg-primary-500">
	<div class="flex {flexDirection} items-center justify-center">
		<slot />
	</div>
</div>

<style>
	.square {
		aspect-ratio: 1/ 1;
		display: flex;
		justify-content: center;
		align-content: center;
	}
</style>
