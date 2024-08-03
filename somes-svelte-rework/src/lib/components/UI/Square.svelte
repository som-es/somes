<script lang="ts">
	let clazz = "";
	export { clazz as class };
</script>

<div class="{clazz} rounded-xl square bg-primary-300">
    <div class="flex flex-col items-center justify-center">
        <slot />
    </div>
</div>

<style>
    .square {
		aspect-ratio: 1/ 1;
        min-width: 140px;
        min-height: 140px;
        max-height: 140px;
        max-width: 140px;
        height: 140px;
        width: 140px;
        display: flex;
        justify-content: center;
        align-content: center;
	}
</style>
