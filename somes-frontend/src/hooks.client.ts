const CLEAR_KEY = 'cleared_multi_parliament';

if (!localStorage.getItem(CLEAR_KEY)) {
	localStorage.clear();
	localStorage.setItem(CLEAR_KEY, 'true');
}
