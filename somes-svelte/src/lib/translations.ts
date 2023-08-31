import i18n, { type Config } from "sveltekit-i18n";

const config: Config = {
	loaders: [
		{
			locale: "en",
			key: "common",
			loader: async () => (await import("./lang/en.json")).default,
		},
		{
			locale: "de",
			key: "common",
			loader: async () => (await import("./lang/de.json")).default,
		},
	],
};

export const { t, locale, locales, loading, loadTranslations } = new i18n(config);
