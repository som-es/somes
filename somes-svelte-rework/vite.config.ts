import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

// the host: '0.0.0.0' setting, exposes the app to all devices on the net 

export default defineConfig({
	plugins: [sveltekit()],
	server: {
		host: '0.0.0.0'
	}
});
