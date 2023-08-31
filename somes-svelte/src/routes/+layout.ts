import { loadTranslations } from "$lib/translations";
import type { Load } from "@sveltejs/kit";

export const load: Load = async ({ url }) => {
	const { pathname } = url;
	const initLocale = "de"; // get from cookie, user session, ...

	await loadTranslations(initLocale, pathname);
	return {};
};
