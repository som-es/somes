<script lang="ts">
	import { Dialog } from "bits-ui";
	import type { Snippet } from "svelte";

    interface Props {
        title?: string;
        children?: Snippet;
    }

    let { title = "Details", children }: Props = $props();

</script>

<Dialog.Root>
    <Dialog.Trigger>
        <button
            class="mr-auto btn sm:btn-lg bg-secondary-500 text-white"
        >
            {title}
        </button>
    </Dialog.Trigger>	
    <Dialog.Portal>
        <Dialog.Overlay
            class="data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 fixed inset-0 z-30 bg-black/80"
        />
        <Dialog.Content class="
            overflow-y-scroll
            data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 
            data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 
            outline-hidden fixed left-[50%] top-[50%] z-30 translate-x-[-50%] translate-y-[-50%] w-7xl 2xl:max-w-7xl max-w-[90%] h-[90vh] bg-primary-100 dark:bg-gray-800 shadow-lg rounded-lg overflow-hidden"
        >
            {@render children?.()}
        </Dialog.Content>
    </Dialog.Portal>
</Dialog.Root>