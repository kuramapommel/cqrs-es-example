import ReservationTable from "./_components/reservation-table";
import type { Reservation } from "./types";

export default async function Home() {
  const { userId } = await fetch("http://localhost:8080/api/backdoor", {
    credentials: "include",
    method: "POST",
  }).then((res) => res.json());
  const reservations: Reservation[] = await fetch(
    `http://localhost:3080/api/reservations?user_id=${userId}`,
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
