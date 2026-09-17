import { createHash } from "node:crypto";
import { createReadStream, existsSync } from "node:fs";
import { readFile } from "node:fs/promises";
import path from "node:path";
import { expect, type Browser, type Locator, type Page, type TestInfo } from "@playwright/test";
import {
  installInstrumentation,
  routeModelFixture,
  settledRafState,
  treeRowTestId,
} from "./ui-foundation/benchmark-harness";

export type AppearanceTheme = "light" | "dark";
export type AppearanceDensity = "comfortable" | "compact";
export type ViewportSize = Readonly<{ width: number; height: number }>;

export const APPEARANCE_THEMES: readonly AppearanceTheme[] = ["light", "dark"];
export const APPEARANCE_DENSITIES: readonly AppearanceDensity[] = ["comfortable", "compact"];
export const APPEARANCE_VIEWPORTS: readonly ViewportSize[] = [
  { width: 1024, height: 768 },
  { width: 1280, height: 800 },
  { width: 1440, height: 920 },
];

export async function attachBrowserIdentity(browser: Browser, testInfo: TestInfo): Promise<void> {
  const session = await browser.newBrowserCDPSession();
  let cdpVersion: unknown;
  try {
    cdpVersion = await session.send("Browser.getVersion");
  } finally {
    await session.detach();
  }
  const configuredChrome = process.env.PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH;
  const macChrome = "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome";
  const executablePath = configuredChrome && existsSync(configuredChrome)
    ? configuredChrome
    : existsSync(macChrome) ? macChrome : null;
  await testInfo.attach("browser-identity", {
    body: JSON.stringify({
      cdpVersion,
      playwrightBrowserVersion: browser.version(),
      executablePath,
      executableSha256: executablePath ? await sha256File(executablePath) : null,
    }, null, 2),
    contentType: "application/json",
  });
}

function sha256File(filePath: string): Promise<string> {
  return new Promise((resolve, reject) => {
    const digest = createHash("sha256");
    const stream = createReadStream(filePath);
    stream.on("data", (chunk) => digest.update(chunk));
    stream.on("error", reject);
    stream.on("end", () => resolve(digest.digest("hex")));
  });
}

// Keep the two late writer bindings here. The final source-release intake only
// has to rebind this small surface if the visible group IDs or measurement
// labels change; the qualification assertions remain unchanged.
export const COMMAND_GROUPS = [
  "Build",
  "Supports",
  "Properties",
  "Loads",
  "Edit",
  "Select and View",
  "Review",
] as const;

export function commandGroupControl(page: Page, group: typeof COMMAND_GROUPS[number]): Locator {
  const slug = group.toLowerCase().replace(/\s+and\s+/g, "-").replace(/\s+/g, "-");
  return page.getByTestId(`toolkit-group-${slug}`);
}

const fixtureRoot = path.join(import.meta.dirname, "ui-foundation", "fixtures");

export async function readFixture(name: string): Promise<{ bytes: string; model: any }> {
  const bytes = await readFile(path.join(fixtureRoot, name), "utf8");
  return { bytes, model: JSON.parse(bytes) };
}

export async function gotoRoutedFixture(
  page: Page,
  name = "precision-origin-base.model.json",
  options: Readonly<{ captureGlobalRaf?: boolean }> = {},
): Promise<any> {
  const fixture = await readFixture(name);
  await installInstrumentation(page, { captureGlobalRaf: options.captureGlobalRaf === true });
  await routeModelFixture(page, fixture);
  await page.goto("/");
  await expect(page.getByTestId("desktop-preview-shell")).toBeVisible();
  await expect(page.getByTestId("viewport-canvas").locator("canvas")).toHaveCount(1);
  return fixture.model;
}

export async function waitForSettledGlobalRaf(page: Page, timeoutMs = 5_000): Promise<any> {
  return settledRafState(page, timeoutMs);
}

export async function gotoModel(page: Page, model: any): Promise<void> {
  await installInstrumentation(page, { captureGlobalRaf: false });
  await routeModelFixture(page, { bytes: JSON.stringify(model) });
  await page.goto("/");
  await expect(page.getByTestId("desktop-preview-shell")).toBeVisible();
  await expect(page.getByTestId("viewport-canvas").locator("canvas")).toHaveCount(1);
}

export function typedTreeRow(page: Page, type: string, id: string): Locator {
  return page.getByTestId(treeRowTestId({ type, id }, "candidate"));
}

export async function ensureRail(page: Page, side: "tree" | "inspector", open: boolean): Promise<void> {
  const toggle = page.getByTestId(side === "tree" ? "toggle-tree" : "toggle-inspector");
  const expected = String(open);
  if (await toggle.getAttribute("aria-expanded") !== expected) {
    await activateWithKeyboard(page, toggle);
  }
  await expect(toggle).toHaveAttribute("aria-expanded", expected);
}

export async function revealTreeRow(page: Page, type: string, id: string): Promise<Locator> {
  await ensureRail(page, "tree", true);
  const filter = page.getByTestId("model-tree-filter-input");
  await filter.fill(id);
  const row = typedTreeRow(page, type, id);
  await expect(row).toBeVisible();
  return row;
}

export async function selectTreeRow(
  page: Page,
  type: string,
  id: string,
  modifiers: Readonly<{ shift?: boolean; toggle?: boolean }> = {},
): Promise<Locator> {
  const row = await revealTreeRow(page, type, id);
  await row.click({
    modifiers: [
      ...(modifiers.shift ? ["Shift" as const] : []),
      ...(modifiers.toggle ? [process.platform === "darwin" ? "Meta" as const : "Control" as const] : []),
    ],
  });
  return row;
}

export async function clearTreeFilter(page: Page): Promise<void> {
  const clear = page.getByTestId("clear-model-tree-filter");
  if (await clear.isEnabled()) await activateWithKeyboard(page, clear);
  await expect(page.getByTestId("model-tree-filter-input")).toHaveValue("");
}

export async function activateWithKeyboard(page: Page, control: Locator): Promise<void> {
  await control.focus();
  await expect(control).toBeFocused();
  await page.keyboard.press("Enter");
}

export async function setAppearance(
  page: Page,
  theme: AppearanceTheme,
  density: AppearanceDensity,
): Promise<void> {
  await page.getByLabel("Appearance theme").selectOption(theme);
  await page.getByLabel("Workspace density").selectOption(density);
  const shell = page.getByTestId("desktop-preview-shell");
  await expect(shell).toHaveAttribute("data-theme", theme);
  await expect(shell).toHaveAttribute("data-theme-preference", theme);
  await expect(shell).toHaveAttribute("data-density", density);
}

export async function openWorkspaceSection(page: Page, id: string): Promise<Locator> {
  const section = page.getByTestId(`workspace-section-${id}`);
  if (!await section.isVisible()) {
    if (id === "operations") {
      await activateWithKeyboard(page, page.getByTestId("workspace-review"));
    } else {
      await activateWithKeyboard(page, page.getByTestId("menu-view"));
      await activateWithKeyboard(page, page.getByTestId(`menu-item-view.section.${id}`));
    }
  }
  await expect(section).toBeVisible();
  return section;
}

export async function currentModelHashThroughVisibleExport(page: Page): Promise<string> {
  await openWorkspaceSection(page, "project");
  const consent = page.getByTestId("project-validation-export-link-local-private-intent");
  if (!await consent.isChecked()) await consent.check();
  const link = page.getByTestId("project-validation-export-link");
  await expect.poll(async () => decodeModelHash(await link.getAttribute("href"))).toMatch(/^sha256:[0-9a-f]{64}$/);
  const hash = decodeModelHash(await link.getAttribute("href"));
  if (!hash) throw new Error("The visible project-validation export did not expose a model hash.");
  await activateWithKeyboard(page, page.getByTestId("workspace-dock-close"));
  return hash;
}

function decodeModelHash(href: string | null): string | null {
  if (!href?.includes(",")) return null;
  try {
    const payload = JSON.parse(decodeURIComponent(href.split(",").slice(1).join(",")));
    return typeof payload?.model_hash?.value === "string" ? payload.model_hash.value : null;
  } catch {
    return null;
  }
}

export async function expectWorkspaceGeometry(page: Page, viewport: ViewportSize): Promise<void> {
  const geometry = await page.evaluate(() => {
    const rect = (selector: string) => document.querySelector<HTMLElement>(selector)?.getBoundingClientRect().toJSON() ?? null;
    return {
      bodyOverflowX: document.documentElement.scrollWidth - document.documentElement.clientWidth,
      bodyOverflowY: document.documentElement.scrollHeight - document.documentElement.clientHeight,
      canvas: rect('[data-testid="viewport-canvas"]'),
      workspace: rect('[data-testid="modeling-workspace"]'),
      tree: rect(".workspace-pane-tree"),
      inspector: rect(".workspace-pane-inspector"),
      shell: rect('[data-testid="desktop-preview-shell"]'),
      window: { width: innerWidth, height: innerHeight },
    };
  });
  expect(geometry.window).toEqual(viewport);
  expect(geometry.bodyOverflowX).toBe(0);
  expect(geometry.bodyOverflowY).toBeLessThanOrEqual(1);
  expect(geometry.canvas?.width).toBeGreaterThan(viewport.width * 0.35);
  expect(geometry.canvas?.height).toBeGreaterThan(viewport.height * 0.35);
  expect((geometry.canvas?.width ?? 0) * (geometry.canvas?.height ?? 0)).toBeGreaterThan(
    Math.max(
      (geometry.tree?.width ?? 0) * (geometry.tree?.height ?? 0),
      (geometry.inspector?.width ?? 0) * (geometry.inspector?.height ?? 0),
    ),
  );
  expect(geometry.canvas?.bottom).toBeLessThanOrEqual(viewport.height + 1);
  expect(geometry.shell?.right).toBeLessThanOrEqual(viewport.width + 1);
}

export async function expectResolvedStyleAndTargets(page: Page): Promise<void> {
  const snapshot = await page.evaluate(() => {
    const rgba = (raw: string): [number, number, number, number] | null => {
      const values = raw.match(/[\d.]+/g)?.map(Number) ?? [];
      if (values.length < 3) return null;
      return [values[0], values[1], values[2], values[3] ?? 1];
    };
    const opaqueBackground = (element: Element): [number, number, number, number] => {
      for (let current: Element | null = element; current; current = current.parentElement) {
        const parsed = rgba(getComputedStyle(current).backgroundColor);
        if (parsed && parsed[3] > 0.98) return parsed;
      }
      return [255, 255, 255, 1];
    };
    const linear = (channel: number) => {
      const value = channel / 255;
      return value <= 0.04045 ? value / 12.92 : ((value + 0.055) / 1.055) ** 2.4;
    };
    const luminance = (color: [number, number, number, number]) =>
      linear(color[0]) * 0.2126 + linear(color[1]) * 0.7152 + linear(color[2]) * 0.0722;
    const ratio = (left: [number, number, number, number] | null, right: [number, number, number, number] | null) => {
      if (!left || !right) return 0;
      const a = luminance(left);
      const b = luminance(right);
      return (Math.max(a, b) + 0.05) / (Math.min(a, b) + 0.05);
    };
    const measure = (selector: string, visibleOnly = false) => {
      const candidates = [...document.querySelectorAll<HTMLElement>(selector)];
      const element = visibleOnly
        ? candidates.find((candidate) => {
            const box = candidate.getBoundingClientRect();
            return box.width > 0 && box.height > 0;
          }) ?? null
        : candidates[0] ?? null;
      if (!element) return null;
      const style = getComputedStyle(element);
      const box = element.getBoundingClientRect();
      return {
        selector,
        box: { width: box.width, height: box.height },
        textRatio: ratio(rgba(style.color), opaqueBackground(element)),
        borderRatio: ratio(rgba(style.borderTopColor), opaqueBackground(element)),
        outlineRatio: ratio(rgba(style.outlineColor), opaqueBackground(element)),
        outlineWidth: Number.parseFloat(style.outlineWidth),
      };
    };
    return {
      command: measure('[data-testid="workspace-review"]'),
      focus: measure('[data-testid="workspace-select"]'),
      treeToggle: measure('[data-testid="toggle-tree"]'),
      disabled: measure('[data-testid="workspace-undo"]'),
      selected: measure('[role="treeitem"][aria-selected="true"]', true),
    };
  });
  for (const entry of [snapshot.command, snapshot.treeToggle]) {
    expect(entry, entry?.selector ?? "missing resolved-style witness").not.toBeNull();
    expect(entry!.textRatio, `${entry!.selector} resolved text contrast`).toBeGreaterThanOrEqual(4.5);
    expect(entry!.box.width, `${entry!.selector} target width`).toBeGreaterThanOrEqual(24);
    expect(entry!.box.height, `${entry!.selector} target height`).toBeGreaterThanOrEqual(24);
  }
  if (snapshot.selected) {
    expect(snapshot.selected.textRatio, "visible selected tree row resolved text contrast").toBeGreaterThanOrEqual(4.5);
    expect(snapshot.selected.box.width, "visible selected tree row target width").toBeGreaterThanOrEqual(24);
    expect(snapshot.selected.box.height, "visible selected tree row target height").toBeGreaterThanOrEqual(24);
  }
  expect(snapshot.command!.borderRatio, "resolved command border contrast").toBeGreaterThanOrEqual(3);
  expect(snapshot.focus?.outlineWidth, "visible focus outline width").toBeGreaterThanOrEqual(2);
  expect(snapshot.focus?.outlineRatio, "resolved focus graphical contrast").toBeGreaterThanOrEqual(3);
  expect(snapshot.disabled?.box.width).toBeGreaterThanOrEqual(24);
  expect(snapshot.disabled?.box.height).toBeGreaterThanOrEqual(24);
}

export async function expectSelectedTreeTarget(row: Locator): Promise<void> {
  const metrics = await row.evaluate((element) => {
    const rgba = (raw: string): [number, number, number, number] | null => {
      const values = raw.match(/[\d.]+/g)?.map(Number) ?? [];
      return values.length < 3 ? null : [values[0], values[1], values[2], values[3] ?? 1];
    };
    const linear = (channel: number) => {
      const value = channel / 255;
      return value <= 0.04045 ? value / 12.92 : ((value + 0.055) / 1.055) ** 2.4;
    };
    const luminance = (color: [number, number, number, number]) =>
      linear(color[0]) * 0.2126 + linear(color[1]) * 0.7152 + linear(color[2]) * 0.0722;
    const style = getComputedStyle(element);
    let background: [number, number, number, number] = [255, 255, 255, 1];
    for (let current: Element | null = element; current; current = current.parentElement) {
      const candidate = rgba(getComputedStyle(current).backgroundColor);
      if (candidate && candidate[3] > 0.98) {
        background = candidate;
        break;
      }
    }
    const foreground = rgba(style.color);
    const box = element.getBoundingClientRect();
    const textRatio = foreground
      ? (Math.max(luminance(foreground), luminance(background)) + 0.05) /
        (Math.min(luminance(foreground), luminance(background)) + 0.05)
      : 0;
    return { box: { width: box.width, height: box.height }, textRatio };
  });
  expect(metrics.textRatio, "visible selected tree row resolved text contrast").toBeGreaterThanOrEqual(4.5);
  expect(metrics.box.width, "visible selected tree row target width").toBeGreaterThanOrEqual(24);
  expect(metrics.box.height, "visible selected tree row target height").toBeGreaterThanOrEqual(24);
}

export type ResolvedContrastWitness = Readonly<{
  name: string;
  locator: Locator;
  minimum?: 3 | 4.5;
  source?: "text" | "graphic";
}>;

export async function expectResolvedContrast(
  witnesses: readonly ResolvedContrastWitness[],
): Promise<readonly unknown[]> {
  const results = [];
  for (const witness of witnesses) {
    await expect(witness.locator, `${witness.name} is rendered`).toBeVisible();
    const result = await witness.locator.evaluate((element) => {
      type Color = [number, number, number, number];
      const parse = (raw: string): Color | null => {
        const values = raw.match(/[\d.]+/g)?.map(Number) ?? [];
        return values.length < 3 ? null : [values[0], values[1], values[2], values[3] ?? 1];
      };
      const over = (foreground: Color, background: Color): Color => {
        const alpha = foreground[3] + background[3] * (1 - foreground[3]);
        if (alpha <= 0) return [0, 0, 0, 0];
        return [
          (foreground[0] * foreground[3] + background[0] * background[3] * (1 - foreground[3])) / alpha,
          (foreground[1] * foreground[3] + background[1] * background[3] * (1 - foreground[3])) / alpha,
          (foreground[2] * foreground[3] + background[2] * background[3] * (1 - foreground[3])) / alpha,
          alpha,
        ];
      };
      const linear = (channel: number) => {
        const value = channel / 255;
        return value <= 0.04045 ? value / 12.92 : ((value + 0.055) / 1.055) ** 2.4;
      };
      const luminance = (color: Color) =>
        linear(color[0]) * 0.2126 + linear(color[1]) * 0.7152 + linear(color[2]) * 0.0722;
      const ratio = (left: Color, right: Color) => {
        const a = luminance(left);
        const b = luminance(right);
        return (Math.max(a, b) + 0.05) / (Math.min(a, b) + 0.05);
      };
      const chain: Element[] = [];
      for (let current: Element | null = element; current; current = current.parentElement) chain.push(current);
      let background: Color = [255, 255, 255, 1];
      const backgroundLayers: Array<{ owner: string; color: string }> = [];
      for (const current of chain.reverse()) {
        const raw = getComputedStyle(current).backgroundColor;
        const parsed = parse(raw);
        if (!parsed || parsed[3] <= 0) continue;
        background = over(parsed, background);
        backgroundLayers.push({
          owner: current.getAttribute("data-testid")
            ?? current.getAttribute("aria-label")
            ?? current.id
            ?? current.tagName,
          color: raw,
        });
      }
      const style = getComputedStyle(element);
      const foreground = parse(style.color);
      const box = element.getBoundingClientRect();
      return {
        box: { x: box.x, y: box.y, width: box.width, height: box.height },
        foreground: style.color,
        effectiveBackground: `rgba(${background.join(", ")})`,
        backgroundLayers,
        ratio: foreground ? ratio(over(foreground, background), background) : 0,
        text: element.textContent?.trim() ?? "",
      };
    });
    const minimum = witness.minimum ?? (witness.source === "graphic" ? 3 : 4.5);
    expect(result.box.width, `${witness.name} has rendered width`).toBeGreaterThan(0);
    expect(result.box.height, `${witness.name} has rendered height`).toBeGreaterThan(0);
    expect(
      result.backgroundLayers.length,
      `${witness.name} resolves at least one actual rendered background layer`,
    ).toBeGreaterThan(0);
    expect(result.ratio, `${witness.name} resolved contrast`).toBeGreaterThanOrEqual(minimum);
    results.push({ ...result, name: witness.name, minimum, source: witness.source ?? "text" });
  }
  return results;
}

export async function expectOverlayAboveWorkspaceControls(
  page: Page,
  surface: Locator,
  controls: readonly Readonly<{ name: string; locator: Locator }>[],
): Promise<unknown> {
  await expect(surface).toBeVisible();
  const surfaceHandle = await surface.elementHandle();
  expect(surfaceHandle, "overlay surface has a DOM handle").not.toBeNull();
  const intersections = [];
  for (const control of controls) {
    if (!await control.locator.isVisible()) continue;
    const witness = await control.locator.evaluate((element, overlay) => {
      const surfaceElement = overlay as HTMLElement;
      const surfaceBox = surfaceElement.getBoundingClientRect();
      const controlBox = element.getBoundingClientRect();
      const left = Math.max(surfaceBox.left, controlBox.left);
      const top = Math.max(surfaceBox.top, controlBox.top);
      const right = Math.min(surfaceBox.right, controlBox.right);
      const bottom = Math.min(surfaceBox.bottom, controlBox.bottom);
      if (right <= left || bottom <= top) return null;
      const point = { x: left + (right - left) / 2, y: top + (bottom - top) / 2 };
      const owner = document.elementFromPoint(point.x, point.y);
      return {
        point,
        control: element.getAttribute("data-testid") ?? element.getAttribute("aria-label") ?? element.tagName,
        topmost: owner instanceof Element
          ? owner.getAttribute("data-testid")
            ?? owner.getAttribute("aria-label")
            ?? owner.id
            ?? owner.tagName
          : null,
        surfaceOwnsPoint: owner instanceof Node && (surfaceElement === owner || surfaceElement.contains(owner)),
      };
    }, surfaceHandle);
    if (!witness) continue;
    expect(
      witness.surfaceOwnsPoint,
      `${control.name} paints or hit-tests above the open overlay at ${JSON.stringify(witness.point)} (${witness.topmost})`,
    ).toBe(true);
    intersections.push({ name: control.name, ...witness });
  }
  await surfaceHandle!.dispose();
  return { intersections };
}

export async function expectTreeRowIdentityReadable(row: Locator, label: string, id: string): Promise<void> {
  const name = row.locator("strong");
  const detail = row.locator("small");
  await expect(name).toHaveText(label);
  await expect(detail).toContainText(id);
  const metrics = await row.evaluate((element) => {
    const rowBox = element.getBoundingClientRect();
    const inspect = (child: Element | null) => {
      if (!(child instanceof HTMLElement)) return null;
      const box = child.getBoundingClientRect();
      const style = getComputedStyle(child);
      return {
        top: box.top,
        bottom: box.bottom,
        height: box.height,
        clientHeight: child.clientHeight,
        scrollHeight: child.scrollHeight,
        clientWidth: child.clientWidth,
        scrollWidth: child.scrollWidth,
        overflow: style.overflow,
        textOverflow: style.textOverflow,
        whiteSpace: style.whiteSpace,
      };
    };
    return {
      row: { top: rowBox.top, bottom: rowBox.bottom },
      name: inspect(element.querySelector("strong")),
      detail: inspect(element.querySelector("small")),
    };
  });
  for (const [kind, entry] of [["name", metrics.name], ["typed ID", metrics.detail]] as const) {
    expect(entry, `${kind} element exists`).not.toBeNull();
    expect(entry!.height, `${kind} has rendered height`).toBeGreaterThan(0);
    expect(entry!.top, `${kind} is not clipped above its fixed-height row`).toBeGreaterThanOrEqual(metrics.row.top - 1);
    expect(entry!.bottom, `${kind} is not clipped below its fixed-height row`).toBeLessThanOrEqual(metrics.row.bottom + 1);
    expect(entry!.scrollHeight, `${kind} does not wrap beyond its visible line box`).toBeLessThanOrEqual(entry!.clientHeight + 1);
    if (entry!.scrollWidth > entry!.clientWidth + 1) {
      expect(entry!.overflow, `${kind} horizontal overflow is controlled`).toBe("hidden");
      expect(entry!.textOverflow, `${kind} horizontal overflow uses an ellipsis`).toBe("ellipsis");
      expect(entry!.whiteSpace, `${kind} ellipsis remains on one line`).toBe("nowrap");
    }
  }
}

export async function expectFlatTokenBorders(page: Page, controls: readonly Locator[]): Promise<void> {
  const snapshots = [];
  for (const control of controls) {
    snapshots.push(await control.evaluate((element) => {
      const style = getComputedStyle(element);
      const shell = element.closest<HTMLElement>(".app-shell");
      if (!shell) throw new Error("Token-controlled border witness is outside the app shell.");
      const resolveToken = (name: string) => {
        const witness = document.createElement("span");
        witness.style.color = `var(${name})`;
        shell.append(witness);
        const value = getComputedStyle(witness).color;
        witness.remove();
        return value;
      };
      return {
        testId: element.getAttribute("data-testid") ?? element.getAttribute("aria-label") ?? element.tagName,
        styles: [style.borderTopStyle, style.borderRightStyle, style.borderBottomStyle, style.borderLeftStyle],
        widths: [style.borderTopWidth, style.borderRightWidth, style.borderBottomWidth, style.borderLeftWidth],
        colors: [style.borderTopColor, style.borderRightColor, style.borderBottomColor, style.borderLeftColor],
        allowedColors: [resolveToken("--ui-border"), resolveToken("--ui-divider"), resolveToken("--ui-accent")],
      };
    }));
  }
  for (const snapshot of snapshots) {
    expect(new Set(snapshot.styles), `${snapshot.testId} uses a flat solid border`).toEqual(new Set(["solid"]));
    expect(new Set(snapshot.widths), `${snapshot.testId} avoids UA bevel widths`).toEqual(new Set(["1px"]));
    for (const color of snapshot.colors) {
      expect(snapshot.allowedColors, `${snapshot.testId} resolves a frozen border token`).toContain(color);
    }
  }
}

// The semantic marker is passive; the rendered orientation frame belongs to
// the main canvas. Painted XYZ quality still requires independent visual review.
export async function choosePaletteNodeThenSelectWithFocusEvidence(page: Page, testInfo: TestInfo, activation: "keyboard" | "pointer" = "keyboard") {
  await page.evaluate(() => {
    const events: unknown[] = [];
    const listener = (event: Event) => {
      const target = event.target as Element | null;
      if (events.length < 64) events.push({ type: event.type, trusted: event.isTrusted,
        key: event instanceof KeyboardEvent ? event.key : null,
        targetTag: target?.tagName, targetId: target?.getAttribute("data-testid") || target?.id || null,
        activeId: document.activeElement?.getAttribute("data-testid") || document.activeElement?.getAttribute("aria-label") || null });
    };
    document.addEventListener("focusin", listener, true);
    document.addEventListener("keydown", listener, true);
    (window as any).__paletteFocusEvidence = { events, dispose: () => {
      document.removeEventListener("focusin", listener, true);
      document.removeEventListener("keydown", listener, true);
    } };
  });
  try {
    for (const [commandId, destinationId] of [
      ["toolkit-build.node", "viewport-create-node-id"],
      ["toolkit-view.select", "model-tree-filter-input"],
    ]) {
      const entry = page.getByTestId("toolkit-entry");
      await expect(entry).toBeEnabled();
      await entry.focus(); await expect(entry).toBeFocused();
      if (activation === "keyboard") await page.keyboard.press("Enter");
      else await entry.click();
      const dialog = page.getByRole("dialog", { name: "Find a modeling tool" });
      await expect(dialog).toBeVisible();
      const command = page.getByTestId(commandId);
      await expect(command).toBeEnabled();
      await command.focus(); await expect(command).toBeFocused();
      if (activation === "keyboard") await page.keyboard.press("Enter");
      else await command.click();
      await expect(dialog).toHaveCount(0);
      await expect(page.getByTestId(destinationId)).toBeVisible();
      await expect(page.getByTestId(destinationId)).toBeFocused();
    }
  } finally {
    const events = await page.evaluate(() => {
      const evidence = (window as any).__paletteFocusEvidence;
      evidence.dispose(); delete (window as any).__paletteFocusEvidence;
      return evidence.events;
    });
    await testInfo.attach(`palette-node-select-${activation}-focus-events`, { body: JSON.stringify(events, null, 2), contentType: "application/json" });
  }
}

export async function expectPassiveOrientationFrame(page: Page, testInfo: TestInfo, name: string) {
  const marker = page.getByTestId("viewport-axis-triad");
  const witness = await page.getByTestId("viewport-canvas").evaluate((host) => {
    const canvases = Array.from(host.querySelectorAll("canvas"));
    const visible = canvases.filter((canvas) => {
      const box = canvas.getBoundingClientRect();
      return box.width > 0 && box.height > 0 && getComputedStyle(canvas).visibility !== "hidden";
    });
    const canvas = visible[0];
    if (!canvas) return null;
    const box = canvas.getBoundingClientRect();
    const frame = canvas.closest(".viewport-frame")!.getBoundingClientRect();
    const width = canvas.clientWidth, height = canvas.clientHeight;
    const size = Math.min(96, Math.floor(Math.min(width, height)));
    const insetX = Math.min(8, Math.max(0, width - size));
    const insetY = Math.min(8, Math.max(0, height - size));
    const clip = { x: box.left + insetX, y: box.top + box.height - insetY - size, width: size, height: size };
    const offset = 0.5 / devicePixelRatio;
    const low = offset, high = size - offset, mid = size / 2;
    const samples = [[mid, mid], [low, low], [high, low], [low, high], [high, high],
      [mid, low], [mid, high], [low, mid], [high, mid]].map(([x, y]) => {
      const point = { x: clip.x + x, y: clip.y + y };
      const hit = document.elementFromPoint(point.x, point.y);
      return { point, ownedByCanvas: hit === canvas, tag: hit?.tagName ?? null,
        testId: hit?.getAttribute("data-testid") || null, id: hit?.id || null,
        owner: hit?.getAttribute("data-testid") || hit?.getAttribute("aria-label") || hit?.id || hit?.tagName || null };
    });
    return { canvas: box.toJSON(), frame: frame.toJSON(), client: { width, height }, clip,
      size, insetX, insetY, dpr: devicePixelRatio, visibleCanvases: visible.length,
      totalMainCanvases: canvases.length, separateGizmoCanvases: document.querySelectorAll('[data-testid="viewport-axis-triad"] canvas').length,
      viewport: { width: innerWidth, height: innerHeight }, samples };
  });
  await testInfo.attach(`${name}-orientation-witness`, { body: JSON.stringify(witness, null, 2), contentType: "application/json" });
  await testInfo.attach(`${name}-orientation-context`, { body: await page.screenshot(), contentType: "image/png" });
  if (witness && witness.size > 0 && witness.clip.x >= 0 && witness.clip.y >= 0 &&
      witness.clip.x + witness.size <= witness.viewport.width && witness.clip.y + witness.size <= witness.viewport.height) {
    await testInfo.attach(`${name}-orientation-xyz-crop`, { body: await page.screenshot({ clip: witness.clip }), contentType: "image/png" });
  }
  await expect(marker).toBeVisible();
  await expect(marker).toHaveAccessibleName(/Orientation gizmo.*X.*Y.*Z/i);
  await expect(marker).toHaveCSS("pointer-events", "none");
  expect(witness).not.toBeNull();
  if (!witness) throw new Error("No visible main canvas for passive orientation witness");
  expect(witness.visibleCanvases).toBe(1);
  expect(witness.totalMainCanvases).toBe(1);
  expect(Math.abs(witness.canvas.width - witness.client.width)).toBeLessThanOrEqual(1);
  expect(Math.abs(witness.canvas.height - witness.client.height)).toBeLessThanOrEqual(1);
  expect(witness.separateGizmoCanvases).toBe(0);
  expect(witness.size).toBeGreaterThan(0);
  if (Math.min(witness.client.width, witness.client.height) >= 96) expect(witness.size).toBe(96);
  for (const bounds of [witness.canvas, witness.frame,
    { left: 0, top: 0, right: witness.viewport.width, bottom: witness.viewport.height }]) {
    expect(witness.clip.x).toBeGreaterThanOrEqual(bounds.left);
    expect(witness.clip.y).toBeGreaterThanOrEqual(bounds.top);
    expect(witness.clip.x + witness.size).toBeLessThanOrEqual(bounds.right);
    expect(witness.clip.y + witness.size).toBeLessThanOrEqual(bounds.bottom);
  }
  for (const sample of witness.samples) expect(sample.ownedByCanvas, `orientation sample owned by ${sample.owner}`).toBe(true);
  return witness;
}

export async function expectCenterUnobscured(
  locator: Locator,
  options: Readonly<{ minimumTarget?: boolean }> = {},
) {
  const witness = await locator.evaluate((element) => {
    const box = element.getBoundingClientRect();
    const center = { x: box.left + box.width / 2, y: box.top + box.height / 2 };
    const top = document.elementFromPoint(center.x, center.y);
    return {
      box: { left: box.left, top: box.top, right: box.right, bottom: box.bottom, width: box.width, height: box.height },
      center,
      centerOwner: top instanceof Element
        ? top.getAttribute("data-testid")
          ?? top.getAttribute("aria-label")
          ?? top.id
          ?? (typeof top.className === "string" && top.className.trim() ? `.${top.className.trim().replace(/\s+/g, ".")}` : top.tagName)
        : null,
      centerOwnedByTarget: top instanceof Node && (element === top || element.contains(top)),
      viewport: { width: innerWidth, height: innerHeight },
    };
  });
  expect(witness.box.left).toBeGreaterThanOrEqual(0);
  expect(witness.box.top).toBeGreaterThanOrEqual(0);
  expect(witness.box.right).toBeLessThanOrEqual(witness.viewport.width);
  expect(witness.box.bottom).toBeLessThanOrEqual(witness.viewport.height);
  expect(witness.centerOwnedByTarget, `center is covered by ${witness.centerOwner ?? "no element"}`).toBe(true);
  if (options.minimumTarget) {
    expect(witness.box.width).toBeGreaterThanOrEqual(24);
    expect(witness.box.height).toBeGreaterThanOrEqual(24);
  }
  return witness;
}

export async function expectClearOfClosedRailHandles(
  page: Page,
  areas: readonly Readonly<{ name: string; locator: Locator }>[],
): Promise<void> {
  const handles = [
    { name: "closed Model handle", locator: page.getByTestId("toggle-tree") },
    { name: "closed Properties handle", locator: page.getByTestId("toggle-inspector") },
  ];
  for (const handle of handles) {
    await expect(handle.locator).toHaveAttribute("aria-expanded", "false");
    await expectCenterUnobscured(handle.locator, { minimumTarget: true });
  }
  for (const area of areas) {
    await expect(area.locator, `${area.name} is visible`).toBeVisible();
    await expectCenterUnobscured(area.locator);
    const areaBox = await area.locator.boundingBox();
    expect(areaBox, `${area.name} has layout geometry`).not.toBeNull();
    for (const handle of handles) {
      const handleBox = await handle.locator.boundingBox();
      expect(handleBox, `${handle.name} has layout geometry`).not.toBeNull();
      const overlapWidth = Math.max(0, Math.min(areaBox!.x + areaBox!.width, handleBox!.x + handleBox!.width) - Math.max(areaBox!.x, handleBox!.x));
      const overlapHeight = Math.max(0, Math.min(areaBox!.y + areaBox!.height, handleBox!.y + handleBox!.height) - Math.max(areaBox!.y, handleBox!.y));
      expect(overlapWidth * overlapHeight, `${handle.name} does not overlap ${area.name}`).toBe(0);
    }
  }
}

export async function expectContentFits(locator: Locator, name: string): Promise<void> {
  const metrics = await locator.evaluate((element) => ({
    clientHeight: element.clientHeight,
    clientWidth: element.clientWidth,
    scrollHeight: element.scrollHeight,
    scrollWidth: element.scrollWidth,
  }));
  expect(metrics.clientWidth, `${name} has rendered width`).toBeGreaterThan(0);
  expect(metrics.clientHeight, `${name} has rendered height`).toBeGreaterThan(0);
  expect(metrics.scrollWidth, `${name} is horizontally complete`).toBeLessThanOrEqual(metrics.clientWidth + 1);
  expect(metrics.scrollHeight, `${name} is vertically complete`).toBeLessThanOrEqual(metrics.clientHeight + 1);
}

export async function captureState(page: Page, testInfo: TestInfo, name: string): Promise<void> {
  const fileName = `${name.replace(/[^a-z0-9-]+/gi, "-").toLowerCase()}.png`;
  const outputPath = testInfo.outputPath(fileName);
  await page.screenshot({ path: outputPath, animations: "disabled" });
  await testInfo.attach(name, { path: outputPath, contentType: "image/png" });
}

export async function capturePageClip(
  page: Page,
  testInfo: TestInfo,
  name: string,
  clip: { x: number; y: number; width: number; height: number },
): Promise<string> {
  const fileName = `${name.replace(/[^a-z0-9-]+/gi, "-").toLowerCase()}.png`;
  const outputPath = testInfo.outputPath(fileName);
  await page.screenshot({ path: outputPath, animations: "disabled", clip });
  await testInfo.attach(name, { path: outputPath, contentType: "image/png" });
  return outputPath;
}

export async function captureElementState(locator: Locator, testInfo: TestInfo, name: string): Promise<string> {
  const fileName = `${name.replace(/[^a-z0-9-]+/gi, "-").toLowerCase()}.png`;
  const outputPath = testInfo.outputPath(fileName);
  await locator.screenshot({ path: outputPath, animations: "disabled" });
  await testInfo.attach(name, { path: outputPath, contentType: "image/png" });
  return outputPath;
}

export async function keyboardMeasureTargets(page: Page, labels: readonly string[]): Promise<Locator> {
  const measure = page.getByRole("button", { name: "Measure", exact: true });
  await activateWithKeyboard(page, measure);
  await expect(measure).toHaveAttribute("aria-pressed", "true");
  for (const label of labels) {
    await activateWithKeyboard(page, page.getByRole("button", { name: label, exact: true }));
  }
  const readout = page.getByTestId("viewport-measurement-readout");
  await expect(readout).toContainText("Distance");
  await expect(readout).toContainText("ΔX");
  await expect(readout).toContainText("ΔY");
  await expect(readout).toContainText("ΔZ");
  return readout;
}

export function withTypedCollision(model: any, sharedId = "same/id?typed"): any {
  const copy = structuredClone(model);
  const oldNode = copy.nodes[0].id;
  const oldPipe = copy.pipe_segments[0].id;
  copy.nodes[0].id = sharedId;
  copy.pipe_segments[0].id = sharedId;
  for (const pipe of copy.pipe_segments) {
    if (pipe.from === oldNode) pipe.from = sharedId;
    if (pipe.to === oldNode) pipe.to = sharedId;
  }
  for (const support of copy.supports ?? []) if (support.node === oldNode) support.node = sharedId;
  for (const component of copy.components ?? []) {
    if (component.node === oldNode) component.node = sharedId;
    if (component.geometry?.rigid_pipe_ref === oldPipe) component.geometry.rigid_pipe_ref = sharedId;
  }
  return copy;
}

export function withOneInvalidOd(model: any): any {
  const copy = structuredClone(model);
  copy.pipe_segments[0].section.wall_thickness.value = 0;
  return copy;
}
