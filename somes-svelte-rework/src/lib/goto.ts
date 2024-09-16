import { goto } from '$app/navigation';
import { base } from '$app/paths';
import { hasGoBackStore } from './stores/stores';

export function gotoHistory(route: string, withHistory: boolean = false) {
	hasGoBackStore.set(withHistory);
	goto(`${base}${route}`);
}
