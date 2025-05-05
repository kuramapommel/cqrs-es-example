import { Hono } from "hono";
import { PrismaClient } from "../generated/prisma";

const app = new Hono();
const prisma = new PrismaClient();

app.get("/reservations", async (c) => {
  const reservations = await prisma.reservations.findMany();
  return c.json(reservations);
});

export default {
  port: 3080,
  fetch: app.fetch,
};
