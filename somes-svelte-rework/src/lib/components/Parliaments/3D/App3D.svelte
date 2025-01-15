<script lang="ts">
	import type { Bubble } from "$lib/parliament";
	import { Canvas } from "@threlte/core";
	import BaseParliament3D from "./BaseParliament3D.svelte";

    export let circles2d: Bubble[][];
	export let selected: Bubble | null;
	export let preview: boolean = false;
	export let select: (bubble: Bubble, event: MouseEvent | KeyboardEvent | null) => void;


</script>
<div class="min-h-[30rem] max-h-[30rem">

	<Canvas>
		<BaseParliament3D {circles2d} {selected} {preview} {select} />
	</Canvas>
</div>