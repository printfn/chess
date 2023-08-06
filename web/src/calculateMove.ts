import {
	default as initWasm,
	calculate_move,
	init_panic_hook,
} from '../../wasm/pkg';

type CalculateMoveEvent = {
	fen: string;
	depth: number;
};

self.addEventListener(
	'message',
	async ({ data: { fen, depth } }: MessageEvent<CalculateMoveEvent>) => {
		await initWasm();
		init_panic_hook();
		const result = calculate_move(fen, depth);
		postMessage(result);
	},
);
