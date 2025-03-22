<script>
	export let value = 0.5; // Must be between -1 and 1
	export let trackColor = '#ccc'; // Default track color
	export let knobColor = '#007bff'; // Default knob color
	export let borderColor = '#333'; // Default border color
	export let borderColorKnob = '#333'; // Default border color
	export let leftLabel = 'Dagegen'; // Left label
	export let rightLabel = 'Dafür'; // Right label
	export let zeroLabel = 'Neutral'; // Label for 0.0 marker

	// Ensure value stays within range
	$: clampedValue = Math.max(-1, Math.min(1, value));

	// Convert value to percentage position
	$: position = ((clampedValue + 1) / 2) * 100;
</script>

<div class="slider-container max-w-60">
	<span>{leftLabel}</span>
	<div class="slider-track" style="--track-color: {trackColor}; --border-color: {borderColor};">
		<div class="zero-marker"></div>
		<div class="zero-label mb-6">{zeroLabel}</div>
		<div
			class="slider-knob"
			style="left: {position}%; --knob-color: {knobColor}; --border-color: {borderColorKnob};"
		></div>
	</div>
	<span>{rightLabel}</span>
</div>

<style>
	.slider-container {
		position: relative;
		width: 100%;
		height: 60px; /* Increased height for labels */
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	.slider-track {
		width: 100%;
		height: 16px; /* Thicker track */
		background-color: var(--track-color);
		border-radius: 6px;
		position: relative;
		border: 2px solid var(--border-color);
	}

	.slider-knob {
		position: absolute;
		top: 50%;
		width: 28px; /* Larger knob */
		height: 28px;
		background-color: var(--knob-color);
		border-radius: 50%;
		transform: translate(-50%, -50%);
		pointer-events: none; /* Locked: cannot move */
		border: 3px solid var(--border-color);
	}

	.zero-marker {
		position: absolute;
		left: 50%;
		top: 50%;
		height: 20px;
		width: 2px;
		background-color: black;
		transform: translate(-50%, -50%);
	}

	.zero-label {
		position: absolute;
		left: 50%;
		top: -28px;
		width: 400px;
		transform: translateX(-20%);
		font-size: 14px;
		font-weight: bold;
	}
</style>
