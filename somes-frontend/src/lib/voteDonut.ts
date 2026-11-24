export const DONUT_VIEWBOX = 20;
export const DONUT_CENTER = DONUT_VIEWBOX / 2;
export const DONUT_RADIUS = 8.5;
export const DONUT_STROKE_WIDTH = 3;
const CIRCUMFERENCE = 2 * Math.PI * DONUT_RADIUS;
const GAP = 1.2;

export function donutSegments<T extends { value: number }>(
	data: T[]
): (T & { dasharray: string; dashoffset: number })[] {
	const total = data.reduce((sum, d) => sum + d.value, 0);
	if (total === 0) {
		return [];
	}
	const gap = data.length > 1 ? GAP : 0;
	let start = 0;
	return data.map((d) => {
		const length = (d.value / total) * CIRCUMFERENCE;
		const drawn = Math.max(length - gap, 0.01);
		const segment = {
			...d,
			dasharray: `${drawn} ${CIRCUMFERENCE - drawn}`,
			dashoffset: -start
		};
		start += length;
		return segment;
	});
}
