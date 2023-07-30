import { useEffect, useState } from 'react';
import { Board } from './Board';
import { Settings } from './Settings';
import { initialTheme } from './lib/theme';
import { GameOver } from './GameOver';

function App() {
	const [theme, setTheme] = useState(initialTheme());
	useEffect(() => {
		localStorage.setItem('theme', theme);
		document.documentElement.setAttribute('data-bs-theme', theme);
	}, [theme]);
	const [perspective, setPerspective] = useState(true);

	return (
		<>
			<Settings id="settings-modal" theme={theme} setTheme={setTheme} />
			<GameOver id="game-over-modal" />
			<div className="container">
				<div className="row justify-content-center p-2">
					<div className="col board-column text-center">
						<h1>Chess</h1>
						<Board perspective={perspective ? 'white' : 'black'} />
					</div>
				</div>
				<div className="row justify-content-center">
					<div className="col board-column">
						<p className="d-inline-flex gap-1">
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
						</p>
					</div>
				</div>
			</div>
		</>
	);
}

export default App;
