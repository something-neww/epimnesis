import { defineConfig } from "tsdown";

export default defineConfig({
  exports: true,
  alias: {
    "@epimnesis/core": "./src",
  },
});
