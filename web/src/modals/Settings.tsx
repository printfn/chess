import React from 'react';
import { Theme, themes } from '../lib/theme';

export function Settings({
	id,
	theme,
	setTheme,
}: {
	id: string;
	theme: Theme;
	setTheme: (theme: Theme) => void;
}) {
	const onChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
		const newValue = e.currentTarget.value;
		if (themes.includes(newValue)) {
			setTheme(newValue as Theme);
		}
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
						<label htmlFor="theme-select-input" className="form-label">
							Theme
						</label>
						<select
							id="theme-select-input"
							className="form-select"
							aria-label="Choose a theme"
							value={theme}
							onChange={onChange}
						>
							<option value="pink">Pink</option>
							<option value="brown">Brown</option>
						</select>
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
