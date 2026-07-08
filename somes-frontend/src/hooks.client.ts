const CLEAR_KEY = 'cleared_july_8';

if (!localStorage.getItem(CLEAR_KEY)) {
	localStorage.clear();
	localStorage.setItem(CLEAR_KEY, 'true');
}
