<script lang="ts">
	import searchIcon from '$lib/assets/misc_icons/search-glass.svg?raw';
    import type { HTMLInputAttributes } from 'svelte/elements';
		
    interface Props extends HTMLInputAttributes {
        searchValue: string;
    }

    let { searchValue = $bindable(), ...rest }: Props = $props();
</script>
<div class="flex h-10 flex-grow rounded-xl border-[2px] border-gray-400 touch-manipulation">
    <div class="flex h-9 w-10 items-center justify-center text-gray-600 dark:text-gray-300">
        {@html searchIcon}
    </div>
    <input
        type="search"
        class="block w-full bg-transparent py-2 placeholder:text-gray-600 dark:placeholder:text-gray-300 focus:outline-none"
        placeholder="Suche..."
        bind:value={searchValue}
        {...rest}
    />
</div>