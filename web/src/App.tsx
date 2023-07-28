import { useState } from 'react';
import { Board } from './Board';

function App() {
	const [count, setCount] = useState(0);

	return (
		<div>
			<button onClick={() => setCount(count => count + 1)}>
				count is {count}
			</button>
			<Board />
		</div>
	);
}

export default App;
