import { default as initWasm, calculate_move } from '../../wasm/pkg';

self.addEventListener('message', async ({ data }) => {
	await initWasm();
	const fen: string = data;
	const result = calculate_move(fen);
	postMessage(result);
});
