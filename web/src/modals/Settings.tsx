import React from 'react';
import { Theme, themes } from '../lib/theme';

export function Settings({
	id,
	theme,
	setTheme,
	depth,
	setDepth,
	enableQuiescence,
	setEnableQuiescence,
}: {
	id: string;
	theme: Theme;
	setTheme: (theme: Theme) => void;
	depth: number;
	setDepth: (depth: number) => void;
	enableQuiescence: boolean;
	setEnableQuiescence: (enableQuiescence: boolean) => void;
}) {
	const onChangeTheme = (e: React.ChangeEvent<HTMLSelectElement>) => {
		const newValue = e.currentTarget.value;
		if (themes.includes(newValue)) {
			setTheme(newValue as Theme);
		}
	};

	const onChangeDepth = (e: React.ChangeEvent<HTMLSelectElement>) => {
		const newValue = parseInt(e.currentTarget.value, 10);
		setDepth(newValue);
	};

	return (
		<div className="modal fade" tabIndex={-1} id={id}>
			<div className="modal-dialog">
				<div className="modal-content">
					<div className="modal-header">
						<h5 className="modal-title">Settings</h5>
						<button
							type="button"
							className="btn-close"
							data-bs-dismiss="modal"
							aria-label="Close"
						></button>
					</div>
					<div className="modal-body">
						<div className="mb-3">
							<label htmlFor="theme-select-input" className="form-label">
								Theme
							</label>
							<select
								id="theme-select-input"
								className="form-select"
								aria-label="Choose a theme"
								value={theme}
								onChange={onChangeTheme}
							>
								<option value="pink">Pink</option>
								<option value="brown">Brown</option>
							</select>
						</div>
						<div className="mb-3">
							<label htmlFor="depth-select-input" className="form-label">
								Calculation Depth
							</label>
							<select
								id="depth-select-input"
								className="form-select"
								aria-label="Set the calculation depth"
								value={depth}
								onChange={onChangeDepth}
							>
								<option value="1">1</option>
								<option value="2">2</option>
								<option value="3">3</option>
								<option value="4">4</option>
							</select>
						</div>
						<div className="mb-3">
							<div className="form-check form-switch">
								<input
									className="form-check-input"
									type="checkbox"
									role="switch"
									id="quiescence-switch-input"
									checked={enableQuiescence}
									onChange={e => setEnableQuiescence(e.currentTarget.checked)}
								/>
								<label
									htmlFor="quiescence-switch-input"
									className="form-check-label"
								>
									Enable Quiescence Search
								</label>
							</div>
						</div>
					</div>
					<div className="modal-footer">
						<button
							type="button"
							className="btn btn-secondary"
							data-bs-dismiss="modal"
						>
							Close
						</button>
					</div>
				</div>
			</div>
		</div>
	);
}
