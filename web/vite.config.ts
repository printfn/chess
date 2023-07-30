import { defineConfig, searchForWorkspaceRoot } from 'vite';
import react from '@vitejs/plugin-react-swc';

export default defineConfig({
	build: {
		target: 'esnext',
		sourcemap: true,
	},
	server: {
		fs: {
			allow: [searchForWorkspaceRoot(process.cwd()), '../wasm/pkg/'],
		},
	},
	plugins: [react()],
});
