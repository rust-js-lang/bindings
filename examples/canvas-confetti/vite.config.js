import { defineConfig } from "vite";
import rustJs from "@rust-js/vite-plugin";

// The example is a Cargo package, whose JS the page imports as
// `rust-js:example`.
export default defineConfig({
  plugins: [rustJs({ cargo: { package: "example" } })],
});
