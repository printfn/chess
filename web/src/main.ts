import './style.scss';
import { Chessground } from 'chessground';
import { Config } from 'chessground/config';

const config: Config = {
	movable: {
		free: false,
		dests: new Map([['e7', ['e6', 'e5']]]),
	},
	animation: {
		enabled: true,
	},
};
const chessElement = document.getElementById('chessboard')!;
const board = Chessground(chessElement, config);
board.move('e2', 'e4');
