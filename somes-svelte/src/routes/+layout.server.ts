import { locales, loadTranslations, translations, defaultLocale } from '$lib/translations';
import type { ServerLoad } from '@sveltejs/kit';

export const load: ServerLoad = async ({ url, cookies, request }) => {
  const { pathname } = url;

  // Try to get the locale from cookie
  let locale = (cookies.get('lang') || '').toLowerCase();

  // Get user preferred locale
  if (!locale) {
    locale = `${`${request.headers.get('accept-language')}`.match(/[a-zA-Z]+?(?=-|_|,|;)/)}`.toLowerCase();
  }

  // Get defined locales
  const supportedLocales = locales.get().map((l) => l.toLowerCase());

  // Use default locale if current locale is not supported
  if (!supportedLocales.includes(locale)) {
    locale = defaultLocale;
  }

  await loadTranslations(locale, pathname);

  return {
    i18n: { locale, route: pathname },
    translations: translations.get(), // `translations` on server contain all translations loaded by different clients
  };
};