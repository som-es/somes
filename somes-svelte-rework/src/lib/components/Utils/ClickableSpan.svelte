<script lang="ts">
	export let action;
</script>
<span class="cursor-pointer" role="button" on:click={() => {
    action();
}} 
    on:keydown={
        (event) => {
            if (event.key === 'Enter' || event.key === ' ') {
                action();
            }	
        }
    }
    tabindex="0"
>
    <slot />
</span>