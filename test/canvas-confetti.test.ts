// canvas-confetti's binding, as an app uses it: the example, installed as
// an app installs it (scripts/install-examples.ts), built by Vite in
// Cargo's way, and run in a browser,
// where the button fires the real library's confetti, and its promise
// resolves when the confetti is gone.

import { expect, test } from "bun:test";
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { chromium } from "@playwright/test";
import { createServer } from "vite";

const example = join(import.meta.dir, "..", "examples", "canvas-confetti");

test("the example fires canvas-confetti's confetti from Rust, and hears it end", async () => {
  // Installed before the tests, by `bun run test`, as an app installs it;
  // its patch tells Cargo where the binding and the crates it uses are.
  const config = readFileSync(join(example, ".cargo", "config.toml"), "utf8");
  for (const crate of ["rust-js-bindings-canvas-confetti", "rust-js-builtins", "rust-js-webapi"]) expect(config).toContain(`${crate} = { path = `);

  const server = await createServer({ root: example, logLevel: "silent", server: { port: 0 } });
  const browser = await chromium.launch({ headless: true });
  try {
    await server.listen();
    const page = await browser.newPage();
    const errors: string[] = [];
    page.on("pageerror", (error) => errors.push(error.message));
    await page.goto(server.resolvedUrls!.local[0]);
    await page.getByRole("button", { name: "Celebrate" }).click();
    // canvas-confetti draws on a canvas of its own, over the page.
    await page.waitForSelector("canvas", { state: "attached", timeout: 5_000 });
    await page.getByText("done").waitFor({ timeout: 15_000 });
    expect(errors).toEqual([]);
    // The Rust is compiled beside itself, as an app commits it.
    expect(existsSync(join(example, "src", "lib.js"))).toBe(true);
  } finally {
    await browser.close();
    await server.close();
  }
}, 180_000);
