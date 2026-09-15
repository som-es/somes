import type { ParamMatcher } from '@sveltejs/kit';
import { PARLIAMENTS } from '$lib/api/parliament';

export const match: ParamMatcher = (param) =>
	PARLIAMENTS.includes(param as (typeof PARLIAMENTS)[number]);
