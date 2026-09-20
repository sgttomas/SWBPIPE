# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: b3-accessibility.spec.ts >> disabled reasons own only visible unconsumed Escape and reset on reentry
- Location: e2e/b3-accessibility.spec.ts:328:1

# Error details

```
Error: expect(locator).toBeFocused() failed

Locator:  getByTestId('agent-strip-open')
Expected: focused
Received: inactive
Timeout:  10000ms

Call log:
  - Expect "toBeFocused" with timeout 10000ms
  - waiting for getByTestId('agent-strip-open')
    24 × locator resolved to <button type="button" aria-label="Agent" aria-disabled="true" class="shell-rail-item" data-testid="agent-strip-open" title="Agent: not available yet" aria-describedby="agent-strip-reason">…</button>
       - unexpected value "inactive"

```

```yaml
- button "Agent" [disabled]
```

# Test source

```ts
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
  255 |     // Browser preview has entered-unit metadata only; alternate unit catalog
  256 |     // choices are an actual Tauri check, not injected into this browser scenario.
  257 |     await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  258 |     await expect(page.getByTestId("workspace-redo")).toBeDisabled();
  259 |     await witness(info, `compact-${layout}-ax`, await nativeAXNames(page));
  260 | 
  261 |     // A retained control's open popup is dismissed on stage change, without remounting its draft.
  262 |     await unit.click();
  263 |     await page.getByTestId("rail-stage-loads").click();
  264 |     await expect(list).toHaveCount(0);
  265 |     await expect(page.getByTestId("rail-stage-loads")).toBeFocused();
  266 |     await page.getByTestId("rail-stage-model").click();
  267 |     await expect(draft).toHaveValue("Retained popup draft");
  268 |     expect(await draft.evaluate((el, old) => el === old, retained)).toBe(true);
  269 | 
  270 |     // Exercise a changed value using options actually available in browser mode.
  271 |     await page.getByTestId("workspace-select").click();
  272 |     await selectTreeEntity(page, "node", "node:N-100");
  273 |     await startPropertyTaskFromCurrentSelection(page, "node", "node:N-100");
  274 |     const field = page.getByTestId("editor-intent-field");
  275 |     await selectCompactOption(field, "position.x");
  276 |     await expect(field).toHaveAttribute("data-value", "position.x");
  277 |     await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  278 |     await expect(page.getByTestId("workspace-redo")).toBeDisabled();
  279 |   });
  280 | }
  281 | 
  282 | test("support family popup dismissal does not invent a missing engineering choice", async ({ page }) => {
  283 |   await page.goto("/");
  284 |   await selectTreeEntity(page, "node", "node:N-100");
  285 |   await page.getByTestId("toolkit-entry").click();
  286 |   await page.getByTestId("toolkit-supports.restraint").click();
  287 |   const form = page.getByRole("region", { name: "Support configuration", exact: true });
  288 |   const family = form.getByRole("combobox", { name: "Support family", exact: true });
  289 |   await expect(family).toHaveAttribute("data-value", "");
  290 |   await expect(family).toContainText("Not provided (preserved)");
  291 |   await family.click();
  292 |   await expect(family).toHaveAttribute("aria-expanded", "true");
  293 |   await page.keyboard.press("Tab");
  294 |   await expect(family).toHaveAttribute("aria-expanded", "false");
  295 |   await expect(family).toHaveAttribute("data-value", "");
  296 |   await form.getByRole("textbox", { name: "Support ID", exact: true }).fill("support:compact-preserved");
  297 |   await form.getByRole("textbox", { name: "Support label", exact: true }).fill("Explicit source preservation test");
  298 |   await form.getByRole("textbox", { name: "Support provenance", exact: true }).fill("Explicit browser test draft");
  299 |   await family.click();
  300 |   await form.getByRole("button", { name: "Queue support creation", exact: true }).click();
  301 |   await expect(form).toContainText("Support creation queued for validation and review.");
  302 |   await expect(family).toHaveAttribute("data-value", "");
  303 |   await expect(family).toContainText("Not provided (preserved)");
  304 |   await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  305 | });
  306 | 
  307 | 
  308 | test("Agent reason preserves hover and lets focused page Close receive the pointer", async ({ page }, info) => {
  309 |   await page.goto("/");
  310 |   await openWorkspaceSection(page, "libraries");
  311 |   const agent = page.getByTestId("agent-strip-open");
  312 |   const reason = page.locator("#agent-strip-reason");
  313 |   const close = page.getByTestId("workspace-dock-close");
  314 |   await agent.focus();
  315 |   await expect(reason).toBeVisible();
  316 |   await reason.hover();
  317 |   await expect(reason).toBeVisible();
  318 |   await witness(info, "agent-reason-close-hit", await close.evaluate(el => {
  319 |     const r = el.getBoundingClientRect();
  320 |     const hit = document.elementFromPoint(r.left + r.width / 2, r.top + r.height / 2);
  321 |     return { close: r.toJSON(), hit: hit?.outerHTML, ownsPointer: el.contains(hit), focused: document.activeElement?.outerHTML };
  322 |   }));
  323 |   await close.click();
  324 |   await expect(close).toHaveCount(0);
  325 | });
  326 | 
  327 | 
  328 | test("disabled reasons own only visible unconsumed Escape and reset on reentry", async ({ page }) => {
  329 |   await page.goto("/");
  330 |   await openWorkspaceSection(page, "libraries");
  331 |   const agent = page.getByTestId("agent-strip-open");
  332 |   const reason = page.locator("#agent-strip-reason");
  333 |   const close = page.getByTestId("workspace-dock-close");
  334 |   await agent.focus();
  335 |   await expect(reason).toBeVisible();
  336 |   // A child has already consumed this Escape; the reason and page must remain.
  337 |   await agent.evaluate(el => el.addEventListener("keydown", event => event.preventDefault(), { once: true }));
  338 |   await page.keyboard.press("Escape");
  339 |   await expect(reason).toBeVisible();
  340 |   await expect(close).toBeVisible();
  341 |   await page.keyboard.press("Escape");
  342 |   await expect(reason).toBeHidden();
> 343 |   await expect(agent).toBeFocused();
      |                       ^ Error: expect(locator).toBeFocused() failed
  344 |   await expect(close).toBeVisible();
  345 |   await close.focus();
  346 |   await agent.focus();
  347 |   await expect(reason).toBeVisible();
  348 | 
  349 |   // Hover alone persists across the visible gap and over the tooltip itself.
  350 |   await close.focus();
  351 |   await agent.hover();
  352 |   const anchorBox = await agent.boundingBox();
  353 |   const reasonBox = await reason.boundingBox();
  354 |   expect(anchorBox).not.toBeNull();
  355 |   expect(reasonBox).not.toBeNull();
  356 |   await page.mouse.move(anchorBox!.x + anchorBox!.width / 2, reasonBox!.y + reasonBox!.height / 2, { steps: 12 });
  357 |   await expect(reason).toBeVisible();
  358 |   await reason.hover();
  359 |   await expect(reason).toBeVisible();
  360 |   const placement = await reason.evaluate(el => {
  361 |     const r = el.getBoundingClientRect();
  362 |     return { contained: r.left >= 0 && r.top >= 0 && r.right <= innerWidth && r.bottom <= innerHeight,
  363 |       topmost: el.contains(document.elementFromPoint(r.left + r.width / 2, r.top + r.height / 2)) };
  364 |   });
  365 |   expect(placement).toEqual({ contained: true, topmost: true });
  366 |   await page.keyboard.press("Escape");
  367 |   await expect(reason).toBeHidden();
  368 |   await expect(close).toBeVisible();
  369 |   await close.hover();
  370 |   await agent.hover();
  371 |   await expect(reason).toBeVisible();
  372 |   await close.hover();
  373 |   await expect(reason).toBeHidden();
  374 |   // No visible reason may consume the page's normal Escape command.
  375 |   await page.keyboard.press("Escape");
  376 |   await expect(close).toHaveCount(0);
  377 | });
  378 | 
  379 | test("simultaneous hovered and focused reasons dismiss before the page", async ({ page }) => {
  380 |   await page.goto("/");
  381 |   await openWorkspaceSection(page, "libraries");
  382 |   const rail = page.getByTestId("rail-stage-results");
  383 |   const railReason = page.locator(`#${await rail.getAttribute("aria-describedby")}`);
  384 |   const agent = page.getByTestId("agent-strip-open");
  385 |   const reason = page.locator("#agent-strip-reason");
  386 |   const close = page.getByTestId("workspace-dock-close");
  387 |   await rail.hover();
  388 |   await agent.focus();
  389 |   await expect(railReason).toBeVisible();
  390 |   await expect(reason).toBeVisible();
  391 |   await page.keyboard.press("Escape");
  392 |   await expect(railReason).toBeHidden();
  393 |   await expect(reason).toBeHidden();
  394 |   await expect(close).toBeVisible();
  395 |   await expect(agent).toBeFocused();
  396 |   await page.keyboard.press("Escape");
  397 |   await expect(close).toHaveCount(0);
  398 | });
  399 | 
```