
const CLEAR_KEY = 'cleared_april_30';

if (!localStorage.getItem(CLEAR_KEY)) {
    localStorage.clear();
    localStorage.setItem(CLEAR_KEY, 'true');
}