import { Chessground } from 'chessground';
import { Api } from 'chessground/api';
import { Config } from 'chessground/config';
import { File, Key, Rank } from 'chessground/types';
import { useEffect, useMemo, useRef, useState } from 'react';
import { valid_moves, apply_move } from '../../wasm/pkg';

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

export function Board() {
	const [fen, setFen] = useState(
		'rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1',
	);
	const [api, setApi] = useState<Api | null>(null);
	const ref = useRef<HTMLDivElement>(null);

	const config: Config = useMemo<Config>(
		() => ({
			fen: fen,
			coordinates: true,
			movable: {
				free: false,
				dests: possibleMoves(fen),
				events: {
					after(from, to) {
						let nextPos = apply_move(fen, from, to);
						if (!nextPos) {
							let promotion = '?';
							while (!['Q', 'R', 'B', 'N'].includes(promotion)) {
								promotion = prompt('Promotion (Q, R, B, N)', 'Q') ?? '?';
							}
							nextPos = apply_move(fen, from, to, promotion);
						}
						setFen(nextPos);
					},
				},
			},
			animation: {
				enabled: true,
			},
		}),
		[fen],
	);

	useEffect(() => {
		if (ref?.current && !api) {
			setApi(Chessground(ref.current, config));
		} else if (ref?.current && api) {
			api.set(config);
		}
	}, [ref, api, config]);

	return <div ref={ref} style={{ height: '500px', width: '500px' }} />;
}
