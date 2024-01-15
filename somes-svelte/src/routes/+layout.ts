import { addTranslations, setLocale, setRoute } from '$lib/translations';
import type { Load } from '@sveltejs/kit';

export const prerender = true;

export const load: Load = async ({ data }: { data: any }) => {
	const { i18n, translations } = data;
	const { locale, route } = i18n;
  
	addTranslations(translations);
  
	await setRoute(route);
	await setLocale(locale);

	return i18n;
  };
