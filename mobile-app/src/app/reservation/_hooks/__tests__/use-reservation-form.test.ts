import type { Reservation } from "@/app/reservation/types";
import { renderHook, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import useReservationForm from "../use-reservation-form";

describe("useReservationForm", () => {
	afterEach(() => {
		vi.resetAllMocks();
	});

	it("should update tableId on change", async () => {
		const mock = async (
			input: string | URL | Request,
			init: RequestInit | undefined,
		) => {
			const expectedUrl = "http://localhost:8080/api/reservation";
			const expectedMethod = "POST";
			const response =
				input !== expectedUrl || !init || init.method !== expectedMethod
					? {
							ok: false,
							json: () =>
								Promise.resolve({
									reservation_id: "test-reservation-id",
								}),
						}
					: { ok: true };

			return response as Response;
		};

		vi.spyOn(global, "fetch").mockImplementationOnce(mock);

		const { result } = renderHook(() =>
			useReservationForm(
				"test-user-id",
				({ id, user_id, table_id }: Reservation) => {
					expect(id).toBe("test-reservation-id");
					expect(user_id).toBe("test-user-id");
					expect(table_id).toBe("test-table-id");
				},
			),
		);
		const event = { target: { value: "test-table-id" } };
		result.current.handleTableIdChange(
			event as React.ChangeEvent<HTMLInputElement>,
		);
		await waitFor(() => {
			expect(result.current.tableId).toBe("test-table-id");
		});
	});
});
