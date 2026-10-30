import { createHash } from "node:crypto";
import { mkdir, readFile } from "node:fs/promises";
import path from "node:path";
import { test } from "@playwright/test";
import {
  captureBrowserProcessAndHeapState,
  captureChromiumEnvironment,
  installInstrumentation,
  loadFixture,
  requireEvidenceRoot,
  routeModelFixture,
  writeJson
} from "./benchmark-harness";
import {
  bindCandidateDriverEntry,
  REQUIRED_CHROMIUM_RUNTIME_BINDING
} from "./candidate-server-response";
import { OWNED_RESOURCE_KEYS, readSettledResourceBoundary } from "./resource-accounting";

const evidenceRoot = requireEvidenceRoot();
const verifierDesktopRoot = path.resolve(import.meta.dirname, "../..");
const candidateDriverBinding = bindCandidateDriverEntry("candidate-resource-lifecycle-source");
const candidateDesktopRoot = path.join(candidateDriverBinding.candidateSourceRoot, "apps/desktop");

async function bindFiles(root: string, files: readonly string[]): Promise<any[]> {
  const binding = [];
  for (const relative of files) {
    const bytes = await readFile(path.join(root, relative));
    binding.push({ relative, bytes: bytes.length, sha256: createHash("sha256").update(bytes).digest("hex") });
  }
  return binding;
}

test("visible test-only controls repeatedly mount and unmount the actual App owner", async ({ page, browserName }) => {
  const fixture = await loadFixture(1_000);
  await installInstrumentation(page, { captureGlobalRaf: false });
  await routeModelFixture(page, fixture);
  await page.goto("/e2e/ui-foundation/resource-lifecycle-page.html", { waitUntil: "domcontentloaded" });
  let environment: any = null;
  const identityFailures = [];
  const processAndHeapBefore = await captureBrowserProcessAndHeapState(page);
  const records = [];
  let staleRequest: any = null;
  let mountedLivePlateau: Record<string, number> | null = null;
  let mountedRendererPlateau: { geometries: number; textures: number } | null = null;
  for (let cycle = 1; cycle <= 5; cycle += 1) {
    await page.getByRole("button", { name: "Mount actual App", exact: true }).click();
    await page.getByTestId("viewport-canvas").locator("canvas").waitFor({ state: "visible", timeout: 180_000 });
    await page.waitForFunction(() => {
      const snapshot = (globalThis as any).__openPipeStressUiDiagnosticsV1?.readCurrent?.();
      return Number.isSafeInteger(snapshot?.model?.generation) &&
        snapshot.model.generation > 0 && snapshot?.viewport?.status !== "unavailable" &&
        snapshot.viewport.mainRender?.generation === snapshot.model.generation &&
        snapshot.viewport.mainRender?.submissionSequence > 0;
    }, undefined, { timeout: 180_000 });
    if (!environment) {
      environment = await captureChromiumEnvironment(page);
      if (environment.browserProtocolVersion?.product !== REQUIRED_CHROMIUM_RUNTIME_BINDING.product) {
        identityFailures.push(`unexpected browser product: ${environment.browserProtocolVersion?.product ?? "missing"}`);
      }
      if (environment.browserProtocolVersion?.revision !== REQUIRED_CHROMIUM_RUNTIME_BINDING.revision) {
        identityFailures.push(`unexpected browser revision: ${environment.browserProtocolVersion?.revision ?? "missing"}`);
      }
    }
    const mounted = await readSettledResourceBoundary(page, true);
    const failures = [];
    if (!mounted.snapshot?.model?.generation || !mounted.snapshot?.viewport?.camera?.sequence) {
      failures.push("mounted diagnostics unavailable");
    }
    if (!mounted.ownerFrozen) failures.push("source owner snapshot is not frozen while mounted");
    const diagnosticsResources = mounted.snapshot?.viewport?.resources;
    if (diagnosticsResources?.ownedPendingRafCount !== 0) {
      failures.push("mounted live diagnostic ownedPendingRafCount is not zero");
    }
    if (!diagnosticsResources?.owned || !diagnosticsResources?.context || !mounted.owner?.owned || !mounted.owner?.context) {
      failures.push("mounted owned-resource snapshots are unavailable");
    } else {
      if (mounted.owner.owned.generation !== diagnosticsResources.owned.generation ||
          mounted.owner.context.generation !== diagnosticsResources.context.generation) {
        failures.push("source owner snapshot generation differs from production diagnostics");
      }
      if (mounted.owner.owned.generation !== mounted.owner.context.generation) {
        failures.push("mounted owned/context generations differ");
      }
      if (JSON.stringify(mounted.owner.owned) !== JSON.stringify(diagnosticsResources.owned) ||
          JSON.stringify(mounted.owner.context) !== JSON.stringify(diagnosticsResources.context)) {
        failures.push("source owner snapshot differs from production diagnostics snapshot");
      }
      const currentMountedLive: Record<string, number> = { ...mounted.owner.owned.live };
      const retainedMountedLivePlateau: Record<string, number> = mountedLivePlateau ?? currentMountedLive;
      mountedLivePlateau = retainedMountedLivePlateau;
      mountedRendererPlateau ??= {
        geometries: diagnosticsResources.rendererInfo.geometries,
        textures: diagnosticsResources.rendererInfo.textures
      };
      for (const key of OWNED_RESOURCE_KEYS) {
        if (mounted.owner.owned.live[key] !== retainedMountedLivePlateau[key]) {
          failures.push(`mounted live plateau changed for ${key}`);
        }
      }
      for (const key of ["geometries", "textures"] as const) {
        if (diagnosticsResources.rendererInfo[key] !== mountedRendererPlateau[key]) {
          failures.push(`mounted rendererInfo.${key} plateau changed`);
        }
      }
    }
    if (cycle > 1) {
      if (mounted.snapshot.model.generation <= staleRequest.modelGeneration ||
          mounted.snapshot.viewport.camera.sequence <= staleRequest.cameraSequence) {
        failures.push("generation or camera sequence did not advance across remount");
      }
      const stale = await page.evaluate((request) =>
        (globalThis as any).__openPipeStressUiDiagnosticsV1.projectAuthoredPoint(request), staleRequest);
      if (stale.status !== "stale") failures.push("prior projection request was not stale");
    }
    staleRequest = {
      modelGeneration: mounted.snapshot.model.generation,
      cameraSequence: mounted.snapshot.viewport.camera.sequence,
      authoredPoint: fixture.model.nodes[0].position
    };
    await page.getByRole("button", { name: "Unmount actual App", exact: true }).click();
    await page.getByTestId("viewport-canvas").waitFor({ state: "detached", timeout: 30_000 });
    const unmounted = await page.evaluate(() => {
      const owner = (globalThis as any).__uifActualAppLifecycleWitnessV1?.readOwnerSnapshot?.();
      return { owner, ownerFrozen: Object.isFrozen(owner) };
    });
    if (!unmounted.ownerFrozen) failures.push("unmounted owner snapshot is not frozen");
    for (const key of OWNED_RESOURCE_KEYS) {
      if (unmounted.owner?.owned?.live?.[key] !== 0) failures.push(`live.${key} is not zero after unmount`);
      if (unmounted.owner?.owned?.created?.[key] - unmounted.owner?.owned?.disposed?.[key] !== 0) {
        failures.push(`created.${key} - disposed.${key} is not zero after unmount`);
      }
    }
    if (unmounted.owner?.ownedPendingRafCount !== 0) failures.push("ownedPendingRafCount is not zero after unmount");
    if (unmounted.owner?.context?.canvasConnected !== false) failures.push("canvas remains connected after unmount");
    if (unmounted.owner?.owned?.generation !== mounted.owner?.owned?.generation ||
        unmounted.owner?.context?.generation !== mounted.owner?.context?.generation) {
      failures.push("unmounted owner snapshot does not describe the resource retired by this cycle");
    }
    records.push({ cycle, status: failures.length ? "FAIL_ACTUAL_APP_UNMOUNT" : "PASS_ACTUAL_APP_UNMOUNT", failures, mounted, unmounted });
    if (failures.length) break;
  }
  const candidateExecutedFiles = [
    "src/App.tsx",
    "src/features/viewport/PipeViewport.tsx",
    "src/features/viewport/viewportResource.ts",
    "e2e/ui-foundation/resource-lifecycle-page.html",
    "e2e/ui-foundation/resource-lifecycle-page.tsx"
  ];
  const candidateDisposalTestFiles = ["src/features/viewport/viewportResource.test.ts"];
  const verifierDriverFiles = [
    "e2e/ui-foundation/playwright.resource-lifecycle-source.config.ts",
    "e2e/ui-foundation/resource-accounting.ts",
    "e2e/ui-foundation/resource-lifecycle-source.benchmark.ts"
  ];
  const candidateExecutedSourceBinding = await bindFiles(candidateDesktopRoot, candidateExecutedFiles);
  const candidateDisposalTestSourceBinding = await bindFiles(candidateDesktopRoot, candidateDisposalTestFiles);
  const verifierDriverSourceBinding = await bindFiles(verifierDesktopRoot, verifierDriverFiles);
  const processAndHeapAfter = await captureBrowserProcessAndHeapState(page);
  const outputDir = path.join(evidenceRoot, "source-browser-actual-app-lifecycle");
  await mkdir(outputDir, { recursive: true });
  await page.screenshot({ path: path.join(outputDir, "after-five-actual-app-unmounts.png") });
  await writeJson(path.join(outputDir, "actual-app-lifecycle.json"), {
    schema: "openpipestress.ui-foundation.source-browser-actual-app-lifecycle/v1",
    status: records.length === 5 && !identityFailures.length &&
        records.every((record) => record.status === "PASS_ACTUAL_APP_UNMOUNT")
      ? "PASS_FIVE_VISIBLE_ACTUAL_APP_MOUNT_UNMOUNT_CYCLES"
      : "FAIL_ACTUAL_APP_LIFECYCLE",
    browserName,
    evidenceClass: "SOURCE_BROWSER_TEST_ONLY_PAGE_NOT_PRODUCTION_TASK_OR_PERFORMANCE",
    candidateDriverBinding,
    requiredChromiumRuntimeBinding: REQUIRED_CHROMIUM_RUNTIME_BINDING,
    environment,
    identityFailures,
    candidateExecutedSourceBinding,
    candidateDisposalTestSourceBinding,
    verifierDriverSourceBinding,
    mountedRafSource: "records[].mounted.snapshot.viewport.resources.ownedPendingRafCount",
    unmountedRafSource: "records[].unmounted.owner.ownedPendingRafCount",
    mountedLivePlateau,
    mountedRendererPlateau,
    processAndHeap: {
      before: processAndHeapBefore,
      after: processAndHeapAfter,
      interpretation: "JavaScript heap and Chromium-reported process RSS remain separate observations; the unmount verdict comes from the owned ledger, pending RAF, canvas connectivity, and renderer plateau evidence."
    },
    records
  });
  if (records.length !== 5 || identityFailures.length ||
      records.some((record) => record.status !== "PASS_ACTUAL_APP_UNMOUNT")) {
    throw new Error(`actual candidate App lifecycle qualification failed: ${JSON.stringify({ identityFailures, records: records.map(({ cycle, status, failures }) => ({ cycle, status, failures })) })}`);
  }
});
