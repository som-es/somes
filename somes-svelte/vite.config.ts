import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig } from "vitest/config";
import { fileURLToPath, URL } from "node:url";

export default defineConfig({
	plugins: [sveltekit()],
	resolve: {
		alias: {
			"@": fileURLToPath(new URL("./src", import.meta.url)),
		},
	},
	test: {
		include: ["src/**/*.{test,spec}.{js,ts}"],
	},
});
