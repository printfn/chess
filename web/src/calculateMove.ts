import {
	default as initWasm,
	calculate_move,
	init_panic_hook,
} from '../../wasm/pkg';

self.addEventListener('message', async ({ data }) => {
	await initWasm();
	init_panic_hook();
	const fen: string = data;
	const result = calculate_move(fen);
	postMessage(result);
});
