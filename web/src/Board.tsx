import { Chessground } from 'chessground';
import { Api } from 'chessground/api';
import { Config } from 'chessground/config';
import { File, Key, Rank } from 'chessground/types';
import { useEffect, useMemo, useRef, useState } from 'react';
import { default as initWasm, valid_moves, apply_move } from '../../wasm/pkg';
import './chessground/chessground-base.css';
import MyWorker from './calculateMove?worker';
import { Modal } from 'bootstrap';
import { PromotionPiece } from './lib/types';

await initWasm();

function possibleMoves(fen: string): Map<Key, Key[]> {
	console.log('getting possible moves for fen', fen);
	const moves: `${File}${Rank}`[][] = JSON.parse(valid_moves(fen));
	const result: Map<Key, Key[]> = new Map();
	for (const [from, to] of moves) {
		if (result.has(from)) {
			result.get(from)!.push(to);
		} else {
			result.set(from, [to]);
		}
	}
	return result;
}

interface Props {
	perspective: 'white' | 'black';
	promote: () => Promise<PromotionPiece>;
}

export function Board({ perspective, promote }: Props) {
	const [fen, setFen] = useState(
		'rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1',
	);
	const [, setWorker] = useState<Worker | null>(null);
	const [lastMove, setLastMove] = useState<[Key, Key] | undefined>(undefined);
	const [block, setBlock] = useState(false);
	const [api, setApi] = useState<Api | null>(null);
	const ref = useRef<HTMLDivElement>(null);

	const config: Config = useMemo<Config>(
		() => ({
			fen: fen,
			coordinates: false,
			orientation: perspective,
			lastMove: lastMove,
			movable: {
				free: false,
				dests: block ? new Map() : possibleMoves(fen),
				events: {
					after: async (from, to) => {
						setLastMove([from, to]);
						let nextPos = apply_move(fen, from, to);
						if (!nextPos) {
							const promotion = await promote();
							nextPos = apply_move(fen, from, to, promotion);
						}
						setFen(nextPos);
						setBlock(true);
						setTimeout(() => {
							const w = new MyWorker();
							w.onmessage = e => {
								setWorker(null);
								if (!e.data) {
									new Modal(document.getElementById('game-over-modal')!).show();
									return;
								}
								const result: { from: Key; to: Key; fen: string } = JSON.parse(
									e.data,
								);
								setBlock(false);
								setFen(result.fen);
								setLastMove([result.from, result.to]);
								if (possibleMoves(result.fen).size === 0) {
									new Modal(document.getElementById('game-over-modal')!).show();
								}
							};
							w.onerror = e => {
								setWorker(null);
								console.error(e);
							};
							w.postMessage(nextPos);
							setWorker(w);
						}, 500);
					},
				},
			},
			animation: {
				enabled: true,
			},
		}),
		[fen, perspective, lastMove, block, promote],
	);

	useEffect(() => {
		if (ref?.current && !api) {
			setApi(Chessground(ref.current, config));
		} else if (ref?.current && api) {
			api.set(config);
		}
	}, [ref, api, config]);

	return <div className="ratio ratio-1x1" ref={ref} />;
}
