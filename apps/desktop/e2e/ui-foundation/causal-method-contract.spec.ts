import { expect, test, type Page } from "@playwright/test";
import { createHash } from "node:crypto";
import { createReadStream } from "node:fs";
import { mkdtemp, readFile, realpath, rm, stat } from "node:fs/promises";
import { tmpdir } from "node:os";
import path from "node:path";
import { validatedPriorBoxBaseline, baselineOutsideFrustumRecord, capturePointInspectorEvidence, expectedVisibleEntityLabel, installInstrumentation, restoreCausalFeedbackInstrumentation, validateMainCanvasHitTarget } from "./benchmark-harness";
import { ORBIT_START_NORMALIZED, assertMainCanvasHitTarget, assertStoppedCausalEvidence, canvasLocalToClient, normalizedCanvasPoint,
  observerValidity, instrumentedPreflightPass, type ObserverRequirements } from "./causal-method-contract";
import { CausalPhaseJournal } from "./causal-phase-journal";
import { selectedEpochEvidence } from "./full-cohort-controller";

const validObserverRequirements: ObserverRequirements = {
  feedbackObservations: [{ markerIdentity: "fixture.1", invocationId: 1, callbackEntryAt: 1, callbackCompletedAt: 1 }],
  pointerTransaction: { down: { eventKind: "pointerdown", pointerId: 1, listenerObservedAt: 0 },
    up: { eventKind: "pointerup", pointerId: 1, listenerObservedAt: 0 } },
  actionMarker: { eventKind: "pointerdown", pointerId: 1, listenerObservedAt: 0 }, actionStartEvent: "pointerdown",
  minimumRafCallbacks: 1, requiredSourceOperations: [], scheduledAdditionalAnimationFrames: 0,
  productObservationRaf: null, restorationStatus: "PASS_FULL_RESTORE"
};

// Existing numerical-accounting controls get explicit zero-cost pointer fixtures.
// Coverage controls below call the production contract directly with independent
// expected observations, so removed/substituted costs cannot alter expectations.
const accountingFixtureValidity: typeof observerValidity = (samples, window, overflow, errors, requirements) => {
  const callback = samples?.find((sample) => sample?.operation === "raf-callback");
  return observerValidity(Array.isArray(samples) ? [...samples,
    { operation: "pointerdown-capture-listener-total", startedAt: 0, completedAt: 0, durationMs: 0 },
    { operation: "pointerup-capture-listener-total", startedAt: 0, completedAt: 0, durationMs: 0 }] : samples,
  window, overflow, errors, requirements ? { ...requirements,
    feedbackObservations: [{ markerIdentity: "fixture.1", invocationId: 1,
      callbackEntryAt: callback?.startedAt ?? 1, callbackCompletedAt: callback?.completedAt ?? 1 }] } : requirements);
};

// The installed Playwright defaults do not promise CDP command-line access.
// Top-level fixture override also covers ordinary playwright.config.ts discovery
// while retaining that project's executablePath and all other launch options.
test.use({
  launchOptions: async ({ launchOptions }, use) => {
    await use({ ...launchOptions,
      args: [...new Set([...(launchOptions.args ?? []), "--enable-automation"])] });
  }
});

async function requireHealthyWebGl1(page: Page): Promise<void> {
  await expect.poll(() => page.evaluate(() => {
    const canvas = document.querySelector("canvas");
    const context = canvas instanceof HTMLCanvasElement ? canvas.getContext("webgl") : null;
    return context !== null && !context.isContextLost();
  }), { timeout: 5_000, message: "a live WebGL1 context is required; this gate cannot pass by skipping" }).toBe(true);
}

async function sha256File(filePath: string): Promise<string> {
  const hash = createHash("sha256");
  await new Promise<void>((resolve, reject) => {
    const stream = createReadStream(filePath);
    stream.on("data", (chunk) => hash.update(chunk));
    stream.once("error", reject);
    stream.once("end", resolve);
  });
  return hash.digest("hex");
}

async function captureSyntheticBrowserIdentity(browser: any, browserName: string, identity: any): Promise<void> {
  let session: any = null;
  let primaryError: unknown;
  let primaryFailed = false;
  identity.status = "CAPTURING";
  try {
    if (browserName !== "chromium") throw new Error(`causal method contract requires Chromium, received ${browserName}`);
    identity.browserName = browserName;
    identity.playwrightBrowserVersion = browser.version();
    session = await browser.newBrowserCDPSession();
    const version = await session.send("Browser.getVersion");
    // Persist into caller-owned evidence before a later command can fail.
    identity.cdp = Object.freeze({
      protocolVersion: version.protocolVersion,
      product: version.product,
      revision: version.revision,
      userAgent: version.userAgent,
      jsVersion: version.jsVersion
    });
    const commandLine = await session.send("Browser.getBrowserCommandLine");
    const args: string[] = Array.isArray(commandLine.arguments) ? commandLine.arguments.map(String) : [];
    identity.commandLineArgumentNames = Object.freeze(args.slice(1).map((argument: string) => {
      const separator = argument.indexOf("=");
      if (!argument.startsWith("--")) return "<positional>";
      return separator >= 0 ? argument.slice(0, separator) : argument;
    }));
    if (!args.includes("--enable-automation")) throw new Error("actual Chromium command line lacks the required identity flag");
    if (!args[0] || !path.isAbsolute(args[0])) throw new Error("actual Chromium command line did not expose an absolute executable");
    const executablePath = await realpath(args[0]);
    const executableStat = await stat(executablePath);
    if (!executableStat.isFile()) throw new Error("actual Chromium executable identity is not a regular file");
    identity.executable = Object.freeze({ path: executablePath, bytes: executableStat.size,
      sha256: await sha256File(executablePath) });
    identity.status = "PASS_BROWSER_IDENTITY_CAPTURE";
  } catch (error) {
    primaryError = error;
    primaryFailed = true;
  } finally {
    if (session !== null) {
      try { await session.detach(); }
      catch (error) {
        identity.detachError = String(error);
        if (!primaryFailed) { primaryError = error; primaryFailed = true; }
      }
    }
  }
  if (primaryFailed) {
    identity.status = "FAIL_BROWSER_IDENTITY_CAPTURE";
    identity.error = String(primaryError);
    throw primaryError;
  }
}

test("nonzero canvas origin and prescribed orbit start retain client coordinates", () => {
  const translated = canvasLocalToClient({ x: 490.5522845034418, y: 299.15649722781455 },
    { x: 228, y: 309.5625, width: 919, height: 533 });
  expect(translated.x).toBeCloseTo(718.5522845034418, 12);
  expect(translated.y).toBeCloseTo(608.7189972278146, 12);
  const start = normalizedCanvasPoint({ x: 10, y: 20, width: 100, height: 200 }, ORBIT_START_NORMALIZED);
  expect(start.x).toBe(60);
  expect(start.y).toBeCloseTo(20 + 200 * (0.5 + 0.23 * Math.sin(Math.PI / 3)), 12);
});

test("overlay target and observer overflow fail closed", () => {
  expect(() => assertMainCanvasHitTarget({ status: "FAIL_MAIN_CANVAS_TARGET", clientPoint: { x: 1, y: 1 },
    canvasEpoch: 1, targetTag: "BUTTON", targetTestId: "viewport-select-node", canvasConnected: true,
    exactCanvasTarget: false })).toThrow(/not the current main canvas/);
  const callback = [{ operation: "raf-callback", startedAt: 1, completedAt: 1.1, durationMs: 0.01 }];
  expect(accountingFixtureValidity(callback, { start: 1, end: 11 }, 1, 0, validObserverRequirements).status).toBe("FAIL_OBSERVER_STRUCTURAL_EVIDENCE");
  expect(accountingFixtureValidity(callback, { start: 1, end: 11 }, 0, 1, validObserverRequirements).status).toBe("FAIL_OBSERVER_STRUCTURAL_EVIDENCE");
});

test("RAF callback percentile cannot be diluted by cheap registrations", () => {
  const callbacks = Array.from({ length: 20 }, (_, index) => ({
    operation: "raf-callback", startedAt: 10 + index * 2, completedAt: 11 + index * 2, durationMs: 1
  }));
  const registrations = Array.from({ length: 500 }, (_, index) => ({
    operation: "raf-registration", startedAt: 100 + index * 0.01,
    completedAt: 100.001 + index * 0.01, durationMs: 0.001
  }));
  const validity = accountingFixtureValidity([...callbacks, ...registrations], { start: 0, end: 1_000 }, 0, 0, validObserverRequirements);
  expect(validity.totalPercent).toBeLessThanOrEqual(3);
  expect(validity.byOperation["raf-registration"].p95Ms).toBe(0.001);
  expect(validity.p95Ms).toBe(1);
  expect(validity.percentilePopulation).toBe("RAF_CALLBACK_ONLY");
  expect(validity.status).toBe("PASS_OBSERVER_STRUCTURAL_EVIDENCE");
  expect(validity.rawOverheadDiagnostics.status).toBe("EXCEEDS_RAW_REFERENCES");
  expect(validity.rawOverheadDiagnostics.references).toEqual({ p95Ms: 0.5, maximumMs: 2, totalPercent: 3 });
});

test("invalid samples fail and crossing samples are charged in full", () => {
  const crossing = { operation: "raf-callback", startedAt: 9.9, completedAt: 10.2, durationMs: 0.2 };
  const crossed = accountingFixtureValidity([crossing], { start: 10, end: 20 }, 0, 0, validObserverRequirements);
  expect(crossed.status).toBe("PASS_OBSERVER_STRUCTURAL_EVIDENCE");
  expect(crossed.boundaryCrossingSampleCount).toBe(1);
  expect(crossed.totalCostMs).toBe(0.2);
  const invalid = accountingFixtureValidity([
    crossing,
    { operation: "raf-registration", startedAt: 11, completedAt: 12, durationMs: -0.1 },
    { operation: "raf-cancellation", startedAt: 12, completedAt: Number.NaN, durationMs: 0.01 }
  ], { start: 10, end: 20 }, 0, 0, validObserverRequirements);
  expect(invalid.invalidSampleCount).toBe(2);
  expect(invalid.status).toBe("FAIL_OBSERVER_STRUCTURAL_EVIDENCE");
});

test("forced derivation failure preserves immutable earlier journals and a terminal failure", async () => {
  const directory = await mkdtemp(path.join(tmpdir(), "uif-causal-journal-"));
  try {
    const journal = new CausalPhaseJournal(directory, "synthetic.journal");
    await journal.write("initialized", { binding: "frozen" });
    await journal.write("prepared", { point: "bound" });
    try { throw new Error("FORCED_DERIVATION_FAILURE"); }
    catch (error) { await journal.write("failed", { primaryError: String(error) }); }
    await expect(readFile(path.join(directory, "00-initialized.json"), "utf8")).resolves.toMatch(/\n$/);
    await expect(readFile(path.join(directory, "01-prepared.json"), "utf8")).resolves.toContain('"point": "bound"');
    await expect(readFile(path.join(directory, "06-failed.json"), "utf8")).resolves.toContain("FORCED_DERIVATION_FAILURE");
    await expect(journal.write("prepared", {})).rejects.toThrow(/immutable/);
  } finally {
    await rm(directory, { recursive: true, force: true });
  }
});

test("observer setup failure still invokes the original RAF callback exactly once and restore removes listeners", async ({ page, browser, browserName }, testInfo) => {
  let observation: any = null;
  let restored: any = null;
  let cleanup: any = null;
  const runtimeIdentity: any = { status: "NOT_STARTED" };
  let runtimeQualification = "NOT_QUALIFIED";
  const configuredChromiumExecutable = (testInfo.config.metadata.chromiumExecutable as any) ?? null;
  const configuredChromiumSource = (testInfo.config.metadata.chromiumSourceBinding as any) ?? null;
  let primaryError: unknown;
  let primaryFailed = false;
  const cleanupErrors: string[] = [];
  const pageErrors: string[] = [];
  const onPageError = (error: Error) => pageErrors.push(error.message);
  page.on("pageerror", onPageError);
  try {
    expect(page.context().browser()).toBe(browser);
    await captureSyntheticBrowserIdentity(browser, browserName, runtimeIdentity);
    if (configuredChromiumExecutable !== null || configuredChromiumSource !== null) {
      expect(configuredChromiumExecutable).not.toBeNull();
      expect(configuredChromiumSource).not.toBeNull();
      expect(runtimeIdentity.executable).toEqual({ path: configuredChromiumExecutable.executablePath,
        bytes: configuredChromiumExecutable.bytes, sha256: configuredChromiumExecutable.sha256 });
      expect(runtimeIdentity.playwrightBrowserVersion).toBe(configuredChromiumSource.browserVersion);
      expect(runtimeIdentity.cdp.product).toBe(configuredChromiumSource.product);
      expect(runtimeIdentity.cdp.revision).toBe(configuredChromiumSource.revision);
      runtimeQualification = "PASS_CONFIGURED_EXECUTABLE_AND_CDP_SOURCE_BINDING";
    } else {
      runtimeQualification = "OBSERVED_ORDINARY_RUNTIME_WITHOUT_STRICT_PIN_METADATA";
    }
    await installInstrumentation(page, { causalFeedbackMarkers: true, syntheticObserverSetupFault: true });
    await page.goto("data:text/html,<div data-testid='viewport-canvas'><canvas width='64' height='64'></canvas></div>");
    await requireHealthyWebGl1(page);
    observation = await page.evaluate(async () => {
      const canvas = document.querySelector("canvas")!;
      const gl = canvas.getContext("webgl");
      if (!gl || gl.isContextLost()) throw new Error("OWNED_FIXTURE_WEBGL1_UNAVAILABLE_OR_LOST");
      // Own the real fixture context through both callbacks and cleanup. This is
      // fixture determinism, not an assertion about the historical failure's cause.
      (globalThis as any).__uifSyntheticSetupFixture = {
        canvas, gl, originalClear: gl.clear,
        clearOwnDescriptor: Object.getOwnPropertyDescriptor(gl, "clear") ?? null
      };
      const state = (globalThis as any).__uifHarness;
      const causal = state.causal;
      const records: any[] = [];
      const requestedAtStart = state.rafRequested;
      const completedAtStart = state.rafCompleted;
      for (const throwsOriginal of [false, true]) {
        const token = throwsOriginal ? "synthetic.setup.throw" : "synthetic.setup.return";
        if (gl.isContextLost()) throw new Error("OWNED_FIXTURE_CONTEXT_LOST_BEFORE_ARM");
        causal.arm({ token, phase: "baseline", feedbackKind: "orbit", actionStartEvent: "pointerdown",
          expectedActionTargetTestId: "viewport-canvas", maximumFeedbackMarkers: 2,
          actionIdentity: { synthetic: true, throwsOriginal }, baselineExpectation: {} });
        if (causal.canvas !== canvas || causal.context !== gl || gl.isContextLost()) {
          throw new Error("OWNED_FIXTURE_ARM_CONTEXT_IDENTITY_OR_HEALTH_FAILED");
        }
        canvas.dispatchEvent(new PointerEvent("pointerdown", { bubbles: true, pointerId: 7, clientX: 2, clientY: 2 }));
        const requestedBefore = state.rafRequested;
        const completedBefore = state.rafCompleted;
        const originalError = new Error(`SYNTHETIC_ORIGINAL_CALLBACK_THROW:${token}`);
        let originalErrorIdentityObserved = false;
        let count = 0;
        let timer = 0;
        let errorListener: ((event: ErrorEvent) => void) | null = null;
        try {
          await new Promise<void>((resolve, reject) => {
            timer = window.setTimeout(() => reject(new Error(`OWNED_FIXTURE_CALLBACK_TIMEOUT:${token}`)), 2_000);
            if (throwsOriginal) {
              errorListener = (event: ErrorEvent) => {
                if (event.error === originalError) {
                  originalErrorIdentityObserved = true;
                  resolve();
                }
              };
              addEventListener("error", errorListener);
            }
            requestAnimationFrame(() => {
              count += 1;
              if (throwsOriginal) throw originalError;
              resolve();
            });
          });
        } finally {
          clearTimeout(timer);
          if (errorListener) removeEventListener("error", errorListener);
        }
        const evidence = causal.stop(token);
        records.push({ token, throwsOriginal, count, originalErrorIdentityObserved,
          expectedErrorMessage: throwsOriginal ? originalError.message : null,
          requested: state.rafRequested - requestedBefore, completed: state.rafCompleted - completedBefore,
          pending: state.pendingRaf.size, activeInvocationRestored: causal.activeInvocation === null,
          sameOwnedContext: causal.context === gl, contextLost: gl.isContextLost(), evidence });
      }
      return { records, requested: state.rafRequested - requestedAtStart,
        completed: state.rafCompleted - completedAtStart, pending: state.pendingRaf.size };
    });
    expect(observation.records).toHaveLength(2);
    expect(observation.requested).toBe(2);
    expect(observation.completed).toBe(2);
    expect(observation.pending).toBe(0);
    for (const record of observation.records) {
      expect(record.count).toBe(1);
      expect(record.requested).toBe(1);
      expect(record.completed).toBe(1);
      expect(record.pending).toBe(0);
      expect(record.activeInvocationRestored).toBe(true);
      expect(record.sameOwnedContext).toBe(true);
      expect(record.contextLost).toBe(false);
      expect(record.evidence.active.actionMarker).not.toBeNull();
      expect(record.evidence.active.feedbackMarkers).toHaveLength(0);
      expect(record.evidence.active.observerErrors).toHaveLength(1);
      expect(record.evidence.active.observerErrors[0]).toMatchObject({
        code: "RAF_OBSERVER_SETUP_FAILED", error: "Error: SYNTHETIC_OBSERVER_SETUP_FAULT"
      });
      expect(record.originalErrorIdentityObserved).toBe(record.throwsOriginal);
    }
  } catch (error) {
    primaryError = error;
    primaryFailed = true;
  } finally {
    try { restored = await restoreCausalFeedbackInstrumentation(page); }
    catch (error) { cleanupErrors.push(`restore: ${String(error)}`); }
    try {
      cleanup = await page.evaluate(async () => {
        const fixture = (globalThis as any).__uifSyntheticSetupFixture;
        if (!fixture) return { fixtureAbsent: true };
        try {
          const state = (globalThis as any).__uifHarness;
          const own = Object.getOwnPropertyDescriptor(fixture.gl, "clear") ?? null;
          const before = fixture.clearOwnDescriptor;
          const clearDescriptorRestored = before === null ? own === null : own !== null &&
            ["configurable", "enumerable", "writable", "value", "get", "set"].every(key => own[key as keyof PropertyDescriptor] === before[key]);
          const clearFunctionRestored = fixture.gl.clear === fixture.originalClear;
          const snapshot = () => JSON.stringify({
            requested: state.rafRequested, completed: state.rafCompleted, cancelled: state.rafCancelled,
            pending: state.pendingRaf.size, eventStarts: state.eventStarts,
            causal: state.causal.read("synthetic.setup.throw")
          });
          const beforeEvents = snapshot();
          fixture.canvas.dispatchEvent(new PointerEvent("pointerdown", { bubbles: true, pointerId: 91 }));
          fixture.canvas.dispatchEvent(new PointerEvent("pointerup", { bubbles: true, pointerId: 91 }));
          const input = document.createElement("input");
          input.setAttribute("data-testid", "synthetic-post-restore-input");
          document.body.append(input);
          try {
            input.value = "after restore";
            input.dispatchEvent(new Event("input", { bubbles: true }));
          } finally { input.remove(); }
          const listenersInactive = snapshot() === beforeEvents;
          fixture.gl.clear(fixture.gl.COLOR_BUFFER_BIT);
          let cancelledCallbackCount = 0;
          let liveCallbackCount = 0;
          const cancelledHandle = requestAnimationFrame(() => { cancelledCallbackCount += 1; });
          cancelAnimationFrame(cancelledHandle);
          let timer = 0;
          try {
            await new Promise<void>((resolve, reject) => {
              timer = window.setTimeout(() => reject(new Error("POST_RESTORE_NATIVE_RAF_TIMEOUT")), 2_000);
              requestAnimationFrame(() => { liveCallbackCount += 1; resolve(); });
            });
          } finally { clearTimeout(timer); }
          return { clearDescriptorRestored, clearFunctionRestored, listenersInactive,
            cancelledCallbackCount, liveCallbackCount, harnessUntouchedByPostRestoreWork: snapshot() === beforeEvents,
            pending: state.pendingRaf.size, contextLost: fixture.gl.isContextLost() };
        } finally { delete (globalThis as any).__uifSyntheticSetupFixture; }
      });
    } catch (error) { cleanupErrors.push(`post-restore effects: ${String(error)}`); }
    page.off("pageerror", onPageError);
    try {
      await testInfo.attach("synthetic-setup-owned-context-proof", { contentType: "application/json",
        body: Buffer.from(JSON.stringify({ configuredChromiumExecutable, configuredChromiumSource, runtimeIdentity, runtimeQualification,
          observation, restored, cleanup, pageErrors, cleanupErrors,
          primaryError: primaryFailed ? String(primaryError) : null }, null, 2)) });
    } catch (error) { cleanupErrors.push(`evidence attachment: ${String(error)}`); }
  }
  if (primaryFailed) throw primaryError;
  expect(cleanupErrors).toEqual([]);
  await expect.poll(() => pageErrors, { timeout: 2_000 }).toEqual([
    "SYNTHETIC_ORIGINAL_CALLBACK_THROW:synthetic.setup.throw"
  ]);
  expect(restored.listenersRemoved).toEqual(["pointerdown", "pointerup", "input"]);
  expect(restored.pendingNativeHandlesUntouched).toBe(0);
  expect(restored.overflow.total).toBe(0);
  expect(restored.errors).toHaveLength(2);
  for (const error of restored.errors) expect(error).toMatchObject({
    code: "RAF_OBSERVER_SETUP_FAILED", error: "Error: SYNTHETIC_OBSERVER_SETUP_FAULT"
  });
  expect(cleanup).toMatchObject({ clearDescriptorRestored: true, clearFunctionRestored: true,
    listenersInactive: true, cancelledCallbackCount: 0, liveCallbackCount: 1,
    harnessUntouchedByPostRestoreWork: true, pending: 0, contextLost: false });
});

test("actual elementFromPoint rejects an intercepting overlay", async ({ page }) => {
  await installInstrumentation(page, { causalFeedbackMarkers: true });
  await page.goto("data:text/html,<div data-testid='viewport-canvas' style='position:relative;width:100px;height:100px'><canvas width='100' height='100'></canvas><button data-testid='overlay' style='position:absolute;inset:0'>overlay</button></div>");
  const evidence = await validateMainCanvasHitTarget(page, { x: 20, y: 20 });
  expect(evidence.status).toBe("FAIL_MAIN_CANVAS_TARGET");
  expect(evidence.targetTestId).toBe("overlay");
});

test("candidate diagnostics without same-callback DOM and positive submission cannot emit qualifying feedback", async ({ page }) => {
  await installInstrumentation(page, { causalFeedbackMarkers: true });
  await page.goto("data:text/html,<div data-testid='viewport-canvas'><canvas width='64' height='64'></canvas></div>");
  await requireHealthyWebGl1(page);
  const evidence = await page.evaluate(async () => {
    const canvas = document.querySelector("canvas") as HTMLCanvasElement;
    const gl = canvas.getContext("webgl")!;
    const expectedRef = { type: "pipe", id: "pipe:expected" };
    const snapshot = Object.freeze({ model: Object.freeze({ generation: 1 }), viewport: Object.freeze({ generation: 1,
      mainRender: Object.freeze({ generation: 1, submissionSequence: 2 }), camera: Object.freeze({ sequence: 2 }),
      selection: Object.freeze({ generation: 1, actionSequence: 1, orderedRefs: Object.freeze([Object.freeze(expectedRef)]),
        primaryRef: Object.freeze(expectedRef), inputKind: "pointer", pointerDownAt: 1, publishedAt: 2,
        renderSubmissionSequence: 0 }), inspector: Object.freeze({ generation: 1, ref: Object.freeze(expectedRef) }) }) });
    Object.defineProperty(globalThis, "__openPipeStressUiDiagnosticsV1", { value: Object.freeze({ readCurrent: () => snapshot }),
      configurable: false, writable: false });
    const causal = (globalThis as any).__uifHarness.causal;
    causal.arm({ token: "synthetic.dom", phase: "candidate", feedbackKind: "point-selection", actionStartEvent: "pointerdown",
      expectedActionTargetTestId: "viewport-canvas", maximumFeedbackMarkers: 1,
      actionIdentity: { expectedCssPoint: { x: 1, y: 1 } }, candidateExpectation: { modelGeneration: 1,
        priorRenderSubmissionSequence: 1, priorActionSequence: 0, expectedRef } });
    canvas.dispatchEvent(new PointerEvent("pointerdown", { bubbles: true, pointerId: 9, clientX: 1, clientY: 1 }));
    await new Promise<void>((resolve) => requestAnimationFrame(() => { gl.clear(gl.COLOR_BUFFER_BIT); resolve(); }));
    canvas.dispatchEvent(new PointerEvent("pointerup", { bubbles: true, pointerId: 9, clientX: 1, clientY: 1 }));
    return causal.stop("synthetic.dom");
  });
  expect(evidence.active.feedbackMarkers).toHaveLength(0);
  expect(evidence.active.feedbackRejectionCounts.CONTENT_NOT_READY).toBeGreaterThan(0);
  expect(() => assertStoppedCausalEvidence(evidence, "point-selection", true)).toThrow(/incomplete/);
});

test("clear sentinel retains original WebGL clear work separately from observer cost", async ({ page }) => {
  await installInstrumentation(page, { causalFeedbackMarkers: true });
  await page.goto("data:text/html,<div data-testid='viewport-canvas'><canvas width='64' height='64'></canvas></div>");
  await requireHealthyWebGl1(page);
  const evidence = await page.evaluate(async () => {
    const canvas = document.querySelector("canvas") as HTMLCanvasElement;
    const gl = canvas.getContext("webgl")!;
    const originalClear = gl.clear.bind(gl);
    Object.defineProperty(gl, "clear", { configurable: true, writable: true, value(mask: number) {
      const startedAt = performance.now();
      while (performance.now() - startedAt < 10) { /* bounded synthetic product work */ }
      return originalClear(mask);
    } });
    const causal = (globalThis as any).__uifHarness.causal;
    causal.arm({ token: "synthetic.clear", phase: "baseline", feedbackKind: "orbit", actionStartEvent: "pointerdown",
      expectedActionTargetTestId: "viewport-canvas", maximumFeedbackMarkers: 2,
      actionIdentity: { synthetic: true }, baselineExpectation: {} });
    canvas.dispatchEvent(new PointerEvent("pointerdown", { bubbles: true, pointerId: 11, clientX: 1, clientY: 1 }));
    await new Promise<void>((resolve) => requestAnimationFrame(() => { gl.clear(gl.COLOR_BUFFER_BIT); resolve(); }));
    canvas.dispatchEvent(new PointerEvent("pointerup", { bubbles: true, pointerId: 11, clientX: 1, clientY: 1 }));
    return causal.stop("synthetic.clear");
  });
  expect(evidence.active.feedbackMarkers).toHaveLength(1);
  const marker = evidence.active.feedbackMarkers[0];
  expect(marker.clearCallCount).toBe(1);
  expect(marker.clearProductWorkDurationMs).toBeGreaterThanOrEqual(8);
  expect(marker.clearSentinelObserverCostMs).toBeGreaterThanOrEqual(0);
  expect(evidence.active.observerSamples.find((sample: any) => sample.operation === "raf-callback").durationMs)
    .toBeLessThan(marker.clearProductWorkDurationMs);
});


test("source-bound page-edge uncertainty retains possibly intersecting observer work", () => {
  const result = accountingFixtureValidity([
    { operation: "raf-callback", startedAt: 9.8, completedAt: 9.9, durationMs: 0.1 },
    { operation: "raf-registration", startedAt: 20.1, completedAt: 20.2, durationMs: 0.1 }
  ], { start: 10, end: 20 }, 0, 0, validObserverRequirements);
  expect(result.sampleCount).toBe(2);
  expect(result.totalCostMs).toBe(0.2);
  expect(result.observedWindowMs).toBe(10);
  expect(result.clockMeasurement.quantumBoundMs).toBe(0.1);
  expect(result.clockMeasurement.oneDifferenceUncertaintyBoundMs).toBe(0.2);
});


for (const headingCase of ["valid", "wrong-label", "wrong-typed-identity"] as const) {
  test(`candidate point feedback requires product-shaped compact heading: ${headingCase}`, async ({ page }) => {
    await installInstrumentation(page, { causalFeedbackMarkers: true });
    await page.goto("data:text/html,<div data-testid='viewport-canvas'><canvas width='64' height='64'></canvas></div><div data-testid='command-selection-readout'></div><div class='workspace-pane-inspector'><div class='panel inspector' data-testid='property-inspector'><h2></h2></div></div>");
    await requireHealthyWebGl1(page);
    const evidence = await page.evaluate(async (variant) => {
      const canvas = document.querySelector("canvas") as HTMLCanvasElement;
      const gl = canvas.getContext("webgl")!;
      const expectedRef = { type: "pipe", id: "pipe:UIF-00266" };
      // Independent fixture-shaped expectation; do not derive this from actual DOM.
      const expectedInspectorHeading = "UI benchmark pipe 00266 — pipe: pipe:UIF-00266";
      const causal = (globalThis as any).__uifHarness.causal;
      let snapshot: any = null;
      Object.defineProperty(globalThis, "__openPipeStressUiDiagnosticsV1", {
        value: Object.freeze({ readCurrent: () => snapshot }), configurable: false, writable: false
      });
      const token = `synthetic.candidate.${variant}`;
      causal.arm({ token, phase: "candidate", feedbackKind: "point-selection", actionStartEvent: "pointerdown",
        expectedActionTargetTestId: "viewport-canvas", maximumFeedbackMarkers: 1,
        actionIdentity: { expectedRef, expectedCssPoint: { x: 1, y: 1 } },
        candidateExpectation: { modelGeneration: 1, priorRenderSubmissionSequence: 1,
          priorActionSequence: 0, expectedRef, expectedInspectorHeading } });
      canvas.dispatchEvent(new PointerEvent("pointerdown", { bubbles: true, pointerId: 17, clientX: 1, clientY: 1 }));
      const pointerDownAt = performance.now();
      await new Promise<void>((resolve) => requestAnimationFrame(() => {
        const heading = document.querySelector("h2")!;
        heading.textContent = variant === "wrong-label" ? "Other pipe " : "UI benchmark pipe 00266 ";
        const typedIdentity = document.createElement("span");
        typedIdentity.className = "typed-identity";
        typedIdentity.textContent = variant === "wrong-typed-identity"
          ? "— node: pipe:UIF-00266" : "— pipe: pipe:UIF-00266";
        heading.append(typedIdentity);
        document.querySelector('[data-testid="command-selection-readout"]')!.textContent =
          "Selected pipe: pipe:UIF-00266; 0 queued";
        // Current publication follows the captured pointer boundary and is part
        // of this actual color-clear callback, not a fabricated feedback record.
        snapshot = Object.freeze({ model: Object.freeze({ generation: 1 }), viewport: Object.freeze({ generation: 1,
          mainRender: Object.freeze({ generation: 1, submissionSequence: 2 }), camera: Object.freeze({ sequence: 2 }),
          selection: Object.freeze({ generation: 1, actionSequence: 1,
            orderedRefs: Object.freeze([Object.freeze(expectedRef)]), primaryRef: Object.freeze(expectedRef),
            inputKind: "pointer", pointerDownAt, publishedAt: performance.now(), renderSubmissionSequence: 2 }),
          inspector: Object.freeze({ generation: 1, ref: Object.freeze(expectedRef) }) }) });
        gl.clear(gl.COLOR_BUFFER_BIT);
        resolve();
      }));
      canvas.dispatchEvent(new PointerEvent("pointerup", { bubbles: true, pointerId: 17, clientX: 1, clientY: 1 }));
      return causal.stop(token);
    }, headingCase);
    expect(evidence.active.actionMarker).not.toBeNull();
    expect(evidence.active.observerErrors).toHaveLength(0);
    expect(evidence.active.rejections).toHaveLength(0);
    if (headingCase === "valid") {
      expect(evidence.active.feedbackMarkers).toHaveLength(1);
      expect(() => assertStoppedCausalEvidence(evidence, "point-selection", true)).not.toThrow();
      expect(evidence.active.feedbackMarkers[0]).toMatchObject({ mainContextColorClearObserved: true,
        observed: { inspectorHeading: "UI benchmark pipe 00266 — pipe: pipe:UIF-00266",
          readout: "Selected pipe: pipe:UIF-00266; 0 queued", mainRenderSubmissionSequence: 2,
          selectionPrimaryRef: { type: "pipe", id: "pipe:UIF-00266" }, inspectorGeneration: 1 } });
    } else {
      expect(evidence.active.feedbackMarkers).toHaveLength(0);
      expect(evidence.active.feedbackRejectionCounts.CONTENT_NOT_READY).toBeGreaterThan(0);
      expect(() => assertStoppedCausalEvidence(evidence, "point-selection", true)).toThrow(/incomplete/);
    }
    expect((await restoreCausalFeedbackInstrumentation(page)).status).toBe("PASS_FULL_RESTORE");
  });
}


test("raw zero costs remain uncertain rather than a true-overhead pass", () => {
  const result = accountingFixtureValidity([{ operation: "raf-callback", startedAt: 1, completedAt: 1, durationMs: 0 }],
    { start: 1, end: 11 }, 0, 0, validObserverRequirements);
  expect(result.status).toBe("PASS_OBSERVER_STRUCTURAL_EVIDENCE");
  expect(result.totalCostMs).toBe(0);
  expect(result.rawOverheadDiagnostics.status).toBe("WITHIN_RAW_REFERENCES_TRUE_COST_UNPROVED");
  expect(result.rawOverheadDiagnostics.trueCostStatus).toBe("UNPROVED_INCLUDING_UNBOUNDED_RECORDING_TAIL");
  expect(result.clockMeasurement.oneDifferenceUncertaintyBoundMs).toBe(0.2);
});

test("missing, malformed, negative and nonfinite observer readings fail structurally", () => {
  const valid = { operation: "raf-callback", startedAt: 1, completedAt: 2, durationMs: 0.1 };
  for (const samples of [undefined, [], [undefined], [{ ...valid, durationMs: undefined }],
    [{ ...valid, durationMs: -0.1 }], [{ ...valid, durationMs: Number.POSITIVE_INFINITY }],
    [{ ...valid, startedAt: Number.NaN }], [{ ...valid, completedAt: 0 }],
    [{ ...valid, operation: "unaccounted-operation" }]]) {
    const result = accountingFixtureValidity(samples as any, { start: 1, end: 11 }, 0, 0, validObserverRequirements);
    expect(result.status).toBe("FAIL_OBSERVER_STRUCTURAL_EVIDENCE");
    expect(result.rawOverheadDiagnostics.status).toBe("UNAVAILABLE_INVALID_EVIDENCE");
  }
});

test("expected RAF and action operation populations cannot disappear", () => {
  const callback = { operation: "raf-callback", startedAt: 1, completedAt: 2, durationMs: 0.1 };
  const requirements = { ...validObserverRequirements, requiredSourceOperations: ["raf-registration",
    "pointerdown-capture-listener-total", "pointerup-capture-listener-total"] };
  for (const required of requirements.requiredSourceOperations) {
    const samples = [callback, ...requirements.requiredSourceOperations.filter((operation) => operation !== required)
      .map((operation) => ({ operation, startedAt: 1, completedAt: 2, durationMs: 0 }))];
    expect(observerValidity(samples, { start: 1, end: 11 }, 0, 0, requirements).structuralFailures)
      .toContain(`MISSING_OPERATION:${required}`);
  }
  expect(observerValidity([{ ...callback, operation: "raf-registration" }], { start: 1, end: 11 }, 0, 0,
    validObserverRequirements).structuralFailures).toContain("MISSING_EXPECTED_RAF_POPULATION");
  expect(observerValidity([callback], { start: 1, end: 11 }, 0, 0,
    { ...validObserverRequirements, minimumRafCallbacks: 2 }).structuralFailures).toContain("MISSING_EXPECTED_RAF_POPULATION");
});

test("bad or missing observer windows cannot receive structural validity", () => {
  const samples = [{ operation: "raf-callback", startedAt: 1, completedAt: 2, durationMs: 0.1 }];
  for (const window of [undefined, { start: 1, end: 1 }, { start: 2, end: 1 },
    { start: -1, end: 10 }, { start: Number.NaN, end: 10 }, { start: 0, end: Number.POSITIVE_INFINITY }]) {
    expect(accountingFixtureValidity(samples, window as any, 0, 0, validObserverRequirements).status)
      .toBe("FAIL_OBSERVER_STRUCTURAL_EVIDENCE");
  }
});

test("extra or unknown observer RAF, product observation RAF, and incomplete restore fail closed", () => {
  const samples = [{ operation: "raf-callback", startedAt: 1, completedAt: 2, durationMs: 0.1 }];
  for (const requirements of [undefined, { ...validObserverRequirements, scheduledAdditionalAnimationFrames: 1 },
    { ...validObserverRequirements, scheduledAdditionalAnimationFrames: undefined },
    { ...validObserverRequirements, productObservationRaf: 42 },
    { ...validObserverRequirements, productObservationRaf: undefined },
    { ...validObserverRequirements, restorationStatus: "FAIL_RESTORE" }]) {
    expect(accountingFixtureValidity(samples, { start: 1, end: 11 }, 0, 0, requirements as any).status)
      .toBe("FAIL_OBSERVER_STRUCTURAL_EVIDENCE");
  }
});

test("instrumented preflight still requires complete causal and pixel evidence above raw overhead references", () => {
  const observer = accountingFixtureValidity([{ operation: "raf-callback", startedAt: 1, completedAt: 5, durationMs: 4 }],
    { start: 1, end: 11 }, 0, 0, validObserverRequirements);
  expect(observer.status).toBe("PASS_OBSERVER_STRUCTURAL_EVIDENCE");
  expect(observer.rawOverheadDiagnostics.status).toBe("EXCEEDS_RAW_REFERENCES");
  expect(observer.totalPercent).toBe(40);
  const complete = { traceComplete: true, extractionPass: true, pointObserver: observer, orbitObserver: observer,
    pointVisualIdentityStatus: "PASS_EXACT_IDENTITY_AND_SOURCE_BOUND_SELECTION_COLOR_IN_PROJECTED_ROI" };
  expect(instrumentedPreflightPass(complete)).toBe(true);
  expect(instrumentedPreflightPass({ ...complete, traceComplete: false })).toBe(false);
  expect(instrumentedPreflightPass({ ...complete, extractionPass: false })).toBe(false);
  expect(instrumentedPreflightPass({ ...complete, pointVisualIdentityStatus: "FAIL_EXACT_IDENTITY_OR_SOURCE_BOUND_SELECTION_COLOR_IN_PROJECTED_ROI" })).toBe(false);
  const invalid = accountingFixtureValidity([], { start: 1, end: 11 }, 0, 0, validObserverRequirements);
  expect(instrumentedPreflightPass({ ...complete, pointObserver: invalid })).toBe(false);
  expect(instrumentedPreflightPass({ ...complete, orbitObserver: invalid })).toBe(false);
});

const coverageFixture = () => {
  const requirements: ObserverRequirements = {
    ...validObserverRequirements, minimumRafCallbacks: 2,
    requiredSourceOperations: ["raf-registration", "pointerdown-capture-listener-total", "pointerup-capture-listener-total"],
    feedbackObservations: [
      { markerIdentity: "coverage.A", invocationId: 1, callbackEntryAt: 110, callbackCompletedAt: 111 },
      { markerIdentity: "coverage.B", invocationId: 2, callbackEntryAt: 150, callbackCompletedAt: 151 }],
    pointerTransaction: { down: { eventKind: "pointerdown", pointerId: 7, listenerObservedAt: 100 },
      up: { eventKind: "pointerup", pointerId: 7, listenerObservedAt: 199 } },
    actionMarker: { eventKind: "pointerdown", pointerId: 7, listenerObservedAt: 100.1 }, actionStartEvent: "pointerdown"
  };
  const samples = [
    { operation: "raf-registration", startedAt: 99, completedAt: 99.1, durationMs: 0.1 },
    { operation: "pointerdown-capture-listener-total", startedAt: 100, completedAt: 100.2, durationMs: 0.1 },
    { operation: "pointerup-capture-listener-total", startedAt: 199, completedAt: 199.2, durationMs: 0.1 },
    { operation: "raf-callback", startedAt: 109.9, completedAt: 111.2, durationMs: 0.2 },
    { operation: "raf-callback", startedAt: 130, completedAt: 130.2, durationMs: 0.1 },
    { operation: "raf-callback", startedAt: 149.9, completedAt: 151.2, durationMs: 0.2 }
  ];
  return { requirements, samples, window: { start: 100, end: 200 } };
};

test("exact observation coverage accepts extra unrelated callbacks and shared pointer action listener", () => {
  const f = coverageFixture();
  const result = observerValidity(f.samples, f.window, 0, 0, f.requirements);
  expect(result.status).toBe("PASS_OBSERVER_STRUCTURAL_EVIDENCE");
  expect(result.byOperation["raf-callback"].sampleCount).toBe(3);
  // Complete stopped observations remain required outside the scored subwindow.
  expect(observerValidity(f.samples, { start: 120, end: 160 }, 0, 0,
    { ...f.requirements, minimumRafCallbacks: 1 }).status).toBe("PASS_OBSERVER_STRUCTURAL_EVIDENCE");
});

test("unrelated RAF cost cannot substitute for missing saved feedback cost", () => {
  const f = coverageFixture();
  f.samples.pop(); // A and X still meet the old count of two; B is uncovered.
  expect(observerValidity(f.samples, f.window, 0, 0, f.requirements).structuralFailures)
    .toContain("MISSING_OR_AMBIGUOUS_OBSERVATION_COST:raf-callback");
  const outside = coverageFixture();
  outside.samples.splice(3, 1); // Missing A is outside the measured window, still required.
  expect(observerValidity(outside.samples, { start: 120, end: 160 }, 0, 0,
    { ...outside.requirements, minimumRafCallbacks: 1 }).structuralFailures)
    .toContain("MISSING_OR_AMBIGUOUS_OBSERVATION_COST:raf-callback");
});

test("missing or non-enclosing pointer and action cost records fail", () => {
  for (const mutation of ["down-missing", "up-missing", "down-moved", "up-moved", "action-outside", "action-other-record"]) {
    const f = coverageFixture();
    if (mutation === "down-missing") f.samples.splice(1, 1);
    if (mutation === "up-missing") f.samples.splice(2, 1);
    if (mutation === "down-moved") { f.samples[1].startedAt = 101; f.samples[1].completedAt = 101.2; }
    if (mutation === "up-moved") { f.samples[2].startedAt = 198; f.samples[2].completedAt = 198.2; }
    if (mutation.startsWith("action-")) {
      f.requirements = { ...f.requirements, actionMarker: { ...f.requirements.actionMarker, listenerObservedAt: 101 } };
      if (mutation === "action-other-record") f.samples.push({ operation: "pointerdown-capture-listener-total",
        startedAt: 100.9, completedAt: 101.1, durationMs: 0.1 });
    }
    expect(observerValidity(f.samples, f.window, 0, 0, f.requirements).status).toBe("FAIL_OBSERVER_STRUCTURAL_EVIDENCE");
  }
});

test("malformed or duplicate expected observation identities and boundaries fail", () => {
  const f = coverageFixture();
  for (const patch of [
    { feedbackObservations: undefined }, { feedbackObservations: [] },
    { feedbackObservations: [null] }, { feedbackObservations: [f.requirements.feedbackObservations[0], f.requirements.feedbackObservations[0]] },
    ...[{ markerIdentity: "" }, { invocationId: Number.NaN }, { callbackEntryAt: Number.NaN },
      { callbackCompletedAt: 109 }, { callbackEntryAt: -1 }].map((change) => ({ feedbackObservations:
        [{ ...f.requirements.feedbackObservations[0], ...change }] })),
    { feedbackObservations: [f.requirements.feedbackObservations[0],
      { ...f.requirements.feedbackObservations[1], markerIdentity: "coverage.A" }] },
    { feedbackObservations: [f.requirements.feedbackObservations[0],
      { ...f.requirements.feedbackObservations[1], invocationId: 1 }] },
    { pointerTransaction: undefined }, { actionStartEvent: "click" },
    { actionMarker: { ...f.requirements.actionMarker, pointerId: 8 } },
    { actionMarker: { ...f.requirements.actionMarker, listenerObservedAt: Number.NaN } },
    { pointerTransaction: { ...f.requirements.pointerTransaction, up: { ...f.requirements.pointerTransaction.up, listenerObservedAt: 99 } } }
  ]) expect(observerValidity(f.samples, f.window, 0, 0, { ...f.requirements, ...patch } as any).structuralFailures)
    .toContain("INVALID_EXPECTED_OBSERVATIONS");
});

test("duplicate ambiguous enclosures and illegal reuse across distinct callbacks fail", () => {
  for (const kind of ["duplicate-raf", "ambiguous-raf", "duplicate-pointer", "reuse"]) {
    const f = coverageFixture();
    if (kind === "duplicate-raf") f.samples.push({ ...f.samples[3] });
    if (kind === "ambiguous-raf") f.samples.push({ ...f.samples[3], startedAt: 109, completedAt: 112 });
    if (kind === "duplicate-pointer") f.samples.push({ ...f.samples[1] });
    if (kind === "reuse") {
      f.samples.splice(3, 3, { operation: "raf-callback", startedAt: 109, completedAt: 152, durationMs: 0.1 },
        { operation: "raf-callback", startedAt: 160, completedAt: 161, durationMs: 0.1 });
    }
    const result = observerValidity(f.samples, f.window, 0, 0, f.requirements);
    expect(result.status).toBe("FAIL_OBSERVER_STRUCTURAL_EVIDENCE");
    if (kind === "reuse") expect(result.structuralFailures).toContain("REUSED_CALLBACK_COST_RECORD");
  }
});

test("zero RAF DOM class requires exact input and mutation records without fabricated callbacks", () => {
  const requirements = { actionClass: "tree-filter" as const, minimumRafCallbacks: 0,
    feedbackObservations: [{ markerIdentity: "dom.1", invocationId: 1, callbackEntryAt: 110, callbackCompletedAt: 111 }],
    expectedOperations: [{ identity: "input", operation: "input-capture-listener-total", start: 100, end: 100 }],
    requiredSourceOperations: ["input-capture-listener-total", "dom-mutation-callback", "dom-stopped-reconciliation"],
    scheduledAdditionalAnimationFrames: 0, productObservationRaf: null, restorationStatus: "PASS_FULL_RESTORE" };
  const samples = [
    { operation: "input-capture-listener-total", startedAt: 99.9, completedAt: 100.2, durationMs: 0.1 },
    { operation: "dom-mutation-callback", startedAt: 109.9, completedAt: 111.2, durationMs: 0.2 },
    { operation: "dom-stopped-reconciliation", startedAt: 120, completedAt: 121, durationMs: 1 }];
  const result = observerValidity(samples, { start: 100, end: 111 }, 0, 0, requirements);
  expect(result.status).toBe("PASS_OBSERVER_STRUCTURAL_EVIDENCE");
  expect(result.p95Ms).toBeNull();
  for (const missing of [0, 1, 2]) expect(observerValidity(samples.filter((_, i) => i !== missing),
    { start: 100, end: 111 }, 0, 0, requirements).status).toBe("FAIL_OBSERVER_STRUCTURAL_EVIDENCE");
  expect(observerValidity(samples, { start: 100, end: 111 }, 0, 0,
    { ...requirements, expectedOperations: [] }).status).toBe("FAIL_OBSERVER_STRUCTURAL_EVIDENCE");
});

for (const variant of ["complete", "pending-mutation"] as const) {
  test(`real input DOM observer binds stopped publication and pending mutation: ${variant}`, async ({ page }) => {
    await installInstrumentation(page, { causalFeedbackMarkers: true });
    await page.goto("data:text/html,<div class='panel model-tree'><input data-testid='model-tree-filter-input'><span data-testid='model-tree-filter-summary'>1 of 1 model entities visible</span><div data-testid='model-tree-virtual'><button role='treeitem' data-testid='tree-row-pipe-p' aria-level='1' aria-posinset='1' aria-setsize='1'><strong>Initial</strong></button></div></div>");
    await page.evaluate(() => {
      const model = { projectId: "project", generation: 1, identityHash: "fixture", indexGeneration: "i1", projectSessionGeneration: 1 };
      let publication: any = { actionSequence: 1, generation: 1, query: "", visibleCount: 1, inputAt: null, inputEventTimeStamp: null, publishedAt: 0 };
      Object.defineProperty(globalThis, "__openPipeStressUiDiagnosticsV1", { value: { readCurrent: () => ({ model,
        tree: { generation: 1, publicationSequence: 2, query: publication.query, visibleCount: 1 }, viewport: { filter: publication } }) } });
      document.querySelector("input")!.addEventListener("input", (event) => {
        publication = { ...publication, actionSequence: 2, query: (event.target as HTMLInputElement).value,
          inputAt: performance.now(), inputEventTimeStamp: event.timeStamp, publishedAt: performance.now() };
        document.querySelector("strong")!.textContent = "Wanted";
      });
      (globalThis as any).__uifHarness.causal.arm({ token: "dom-real", phase: "candidate", feedbackKind: "tree-filter", actionStartEvent: "input",
        expectedActionTargetTestId: "model-tree-filter-input", maximumFeedbackMarkers: 10, actionIdentity: { query: "wanted" },
        treeExpectation: { query: "wanted", totalCount: 1, visibleCount: 1,
          rows: [{ testId: "tree-row-pipe-p", label: "Wanted", level: 1, position: 1, setSize: 1 }] },
        candidateExpectation: { projectId: "project", modelIdentityHash: "fixture", modelGeneration: 1,
          indexGeneration: "i1", projectSessionGeneration: 1, priorActionSequence: 1 } });
    });
    const activeDrainRejected = await page.evaluate(() => {
      try { (globalThis as any).__uifHarness.causal.acknowledgeStopped("dom-real", "a".repeat(64)); return false; }
      catch { return true; }
    });
    expect(activeDrainRejected).toBe(true);
    await page.getByTestId("model-tree-filter-input").fill("wanted");
    await page.waitForTimeout(20);
    const evidence = await page.evaluate((pending) => {
      if (pending) document.querySelector("strong")!.textContent = "Not wanted";
      return (globalThis as any).__uifHarness.causal.stop("dom-real");
    }, variant === "pending-mutation");
    expect(evidence.active.feedbackMarkers.length).toBeGreaterThan(0);
    expect(evidence.active.actionMarker.isTrusted).toBe(true);
    expect(evidence.active.contentProof.status).toBe(variant === "complete" ? "PASS_EXACT_STOPPED_CONTENT" : "FAIL_STOPPED_CONTENT");
    if (variant === "pending-mutation") expect(evidence.active.contentProof.pendingMutationCount).toBeGreaterThan(0);
    if (variant === "complete") {
      const acknowledgement = createHash("sha256").update(JSON.stringify(evidence)).digest("hex");
      const drain = await page.evaluate((hash) => {
        const causal = (globalThis as any).__uifHarness.causal;
        const rejected = [() => causal.acknowledgeStopped("wrong-token", hash),
          () => causal.acknowledgeStopped("dom-real", "not-a-hash")].map((call) => { try { call(); return false; } catch { return true; } });
        const before = (globalThis as any).__openPipeStressUiDiagnosticsV1.readCurrent();
        const acknowledged = causal.acknowledgeStopped("dom-real", hash);
        return { rejected, acknowledged, active: causal.active, epoch: causal.evidenceEpoch,
          records: causal.records.length, samples: causal.observerSamples.length,
          productUnchanged: JSON.stringify(before) === JSON.stringify((globalThis as any).__openPipeStressUiDiagnosticsV1.readCurrent()) };
      }, acknowledgement);
      expect(drain).toMatchObject({ rejected: [true, true], active: null, epoch: 1, records: 0, samples: 0, productUnchanged: true });
      expect(drain.acknowledged.persistedEvidenceSha256).toBe(acknowledgement);
    }
    expect((await restoreCausalFeedbackInstrumentation(page)).status).toBe("PASS_FULL_RESTORE");
  });
}

for (const variant of ["complete", "same-primary-missing-member"] as const) {
  test(`box feedback checks full membership even when primary is unchanged: ${variant}`, async ({ page }) => {
    await installInstrumentation(page, { causalFeedbackMarkers: true });
    await page.goto("data:text/html,<div data-testid='viewport-canvas'><canvas width='64' height='64'></canvas></div>");
    await requireHealthyWebGl1(page);
    const evidence = await page.evaluate(async (variant) => {
      const canvas = document.querySelector("canvas") as HTMLCanvasElement, gl = canvas.getContext("webgl")!;
      const a = { type: "pipe", id: "a" }, b = { type: "pipe", id: "b" };
      let snapshot: any = null;
      Object.defineProperty(globalThis, "__openPipeStressUiDiagnosticsV1", { value: { readCurrent: () => snapshot } });
      const causal = (globalThis as any).__uifHarness.causal;
      document.body.insertAdjacentHTML("beforeend", `<span data-testid="command-selection-readout">Selected pipe: b; 0 queued</span><div class="workspace-pane-inspector"><div class="panel inspector"><h2>2 selected items</h2><div data-testid="aggregate-property-inspector"><dl><div><dt>pipe</dt><dd>2</dd></div><div><dt>Primary</dt><dd>pipe: b</dd></div></dl></div></div></div>`);
      causal.arm({ token: "box-full", phase: "candidate", feedbackKind: "box-selection", actionStartEvent: "pointerup",
        expectedActionTargetTestId: "viewport-canvas", maximumFeedbackMarkers: 1, actionIdentity: { start: { x: 1, y: 1 }, end: { x: 20, y: 20 } },
        candidateExpectation: { modelGeneration: 1, priorRenderSubmissionSequence: 1, priorBoxActionSequence: 1,
          direction: "left-to-right", filter: "pipes", orderedRefs: [a, b], primaryRef: b, projectId: "project", oracleSha256: "frozen", resourceGeneration: 1, expectedInspectorHeading: "2 selected items" } });
      canvas.dispatchEvent(new PointerEvent("pointerdown", { bubbles: true, pointerId: 9, clientX: 1, clientY: 1 }));
      canvas.dispatchEvent(new PointerEvent("pointerup", { bubbles: true, pointerId: 9, clientX: 20, clientY: 20 }));
      await new Promise<void>((resolve) => requestAnimationFrame(() => {
        const refs = variant === "complete" ? [a, b] : [b];
        snapshot = { snapshotSequence: 2, capturedAt: performance.now(), model: { generation: 1 }, viewport: { generation: 1, mainRender: { generation: 1, submissionSequence: 2,
          selectionPresentation: { resourceGeneration: 1, modelGeneration: 1, revision: 1, appliedAfterSubmissionSequence: 1, renderedSubmissionSequence: 2, orderedRefs: refs } },
          box: { generation: 1, actionSequence: 2, direction: "left-to-right", filter: "pipes", orderedRefs: refs,
            primaryRef: b, renderSubmissionSequence: 2, publishedAt: performance.now() },
          selection: { generation: 1, actionSequence: 2, inputKind: "programmatic", pointerDownAt: null, publishedAt: performance.now(), renderSubmissionSequence: 2, orderedRefs: refs, primaryRef: b }, inspector: { generation: 1, publicationSequence: 2, publishedAt: performance.now(), ref: b } } };
        gl.clear(gl.COLOR_BUFFER_BIT); resolve();
      }));
      return causal.stop("box-full");
    }, variant);
    expect(evidence.active.feedbackMarkers).toHaveLength(variant === "complete" ? 1 : 0);
    expect((await restoreCausalFeedbackInstrumentation(page)).status).toBe("PASS_FULL_RESTORE");
  });
}

for (const variant of ["complete", "stopped-model-drift"] as const) {
  test(`assignment retains original product start and reconciles stopped identity: ${variant}`, async ({ page }) => {
    await installInstrumentation(page, { causalFeedbackMarkers: true, assignmentSpecification: {
      token: "assignment-original", phase: "candidate", feedbackKind: "assignment", actionStartEvent: "assignment",
      expectedActionTargetTestId: "model-tree-virtual", maximumFeedbackMarkers: 1, actionIdentity: { fixture: "synthetic-control" },
      treeExpectation: { query: "", totalCount: 1, visibleCount: 1,
        rows: [{ testId: "tree-row-project-p", label: "Project", level: 1, position: 1, setSize: 1 }] },
      candidateExpectation: { projectId: "p", modelIdentityHash: "fixture" }
    } });
    await page.goto("data:text/html,<div data-testid='viewport-canvas'><canvas width='64' height='64'></canvas></div><div class='panel model-tree'><input data-testid='model-tree-filter-input'><span data-testid='model-tree-filter-summary'>1 of 1 model entities visible</span><div data-testid='model-tree-virtual'><button role='treeitem' data-testid='tree-row-project-p' aria-level='1' aria-posinset='1' aria-setsize='1'><strong>Project</strong></button></div></div>");
    await requireHealthyWebGl1(page);
    const result = await page.evaluate(async (variant) => {
      const causal = (globalThis as any).__uifHarness.causal;
      let snapshot: any = { model: { assignment: { status: "idle" } } };
      Object.defineProperty(globalThis, "__openPipeStressUiDiagnosticsV1", { value: { readCurrent: () => snapshot } });
      causal.prepareAssignment();
      const startedAt = performance.now();
      const gl = document.querySelector("canvas")!.getContext("webgl")!;
      await new Promise<void>((resolve) => requestAnimationFrame(() => {
        snapshot = { model: { projectId: "p", generation: 1, identityHash: "fixture", indexGeneration: "i1",
          projectSessionGeneration: 1, assignment: { status: "committed", startedAt } },
          tree: { generation: 1, publicationSequence: 1, query: "", visibleCount: 1 },
          viewport: { generation: 1, mainRender: { generation: 1, submissionSequence: 1, submittedAt: performance.now() },
            filter: { generation: 1, query: "", visibleCount: 1, publishedAt: performance.now(), inputAt: null, inputEventTimeStamp: null } } };
        gl.clear(gl.COLOR_BUFFER_BIT); resolve();
      }));
      if (variant === "stopped-model-drift") snapshot.model.identityHash = "other-model";
      return { startedAt, evidence: causal.stop("assignment-original") };
    }, variant);
    expect(result.evidence.active.actionMarker.listenerObservedAt).toBe(result.startedAt);
    expect(result.evidence.active.feedbackMarkers).toHaveLength(1);
    expect(result.evidence.active.feedbackMarkers[0].callbackEntryAt).toBeGreaterThanOrEqual(result.startedAt);
    expect(result.evidence.active.contentProof.status).toBe(variant === "complete" ? "PASS_EXACT_STOPPED_CONTENT" : "FAIL_STOPPED_CONTENT");
    expect((await restoreCausalFeedbackInstrumentation(page)).status).toBe("PASS_FULL_RESTORE");
  });
}


test("baseline null outside hit records limitation and continues before label lookup", () => {
  const model = { pipe_segments: [{ id: "p", label: "Expected pipe" }] };
  const probes = [
    { baseline: { actionability: "NOT_ATTEMPTED_OUTSIDE_BASELINE_FRUSTUM", ndc: { x: 2, y: 0 },
      oracle: { expectedHitRef: null, status: "OUTSIDE", reason: "outside frozen frustum" } } },
    { baseline: { actionability: "ACTIONABLE_IN_BASELINE_FRUSTUM", oracle: { expectedHitRef: { type: "pipe", id: "p" } } } }
  ];
  const records = [], labels = [];
  for (const [i, probe] of probes.entries()) {
    const limitation = baselineOutsideFrustumRecord({ sample: i + 1, probe_anchor_ref: "anchor" }, probe);
    if (limitation) { records.push(limitation); continue; }
    labels.push(expectedVisibleEntityLabel(model, probe.baseline.oracle.expectedHitRef!));
  }
  expect(records).toEqual([{ sample: 1, probeAnchorRef: "anchor", status: "NOT_ATTEMPTED_OUTSIDE_BASELINE_FRUSTUM",
    durationMs: null, baselineNdc: { x: 2, y: 0 }, actionability: "NOT_ATTEMPTED_OUTSIDE_BASELINE_FRUSTUM",
    oracleStatus: "OUTSIDE", oracleReason: "outside frozen frustum" }]);
  expect(labels).toEqual(["Expected pipe"]);
  expect(baselineOutsideFrustumRecord({ sample: 3 }, { baseline: { actionability: "ACTIONABLE_IN_BASELINE_FRUSTUM",
    oracle: { expectedHitRef: null } } })).toBeNull(); // Invalid actionable identity cannot become a skipped limitation.
});

test("secondary inspector capture uses phase-specific exact independently supplied typed heading", async ({ page }, testInfo) => {
  const ref = { type: "pipe", id: "shared-id" }, label = "Expected pipe";
  for (const [i, entry] of [
    { phase: "baseline" as const, text: label, pass: true },
    { phase: "candidate" as const, text: `${label} — pipe: shared-id`, pass: true },
    { phase: "candidate" as const, text: "Wrong label — pipe: shared-id", pass: false },
    { phase: "candidate" as const, text: `${label} — node: shared-id`, pass: false },
    { phase: "candidate" as const, text: `${label} — pipe: wrong-id`, pass: false },
    { phase: "candidate" as const, text: label, pass: false },
    { phase: "baseline" as const, text: `${label} — pipe: shared-id`, pass: false }
  ].entries()) {
    await page.setContent('<div class="workspace-pane-inspector"><section class="panel inspector" data-testid="property-inspector"><h2></h2></section></div>');
    await page.locator("h2").evaluate((heading, text) => { heading.textContent = text; }, entry.text);
    const evidence = await capturePointInspectorEvidence(page, entry.phase, label, ref, testInfo.outputPath(`heading-${i}.png`));
    expect(evidence.status).toBe(entry.pass ? "PASS_EXACT_VISIBLE_TEXT_AND_DECODED_PNG" : "FAIL_EXACT_VISIBLE_TEXT_OR_PNG");
    expect(evidence.observedText).toBe(entry.text);
  }
});

for (const variant of ["two-epochs", "same-epoch", "no-clear", "stale-submission", "wrong-model", "unready-tree",
  "final-without-frame", "pending-at-stop", "capacity"] as const) {
  test(`assignment epoch history requires genuine eligible advancing frames: ${variant}`, async ({ page }) => {
    await page.addInitScript(() => {
      const nativeRequest = window.requestAnimationFrame;
      const counts = { requests: 0, callbacks: 0 };
      (globalThis as any).__fixtureNativeRafCounts = counts;
      window.requestAnimationFrame = (callback) => {
        counts.requests++;
        return nativeRequest.call(window, (time) => { counts.callbacks++; callback(time); });
      };
    });
    await installInstrumentation(page, { causalFeedbackMarkers: true, assignmentSpecification: {
      token: "assignment-epochs", phase: "candidate", feedbackKind: "assignment", actionStartEvent: "assignment",
      expectedActionTargetTestId: "model-tree-virtual", maximumFeedbackMarkers: variant === "capacity" || variant === "same-epoch" ? 1 : 4,
      actionIdentity: { fixture: "epoch-control" }, treeExpectation: { query: "", totalCount: 1, visibleCount: 1,
        rows: [{ testId: "tree-row-project-p", label: "Project", level: 1, position: 1, setSize: 1 }] },
      candidateExpectation: { projectId: "p", modelIdentityHash: "fixture", priorRenderSubmissionSequence: 0 }
    } });
    await page.goto("data:text/html,<div data-testid='viewport-canvas'><canvas width='64' height='64'></canvas></div><div class='panel model-tree'><input data-testid='model-tree-filter-input'><span data-testid='model-tree-filter-summary'>1 of 1 model entities visible</span><div data-testid='model-tree-virtual'><button role='treeitem' data-testid='tree-row-project-p' aria-level='1' aria-posinset='1' aria-setsize='1'><strong>Project</strong></button></div></div>");
    await requireHealthyWebGl1(page);
    const result = await page.evaluate(async (variant) => {
      const causal = (globalThis as any).__uifHarness.causal;
      const counts = (globalThis as any).__fixtureNativeRafCounts;
      counts.requests = 0; counts.callbacks = 0;
      let scheduled = 0, callbacks = 0;
      let snapshot: any = { model: { assignment: { status: "idle" } } };
      Object.defineProperty(globalThis, "__openPipeStressUiDiagnosticsV1", { value: { readCurrent: () => snapshot } });
      causal.prepareAssignment();
      const startedAt = performance.now();
      const gl = document.querySelector("canvas")!.getContext("webgl")!;
      const root = document.querySelector(".model-tree") as HTMLElement;
      const input = document.querySelector("input")!;
      let mutationNumber = 0;
      const mutate = async () => { root.dataset.fixtureEpoch = String(++mutationNumber); await Promise.resolve(); };
      const frame = async (submission: number, clear = true, hash = "fixture") => {
        scheduled++;
        await new Promise<void>((resolve) => requestAnimationFrame(() => {
          callbacks++;
          snapshot = { model: { projectId: "p", generation: 1, identityHash: hash, indexGeneration: "i1",
            projectSessionGeneration: 1, assignment: { status: "committed", startedAt } },
            tree: { generation: 1, publicationSequence: 1, query: "", visibleCount: 1 },
            viewport: { generation: 1, mainRender: { generation: 1, submissionSequence: submission, submittedAt: performance.now() },
              filter: { generation: 1, query: "", visibleCount: 1, publishedAt: performance.now(), inputAt: null, inputEventTimeStamp: null } } };
          if (clear) gl.clear(gl.COLOR_BUFFER_BIT);
          resolve();
        }));
      };
      await mutate(); await frame(1);
      const first = causal.read("assignment-epochs").active.feedbackMarkers[0];
      let afterIneligible: any = null;
      if (variant === "same-epoch" || variant === "capacity") await frame(2);
      const afterDuplicate = causal.read("assignment-epochs");
      if (variant === "pending-at-stop") root.dataset.fixtureEpoch = "pending";
      else if (variant !== "same-epoch") {
        await mutate();
        if (variant !== "final-without-frame") {
          if (["no-clear", "stale-submission", "wrong-model", "unready-tree", "capacity"].includes(variant)) {
            if (variant === "unready-tree") input.value = "unready";
            await frame(variant === "stale-submission" ? 1 : 2, variant !== "no-clear" && variant !== "capacity",
              variant === "wrong-model" ? "wrong" : "fixture");
            afterIneligible = causal.read("assignment-epochs");
            input.value = "";
          }
          await frame(3);
        }
      }
      const evidence = causal.stop("assignment-epochs");
      return { first, afterDuplicate, afterIneligible, evidence, startedAt, scheduled, callbacks, native: counts };
    }, variant);
    expect(result.evidence.active.actionMarker.listenerObservedAt).toBe(result.startedAt);
    expect(result.native.requests).toBe(result.scheduled);
    expect(result.native.callbacks).toBe(result.callbacks);
    expect(result.callbacks).toBe(result.scheduled);
    expect(result.native.requests - result.scheduled).toBe(0); // Observer-added RAF requests.
    expect(result.evidence.active.observerSamples.filter((s: any) => s.operation === "raf-callback")).toHaveLength(result.scheduled);
    expect(result.evidence.active.feedbackMarkers[0]).toEqual(result.first);
    if (["capacity", "pending-at-stop", "final-without-frame"].includes(variant)) {
      expect(() => selectedEpochEvidence(result.evidence)).toThrow();
    } else {
      const selected = selectedEpochEvidence(result.evidence);
      expect(selected.active.feedbackMarkers).toHaveLength(1);
      expect(selected.active.feedbackMarkers[0].domEpoch).toBe(result.evidence.active.domEpoch);
      expect(selected.excludedInvalidatedContentMarkers).toEqual(variant === "same-epoch" ? [] : [result.first]);
    }
    expect(result.afterDuplicate.active.feedbackMarkers).toHaveLength(1);
    expect(result.afterDuplicate.active.overflow.total).toBe(0);
    if (result.afterIneligible) {
      expect(result.afterIneligible.active.feedbackMarkers).toHaveLength(1);
      expect(result.afterIneligible.active.overflow.total).toBe(0);
      expect(result.afterIneligible.active.stopped).toBe(false);
    }
    if (variant === "capacity") {
      expect(result.evidence.active.feedbackMarkers).toHaveLength(1);
      expect(result.evidence.active.overflow.byCollection.feedbackMarkers).toBe(1);
      expect(result.evidence.active.stopReason).toBe("MAXIMUM_FEEDBACK_MARKERS_REACHED");
    } else {
      expect(result.evidence.active.overflow.total).toBe(0);
      if (variant === "pending-at-stop") {
        expect(result.evidence.active.contentProof.status).toBe("FAIL_STOPPED_CONTENT");
        expect(result.evidence.active.contentProof.pendingMutationCount).toBeGreaterThan(0);
      } else {
        expect(result.evidence.active.contentProof.status).toBe("PASS_EXACT_STOPPED_CONTENT");
        const markers = result.evidence.active.feedbackMarkers;
        if (variant === "final-without-frame") expect(markers.filter((m: any) => m.domEpoch === result.evidence.active.domEpoch)).toHaveLength(0);
        else {
          expect(markers).toHaveLength(variant === "same-epoch" ? 1 : 2);
          const final = markers.filter((m: any) => m.domEpoch === result.evidence.active.domEpoch);
          expect(final).toHaveLength(1);
          if (variant !== "same-epoch") expect(final[0].observed.mainRenderSubmissionSequence).toBeGreaterThan(result.first.observed.mainRenderSubmissionSequence);
        }
      }
    }
    expect((await restoreCausalFeedbackInstrumentation(page)).status).toBe("PASS_FULL_RESTORE");
  });
}


test("box callback consumes actual unavailable or active union and rejects numerical and ordered-state faults", async ({ page }) => {
  await installInstrumentation(page, { causalFeedbackMarkers: true });
  for (const variant of ["empty", "next-active", "duplicate-empty", "selected-project-empty", "wrong-inspector", "nonadvancing", "unsafe", "stale", "stale-render", "reordered", "typed-collision"]) {
    await page.goto("data:text/html,<div data-testid='viewport-canvas'><canvas width='64' height='64'></canvas></div>");
    await requireHealthyWebGl1(page);
    const project = { type: "project", id: "p" }, a = { type: "pipe", id: "collision" }, b = { type: "pipe", id: "b" };
    const nonempty = ["next-active", "reordered", "typed-collision"].includes(variant);
    const expectedRefs = nonempty ? [a, b] : [], primary = nonempty ? b : null;
    const prior = validatedPriorBoxBaseline({ model: { generation: 1 }, viewport: { generation: 1,
      box: ["next-active", "duplicate-empty"].includes(variant) ? { generation: 1, actionSequence: 1, renderSubmissionSequence: 1, publishedAt: 0,
        direction: "left-to-right", filter: "all", orderedRefs: [], primaryRef: null } : { status: "unavailable" } } });
    const evidence = await page.evaluate(async ({ variant, prior, expectedRefs, primary, project }) => {
      const canvas = document.querySelector("canvas") as HTMLCanvasElement, gl = canvas.getContext("webgl")!;
      const causal = (globalThis as any).__uifHarness.causal;
      let snapshot: any = null;
      Object.defineProperty(globalThis, "__openPipeStressUiDiagnosticsV1", { value: { readCurrent: () => snapshot } });
      const identity = primary ?? project;
      const heading = expectedRefs.length > 1 ? `${expectedRefs.length} selected items` : `Project — ${identity.type}: ${identity.id}`;
      document.body.insertAdjacentHTML("beforeend", `<span data-testid="command-selection-readout">Selected ${identity.type}: ${identity.id}; 0 queued</span><div class="workspace-pane-inspector"><div class="panel inspector"><h2>${heading}</h2>${expectedRefs.length > 1 ? `<div data-testid="aggregate-property-inspector"><dl><div><dt>pipe</dt><dd>2</dd></div><div><dt>Primary</dt><dd>${identity.type}: ${identity.id}</dd></div></dl></div>` : ""}</div></div>`);
      causal.arm({ token: "box-union", phase: "candidate", feedbackKind: "box-selection", actionStartEvent: "pointerup",
        expectedActionTargetTestId: "viewport-canvas", maximumFeedbackMarkers: 1,
        actionIdentity: { start: { x: 1, y: 1 }, end: { x: 20, y: 20 } },
        candidateExpectation: { modelGeneration: 1, priorRenderSubmissionSequence: 1, priorBoxActionSequence: prior.actionSequence,
          priorBoxBaseline: prior, direction: "left-to-right", filter: "all", orderedRefs: expectedRefs, primaryRef: primary,
          projectId: "p", oracleSha256: "frozen", resourceGeneration: 1, expectedInspectorHeading: heading } });
      canvas.dispatchEvent(new PointerEvent("pointerdown", { bubbles: true, pointerId: 9, clientX: 1, clientY: 1 }));
      canvas.dispatchEvent(new PointerEvent("pointerup", { bubbles: true, pointerId: 9, clientX: 20, clientY: 20 }));
      await new Promise<void>((resolve) => requestAnimationFrame(() => {
        const refs = variant === "reordered" ? [...expectedRefs].reverse() : variant === "typed-collision"
          ? [{ type: "node", id: "collision" }, expectedRefs[1]] : expectedRefs;
        snapshot = { snapshotSequence: 2, capturedAt: performance.now(), model: { generation: 1 }, viewport: { generation: 1, mainRender: { generation: 1, submissionSequence: variant === "stale-render" ? 1 : 2,
          selectionPresentation: { resourceGeneration: 1, modelGeneration: 1, revision: 1, appliedAfterSubmissionSequence: 1, renderedSubmissionSequence: 2, orderedRefs: refs } },
          box: { generation: variant === "stale" ? 2 : 1, actionSequence: variant === "nonadvancing" ? prior.actionSequence : variant === "unsafe" ? Number.MAX_SAFE_INTEGER + 1 : prior.actionSequence + 1,
            direction: "left-to-right", filter: "all", orderedRefs: refs, primaryRef: primary,
            renderSubmissionSequence: 2, publishedAt: performance.now() },
          selection: { generation: 1, actionSequence: 2, inputKind: "programmatic", pointerDownAt: null, publishedAt: performance.now(), renderSubmissionSequence: 2, orderedRefs: refs, primaryRef: variant === "selected-project-empty" ? project : primary },
          inspector: { generation: 1, publicationSequence: 2, publishedAt: performance.now(), ref: variant === "wrong-inspector" ? null : primary ?? project } } };
        gl.clear(gl.COLOR_BUFFER_BIT); resolve();
      }));
      return causal.stop("box-union");
    }, { variant, prior, expectedRefs, primary, project });
    expect(evidence.active.feedbackMarkers).toHaveLength(["empty", "next-active", "duplicate-empty"].includes(variant) ? 1 : 0);
    if (["nonadvancing", "unsafe", "stale", "stale-render"].includes(variant)) {
      const rejected = evidence.active.feedbackRejectionDetails.find((r: any) => r.reason === "BOX_SUBMISSION_NOT_READY");
      expect(rejected.observed.failedPredicates.length).toBeGreaterThan(0);
      expect(rejected.observed.priorKind).toBe(prior.kind);
      expect(rejected.observed.priorActionSequence).toBe(prior.actionSequence);
    }
    expect((await restoreCausalFeedbackInstrumentation(page)).status).toBe("PASS_FULL_RESTORE");
  }
});


test("Box producer phases are reevaluated only on a fresh genuine callback and retain bounded failed facts", async ({ page }) => {
  await installInstrumentation(page, { causalFeedbackMarkers: true });
  const variants = ["v27", "stop-only", "selection-render", "applied-old", "applied-not-drawn", "resource", "model", "missing", "unsafe-revision", "same-clock-old-frame", "inspector", "heading", "readout", "order", "typed-collision", "aggregate-count", "aggregate-single-heading", "aggregate-primary-type", "aggregate-primary-id", "aggregate-duplicate"];
  for (const variant of variants) {
    await page.goto("data:text/html,<div data-testid='viewport-canvas'><canvas width='64' height='64'></canvas></div><span data-testid='command-selection-readout'>Selected project: p; 0 queued</span><div class='workspace-pane-inspector'><div class='panel inspector'><h2>Project — project: p</h2></div></div>");
    await requireHealthyWebGl1(page);
    const result = await page.evaluate(async (variant) => {
      const canvas = document.querySelector("canvas") as HTMLCanvasElement, gl = canvas.getContext("webgl")!;
      const causal = (globalThis as any).__uifHarness.causal;
      const project = { type: "project", id: "p" }, a = { type: "pipe", id: "a" }, b = { type: "pipe", id: "b" };
      const refs = ["order", "typed-collision", "aggregate-count", "aggregate-single-heading", "aggregate-primary-type", "aggregate-primary-id", "aggregate-duplicate"].includes(variant) ? [a, b] : [];
      const primary = refs.length ? b : null, identity = primary ?? project;
      const heading = refs.length > 1 ? `${refs.length} selected items` : `Project — ${identity.type}: ${identity.id}`;
      const setDom = () => {
        document.querySelector('[data-testid="command-selection-readout"]')!.textContent = `Selected ${identity.type}: ${identity.id}; 0 queued`;
        document.querySelector("h2")!.textContent = heading;
        document.querySelectorAll('[data-testid="aggregate-property-inspector"]').forEach((node) => node.remove());
        if (refs.length > 1) document.querySelector('.panel.inspector')!.insertAdjacentHTML("beforeend", `<div data-testid="aggregate-property-inspector"><dl><div><dt>pipe</dt><dd>2</dd></div><div><dt>Primary</dt><dd>${identity.type}: ${identity.id}</dd></div></dl></div>`);
      };
      setDom();
      let snapshot: any;
      Object.defineProperty(globalThis, "__openPipeStressUiDiagnosticsV1", { value: { readCurrent: () => snapshot } });
      causal.arm({ token: "staggered", phase: "candidate", feedbackKind: "box-selection", actionStartEvent: "pointerup", expectedActionTargetTestId: "viewport-canvas", maximumFeedbackMarkers: 1,
        actionIdentity: { start: { x: 1, y: 1 }, end: { x: 20, y: 20 } }, candidateExpectation: {
          modelGeneration: 1, resourceGeneration: 1, priorRenderSubmissionSequence: 1, priorBoxActionSequence: 0,
          orderedRefs: refs, primaryRef: primary, projectId: "p", direction: "left-to-right", filter: "all", expectedInspectorHeading: heading } });
      canvas.dispatchEvent(new PointerEvent("pointerdown", { bubbles: true, pointerId: 9, clientX: 1, clientY: 1 }));
      canvas.dispatchEvent(new PointerEvent("pointerup", { bubbles: true, pointerId: 9, clientX: 20, clientY: 20 }));
      const coherent = (submission: number) => ({ snapshotSequence: submission + 30, capturedAt: performance.now(), model: { generation: 1 }, viewport: {
        generation: 1, box: { generation: 1, actionSequence: 1, direction: "left-to-right", filter: "all", orderedRefs: refs, primaryRef: primary, publishedAt: performance.now(), renderSubmissionSequence: 2 },
        selection: { generation: 1, actionSequence: 9, inputKind: "programmatic", pointerDownAt: null, orderedRefs: refs, primaryRef: primary, publishedAt: performance.now(), renderSubmissionSequence: submission },
        inspector: { generation: 1, publicationSequence: 9, publishedAt: performance.now(), ref: identity },
        mainRender: { generation: 1, submissionSequence: submission, submittedAt: performance.now(), selectionPresentation: {
          resourceGeneration: 1, modelGeneration: 1, revision: 2, appliedAfterSubmissionSequence: 1, renderedSubmissionSequence: submission, orderedRefs: refs } } } });
      await new Promise<void>((resolve) => requestAnimationFrame(() => {
        snapshot = coherent(2);
        const v = snapshot.viewport, r = v.mainRender.selectionPresentation;
        if (["v27", "stop-only"].includes(variant)) { v.selection.orderedRefs = [project]; v.selection.primaryRef = project;
          v.selection.actionSequence = 8; v.selection.publishedAt = 0; v.inspector.publicationSequence = 8; }
        if (variant === "selection-render") v.selection.renderSubmissionSequence = 0;
        if (variant === "applied-old") r.orderedRefs = [project];
        if (variant === "applied-not-drawn") r.appliedAfterSubmissionSequence = 2;
        if (variant === "resource") r.resourceGeneration = 2;
        if (variant === "model") r.modelGeneration = 2;
        if (variant === "missing") v.mainRender.selectionPresentation = null;
        if (variant === "unsafe-revision") r.revision = Number.MAX_SAFE_INTEGER + 1;
        if (variant === "same-clock-old-frame") { r.renderedSubmissionSequence = 1; v.selection.renderSubmissionSequence = 1; }
        if (variant === "inspector") v.inspector.ref = { type: "pipe", id: "p" };
        if (variant === "heading") document.querySelector("h2")!.textContent = "Wrong label — project: p";
        if (variant === "readout") document.querySelector('[data-testid="command-selection-readout"]')!.textContent = "Selected pipe: p; 0 queued";
        if (variant === "order") v.selection.orderedRefs = [b, a];
        if (variant === "typed-collision") v.selection.orderedRefs = [{ type: "node", id: "a" }, b];
        if (variant === "aggregate-count") document.querySelector("h2")!.textContent = "3 selected items";
        if (variant === "aggregate-single-heading") document.querySelector("h2")!.textContent = "Pipe B — pipe: b";
        if (variant === "aggregate-primary-type") document.querySelector('[data-testid="aggregate-property-inspector"] dl > div:last-child > dd')!.textContent = "node: b";
        if (variant === "aggregate-primary-id") document.querySelector('[data-testid="aggregate-property-inspector"] dl > div:last-child > dd')!.textContent = "pipe: wrong";
        if (variant === "aggregate-duplicate") { const node = document.querySelector('[data-testid="aggregate-property-inspector"]')!; node.after(node.cloneNode(true)); }
        gl.clear(gl.COLOR_BUFFER_BIT); resolve();
      }));
      // Read active state without invoking feedback; stop/pull cannot fabricate it.
      const before = causal.active.feedbackMarkers.length;
      const rejected = JSON.parse(JSON.stringify(causal.active.feedbackRejectionDetails));
      snapshot = coherent(3); setDom();
      if (variant !== "stop-only") await new Promise<void>((resolve) => requestAnimationFrame(() => { gl.clear(gl.COLOR_BUFFER_BIT); resolve(); }));
      return { before, rejected, evidence: causal.stop("staggered") };
    }, variant);
    expect(result.before).toBe(0);
    const rejected = result.rejected.find((r: any) => r.reason === "BOX_SUBMISSION_NOT_READY");
    expect(rejected.observed.failedPredicates.length).toBeGreaterThan(0);
    if (["v27", "stop-only"].includes(variant)) expect(rejected.observed.failedPredicates).toEqual(["selectionOrderedRefs", "selectionPrimary"]);
    expect(JSON.stringify(rejected).length).toBeLessThan(6000);
    expect(result.evidence.active.feedbackMarkers).toHaveLength(variant === "stop-only" ? 0 : 1);
    if (variant !== "stop-only") expect(result.evidence.active.feedbackMarkers[0].observed.mainRenderSubmissionSequence).toBe(3);
    expect((await restoreCausalFeedbackInstrumentation(page)).status).toBe("PASS_FULL_RESTORE");
  }
});
