import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App.tsx';

ReactDOM.createRoot(document.getElementById('root')!).render(
	<React.StrictMode>
		<App />
	</React.StrictMode>,
);

// import './style.scss';
// import { Chessground } from 'chessground';
// import { Config } from 'chessground/config';
// import { greet } from '../../wasm/pkg';

// const config: Config = {
// 	movable: {
// 		free: false,
// 		dests: new Map([['e7', ['e6', 'e5']]]),
// 	},
// 	animation: {
// 		enabled: true,
// 	},
// };
// const chessElement = document.getElementById('chessboard')!;
// const board = Chessground(chessElement, config);
// board.move('e2', 'e4');
// greet();
