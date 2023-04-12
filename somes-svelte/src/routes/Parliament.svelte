<script lang="ts">
    export let seats: number[];
    import Halfcircle from "./Halfcircle.svelte";
</script>

{#each seats as seat, idx}
    <Halfcircle n={seat} r={70 + idx * (idx == 1 ? 30 : 20) + (idx >= 2 ? 30: 0)} />
{/each}