import { useCallback, useEffect, useRef, useState } from 'react';
import { PromotionPiece } from '../lib/types';

interface Props {
	id: string;
	onHide: (piece: PromotionPiece) => void;
}

export function Promotion({ id, onHide }: Props) {
	const [piece, setPiece] = useState<PromotionPiece>('Q');
	const modalRef = useRef<HTMLDivElement>(null);
	const hiddenHandler = useCallback(() => {
		onHide(piece);
	}, [onHide, piece]);
	useEffect(() => {
		if (!modalRef.current) return;
		const elem = modalRef.current;
		elem.addEventListener('hidden.bs.modal', hiddenHandler);
		return () => {
			elem.removeEventListener('hidden.bs.modal', hiddenHandler);
		};
	}, [modalRef, hiddenHandler]);

	return (
		<div
			className="modal fade"
			tabIndex={-1}
			id={id}
			ref={modalRef}
			data-bs-backdrop="static"
			data-bs-keyboard="false"
		>
			<div className="modal-dialog">
				<div className="modal-content">
					<div className="modal-header">
						<h5 className="modal-title">Promotion</h5>
						<button
							type="button"
							className="btn-close"
							data-bs-dismiss="modal"
							aria-label="Close"
						></button>
					</div>
					<div className="modal-body">
						<label htmlFor="theme-select-input" className="form-label">
							Choose which piece to promote to:
						</label>
						<select
							id="theme-select-input"
							className="form-select"
							aria-label="Choose a piece"
							value={piece}
							onChange={e => setPiece(e.currentTarget.value as PromotionPiece)}
						>
							<option value="Q">Queen</option>
							<option value="R">Rook</option>
							<option value="B">Bishop</option>
							<option value="N">Knight</option>
						</select>
					</div>
					<div className="modal-footer">
						<button
							type="button"
							className="btn btn-primary"
							data-bs-dismiss="modal"
						>
							Confirm
						</button>
					</div>
				</div>
			</div>
		</div>
	);
}
