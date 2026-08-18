import type { de } from './de';

/**
 * The shape of both message catalogs. Derived from the German catalog so that
 * every translation file must contain exactly the same keys.
 */
export type Messages = { [K in keyof typeof de]: string };
