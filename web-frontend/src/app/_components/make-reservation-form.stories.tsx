import { expect, jest } from "@storybook/jest";
import type { Meta, StoryObj } from "@storybook/react";
import type { ReactRenderer } from "@storybook/react";
import { userEvent, waitFor, within } from "@storybook/testing-library";
import type { StoryContext } from "@storybook/types";
import type { Reservation } from "../types";
import MakeReservationForm from "./make-reservation-form";

const meta: Meta<typeof MakeReservationForm> = {
  title: "App/Components/MakeReservationForm",
  component: MakeReservationForm,
  tags: ["autodocs"],
  parameters: {
    layout: "centered",
  },
  render: () => {
    jest.spyOn(global, "fetch").mockImplementation(async (input, init) => {
      if (
        input === "http://localhost:8080/api/reservation" &&
        !!init &&
        init.method === "POST"
      ) {
        const response = {
          ok: true,
          json: () =>
            Promise.resolve({
              reservation_id: "test-reservation-id",
            }),
        };

        return response as Response;
      }

      return await fetch(input, init);
    });

    return (
      <MakeReservationForm
        userId="test-user-id"
        onMade={(reservation) => console.log(JSON.stringify(reservation))}
      />
    );
  },
};
export default meta;
type Story = StoryObj<typeof MakeReservationForm>;

export const Default: Story = {
  play: async ({ canvasElement }: { canvasElement: HTMLElement }) => {
    const canvas = within(canvasElement);
    const makeReservationButton = await canvas.findByRole("button", {
      name: "Make Reservation",
    });
    const tableIdInput = await canvas.findByLabelText("table-id");
    await waitFor(() => {
      expect(makeReservationButton).toBeEnabled();
      expect(tableIdInput).toBeInTheDocument();
      expect(tableIdInput).toHaveValue("");
    });
  },
};
export const MakeReservationButton: Story = {
  play: async (
    ctx: StoryContext<
      ReactRenderer,
      {
        userId: string;
        onMade: (newReservation: Reservation) => void;
      }
    >,
  ) => {
    await Default.play?.(ctx);
    const canvas = within(ctx.canvasElement);
    await userEvent.type(canvas.getByLabelText("table-id"), "test-table-id");
    await userEvent.click(
      canvas.getByRole("button", {
        name: "Make Reservation",
      }),
    );
    await waitFor(() => {
      expect(canvas.getByLabelText("table-id")).toHaveValue("");
    });
  },
};
