import { Chessground } from 'chessground';
import { Api } from 'chessground/api';
import { Config } from 'chessground/config';
import { Key } from 'chessground/types';
import { useEffect, useMemo, useRef, useState } from 'react';
import {
	default as initWasm,
	game_state,
	apply_move,
	init_panic_hook,
} from '../../wasm/pkg';
import './chessground/chessground-base.css';
import MyWorker from './calculateMove?worker';
import { Modal } from 'bootstrap';
import { PromotionPiece } from './lib/types';

await initWasm();
init_panic_hook();

const INITIAL_POSITION =
	'rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1';

function possibleMoves(fen: string): Map<Key, Key[]> {
	console.log('getting possible moves for fen', fen);
	const gameState = game_state(fen);
	const result: Map<Key, Key[]> = new Map();
	for (const { from, to } of gameState.moves) {
		if (result.has(from)) {
			result.get(from)!.push(to);
		} else {
			result.set(from, [to]);
		}
	}
	return result;
}

interface Props {
	promote: () => Promise<PromotionPiece>;
}

type CalculateMoveResult = { from: Key; to: Key; fen: string };
function calculateMove(fen: string): Promise<CalculateMoveResult> {
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
			w.postMessage(fen);
		}, 500);
	});
}

export function Board({ promote }: Props) {
	const [fen, setFen] = useState(INITIAL_POSITION);
	const [perspective, setPerspective] = useState(true);
	const [lastMove, setLastMove] = useState<[Key, Key] | undefined>(undefined);
	const [block, setBlock] = useState(false);
	const [api, setApi] = useState<Api | null>(null);
	const ref = useRef<HTMLDivElement>(null);

	const config: Config = useMemo<Config>(
		() => ({
			fen: fen,
			coordinates: false,
			orientation: perspective ? 'white' : 'black',
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
						if (possibleMoves(nextPos).size === 0) {
							new Modal(document.getElementById('game-over-modal')!).show();
							return;
						}
						const result = await calculateMove(nextPos);
						setFen(result.fen);
						setLastMove([result.from, result.to]);
						if (possibleMoves(result.fen).size === 0) {
							new Modal(document.getElementById('game-over-modal')!).show();
							return;
						}
						setBlock(false);
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

	const newGame = async (color: 'white' | 'black' | 'random') => {
		if (color === 'random') {
			color = Math.random() > 0.5 ? 'white' : 'black';
		}
		setFen(INITIAL_POSITION);
		setLastMove(undefined);
		setPerspective(color === 'white');
		setLastMove(undefined);
		setBlock(false);
		if (color === 'black') {
			const result = await calculateMove(INITIAL_POSITION);
			setFen(result.fen);
			setLastMove([result.from, result.to]);
		}
	};

	return (
		<div className="row justify-content-center">
			<div className="col board-column">
				<div className="mb-2">
					<div className="ratio ratio-1x1" ref={ref} />
				</div>
				<div className="d-grid gap-2 mb-2">
					<button
						className="btn btn-outline-primary"
						onClick={() => setPerspective(!perspective)}
					>
						Flip Board
					</button>
					<button
						className="btn btn-outline-primary"
						data-bs-toggle="modal"
						data-bs-target="#settings-modal"
					>
						Settings
					</button>
					<button
						className="btn btn-outline-primary"
						onClick={() => newGame('white')}
					>
						New Game (White)
					</button>
					<button
						className="btn btn-outline-primary"
						onClick={() => newGame('black')}
					>
						New Game (Black)
					</button>
					<button
						className="btn btn-outline-primary"
						onClick={() => newGame('random')}
					>
						New Game (Random)
					</button>
				</div>
			</div>
		</div>
	);
}
