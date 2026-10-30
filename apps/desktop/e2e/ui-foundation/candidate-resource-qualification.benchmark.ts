import { mkdir } from "node:fs/promises";
import path from "node:path";
import { test } from "@playwright/test";
import {
  captureBrowserProcessAndHeapState,
  captureChromiumEnvironment,
  installInstrumentation,
  loadFixture,
  requireEvidenceRoot,
  routeModelFixture,
  waitForFirstUsable,
  writeJson
} from "./benchmark-harness";
import { exerciseRealOpenReplacementCycles } from "./resource-accounting";
import {
  assertBoundCandidateDocumentResponse,
  bindCandidateDriverEntry,
  REQUIRED_CHROMIUM_RUNTIME_BINDING
} from "./candidate-server-response";

const evidenceRoot = requireEvidenceRoot();
const candidateDriverBinding = bindCandidateDriverEntry("candidate-resource-qualification");

test.describe.serial("candidate owned-resource production-browser qualification", () => {
  for (const pipeCount of [1_000, 10_000]) {
    test(`candidate N=${pipeCount} survives 20 real Open local replacements`, async ({ page }) => {
      const fixture = await loadFixture(pipeCount);
      await installInstrumentation(page, { captureGlobalRaf: false });
      await routeModelFixture(page, fixture);
      const documentResponse = await page.goto("/", { waitUntil: "domcontentloaded" });
      assertBoundCandidateDocumentResponse(documentResponse);
      await waitForFirstUsable(page, fixture, 180_000, "candidate");
      const environment = await captureChromiumEnvironment(page);
      const identityFailures = [];
      if (environment.browserProtocolVersion?.product !== REQUIRED_CHROMIUM_RUNTIME_BINDING.product) {
        identityFailures.push(`unexpected browser product: ${environment.browserProtocolVersion?.product ?? "missing"}`);
      }
      if (environment.browserProtocolVersion?.revision !== REQUIRED_CHROMIUM_RUNTIME_BINDING.revision) {
        identityFailures.push(`unexpected browser revision: ${environment.browserProtocolVersion?.revision ?? "missing"}`);
      }
      const processAndHeapBefore = await captureBrowserProcessAndHeapState(page);
      const result = await exerciseRealOpenReplacementCycles(page, 20, 180_000);
      const processAndHeapAfter = await captureBrowserProcessAndHeapState(page);
      const outputDir = path.join(evidenceRoot, "candidate-resource-qualification", String(pipeCount));
      await mkdir(outputDir, { recursive: true });
      await page.screenshot({ path: path.join(outputDir, "after-20-open-local-cycles.png") });
      await writeJson(path.join(outputDir, "resource-replacement-cycles.json"), {
        candidateDriverBinding,
        requiredChromiumRuntimeBinding: REQUIRED_CHROMIUM_RUNTIME_BINDING,
        environment,
        identityFailures,
        processAndHeap: {
          before: processAndHeapBefore,
          after: processAndHeapAfter,
          interpretation: "JavaScript heap and Chromium-reported process RSS are separate observational series; neither is substituted for the owned-resource ledger or renderer.info invariants."
        },
        ...result
      });
      if (identityFailures.length || result.status !== "PASS_20_REAL_OPEN_LOCAL_REPLACEMENT_CYCLES") {
        throw new Error(`candidate resource qualification failed: ${JSON.stringify({ identityFailures, status: result.status })}`);
      }
    });
  }
});
