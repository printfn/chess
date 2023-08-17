import { default as initWasm, game_state, apply_move, init_panic_hook } from '../../../wasm/pkg';
import type { Key } from 'chessground/types';
import MyWorker from './worker?worker';

await initWasm();
init_panic_hook();

export type CalculateMoveResult = {
	from: Key;
	to: Key;
	fen: string;
};

export type CalculateMoveArgs = {
	fen: string;
	depth: number;
	enableQuiescence: boolean;
};

export type PromotionPiece = 'Q' | 'R' | 'B' | 'N';

export function possibleMoves(fen: string): Map<Key, Key[]> {
	console.log('getting possible moves for fen', fen);
	const gameState = game_state(fen);
	const result: Map<Key, Key[]> = new Map();
	for (const { from, to } of gameState.moves) {
		const x = result.get(from);
		if (x !== undefined) {
			x.push(to);
		} else {
			result.set(from, [to]);
		}
	}
	return result;
}

export function calculateMove(
	fen: string,
	depth: number,
	enableQuiescence: boolean,
): Promise<CalculateMoveResult> {
	return new Promise((resolve, reject) => {
		setTimeout(() => {
			const w = new MyWorker();
			w.onmessage = e => {
				const result: { from: Key; to: Key; fen: string } = JSON.parse(e.data);
				resolve(result);
			};
			w.onerror = e => {
				console.error(e);
				reject(e);
			};
			const args: CalculateMoveArgs = { fen, depth, enableQuiescence };
			w.postMessage(args);
		}, 500);
	});
}

export function applyMove(fen: string, from: Key, to: Key, promotion?: PromotionPiece) {
	return apply_move(fen, from, to, promotion);
}
