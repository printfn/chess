import { useState } from 'react';
import { Board } from './Board';

function App() {
	const [perspective, setPerspective] = useState(true);

	return (
		<div>
			<button onClick={() => setPerspective(!perspective)}>Flip Board</button>
			<Board perspective={perspective ? 'white' : 'black'} />
		</div>
	);
}

export default App;
