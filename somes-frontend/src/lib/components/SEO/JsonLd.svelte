<script lang="ts">
	import { jsonLdScript, type SchemaNode } from '$lib/seo/schema';

	interface Props {
		/**
		 * schema.org nodes, rendered as one `@graph` so that nodes can reference
		 * each other by `@id` (e.g. an event's `organizer`).
		 */
		nodes: SchemaNode[];
	}

	let { nodes }: Props = $props();

	// The tag string is built in `$lib/seo/schema.ts`: a literal `<script>` in a
	// `.svelte` file is parsed as raw text, so the mustache would not be
	// interpolated, and a literal closing script tag would end this block.
	let tag = $derived(jsonLdScript(nodes));
</script>

<svelte:head>
	{@html tag}
</svelte:head>
