import preprocess from "svelte-preprocess";
import adapter from "@sveltejs/adapter-static";
import { vitePreprocess } from "@sveltejs/kit/vite";

/** @type {import('@sveltejs/kit').Config} */
const config = {
        preprocess: [
                vitePreprocess(),
                preprocess({
                        postcss: true,
                }),
        ],

        kit: {
                adapter: adapter({
                        pages: 'build',
                        assets: 'build',
                        fallback: null,
                        precompress: false,
                        strict: true
                }),
                paths: {
                        base: '/alpha'
                },
		prerender: {
			handleHttpError: ({ path, referrer, message }) => {
				// ignore deliberate link to shiny 404 page
				if (path === '/not-found' && referrer === '/blog/how-we-built-our-404-page') {
					return;
				}

				// otherwise fail the build
				// throw new Error(message);
			}
		}
        },
};

export default config;