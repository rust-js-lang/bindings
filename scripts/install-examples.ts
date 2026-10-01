// Install each example as an app installs it, before the tests run: an
// install inside the test's own process isn't seen by its module
// resolution, which has looked at the example's node_modules already.
// Its postinstall, `rust-js-patch`, writes the patch Cargo reads.
import { readdirSync } from "node:fs";
import { join } from "node:path";

const examples = join(import.meta.dir, "..", "examples");
for (const name of readdirSync(examples, { withFileTypes: true }).filter((e) => e.isDirectory()).map((e) => e.name).sort()) {
  const installed = Bun.spawnSync([process.execPath, "install"], { cwd: join(examples, name), stdout: "inherit", stderr: "inherit" });
  if (installed.exitCode !== 0) throw new Error(`examples/${name} didn't install`);
}
