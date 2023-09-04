import i18n, { type Config } from "sveltekit-i18n";
import { dev } from '$app/environment';

export const defaultLocale = 'en';

export const config: Config = {
    log: {
        level: dev ? 'warn' : 'error',
    },
    loaders: [
        {
            locale: "en",
            key: "common",
            loader: async () => (await import("./en/common.json")).default,
        },
        {
            locale: "de",
            key: "common",
            loader: async () => (await import("./de/common.json")).default,
        },
    ],
};

export const { t, loading, locales, locale, translations, loadTranslations, addTranslations, setLocale, setRoute } = new i18n(config);

loading.subscribe(($loading) => $loading && console.log('Loading translations...'));
