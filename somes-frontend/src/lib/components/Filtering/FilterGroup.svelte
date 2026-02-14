<script lang="ts">
	import type { GenericFilterGroup } from "./types";

    interface Props {
        group: GenericFilterGroup<string> | GenericFilterGroup<boolean>
    } 

    let { group = $bindable() }: Props = $props();

</script>

{#if !group.hidden}
    <div class="mt-4 first:mt-0">
        <span class="text-base font-semibold text-gray-800">{group.title}</span>
        <div class="flex w-fit gap-1 rounded-lg border border-primary-300 text-sm">
            {#each group.options as option}
                <button
                    class="cursor-pointer rounded-lg px-2 py-1 text-xs md:text-sm"
                    class:bg-primary-300={group.activeValue === option.value}
                    onclick={() => {
                        group.activeValue = option.value;
                    }}
                >
                    <span class="text-nowrap">{option.title}</span>
                </button>
            {/each}
        </div>
    </div>
{/if}