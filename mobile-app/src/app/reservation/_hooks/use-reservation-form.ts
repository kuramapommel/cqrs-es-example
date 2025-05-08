import { useState } from "react";
import type { Reservation } from "../types";

export default function useReservationForm(
	userId: string,
	onMade: (newReservation: Reservation) => void,
) {
	const [tableId, setTableId] = useState("");

	const handleTableIdChange = (event: React.ChangeEvent<HTMLInputElement>) => {
		setTableId(event.target.value);
	};

	const handleSubmit = async (event: React.FormEvent<HTMLFormElement>) => {
		event.preventDefault();
		/**
		 * curl -X POST http://localhost:8080/api/reservation \
		 * --cookie "userId=test-user-id" \
		 * -H "Content-Type: application/json" \
		 * -d '{"tableId": "test-table-id"}'
		 */
		const { reservation_id }: { reservation_id: string } = await fetch(
			"http://localhost:8080/api/reservation",
			{
				method: "POST",
				credentials: "include",
				headers: {
					"Content-Type": "application/json",
				},
				body: JSON.stringify({
					tableId: tableId,
				}),
			},
		).then((res) => res.json());

		onMade({
			id: reservation_id,
			user_id: userId,
			table_id: tableId,
		} as Reservation);
		setTableId("");
	};

	return { tableId, handleTableIdChange, handleSubmit };
}
