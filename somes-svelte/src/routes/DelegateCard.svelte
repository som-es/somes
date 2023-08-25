<script lang="ts">
	import { modalStore } from '@skeletonlabs/skeleton';
    import type { ModalSettings, ModalComponent } from '@skeletonlabs/skeleton';

    // export let id: number
    
    export let name: string;
    export let party: string;
    export let image: string;

</script>

<div class="card card-hover h-[60vh] aspect-[3/5] overflow-hidden grid grid-rows-[max-content_1fr_max-content] drop-shadow-lg hover:cursor-pointer">
	<header>
        <img src={image} class="bg-black/50 w-full aspect-[1/1]" alt="image of politician {name}">
    </header>
	<section class="p-4">
        <h4>
            {name}
        </h4>
        <span>
            {party}
        </span>
    </section>
    <hr class="!border-t-2 my-4" />
	<footer class="card-footer flex justify-between items-center">
        <span>Lorem Ipsum</span>
    </footer>
</div>