import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig, searchForWorkspaceRoot } from 'vite';
import tailwindcss from '@tailwindcss/vite';

export default defineConfig({
	build: {
		target: 'esnext',
		sourcemap: true,
	},
	plugins: [sveltekit(), tailwindcss()],
	server: {
		fs: {
			allow: [searchForWorkspaceRoot(process.cwd()), '../wasm/pkg/'],
		},
	},
});
