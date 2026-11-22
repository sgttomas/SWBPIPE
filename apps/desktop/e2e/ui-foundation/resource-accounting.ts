import type { Page } from "@playwright/test";
import { readCandidateDiagnostics } from "./benchmark-harness";

export const OWNED_RESOURCE_KEYS = [
  "pipeMeshes", "nodeMeshes", "supportMeshes", "componentMeshes",
  "geometries", "materials", "textures", "instanceMatrices", "instanceColors",
  "controls", "eventBindings", "resizeObservers"
] as const;

const REPLACED_MODEL_RESOURCE_KEYS = [
  "pipeMeshes", "nodeMeshes", "supportMeshes", "componentMeshes",
  "instanceMatrices", "instanceColors"
] as const;

const PERSISTENT_VIEWPORT_LIFECYCLE_KEYS = ["controls", "eventBindings", "resizeObservers"] as const;

type OwnedResourceKey = typeof OWNED_RESOURCE_KEYS[number];
type ResourceDelta = Readonly<{ created: number; disposed: number }>;

export function validateOwnedResourceSnapshot(snapshot: any): any {
  const resources = snapshot?.viewport?.resources;
  if (!resources?.owned || !resources?.context) throw new Error("candidate owned-resource snapshot is unavailable");
  const failures = [];
  for (const key of OWNED_RESOURCE_KEYS) {
    for (const field of ["live", "created", "disposed"] as const) {
      const value = resources.owned[field]?.[key];
      if (!Number.isSafeInteger(value) || value < 0) failures.push(`${field}.${key} is not a nonnegative safe integer`);
    }
    if (resources.owned.created?.[key] - resources.owned.disposed?.[key] !== resources.owned.live?.[key]) {
      failures.push(`created.${key} - disposed.${key} != live.${key}`);
    }
  }
  if (!Number.isSafeInteger(resources.owned.generation) || resources.owned.generation < 0) failures.push("owned.generation invalid");
  if (!Number.isSafeInteger(resources.ownedPendingRafCount) || resources.ownedPendingRafCount < 0) failures.push("ownedPendingRafCount invalid");
  if (!Number.isSafeInteger(resources.context.generation) || resources.context.generation < 0) failures.push("context.generation invalid");
  for (const key of ["lostCount", "restoredCount"]) {
    if (!Number.isSafeInteger(resources.context[key]) || resources.context[key] < 0) failures.push(`context.${key} invalid`);
  }
  if (resources.context.canvasConnected !== true) failures.push("context.canvasConnected is not true while viewport is mounted");
  for (const key of ["geometries", "textures"] as const) {
    const value = resources.rendererInfo?.[key];
    if (!Number.isSafeInteger(value) || value < 0) failures.push(`rendererInfo.${key} is not a nonnegative safe integer`);
  }
  return { status: failures.length ? "FAIL_RESOURCE_ACCOUNTING" : "PASS_RESOURCE_ACCOUNTING", failures, resources };
}

/** Two live, timer-separated stable samples; never drives or counts global RAF. */
export async function readSettledResourceBoundary(page: Page, captureOwner = false): Promise<any> {
  await readCandidateDiagnostics(page, true);
  const boundary = await page.evaluate(async ({ captureOwner, keys }) => {
    const startedAt = performance.now();
    const deadline = startedAt + 5_000;
    const observations: any[] = [];
    let prior: { at: number; signature: string; observation: any } | null = null;
    while (performance.now() <= deadline) {
      const snapshot = (globalThis as any).__openPipeStressUiDiagnosticsV1?.readCurrent?.();
      const at = performance.now();
      const resources = snapshot?.viewport?.resources;
      const model = snapshot?.model;
      const render = snapshot?.viewport?.mainRender;
      const cameraSequence = snapshot?.viewport?.camera?.sequence;
      const integer = (value: unknown): boolean => Number.isSafeInteger(value) && Number(value) >= 0;
      const balanced = keys.every((key) =>
        ["created", "disposed", "live"].every((field) => integer(resources?.owned?.[field]?.[key])) &&
        resources.owned.created[key] - resources.owned.disposed[key] === resources.owned.live[key]);
      const valid = balanced && integer(model?.generation) && model.generation > 0 &&
        integer(model?.projectSessionGeneration) && typeof model?.identityHash === "string" &&
        snapshot?.viewport?.generation === model.generation && render?.generation === model.generation &&
        integer(render?.submissionSequence) && render.submissionSequence > 0 &&
        Number.isFinite(render?.submittedAt) && integer(cameraSequence) && cameraSequence > 0 &&
        integer(resources?.owned?.generation) && resources.owned.generation > 0 &&
        resources?.context?.generation === resources.owned.generation &&
        resources.context.canvasConnected === true && integer(resources.context.lostCount) &&
        integer(resources.context.restoredCount) && integer(resources?.rendererInfo?.geometries) &&
        integer(resources?.rendererInfo?.textures) && resources?.ownedPendingRafCount === 0;
      const signature = JSON.stringify({ modelGeneration: model?.generation,
        sessionGeneration: model?.projectSessionGeneration, identityHash: model?.identityHash,
        viewportGeneration: snapshot?.viewport?.generation, render, cameraSequence, resources });
      const observation = { at, valid, modelGeneration: model?.generation,
        sessionGeneration: model?.projectSessionGeneration, identityHash: model?.identityHash,
        viewportGeneration: snapshot?.viewport?.generation, render, cameraSequence, resources };
      observations.push(observation);
      if (valid && prior && prior.signature === signature && at - prior.at >= 100 && at <= deadline) {
        const owner = captureOwner
          ? (globalThis as any).__uifActualAppLifecycleWitnessV1?.readOwnerSnapshot?.() : null;
        return { snapshot, owner, ownerFrozen: captureOwner ? Object.isFrozen(owner) : null,
          settling: { status: "PASS_TWO_STABLE_LIVE_ZERO_SAMPLES", startedAt, deadline,
            minimumSeparationMs: 100, observedSeparationMs: at - prior.at,
            accepted: [prior.observation, observation], observations,
            rafSource: "snapshot.viewport.resources.ownedPendingRafCount (live readCurrent pull)",
            cachedOwnerRafMeaning: "resource-state/teardown cache; not mounted scheduler truth" } };
      }
      if (!valid) prior = null;
      else if (!prior || prior.signature !== signature) prior = { at, signature, observation };
      await new Promise<void>((resolve) => setTimeout(resolve, 100));
    }
    throw new Error(`resource settling timed out: ${JSON.stringify({ startedAt, deadline, observations })}`);
  }, { captureOwner, keys: [...OWNED_RESOURCE_KEYS] });
  const validation = validateOwnedResourceSnapshot(boundary.snapshot);
  if (validation.failures.length || boundary.snapshot.viewport.resources.ownedPendingRafCount !== 0) {
    throw new Error(`invalid settled boundary: ${JSON.stringify({ validation, boundary })}`);
  }
  return boundary;
}

function resourceDeltas(before: any, after: any): Record<OwnedResourceKey, ResourceDelta> {
  return Object.fromEntries(OWNED_RESOURCE_KEYS.map((key) => [key, Object.freeze({
    created: after.owned.created[key] - before.owned.created[key],
    disposed: after.owned.disposed[key] - before.owned.disposed[key]
  })])) as Record<OwnedResourceKey, ResourceDelta>;
}

export async function exerciseRealOpenReplacementCycles(page: Page, cycles: number, timeoutMs: number): Promise<any> {
  if (cycles !== 20) throw new Error("resource qualification requires exactly 20 replacement cycles");
  const saveButton = page.getByRole("button", { name: "Save local", exact: true });
  const initialMessage = await page.getByTestId("local-project-message").textContent();
  await saveButton.click({ timeout: timeoutMs });
  await page.waitForFunction((previous) => {
    const message = document.querySelector('[data-testid="local-project-message"]')?.textContent ?? "";
    const button = [...document.querySelectorAll("button")].find((entry) => entry.textContent?.trim().includes("Save local"));
    return message !== previous && !button?.hasAttribute("disabled");
  }, initialMessage, { timeout: timeoutMs });
  const initialBoundary = await readSettledResourceBoundary(page);
  const initialSnapshot = initialBoundary.snapshot;
  const initialValidation = validateOwnedResourceSnapshot(initialSnapshot);
  if (initialValidation.failures.length) throw new Error(JSON.stringify(initialValidation));
  const initialIdentityHash = initialSnapshot.model.identityHash;
  const plateau = initialValidation.resources.owned.live;
  const rendererPlateau = {
    geometries: initialValidation.resources.rendererInfo.geometries,
    textures: initialValidation.resources.rendererInfo.textures
  };
  const stableResourceGeneration = initialValidation.resources.owned.generation;
  const stableContextGeneration = initialValidation.resources.context.generation;
  const settledPendingRafPlateau = initialValidation.resources.ownedPendingRafCount;
  if (settledPendingRafPlateau !== 0) throw new Error("initial settled owned RAF must be zero");
  if (stableResourceGeneration !== stableContextGeneration) {
    throw new Error("initial owned/context resource generation mismatch");
  }
  const records = [];
  let replacementProfile: Record<OwnedResourceKey, ResourceDelta> | null = null;
  for (let cycle = 1; cycle <= cycles; cycle += 1) {
    const beforeBoundary = await readSettledResourceBoundary(page);
    const before = beforeBoundary.snapshot;
    const beforeValidation = validateOwnedResourceSnapshot(before);
    const openButton = page.getByTestId("open-local-project");
    await openButton.click({ timeout: timeoutMs });
    await page.waitForFunction(({ priorGeneration, priorRenderSequence }) => {
      const snapshot = (globalThis as any).__openPipeStressUiDiagnosticsV1?.readCurrent?.();
      return snapshot?.model?.generation > priorGeneration &&
        snapshot?.viewport?.generation === snapshot.model.generation &&
        snapshot.viewport.mainRender?.generation === snapshot.model.generation &&
        snapshot.viewport.mainRender?.submissionSequence > priorRenderSequence &&
        typeof snapshot.viewport.mainRender?.submittedAt === "number";
    }, {
      priorGeneration: before.model.generation,
      priorRenderSequence: before.viewport.mainRender.submissionSequence
    }, { timeout: timeoutMs });
    const afterBoundary = await readSettledResourceBoundary(page);
    const after = afterBoundary.snapshot;
    const afterValidation = validateOwnedResourceSnapshot(after);
    const failures = [...beforeValidation.failures, ...afterValidation.failures];
    if (after.model.identityHash !== initialIdentityHash) failures.push("model identityHash changed across same-snapshot Open local");
    if (after.model.generation <= before.model.generation) failures.push("model generation did not advance across Open local");
    if (after.model.projectSessionGeneration <= before.model.projectSessionGeneration) {
      failures.push("project session generation did not advance across Open local");
    }
    for (const [boundary, snapshot] of [["before", before], ["after", after]] as const) {
      if (snapshot.viewport.resources.owned.generation !== stableResourceGeneration) {
        failures.push(`${boundary} owned resource generation changed across model replacement`);
      }
      if (snapshot.viewport.resources.context.generation !== stableContextGeneration) {
        failures.push(`${boundary} WebGL context generation changed across model replacement`);
      }
      if (snapshot.viewport.resources.owned.generation !== snapshot.viewport.resources.context.generation) {
        failures.push(`${boundary} owned/context resource generations differ`);
      }
    }
    if (after.viewport.resources.context.lostCount !== initialValidation.resources.context.lostCount) failures.push("unexpected WebGL context loss");
    if (after.viewport.resources.context.restoredCount !== initialValidation.resources.context.restoredCount) failures.push("unexpected WebGL context restore");
    for (const key of OWNED_RESOURCE_KEYS) {
      if (after.viewport.resources.owned.live[key] !== plateau[key]) failures.push(`live plateau changed for ${key}`);
    }
    for (const [boundary, snapshot] of [["before", before], ["after", after]] as const) {
      if (snapshot.viewport.resources.ownedPendingRafCount !== 0) {
        failures.push(`${boundary} settled owned RAF must be zero`);
      }
    }
    for (const key of ["geometries", "textures"] as const) {
      if (after.viewport.resources.rendererInfo[key] !== rendererPlateau[key]) {
        failures.push(`rendererInfo.${key} changed from settled plateau`);
      }
    }
    const deltas = resourceDeltas(before.viewport.resources, after.viewport.resources);
    for (const key of OWNED_RESOURCE_KEYS) {
      const delta = deltas[key];
      if (!Number.isSafeInteger(delta.created) || delta.created < 0 ||
          !Number.isSafeInteger(delta.disposed) || delta.disposed < 0) {
        failures.push(`replacement ${key} delta is not a nonnegative safe integer`);
      }
      if (delta.created !== delta.disposed) {
        failures.push(`replacement created/disposed deltas differ for ${key}`);
      }
      if (replacementProfile &&
          (delta.created !== replacementProfile[key].created || delta.disposed !== replacementProfile[key].disposed)) {
        failures.push(`replacement allocation profile changed for ${key}`);
      }
    }
    for (const key of PERSISTENT_VIEWPORT_LIFECYCLE_KEYS) {
      if (deltas[key].created !== 0 || deltas[key].disposed !== 0) {
        failures.push(`persistent viewport lifecycle resource churned during model replacement: ${key}`);
      }
    }
    for (const key of REPLACED_MODEL_RESOURCE_KEYS) {
      if (before.viewport.resources.owned.live[key] > 0 &&
          deltas[key].disposed < before.viewport.resources.owned.live[key]) {
        failures.push(`model-owned allocation retirement is incomplete for ${key}`);
      }
    }
    replacementProfile ??= deltas;
    records.push({
      cycle,
      status: failures.length ? "FAIL_RESOURCE_REPLACEMENT_CYCLE" : "PASS_RESOURCE_REPLACEMENT_CYCLE",
      failures,
      before: { modelGeneration: before.model.generation, mainRender: before.viewport.mainRender, resources: before.viewport.resources },
      after: { modelGeneration: after.model.generation, mainRender: after.viewport.mainRender, resources: after.viewport.resources },
      settling: { before: beforeBoundary.settling, after: afterBoundary.settling },
      replacementDeltas: deltas
    });
  }
  return {
    status: records.length === 20 && records.every((record) => record.status === "PASS_RESOURCE_REPLACEMENT_CYCLE")
      ? "PASS_20_REAL_OPEN_LOCAL_REPLACEMENT_CYCLES"
      : "FAIL_RESOURCE_REPLACEMENT_QUALIFICATION",
    stableResourceGeneration,
    stableContextGeneration,
    settledPendingRafPlateau,
    initialSettling: initialBoundary.settling,
    settledOwnedLivePlateau: plateau,
    settledRendererPlateau: rendererPlateau,
    replacementProfile,
    cycles: records,
    lifetimeContract: {
      replacements: "The renderer, context, controls, event bindings, ResizeObserver, and gizmo scene persist. Every replacement returns all owned live counters and renderer memory counters to the same settled plateau; created and disposed replacement deltas match; model-owned mesh and instance allocations are retired.",
      finalUnmount: "The separate actual-App lifecycle witness requires every owned live counter and ledger balance to reach zero, no pending owned RAF, and a disconnected canvas."
    },
    limitation: "Owned current/cumulative counters, renderer.info, and source disposal tests do not inspect driver-private allocations or prove global process/GPU memory stability. JavaScript heap and browser-process RSS are reported separately."
  };
}
