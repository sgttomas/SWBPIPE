import { createHash } from "node:crypto";
import { mkdir, readFile, writeFile } from "node:fs/promises";
import path from "node:path";
import { chromium } from "playwright";

const evidenceDir = process.argv[2];
const run = Number(process.argv[3] ?? "1");
if (!evidenceDir || !path.isAbsolute(evidenceDir)) {
  throw new Error("usage: node load-availability-diagnostic.mjs <absolute-evidence-dir> <run-number>");
}

const here = path.dirname(new URL(import.meta.url).pathname);
const fixturePath = path.join(here, "fixtures", "ui-foundation-10000.model.json");
const fixtureBytes = await readFile(fixturePath, "utf8");
const fixture = JSON.parse(fixtureBytes);
const executablePath = process.env.PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH ??
  "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome";
const runDir = path.join(evidenceDir, `availability-run-${String(run).padStart(2, "0")}`);
await mkdir(runDir, { recursive: true });

const startedWallTime = new Date().toISOString();
const browser = await chromium.launch({ headless: true, executablePath });
let agentInitiatedClose = false;
const events = [];
const push = (type, detail = {}) => events.push({ type, atWallTime: new Date().toISOString(), ...detail });
browser.on("disconnected", () => push("browser-disconnected", { agentInitiatedClose }));

let systemInfo;
try {
  const browserSession = await browser.newBrowserCDPSession();
  systemInfo = await browserSession.send("SystemInfo.getInfo");
  await browserSession.detach();
} catch (error) {
  systemInfo = { status: "UNAVAILABLE", error: String(error) };
}

const gpuProbePage = await browser.newPage();
await gpuProbePage.goto("data:text/html,<canvas id='c'></canvas>");
const webgl = await gpuProbePage.evaluate(() => {
  const canvas = document.querySelector("canvas");
  const gl = canvas?.getContext("webgl2") ?? canvas?.getContext("webgl");
  if (!gl) return { status: "UNAVAILABLE" };
  const debug = gl.getExtension("WEBGL_debug_renderer_info");
  return {
    status: "AVAILABLE",
    vendor: gl.getParameter(gl.VENDOR),
    renderer: gl.getParameter(gl.RENDERER),
    unmaskedVendor: debug ? gl.getParameter(debug.UNMASKED_VENDOR_WEBGL) : null,
    unmaskedRenderer: debug ? gl.getParameter(debug.UNMASKED_RENDERER_WEBGL) : null
  };
});
await gpuProbePage.close();

const page = await browser.newPage({ viewport: { width: 1440, height: 920 }, deviceScaleFactor: 2 });
let observerSettled = false;
let resolveTerminal;
const terminal = new Promise((resolve) => { resolveTerminal = resolve; });
const terminate = (status, detail = {}) => {
  push(status, detail);
  if (!observerSettled) {
    observerSettled = true;
    resolveTerminal({ status, detail, observedAtWallTime: new Date().toISOString() });
  }
};
page.on("crash", () => terminate("PAGE_CRASH"));
page.on("close", () => {
  if (!agentInitiatedClose) terminate("PAGE_CLOSED_UNEXPECTEDLY");
});
page.on("pageerror", (error) => push("pageerror", { message: String(error), stack: error?.stack ?? null }));
page.on("console", (message) => push("console", { level: message.type(), text: message.text() }));

const moduleBody = [
  `const model=${fixtureBytes.trim()};`,
  "if(globalThis.__uifLoadDiagnostic){globalThis.__uifLoadDiagnostic.fixtureModuleEvaluated=performance.now();}",
  "export { model as default };"
].join("\n");
await page.addInitScript(() => {
  globalThis.__uifLoadDiagnostic = { fixtureModuleEvaluated: null, navigationStart: performance.now() };
});
await page.route(/(?:invented_preview_model-[^/]+\.js|fixtures\/product_preview\/invented_preview_model\.json)(?:\?.*)?$/, (route) =>
  route.fulfill({ status: 200, contentType: "application/javascript", body: moduleBody }));

const navigationStarted = Date.now();
let navigation;
try {
  const response = await page.goto("http://127.0.0.1:5176/", { waitUntil: "domcontentloaded", timeout: 30_000 });
  navigation = { status: "DOM_CONTENT_LOADED", httpStatus: response?.status() ?? null, durationMs: Date.now() - navigationStarted };
} catch (error) {
  navigation = { status: "NAVIGATION_FAILED", error: String(error), durationMs: Date.now() - navigationStarted };
  terminate("NAVIGATION_FAILED", { error: String(error) });
}

const expectedTreeCount = 1 + fixture.materials.length + fixture.sections.length + fixture.nodes.length +
  fixture.pipe_segments.length + fixture.supports.length + fixture.components.length + fixture.load_cases.length +
  (fixture.combinations?.length ?? 0);
const ready = (async () => {
  await page.getByTestId(`tree-row-${fixture.project.id}`).waitFor({ state: "visible", timeout: 60_000 });
  await page.getByTestId("viewport-canvas").waitFor({ state: "visible", timeout: 60_000 });
  await page.waitForFunction((count) => {
    const summary = document.querySelector('[data-testid="model-tree-filter-summary"]')?.textContent ?? "";
    return summary.includes(`${count} of ${count}`) && Boolean(document.querySelector('[data-testid="viewport-canvas"] canvas'));
  }, expectedTreeCount, { timeout: 60_000 });
  return { status: "FIRST_USABLE_OBSERVED", observedAtWallTime: new Date().toISOString() };
})().catch((error) => ({ status: "READINESS_REJECTED", error: String(error), observedAtWallTime: new Date().toISOString() }));
const timeout = new Promise((resolve) => setTimeout(() => resolve({
  status: "BOUNDED_TIMEOUT_60S",
  observedAtWallTime: new Date().toISOString()
}), 60_000));
const outcome = await Promise.race([ready, terminal, timeout]);

let finalPageState = null;
if (!page.isClosed()) {
  finalPageState = await page.evaluate(() => ({
    url: location.href,
    title: document.title,
    bodyTextPrefix: document.body?.innerText?.slice(0, 500) ?? null,
    diagnostic: globalThis.__uifLoadDiagnostic ?? null,
    performanceNow: performance.now()
  })).catch((error) => ({ status: "PAGE_EVALUATION_FAILED", error: String(error) }));
  await page.screenshot({ path: path.join(runDir, "final-page.png") }).catch((error) => push("screenshot-error", { error: String(error) }));
}

const result = {
  schema: "openpipestress.ui-foundation.load-availability-diagnostic/v1",
  run,
  startedWallTime,
  finishedWallTime: new Date().toISOString(),
  fixture: {
    path: fixturePath,
    sha256: createHash("sha256").update(fixtureBytes).digest("hex"),
    pipeCount: fixture.pipe_segments.length,
    expectedTreeCount
  },
  browser: { version: browser.version(), executablePath, connectedBeforeAgentClose: browser.isConnected() },
  launch: { headless: true, debugLogContract: "Run with DEBUG=pw:browser to retain launch command and browser stderr." },
  systemInfo,
  webgl,
  navigation,
  outcome,
  finalPageState,
  events
};
await writeFile(path.join(runDir, "result.json"), `${JSON.stringify(result, null, 2)}\n`);
agentInitiatedClose = true;
if (browser.isConnected()) await browser.close();
process.stdout.write(`${JSON.stringify({ run, outcome, webgl }, null, 2)}\n`);
