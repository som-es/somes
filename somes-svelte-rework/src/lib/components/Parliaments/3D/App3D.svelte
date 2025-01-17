<script lang="ts">
	import type { Bubble } from "$lib/parliament";
	import { Canvas } from "@threlte/core";
	import BaseParliament3D from "./BaseParliament3D.svelte";

    export let circles2d: Bubble[][];
	export let selected: Bubble | null;
	export let preview: boolean = false;
	export let select: (bubble: Bubble, event: MouseEvent | KeyboardEvent | null) => void;


	const enterFullscreen = () => {
		const canvasElement = document.getElementById("canvas");
		  if (!document.fullscreenElement && canvasElement) {
			canvasElement.requestFullscreen().catch(err => {
				alert(`Error attempting to enable full-screen mode: ${err.message} (${err.name})`);
			});
		} else {
			document.exitFullscreen();
		}
	}

</script>

<div id="canvas" class="min-h-[30rem] max-h-[30rem">
	<Canvas>
		<BaseParliament3D {circles2d} {selected} {preview} {select} />
	</Canvas>
</div>

<button on:click={enterFullscreen} class="js-toggle-fullscreen-btn toggle-fullscreen-btn float-right" aria-label="Enter fullscreen mode" >
	<svg class="toggle-fullscreen-svg" width="28" height="28" viewBox="-2 -2 28 28">
		<g class="icon-fullscreen-enter">
			<path d="M 2 9 v -7 h 7" />
			<path d="M 22 9 v -7 h -7" />
			<path d="M 22 15 v 7 h -7" />
			<path d="M 2 15 v 7 h 7" />
		</g>
		
	</svg>
</button>

<style>
	.toggle-fullscreen-btn {
		border: 0;
		padding: 0;
		background: none;
		cursor: pointer;
		outline: none;
	}


	.toggle-fullscreen-svg {
		display: block;
		height: auto;
	}

	.toggle-fullscreen-svg path {
		transform-box: view-box;
		transform-origin: 12px 12px;
		fill: none;
		stroke: hsl(225, 10%, 8%);
		stroke-width: 4;
	}

</style>