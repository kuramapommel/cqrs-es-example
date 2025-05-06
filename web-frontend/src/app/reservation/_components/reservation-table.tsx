"use client";

import { useState } from "react";
import type { Reservation } from "../types";
import MakeReservationForm from "./make-reservation-form";

type Props = {
  userId: string; // 動作確認のために props で渡しているが, context などで渡すのが望ましい
  reservations: Reservation[];
};

const ReservationTable = ({ userId, reservations }: Props) => {
  const [reservationList, setReservationList] =
    useState<Reservation[]>(reservations);

  return (
    <>
      <table aria-label="予約一覧">
        <thead>
          <tr>
            <th>予約 ID</th>
            <th>ユーザ ID</th>
            <th>テーブル ID</th>
          </tr>
        </thead>
        <tbody>
          {reservationList.map((reservation: Reservation) => (
            <tr key={reservation.id}>
              <td>{reservation.id}</td>
              <td>{reservation.user_id}</td>
              <td>{reservation.table_id}</td>
            </tr>
          ))}
        </tbody>
      </table>
      <MakeReservationForm
        userId={userId}
        onMade={(newReservation: Reservation) => {
          setReservationList((prev) => [...prev, newReservation]);
        }}
      />
    </>
  );
};

export default ReservationTable;
