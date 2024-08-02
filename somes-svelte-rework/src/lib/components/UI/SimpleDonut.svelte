<script lang="ts">
    import { afterUpdate } from 'svelte';

	import type { ConicStop } from "@skeletonlabs/skeleton";

    export let stops: ConicStop[] = [{ color: "red", start: 0, end: 360 }];

    function setColorValue(color: any): string {
        return color
	}
    let cone: string;

	function genConicGradient(): void {
		let d: any = stops.map((v) => `${setColorValue(v.color)} ${v.start}deg ${v.end}deg`);
		cone = `conic-gradient(${d.join(', ')})`;
	}
    afterUpdate(() => {
		genConicGradient();
	});
</script>

{#if cone}
    <div class="donut" style:background={cone}></div>
{/if}

<style>
    .donut::before {
        content: "";
        width: 40px; height: 40px;
        border-radius: 50%;
        /* background: rgb(var(--bg-primary-300)); */
        background: rgb(var(--color-primary-300));
        z-index: 10;
    }
    .donut {

        width: 60px; height: 60px;
        border-radius: 50%;

        display: flex;
        align-items: center;
        justify-content: center;
    }

</style>