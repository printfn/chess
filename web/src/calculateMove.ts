import {
	default as initWasm,
	calculate_move,
	init_panic_hook,
} from '../../wasm/pkg';
import { CalculateMoveArgs } from './lib/CalculateMoveArgs';

self.addEventListener(
	'message',
	async ({
		data: { fen, depth, enableQuiescence },
	}: MessageEvent<CalculateMoveArgs>) => {
		await initWasm();
		init_panic_hook();
		const result = calculate_move(fen, depth, enableQuiescence);
		postMessage(result);
	},
);
