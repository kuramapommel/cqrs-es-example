import { defineConfig } from "vitest/config";
import react from "@vitejs/plugin-react";
import tsconfigPaths from "vite-tsconfig-paths";

export default defineConfig({
  plugins: [react(), tsconfigPaths()],
  test: {
    name: "unit",
    environment: "jsdom",
    exclude: ["**/node_modules/**", "**/e2e/**"],
    include: ["src/**/*.test.?(m)[jt]s?(x)"],
  },
});
