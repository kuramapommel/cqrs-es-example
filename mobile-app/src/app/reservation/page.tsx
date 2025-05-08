"use client";

import { useUser } from "@/contexts/user-context";
import { useEffect, useState } from "react";
import ReservationTable from "./_components/reservation-table";
import type { Reservation } from "./types";

export default function Home() {
	const { user } = useUser();
	const [reservations, setReservations] = useState<Reservation[]>([]);
	useEffect(() => {
		const fetchReservations = async () => {
			await fetch(`http://localhost:3080/api/reservations?userId=${user.id}`, {
				method: "GET",
				credentials: "include",
			})
				.then((res) => res.json())
				.then((data) => {
					console.log("reservations", data);
					setReservations(data.reservations);
				});
		};

		fetchReservations();
	});

	useEffect(() => {});

	return (
		<>
			<header>
				<h1>Reservations</h1>
			</header>
			<main>
				<ReservationTable userId={user.id} reservations={reservations} />
			</main>
		</>
	);
}
