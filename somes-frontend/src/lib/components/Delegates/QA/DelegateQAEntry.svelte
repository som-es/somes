<script lang="ts">
	import type { DelegateQA } from '$lib/types';

	export let delegateQa: DelegateQA;
	let clazz = '';
	export { clazz as class };
</script>

<div class={clazz}>
	<span class="font-bold text-xl sm:text-3xl">
		{delegateQa.question}
	</span>
	<hr />
	<div class="mt-1 sm:text-xl">{delegateQa.answer}</div>
</div>
