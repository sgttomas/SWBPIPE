import { writeFile } from "node:fs/promises";
import { expect, test, type Page } from "@playwright/test";
import { ensureInspectorExpanded, openWorkspaceSection, selectCompactOption, selectTreeEntity, startPropertyTaskFromCurrentSelection } from "./workspace-driver";

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

for (const section of ["libraries", "project", "solve"] as const) {
  test(`keyboard page Close restores visible focus and retained state: ${section}`, async ({ page }, info) => {
    await page.goto("/");
    await selectTreeEntity(page, "pipe", "pipe:P-100");
    await startPropertyTaskFromCurrentSelection(page, "pipe", "pipe:P-100");
    await page.getByTestId("editor-intent-value").fill("Retained keyboard Close draft");
    const canvas = await page.locator("canvas").elementHandle();
    const inspector = await page.getByTestId("property-inspector").elementHandle();
    const draft = await page.getByTestId("editor-intent-value").elementHandle();
    if (section === "libraries") {
      await tabTo(page, "rail-page-libraries");
      await page.keyboard.press("Enter");
    } else {
      // The menu command is removed when activated, so Close needs a safe fallback.
      await page.getByTestId("menu-view").click();
      await page.getByTestId(`menu-item-view.section.${section}`).focus();
      await page.keyboard.press("Enter");
    }
    await expect(page.getByTestId(`workspace-section-${section}`)).toBeVisible();
    await tabTo(page, "workspace-dock-close");
    await page.keyboard.press("Enter");
    await expect(page.getByTestId("workspace-dock-close")).toHaveCount(0);
    // No focus call after Close: the application must establish this destination.
    const focus = await page.evaluate(() => {
      const el = document.activeElement as HTMLElement | null;
      if (!el) return null;
      const r = el.getBoundingClientRect(), style = getComputedStyle(el);
      return {
        tag: el.tagName, testId: el.dataset.testid, text: el.textContent?.trim().slice(0, 200),
        inert: Boolean(el.closest("[inert]")), rect: r.toJSON(),
        visible: style.visibility === "visible" && style.display !== "none" && r.width > 0 && r.height > 0,
        unobscured: el.contains(document.elementFromPoint(r.left + r.width / 2, r.top + r.height / 2))
      };
    });
    const session = await page.context().newCDPSession(page);
    let focusedAX;
    try {
      const { result } = await session.send("Runtime.evaluate", { expression: "document.activeElement" });
      const { nodes } = await session.send("Accessibility.getPartialAXTree", { objectId: result.objectId!, fetchRelatives: false });
      focusedAX = nodes.map(node => ({ ignored: node.ignored, role: node.role?.value, name: node.name?.value }));
    } finally { await session.detach(); }
    await witness(info, `${section}-keyboard-close-focus`, { focus, focusedAX });
    expect.soft(focus?.tag).not.toBe("BODY");
    expect.soft(focus?.visible).toBe(true);
    expect.soft(focus?.inert).toBe(false);
    expect.soft(focus?.unobscured).toBe(true);
    expect.soft(focusedAX.some(node => !node.ignored && node.role === "button")).toBe(true);
    if (section === "libraries") await expect.soft(page.getByTestId("rail-page-libraries")).toBeFocused();
    expect(await canvas!.evaluate(el => el === document.querySelector("canvas"))).toBe(true);
    expect(await inspector!.evaluate(el => el === document.querySelector('[data-testid="property-inspector"]'))).toBe(true);
    expect(await draft!.evaluate(el => el === document.querySelector('[data-testid="editor-intent-value"]'))).toBe(true);
    await expect(page.getByTestId("editor-intent-value")).toHaveValue("Retained keyboard Close draft");
    expect(await nativeAXNames(page)).toContain("region: Modeling workspace");
  });
}

for (const layout of ["configured", "narrow"] as const) {
  test(`compact routing selector owns popup Escape before ${layout} shell consumers`, async ({ page }, info) => {
    if (layout === "narrow") await page.setViewportSize({ width: 1200, height: 800 });
    await page.goto("/");
    await page.getByTestId("command-node").click();
    const unit = page.getByTestId("viewport-create-node-unit");
    const draft = page.getByTestId("viewport-create-node-label");
    await draft.fill("Retained popup draft");
    const retained = await draft.elementHandle();
    await unit.focus();
    await page.keyboard.press("Space");
    await expect(unit).toHaveAttribute("aria-expanded", "true");
    const list = page.getByRole("listbox", { name: "New node coordinate unit" });
    await expect(list).toBeVisible();
    const placement = await list.evaluate(el => {
      const rect = el.getBoundingClientRect();
      const center = document.elementFromPoint((rect.left + rect.right) / 2, (rect.top + rect.bottom) / 2);
      return { rect: rect.toJSON(), width: innerWidth, height: innerHeight, portalled: el.parentElement === document.body,
        topmost: el.contains(center) };
    });
    await witness(info, `compact-${layout}-popup-placement`, placement);
    expect(placement.portalled).toBe(true);
    expect(placement.topmost).toBe(true);
    expect(placement.rect.left).toBeGreaterThanOrEqual(0);
    expect(placement.rect.right).toBeLessThanOrEqual(placement.width);
    expect(placement.rect.top).toBeGreaterThanOrEqual(0);
    expect(placement.rect.bottom).toBeLessThanOrEqual(placement.height);
    await page.keyboard.press("ArrowDown");
    await expect(unit).toHaveAttribute("data-value", "m");
    await page.keyboard.press("Escape");
    await expect(list).toHaveCount(0);
    await expect(unit).toBeFocused();
    await expect(unit).toHaveAttribute("data-value", "m");
    await expect(page.getByTestId("toggle-inspector")).toHaveAttribute("aria-expanded", "true");
    await page.keyboard.press("Escape");
    await expect(page.getByTestId("toggle-inspector")).toHaveAttribute("aria-expanded", "false");
    await expect(page.getByTestId("toggle-inspector")).toBeFocused();
    await page.getByTestId("toggle-inspector").click();
    await expect(draft).toHaveValue("Retained popup draft");
    expect(await draft.evaluate((el, old) => el === old, retained)).toBe(true);

    // Silent same-value commits use the same explicit lifecycle as changed commits.
    await unit.click();
    await list.locator('[role="option"][data-value="m"]').click();
    await expect(unit).toHaveAttribute("aria-expanded", "false");
    await expect(unit).toBeFocused();
    await page.keyboard.press("Escape");
    await expect(page.getByTestId("toggle-inspector")).toHaveAttribute("aria-expanded", "false");
    await page.getByTestId("toggle-inspector").click();
    await unit.focus();
    await page.keyboard.press("Space");
    await page.keyboard.press("Enter");
    await expect(unit).toHaveAttribute("aria-expanded", "false");
    await expect(unit).toHaveAttribute("data-value", "m");
    // Browser preview has entered-unit metadata only; alternate unit catalog
    // choices are an actual Tauri check, not injected into this browser scenario.
    await expect(page.getByTestId("workspace-undo")).toBeDisabled();
    await expect(page.getByTestId("workspace-redo")).toBeDisabled();
    await witness(info, `compact-${layout}-ax`, await nativeAXNames(page));

    // A retained control's open popup is dismissed on stage change, without remounting its draft.
    await unit.click();
    await page.getByTestId("rail-stage-loads").click();
    await expect(list).toHaveCount(0);
    await expect(page.getByTestId("rail-stage-loads")).toBeFocused();
    await page.getByTestId("rail-stage-model").click();
    await expect(draft).toHaveValue("Retained popup draft");
    expect(await draft.evaluate((el, old) => el === old, retained)).toBe(true);

    // Exercise a changed value using options actually available in browser mode.
    await page.getByTestId("workspace-select").click();
    await selectTreeEntity(page, "node", "node:N-100");
    await startPropertyTaskFromCurrentSelection(page, "node", "node:N-100");
    const field = page.getByTestId("editor-intent-field");
    await selectCompactOption(field, "position.x");
    await expect(field).toHaveAttribute("data-value", "position.x");
    await expect(page.getByTestId("workspace-undo")).toBeDisabled();
    await expect(page.getByTestId("workspace-redo")).toBeDisabled();
  });
}
