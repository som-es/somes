import { goto } from '$app/navigation';
import { hasGoBackStore } from './stores/stores';

export function gotoHistory(route: string, withHistory: boolean = false) {
	hasGoBackStore.value = withHistory;
	goto(route);
}
