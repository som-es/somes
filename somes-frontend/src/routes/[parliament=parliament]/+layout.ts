import type { LayoutLoad } from './$types';
import type { Parliament } from '$lib/api/parliament';

export const load: LayoutLoad = ({ params }) => {
	return { parliament: params.parliament as Parliament };
};
