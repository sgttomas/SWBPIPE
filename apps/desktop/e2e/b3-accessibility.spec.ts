import { writeFile } from "node:fs/promises";
import { expect, test, type Page } from "@playwright/test";
import { ensureInspectorExpanded, openWorkspaceSection, selectTreeEntity, startPropertyTaskFromCurrentSelection } from "./workspace-driver";

// Playwright ariaSnapshot is DOM-derived and includes inert content; query Chromium AX directly.
async function nativeAXNames(page: Page) {
  const session = await page.context().newCDPSession(page);
  try {
    const { nodes } = await session.send("Accessibility.getFullAXTree");
    return nodes.filter(node => !node.ignored).map(node => `${node.role?.value}: ${node.name?.value}`);
  } finally { await session.detach(); }
}

async function witness(info: import("@playwright/test").TestInfo, name: string, value: unknown) {
  const body = JSON.stringify(value, null, 2);
  await info.attach(name, { body, contentType: "application/json" });
  const directory = process.env.B3_A11Y_EVIDENCE_DIR;
  if (directory) await writeFile(`${directory}/${info.project.name}-${name}.json`, body);
}

async function tabTo(page: Page, testId: string) {
  for (let i = 0; i < 150; i++) {
    await page.keyboard.press("Tab");
    if (await page.getByTestId(testId).evaluate(el => el === document.activeElement)) return;
  }
  throw new Error(`Tab did not reach ${testId}`);
}

test("disabled rail and Agent reasons escape clipping under keyboard focus", async ({ page }, info) => {
  await page.goto("/");
  await expect(page.getByTestId("workspace-toolbar")).toBeVisible();
  for (const id of ["rail-stage-results", "rail-stage-review", "agent-strip-open"]) {
    await tabTo(page, id);
    const reasonId = await page.getByTestId(id).getAttribute("aria-describedby");
    const reason = page.locator(`#${reasonId}`);
    await expect(reason).toBeVisible();
    const witness = await reason.evaluate(el => {
      const r = el.getBoundingClientRect();
      const clips: string[] = [];
      for (let p = el.parentElement; p; p = p.parentElement) {
        const s = getComputedStyle(p), b = p.getBoundingClientRect();
        if ((/(hidden|clip|auto|scroll)/.test(s.overflowX) && (r.left < b.left || r.right > b.right)) ||
            (/(hidden|clip|auto|scroll)/.test(s.overflowY) && (r.top < b.top || r.bottom > b.bottom))) clips.push(p.className);
      }
      const points = [[r.left + 2, r.top + 2], [r.right - 2, r.top + 2], [r.left + 2, r.bottom - 2], [r.right - 2, r.bottom - 2], [(r.left+r.right)/2, (r.top+r.bottom)/2]];
      return { text: el.textContent, rect: r.toJSON(), clips, unobscured: points.every(([x,y]) => el.contains(document.elementFromPoint(x,y))) };
    });
    await info.attach(id, { body: JSON.stringify(witness), contentType: "application/json" });
    if (process.env.B3_A11Y_EVIDENCE_DIR) await writeFile(`${process.env.B3_A11Y_EVIDENCE_DIR}/${info.project.name}-${id}.json`, JSON.stringify(witness, null, 2));
    expect.soft(witness.clips, id).toEqual([]);
    expect.soft(witness.unobscured, id).toBe(true);
    if (process.env.B3_A11Y_EVIDENCE_DIR && id === "rail-stage-results") await reason.screenshot({ path: `${process.env.B3_A11Y_EVIDENCE_DIR}/${info.project.name}-results-reason.png` });
  }
});

test("covered pages exclude retained stage controls from Tab and accessibility", async ({ page }, info) => {
  await page.goto("/");
  await selectTreeEntity(page, "pipe", "pipe:P-100");
  await startPropertyTaskFromCurrentSelection(page, "pipe", "pipe:P-100");
  await page.getByTestId("editor-intent-value").fill("Retained accessibility draft");
  const canvas = await page.locator("canvas").elementHandle();
  const draft = await page.getByTestId("editor-intent-value").elementHandle();
  const inspector = await page.getByTestId("property-inspector").elementHandle();
  for (const section of ["libraries", "project", "solve"] as const) {
    await openWorkspaceSection(page, section);
    const ax = await nativeAXNames(page);
    await witness(info, `${section}-native-ax`, ax);
    expect.soft(ax, section).not.toContain("region: Modeling workspace");
    // Starting immediately before the surfaces ensures this traversal crosses their position.
    await page.getByTestId("rail-issues").focus();
    const coveredFocus: string[] = [];
    for (let i = 0; i < 70; i++) {
      await page.keyboard.press("Tab");
      const target = await page.evaluate(() => document.activeElement?.closest('.shell-surfaces') ? document.activeElement?.outerHTML.slice(0, 220) : null);
      if (target) coveredFocus.push(target);
    }
    await witness(info, `${section}-covered-focus`, coveredFocus);
    expect.soft(coveredFocus, section).toEqual([]);
    await page.getByTestId("workspace-dock-close").click();
    expect(await canvas!.evaluate(el => el === document.querySelector("canvas"))).toBe(true);
    expect(await inspector!.evaluate(el => el === document.querySelector('[data-testid="property-inspector"]'))).toBe(true);
    expect(await draft!.evaluate(el => el === document.querySelector('[data-testid="editor-intent-value"]'))).toBe(true);
    await expect(page.getByTestId("editor-intent-value")).toHaveValue("Retained accessibility draft");
    await page.getByTestId("editor-intent-value").focus();
    await expect(page.getByTestId("editor-intent-value")).toBeFocused();
    expect(await nativeAXNames(page)).toContain("region: Modeling workspace");
  }
});

test("Both inspector Escape closes from property tab and input to opener", async ({ page }) => {
  await page.goto("/");
  await selectTreeEntity(page, "pipe", "pipe:P-100");
  await startPropertyTaskFromCurrentSelection(page, "pipe", "pipe:P-100");
  for (const target of [page.getByTestId("property-inspector").getByRole("tab", { name: "Task", exact: true }), page.getByTestId("editor-intent-value")]) {
    await ensureInspectorExpanded(page);
    await target.focus();
    await page.keyboard.press("Escape");
    await expect.soft(page.getByTestId("toggle-inspector")).toHaveAttribute("aria-expanded", "false");
    await expect.soft(page.getByTestId("toggle-inspector")).toBeFocused();
  }
});

test("Model inspector stays docked on Escape", async ({ page }) => {
  await page.goto("/");
  await page.getByTestId("view-switch-model").click();
  const tab = page.getByTestId("property-inspector").getByRole("tab", { name: "Properties", exact: true });
  await tab.focus();
  await page.keyboard.press("Escape");
  await expect(page.getByTestId("toggle-inspector")).toHaveAttribute("aria-expanded", "true");
  await expect(page.getByTestId("property-inspector")).toBeVisible();
});

test("toolkit child consumes first Escape before inspector close", async ({ page }) => {
  await page.goto("/");
  await ensureInspectorExpanded(page);
  await page.getByTestId("toolkit-entry").click();
  await expect(page.getByRole("searchbox", { name: "Find a tool" })).toBeFocused();
  await expect(page.locator("#toolkit-commands")).toBeVisible();
  await page.keyboard.press("Escape");
  await expect(page.locator("#toolkit-commands")).toHaveCount(0);
  await expect(page.getByTestId("toolkit-entry")).toBeFocused();
  await expect(page.getByTestId("toggle-inspector")).toHaveAttribute("aria-expanded", "true");
  await page.getByTestId("property-inspector").getByRole("tab", { name: "Properties", exact: true }).focus();
  await page.keyboard.press("Escape");
  await expect(page.getByTestId("toggle-inspector")).toHaveAttribute("aria-expanded", "false");
  await expect(page.getByTestId("toggle-inspector")).toBeFocused();
});

test("portalled routing Escape closes explicitly and supersedes automatic restore", async ({ page }) => {
  await page.goto("/");
  // Prior open state is the dangerous case: disarming must not reopen after explicit close.
  await ensureInspectorExpanded(page);
  await page.getByTestId("command-pipe").click();
  const input = page.locator("#shell-routing-panel").getByTestId("viewport-create-pipe-id");
  await expect(input).toBeVisible();
  await input.fill("pipe:keyboard-retained");
  await page.keyboard.press("Escape");
  await expect(page.getByTestId("toggle-inspector")).toHaveAttribute("aria-expanded", "false");
  await expect(page.getByTestId("toggle-inspector")).toBeFocused();
  await page.getByTestId("workspace-select").click();
  await expect(page.getByTestId("command-pipe")).toHaveAttribute("aria-pressed", "false");
  await expect(page.getByTestId("toggle-inspector")).toHaveAttribute("aria-expanded", "false");
});
