"use client";

import { useUser } from "@/contexts/user-context";
import Link from "next/link";
import { useEffect } from "react";

export default function Home() {
	const { setUser } = useUser();
	useEffect(() => {
		const fetchId = async () => {
			await fetch("http://localhost:8080/api/backdoor", {
				credentials: "include",
				method: "POST",
			})
				.then((res) => {
					return res.json();
				})
				.then((data) => {
					setUser({ id: data.userId });
				});
		};

		fetchId();
	});
	return (
		<Link href="/reservation" aria-label="予約ページリンク">
			Reservation
		</Link>
	);
}
