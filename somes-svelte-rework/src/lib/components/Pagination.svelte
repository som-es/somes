<script lang="ts">
	import { onMount } from "svelte";
	import SButton from "./UI/SButton.svelte";

    export let maxPage: number;
    export let page: number;

    const offsets = [
        [0, 11, 4, 4, 4, 4, 4, 7, 0],
        [0, 10, 3, 3, 3, 3, 3, 6, 0],
        [0, 9, 2, 2, 2, 2, 2, 5, 0],
        [0, 8, 1, 1, 1, 1, 1, 4, 0],
        [0, 7, 0, 0, 0, 0, 0, 3, 0],
        [0, 6, 0, 0, 0, 0, 0, 2, 0],
        [0, 5, 0, 0, 0, 0, 0, 1, 0],
        [0, 4, 0, 0, 0, 0, 0, 0, 0],
        [0, 3, 0, 0, 0, 0, 0, 0, 0],
        [0, 2, 0, 0, 0, 0, 0, 0, 0],
        [0, 1, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
    ];
    let pageSuggestions: number[] = [];
    console.log(page); 
    $: if(page) {
        const otherPage = page + 0;
        const baseLayout = [1, otherPage - 10, otherPage-2, otherPage -1, otherPage, otherPage + 1, otherPage + 2, otherPage + 10, maxPage];
        const offsetFn = (i: number) => {
            if (page > 0 && page < 12) {
                return offsets[page-1][i];
            }
            if (page <= maxPage && page > maxPage-12) {
                return offsets[maxPage - page][offsets[0].length - i - 1] * -1;
            }
            return 0
        }
        pageSuggestions = baseLayout.map((el, i) => el + offsetFn(i));
    }

    const isActive = (suggestion: number) => {
        return suggestion == page;
    }
    // $: pageSuggestions = [1, page - 10, page-2, page -1, page, page + 1, page + 2, page + 10, maxPage-1]
</script>
<div class="flex flex-row flex-wrap">
    <SButton class="mt-5 mb-5 bg-secondary-500 text-center" on:click={() => {if (page > 0) page--}}> {'<'} </SButton>  
    {#each pageSuggestions as suggestion}
        {#if suggestion > 0 && suggestion <= maxPage}
            <SButton class="mt-5 mb-5 w-14 text-center {isActive(suggestion) ? "bg-tertiary-500" : "bg-secondary-500"}" on:click={() => {page = suggestion}}>{suggestion}</SButton>
        {/if}
    {/each}
    <SButton class="mt-5 mb-5 bg-secondary-500" on:click={() => {if (page < maxPage) page++}}>{'>'}</SButton>  
</div>