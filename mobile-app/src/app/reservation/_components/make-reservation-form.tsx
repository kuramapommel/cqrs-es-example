"use client";

import useReservationForm from "../_hooks/use-reservation-form";
import type { Reservation } from "../types";

type Props = {
	userId: string;
	onMade: (newReservation: Reservation) => void;
};

const MakeReservationForm = ({ userId, onMade }: Props) => {
	const { tableId, handleTableIdChange, handleSubmit } = useReservationForm(
		userId,
		onMade,
	);
	return (
		<form onSubmit={handleSubmit}>
			<input
				onChange={handleTableIdChange}
				aria-label="テーブル ID 入力欄"
				type="text"
				name="table_id"
				placeholder="Table ID"
				value={tableId}
			/>
			<button type="submit">予約登録</button>
		</form>
	);
};

export default MakeReservationForm;
