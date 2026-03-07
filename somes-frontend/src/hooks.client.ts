
const CLEAR_KEY = 'cleared_march_7';

if (!localStorage.getItem(CLEAR_KEY)) {
    localStorage.clear();
    localStorage.setItem(CLEAR_KEY, 'true');
}