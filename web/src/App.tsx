import { useState } from 'react';
import { Board } from './Board';

function App() {
	const [perspective, setPerspective] = useState(true);

	return (
		<div className="container">
			<div className="row justify-content-center p-2">
				<div className="col board-column text-center">
					<h1>Chess</h1>
					<Board perspective={perspective ? 'white' : 'black'} />
				</div>
			</div>
			<div className="row justify-content-center">
				<div className="col board-column">
					<button
						className="btn btn-outline-primary"
						onClick={() => setPerspective(!perspective)}
					>
						Flip Board
					</button>
				</div>
			</div>
		</div>
	);
}

export default App;
