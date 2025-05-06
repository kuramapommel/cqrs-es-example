import { cookies } from "next/headers";
import ReservationTable from "./_components/reservation-table";
import type { Reservation } from "./types";

export default async function Home() {
  const cookie = await cookies();
  const userId = cookie.get("userId")?.value || "";

  const reservations: Reservation[] = await fetch(
    `http://localhost:3080/api/reservations?userId=${userId}`,
  )
    .then((res) => res.json())
    .catch((err) => {
      console.log(err);
      return [];
    });

  return (
    <>
      <header>
        <h1>Reservations</h1>
      </header>
      <main>
        <ReservationTable userId={userId} reservations={reservations} />
      </main>
    </>
  );
}
