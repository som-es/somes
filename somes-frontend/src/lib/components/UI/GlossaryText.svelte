<script lang="ts">
	import type { Glossary } from '$lib/ai_summary_types';
	import { tick } from 'svelte';

	let {
		glossary,
		text,
		className = ''
	}: { glossary: Glossary; text: string; className?: string } = $props();

	let processedText = $derived.by(() => {
		const sortedTerms = [...glossary.difficult_terms].sort((a, b) => b.term.length - a.term.length);

		const definitionMap = new Map(
			sortedTerms.map((item) => [item.term.toLowerCase(), item.simple_definition])
		);

		const termReplacements: { start: number; end: number; definition: string; match: string }[] =
			[];
		for (const [term, definition] of definitionMap) {
			const termRegex = new RegExp(`\\b${term}`, 'gi');

			const match = termRegex.exec(text);
			if (match) {
				const start = match.index;
				const end = match.index + match[0].length;
				if (
					termReplacements.findIndex((term) => {
						return start <= term.end && end >= term.start;
					}) == -1
				) {
					termReplacements.push({ start, end, definition, match: match[0] });
				}
			}
		}

		termReplacements.sort((a, b) => a.start - b.start);

		const textParts: { text: string; definition: string | null }[] = [];
		let start = 0;
		for (const termReplacement of termReplacements) {
			const newStart = termReplacement.start;
			const outsiteTextPart = text.slice(start, newStart);
			if (outsiteTextPart && outsiteTextPart.length > 0) {
				textParts.push({ text: outsiteTextPart, definition: null });
			}
			textParts.push({
				text: text.slice(newStart, termReplacement.end),
				definition: termReplacement.definition
			});
			start = termReplacement.end;
		}
		const outsiteTextPart = text.slice(start);
		if (outsiteTextPart && outsiteTextPart.length > 0) {
			textParts.push({ text: outsiteTextPart, definition: null });
		}
		return textParts;
	});

	let hoverTerm = $state<HTMLElement | null>(null);
	let pinnedTerm = $state<HTMLElement | null>(null);
	let activeTerm = $derived(pinnedTerm ?? hoverTerm);
	let tooltipEl = $state<HTMLElement | null>(null);
	let tooltipStyle = $state('');
	let arrowStyle = $state('');
	let placement = $state<'top' | 'bottom'>('top');

	const PADDING = 8;
	const ARROW = 8;

	function termFromEvent(e: Event): HTMLElement | null {
		const target = e.target as HTMLElement | null;
		return target?.closest?.('.glossary-term') as HTMLElement | null;
	}

	function onPointerOver(e: PointerEvent) {
		if (e.pointerType != 'mouse') return;
		const term = termFromEvent(e);
		if (term) hoverTerm = term;
	}

	function onPointerOut(e: PointerEvent) {
		if (e.pointerType != 'mouse') return;
		const term = termFromEvent(e);
		if (term && term == hoverTerm) hoverTerm = null;
	}

	function onClick(e: MouseEvent) {
		const term = termFromEvent(e);
		if (!term) return;
		pinnedTerm = pinnedTerm == term ? null : term;
	}

	function onKeyDown(e: KeyboardEvent) {
		const term = termFromEvent(e);
		if (!term) return;
		if (e.key == 'Enter' || e.key == ' ') {
			e.preventDefault();
			pinnedTerm = pinnedTerm == term ? null : term;
		}
	}

	function onFocusIn(e: FocusEvent) {
		const term = termFromEvent(e);
		if (term) hoverTerm = term;
	}

	function onFocusOut(e: FocusEvent) {
		const term = termFromEvent(e);
		if (term && term == hoverTerm) hoverTerm = null;
	}

	function position() {
		if (!activeTerm || !tooltipEl) return;
		const termRect = activeTerm.getBoundingClientRect();
		const tipRect = tooltipEl.getBoundingClientRect();
		const vw = window.innerWidth;
		const vh = window.innerHeight;

		const termCenter = termRect.left + termRect.width / 2;
		const maxLeft = Math.max(PADDING, vw - PADDING - tipRect.width);
		const left = Math.min(Math.max(PADDING, termCenter - tipRect.width / 2), maxLeft);

		const spaceAbove = termRect.top - PADDING;
		const fitsAbove = spaceAbove >= tipRect.height + ARROW;
		const spaceBelow = vh - termRect.bottom - PADDING;
		placement = fitsAbove || spaceAbove >= spaceBelow ? 'top' : 'bottom';
		const top =
			placement == 'top'
				? Math.max(PADDING, termRect.top - ARROW - tipRect.height)
				: Math.min(vh - PADDING - tipRect.height, termRect.bottom + ARROW);

		tooltipStyle = `left:${left}px;top:${top}px;`;
		const arrowLeft = Math.min(
			Math.max(ARROW + 4, termCenter - left),
			tipRect.width - ARROW - 4
		);
		arrowStyle = `left:${arrowLeft}px;`;
	}

	function close() {
		hoverTerm = null;
		pinnedTerm = null;
	}

	$effect(() => {
		if (!activeTerm) {
			tooltipStyle = '';
			return;
		}
		// Measure after the tooltip has been rendered.
		tick().then(position);

		const onScrollOrResize = () => position();
		const onPointerDown = (e: PointerEvent) => {
			const target = e.target as Node | null;
			if (activeTerm?.contains(target) || tooltipEl?.contains(target)) return;
			close();
		};
		const onKey = (e: KeyboardEvent) => {
			if (e.key == 'Escape') close();
		};
		window.addEventListener('scroll', onScrollOrResize, true);
		window.addEventListener('resize', onScrollOrResize);
		document.addEventListener('pointerdown', onPointerDown, true);
		document.addEventListener('keydown', onKey);
		return () => {
			window.removeEventListener('scroll', onScrollOrResize, true);
			window.removeEventListener('resize', onScrollOrResize);
			document.removeEventListener('pointerdown', onPointerDown, true);
			document.removeEventListener('keydown', onKey);
		};
	});

	// Render the tooltip directly under <body> so that it is never clipped by scrolling or
	// transformed ancestors (e.g. the speech modal) and always stays within the viewport.
	function portal(node: HTMLElement) {
		document.body.appendChild(node);
		return {
			destroy() {
				node.remove();
			}
		};
	}
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<span
	class={className}
	onpointerover={onPointerOver}
	onpointerout={onPointerOut}
	onclick={onClick}
	onkeydown={onKeyDown}
	onfocusin={onFocusIn}
	onfocusout={onFocusOut}
>
	{#each processedText as item}
		{#if item.definition}
			<span class="glossary-term" data-definition={item.definition} tabindex="0" role="button"
				>{item.text}</span
			>
		{:else}
			{item.text}
		{/if}
	{/each}
</span>

{#if activeTerm}
	<span
		use:portal
		bind:this={tooltipEl}
		role="tooltip"
		class="glossary-tooltip glossary-tooltip-{placement}"
		class:glossary-tooltip-visible={tooltipStyle != ''}
		style={tooltipStyle}
	>
		{activeTerm.dataset.definition}
		<span class="glossary-tooltip-arrow" style={arrowStyle}></span>
	</span>
{/if}

<style>
	:global(.glossary-term) {
		display: inline-block;
		-webkit-text-decoration: underline dotted;
		text-decoration: underline dotted;
		text-decoration-line: underline;
		text-decoration-style: dotted;
		text-underline-offset: 4px;
		cursor: help;
		outline: none;
	}

	:global(.glossary-tooltip) {
		position: fixed;
		z-index: 1000;
		box-sizing: border-box;
		background-color: #333;
		color: #fff;
		padding: 8px 12px;
		border-radius: 4px;
		font-size: 0.85rem;
		line-height: 1.4;
		width: max-content;
		max-width: min(250px, calc(100vw - 16px));
		white-space: normal;
		overflow-wrap: break-word;
		box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
		pointer-events: none;
		visibility: hidden;
	}

	:global(.glossary-tooltip-visible) {
		visibility: visible;
	}

	:global(.glossary-tooltip-arrow) {
		position: absolute;
		border: 8px solid transparent;
		transform: translateX(-50%);
	}

	:global(.glossary-tooltip-top .glossary-tooltip-arrow) {
		top: 100%;
		border-top-color: #333;
	}

	:global(.glossary-tooltip-bottom .glossary-tooltip-arrow) {
		bottom: 100%;
		border-bottom-color: #333;
	}
</style>
