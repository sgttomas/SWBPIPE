import path from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
const workingRoot = process.env.FINAL_WORKING_ROOT!;
const here = path.dirname(fileURLToPath(import.meta.url));
export default defineConfig({ resolve:{alias:{"@final-product":path.join(workingRoot,"apps/desktop/src")}}, test:{environment:"node",include:[path.join(here,"minimal_raw_modulus_constructor_repro.test.ts")],maxWorkers:1,pool:"forks"} });
