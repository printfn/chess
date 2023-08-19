import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig, searchForWorkspaceRoot } from 'vite';

export default defineConfig({
	build: {
		target: 'esnext',
		sourcemap: true,
	},
	plugins: [sveltekit()],
	server: {
		fs: {
			allow: [searchForWorkspaceRoot(process.cwd()), '../wasm/pkg/'],
		},
	},
});
