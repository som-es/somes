<script lang="ts">
	import type { Decree } from "$lib/components/Delegates/Decrees/types";
	import Square from "$lib/components/UI/Square.svelte";
	import { dashDateToDotDate } from "$lib/date";

    export let decree: Decree;
	export let isCenter = false;
</script>
<div class="flex flex-wrap {isCenter ? 'justify-center' : ''} info-item gap-3">
	<Square class="min-w-[250px]!">
		<div class="cursor-pointer text-center text-lg ">
			{decree.ministrial_issuer}
		</div>
		<span class="font-bold text-xl">Ressort</span>
	</Square>
	<Square>
		<div class="font-bold text-lg">
			{dashDateToDotDate(decree.publication_date.split('T')[0].toString())}
		</div>
		<div>Veröffentlicht am</div>
	</Square>
</div>