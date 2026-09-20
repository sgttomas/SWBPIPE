# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: b3-accessibility.spec.ts >> compact routing selector owns popup Escape before narrow shell consumers
- Location: e2e/b3-accessibility.spec.ts:202:3

# Error details

```
Error: locator.click: Test ended.
Call log:
  - waiting for getByRole('listbox', { name: 'New node coordinate unit' }).locator('[role="option"][data-value="mm"]')

```

# Test source

```ts
  156 |       await page.keyboard.press("Enter");
  157 |     } else {
  158 |       // The menu command is removed when activated, so Close needs a safe fallback.
  159 |       await page.getByTestId("menu-view").click();
  160 |       await page.getByTestId(`menu-item-view.section.${section}`).focus();
  161 |       await page.keyboard.press("Enter");
  162 |     }
  163 |     await expect(page.getByTestId(`workspace-section-${section}`)).toBeVisible();
  164 |     await tabTo(page, "workspace-dock-close");
  165 |     await page.keyboard.press("Enter");
  166 |     await expect(page.getByTestId("workspace-dock-close")).toHaveCount(0);
  167 |     // No focus call after Close: the application must establish this destination.
  168 |     const focus = await page.evaluate(() => {
  169 |       const el = document.activeElement as HTMLElement | null;
  170 |       if (!el) return null;
  171 |       const r = el.getBoundingClientRect(), style = getComputedStyle(el);
  172 |       return {
  173 |         tag: el.tagName, testId: el.dataset.testid, text: el.textContent?.trim().slice(0, 200),
  174 |         inert: Boolean(el.closest("[inert]")), rect: r.toJSON(),
  175 |         visible: style.visibility === "visible" && style.display !== "none" && r.width > 0 && r.height > 0,
  176 |         unobscured: el.contains(document.elementFromPoint(r.left + r.width / 2, r.top + r.height / 2))
  177 |       };
  178 |     });
  179 |     const session = await page.context().newCDPSession(page);
  180 |     let focusedAX;
  181 |     try {
  182 |       const { result } = await session.send("Runtime.evaluate", { expression: "document.activeElement" });
  183 |       const { nodes } = await session.send("Accessibility.getPartialAXTree", { objectId: result.objectId!, fetchRelatives: false });
  184 |       focusedAX = nodes.map(node => ({ ignored: node.ignored, role: node.role?.value, name: node.name?.value }));
  185 |     } finally { await session.detach(); }
  186 |     await witness(info, `${section}-keyboard-close-focus`, { focus, focusedAX });
  187 |     expect.soft(focus?.tag).not.toBe("BODY");
  188 |     expect.soft(focus?.visible).toBe(true);
  189 |     expect.soft(focus?.inert).toBe(false);
  190 |     expect.soft(focus?.unobscured).toBe(true);
  191 |     expect.soft(focusedAX.some(node => !node.ignored && node.role === "button")).toBe(true);
  192 |     if (section === "libraries") await expect.soft(page.getByTestId("rail-page-libraries")).toBeFocused();
  193 |     expect(await canvas!.evaluate(el => el === document.querySelector("canvas"))).toBe(true);
  194 |     expect(await inspector!.evaluate(el => el === document.querySelector('[data-testid="property-inspector"]'))).toBe(true);
  195 |     expect(await draft!.evaluate(el => el === document.querySelector('[data-testid="editor-intent-value"]'))).toBe(true);
  196 |     await expect(page.getByTestId("editor-intent-value")).toHaveValue("Retained keyboard Close draft");
  197 |     expect(await nativeAXNames(page)).toContain("region: Modeling workspace");
  198 |   });
  199 | }
  200 | 
  201 | for (const layout of ["configured", "narrow"] as const) {
  202 |   test(`compact routing selector owns popup Escape before ${layout} shell consumers`, async ({ page }, info) => {
  203 |     if (layout === "narrow") await page.setViewportSize({ width: 1200, height: 800 });
  204 |     await page.goto("/");
  205 |     await page.getByTestId("command-node").click();
  206 |     const unit = page.getByTestId("viewport-create-node-unit");
  207 |     const draft = page.getByTestId("viewport-create-node-label");
  208 |     await draft.fill("Retained popup draft");
  209 |     const retained = await draft.elementHandle();
  210 |     await unit.focus();
  211 |     await page.keyboard.press("Space");
  212 |     await expect(unit).toHaveAttribute("aria-expanded", "true");
  213 |     const list = page.getByRole("listbox", { name: "New node coordinate unit" });
  214 |     await expect(list).toBeVisible();
  215 |     const placement = await list.evaluate(el => {
  216 |       const rect = el.getBoundingClientRect();
  217 |       const center = document.elementFromPoint((rect.left + rect.right) / 2, (rect.top + rect.bottom) / 2);
  218 |       return { rect: rect.toJSON(), width: innerWidth, height: innerHeight, portalled: el.parentElement === document.body,
  219 |         topmost: el.contains(center) };
  220 |     });
  221 |     await witness(info, `compact-${layout}-popup-placement`, placement);
  222 |     expect(placement.portalled).toBe(true);
  223 |     expect(placement.topmost).toBe(true);
  224 |     expect(placement.rect.left).toBeGreaterThanOrEqual(0);
  225 |     expect(placement.rect.right).toBeLessThanOrEqual(placement.width);
  226 |     expect(placement.rect.top).toBeGreaterThanOrEqual(0);
  227 |     expect(placement.rect.bottom).toBeLessThanOrEqual(placement.height);
  228 |     await page.keyboard.press("ArrowDown");
  229 |     await expect(unit).toHaveAttribute("data-value", "m");
  230 |     await page.keyboard.press("Escape");
  231 |     await expect(list).toHaveCount(0);
  232 |     await expect(unit).toBeFocused();
  233 |     await expect(unit).toHaveAttribute("data-value", "m");
  234 |     await expect(page.getByTestId("toggle-inspector")).toHaveAttribute("aria-expanded", "true");
  235 |     await page.keyboard.press("Escape");
  236 |     await expect(page.getByTestId("toggle-inspector")).toHaveAttribute("aria-expanded", "false");
  237 |     await expect(page.getByTestId("toggle-inspector")).toBeFocused();
  238 |     await page.getByTestId("toggle-inspector").click();
  239 |     await expect(draft).toHaveValue("Retained popup draft");
  240 |     expect(await draft.evaluate((el, old) => el === old, retained)).toBe(true);
  241 | 
  242 |     // Silent same-value commits use the same explicit lifecycle as changed commits.
  243 |     await unit.click();
  244 |     await list.locator('[role="option"][data-value="m"]').click();
  245 |     await expect(unit).toHaveAttribute("aria-expanded", "false");
  246 |     await expect(unit).toBeFocused();
  247 |     await page.keyboard.press("Escape");
  248 |     await expect(page.getByTestId("toggle-inspector")).toHaveAttribute("aria-expanded", "false");
  249 |     await page.getByTestId("toggle-inspector").click();
  250 |     await unit.focus();
  251 |     await page.keyboard.press("Space");
  252 |     await page.keyboard.press("Enter");
  253 |     await expect(unit).toHaveAttribute("aria-expanded", "false");
  254 |     await expect(unit).toHaveAttribute("data-value", "m");
  255 |     await unit.click();
> 256 |     await list.locator('[role="option"][data-value="mm"]').click();
      |                                                            ^ Error: locator.click: Test ended.
  257 |     await expect(unit).toHaveAttribute("data-value", "mm");
  258 |     await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  259 |     await expect(page.getByTestId("workspace-redo")).toBeDisabled();
  260 |     await witness(info, `compact-${layout}-ax`, await nativeAXNames(page));
  261 | 
  262 |     // A retained control's open popup is dismissed on stage change, without remounting its draft.
  263 |     await unit.click();
  264 |     await page.getByTestId("rail-stage-loads").click();
  265 |     await expect(list).toHaveCount(0);
  266 |     await expect(page.getByTestId("rail-stage-loads")).toBeFocused();
  267 |     await page.getByTestId("rail-stage-model").click();
  268 |     await expect(draft).toHaveValue("Retained popup draft");
  269 |     expect(await draft.evaluate((el, old) => el === old, retained)).toBe(true);
  270 |   });
  271 | }
  272 | 
```