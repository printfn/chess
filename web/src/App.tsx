import { useState } from 'react';
import { greet } from '../../wasm/pkg';
import { Board } from './Board';
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

function App() {
	const [count, setCount] = useState(0);

	return (
		<div>
			<button onClick={() => setCount(count => count + 1)}>
				count is {count}
			</button>
			<button onClick={() => greet()}>Greet</button>
			<Board config={config} />
		</div>
	);
}

export default App;
