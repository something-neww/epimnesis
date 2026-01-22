import { defineConfig } from "tsdown";

export default defineConfig({
  exports: true,
  external: ["@epimnesis/node"],
  alias: {
    "@epimnesis/core": "./src",
  },
});
