<script lang="ts">
    export let seats: number[];
    import Halfcircle from "./Halfcircle.svelte";
</script>

{#each seats as seat, idx}
    <Halfcircle n={seat} r={100 + idx * 20} />
{/each}