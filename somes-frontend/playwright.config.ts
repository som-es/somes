import { defineConfig } from '@playwright/test';

const node = JSON.stringify(process.execPath);

export default defineConfig({
	webServer: {
		command: `${node} node_modules/vite/bin/vite.js build && ${node} node_modules/vite/bin/vite.js preview`,
		port: 4173
	},
	testDir: 'e2e'
});
