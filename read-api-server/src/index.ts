import { Hono } from "hono";
import { PrismaClient } from "../generated/prisma";

const app = new Hono();
const prisma = new PrismaClient();

app.get("/api/reservations", async (c) => {
  const userId = c.req.query("user_id"); // ?q=hello の値を取得
  const reservations = await prisma.reservations.findMany({
    where: {
      user_id: userId,
    },
  });
  return c.json(reservations);
});

export default {
  port: 3080,
  fetch: app.fetch,
};
