<script lang="ts">
	import downArrowIcon from '$lib/assets/misc_icons/down-arrow.svg?raw';
    interface Props {
        title: string;
        activefilterCount: number;
        isOpen: boolean;
    }

    let { title, activefilterCount, isOpen }: Props = $props();
</script>
<div
    class="flex h-full grow items-center justify-center gap-1 rounded-xl bg-secondary-500 px-2 md:ml-2 md:grow-0"
>
    {#if activefilterCount > 0}
        <span
            class="flex h-5 w-5 items-center justify-center rounded-full border text-xs font-semibold text-white"
        >
            {activefilterCount}
        </span>
    {/if}
    <span class="ml-1 text-white">{title}</span>
    <span
        class="block w-4 text-white transition-transform duration-200"
        class:rotate-180={isOpen}
    >
        {@html downArrowIcon}
    </span>
</div>