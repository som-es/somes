
const CLEAR_KEY = 'cleared_feb_15';

if (!localStorage.getItem(CLEAR_KEY)) {
    localStorage.clear();
    localStorage.setItem(CLEAR_KEY, 'true');
}