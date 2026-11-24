import { error } from '@sveltejs/kit';
import { isHasError } from './api';
import type { HasError } from '$lib/types';

export function is_not_found(result: HasError): boolean {
	return result.status === 404 || result.field === 'NotFound';
}

export function or_error<T>(result: T | HasError): T {
	if (!isHasError(result)) return result;
	if (is_not_found(result)) error(404, 'Not found');
	error(503, 'Service unavailable');
}
