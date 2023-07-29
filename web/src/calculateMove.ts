import {default as initWasm, calculate_move} from '../../wasm/pkg';

self.addEventListener('message', ({ data }) => {
	initWasm().then(() => {
		console.log('calculateMove.ts: data = ', data);
		const fen: string = data;
		const result = calculate_move(fen);
		postMessage(result);
	});
});
