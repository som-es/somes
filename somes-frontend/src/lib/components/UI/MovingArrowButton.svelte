<script lang="ts">
    let { title = "Mehr Details", ...props} = $props();
</script>

<button
    {...props}
    class="group flex gap-1 rounded-lg bg-primary-300 px-2 py-1"
>
    <span class="text-base font-semibold">{title}</span>
    <span class="text-base font-semibold transition-transform group-hover:translate-x-1">&#8594;</span>
</button>