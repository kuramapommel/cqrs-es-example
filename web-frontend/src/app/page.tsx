"use client";

import Link from "next/link";
import { useEffect } from "react";

export default function Home() {
  useEffect(() => {
    const fetchUserId = async () => {
      await fetch("http://localhost:8080/api/backdoor", {
        credentials: "include",
        method: "POST",
      });
    };
    fetchUserId();
  });
  return (
    <Link href="/reservation" aria-label="予約ページリンク">
      Reservation
    </Link>
  );
}
