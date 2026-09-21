# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: b4-sections.spec.ts >> B4 Sections enum explicit completion, passive cancellation and raw review retention
- Location: e2e/b4-sections.spec.ts:36:1

# Error details

```
Error: light opaque popup surface

expect(received).toBe(expected) // Object.is equality

Expected: "rgb(255, 255, 255)"
Received: "rgba(0, 0, 0, 0)"
```

# Page snapshot

```yaml
- main [ref=e3]:
  - navigation "Application menu" [ref=e4]:
    - button "File" [ref=e6] [cursor=pointer]
    - button "Edit" [ref=e8] [cursor=pointer]
    - button "View" [ref=e10] [cursor=pointer]
    - button "Insert" [ref=e12] [cursor=pointer]
    - button "Analyze" [ref=e14] [cursor=pointer]
  - generic "Toolbar" [ref=e15]:
    - generic [ref=e16]:
      - heading "SWBPIPE" [level=1] [ref=e17]
      - paragraph [ref=e18]: Generated UI local-render-origin precision probe
    - group "Editing tools" [ref=e19]:
      - button "Undo model edit" [disabled] [ref=e20]:
        - img [ref=e21]
      - button "Redo model edit" [disabled] [ref=e24]:
        - img [ref=e25]
      - button "Select" [pressed] [ref=e28] [cursor=pointer]:
        - img [ref=e29]
        - generic [ref=e31]: Select
    - group "View" [ref=e32]:
      - button "Table" [pressed] [ref=e34] [cursor=pointer]:
        - img [ref=e35]
        - generic [ref=e37]: Table
      - button "Model" [ref=e39] [cursor=pointer]:
        - img [ref=e40]
        - generic [ref=e43]: Model
      - button "Both" [ref=e45] [cursor=pointer]:
        - img [ref=e46]
        - generic [ref=e48]: Both
    - generic [ref=e49]:
      - button "Run" [ref=e50] [cursor=pointer]:
        - img [ref=e51]
        - generic [ref=e53]: Run
      - button "Issues, 2" [ref=e54] [cursor=pointer]:
        - img [ref=e55]
        - generic [ref=e57]: Issues
        - generic [ref=e58]: "2"
    - group "Panels" [ref=e59]:
      - button "Inspector" [disabled] [ref=e61]:
        - img [ref=e62]
        - generic [ref=e64]: Inspector
      - button "Agent" [disabled] [ref=e66]:
        - img [ref=e67]
        - generic [ref=e70]: Agent
    - generic "Display units" [ref=e71]:
      - combobox "Display units" [ref=e72]:
        - option "Entered" [selected]
        - option "SI"
        - option "US"
    - group [ref=e73]:
      - generic "Appearance" [active] [ref=e74] [cursor=pointer]:
        - img [ref=e75]
      - option "System"
      - option "Light" [selected]
      - option "Dark"
      - option "Comfortable" [selected]
      - option "Compact"
    - region "Human toolkit" [ref=e77]:
      - button "Find modeling commands" [ref=e78] [cursor=pointer]:
        - img [ref=e79]
        - generic [ref=e82]: Search or command…
        - generic "Command K" [ref=e83]: ⌘K
  - generic [ref=e84]:
    - navigation "Stages" [ref=e85]:
      - list [ref=e86]:
        - listitem [ref=e87]:
          - button "Model" [pressed] [ref=e88] [cursor=pointer]:
            - img [ref=e89]
            - generic [ref=e92]: Model
        - listitem [ref=e93]:
          - button "Loads" [ref=e94] [cursor=pointer]:
            - img [ref=e95]
            - generic [ref=e99]: Loads
        - listitem [ref=e100]:
          - button "Results" [disabled] [ref=e101]:
            - img [ref=e102]
            - generic [ref=e105]: Results
        - listitem [ref=e106]:
          - button "Review" [disabled] [ref=e107]:
            - img [ref=e108]
            - generic [ref=e112]: Review
      - separator [ref=e113]
      - list [ref=e114]:
        - listitem [ref=e115]:
          - button "Libraries" [ref=e116] [cursor=pointer]:
            - img [ref=e117]
            - generic [ref=e119]: Libraries
        - listitem [ref=e120]:
          - button "Rules" [ref=e121] [cursor=pointer]:
            - img [ref=e122]
            - generic [ref=e126]: Rules
        - listitem [ref=e127]:
          - button "Issues, 2" [ref=e128] [cursor=pointer]:
            - img [ref=e129]
            - generic [ref=e131]: Issues
            - generic [ref=e132]: "2"
    - region "Modeling workspace" [ref=e134]:
      - generic [ref=e135]:
        - group "Tables" [ref=e136]:
          - button "Model" [pressed] [ref=e137] [cursor=pointer]
          - button "Review changes" [ref=e138] [cursor=pointer]
          - button "Collapse table drawer" [disabled] [expanded] [ref=e141]:
            - img [ref=e142]
        - generic "Model tree" [ref=e146]:
          - generic [ref=e147]: Model
          - region "Layout grid mode" [ref=e148]:
            - button "Tree" [ref=e149]:
              - img [ref=e150]
              - text: Tree
            - button "Grid" [pressed] [ref=e153]:
              - img [ref=e154]
              - text: Grid
          - region "Model tree filtering" [ref=e156]:
            - generic [ref=e157]:
              - img [ref=e158]
              - generic [ref=e161]: Filter model
              - searchbox "Filter model tree" [ref=e162]
            - generic [ref=e163]: 46 of 46 model entities visible
            - button "Clear model tree filter" [disabled] [ref=e164]:
              - img [ref=e165]
          - region "Bulk entity grid" [ref=e169]:
            - generic "Grid entity type" [ref=e170]:
              - button "Nodes" [ref=e171]
              - button "Pipes" [ref=e172]
              - button "Supports" [ref=e173]
              - button "Materials" [ref=e174]
              - button "Sections" [pressed] [ref=e175]
              - button "Components" [ref=e176]
              - button "Load Cases" [ref=e177]
              - button "Combinations" [ref=e178]
            - generic [ref=e180]:
              - grid "Section fields" [ref=e181]:
                - row "Section Sort Name Sort Type Sort Outside dia. Sort Wall Sort Provenance" [ref=e182]:
                  - columnheader "Section" [ref=e183]
                  - columnheader "Sort Name" [ref=e184]:
                    - button "Sort Name" [ref=e185]: Name ↕
                  - columnheader "Sort Type" [ref=e186]:
                    - button "Sort Type" [ref=e187]: Type ↕
                  - columnheader "Sort Outside dia." [ref=e188]:
                    - button "Sort Outside dia." [ref=e189]: Outside dia. [per-row entered unit] ↕
                  - columnheader "Sort Wall" [ref=e190]:
                    - button "Sort Wall" [ref=e191]: Wall [per-row entered unit] ↕
                  - columnheader "Sort Provenance" [ref=e192]:
                    - button "Sort Provenance" [ref=e193]: Provenance ↕
                - rowgroup [ref=e195]:
                  - generic [ref=e196]:
                    - 'row "section:B4-0 section:B4-0 Name: Invented 0 section:B4-0 Type: p section:B4-0 Outside dia.: 2 m m Quantity readout section:B4-0 Wall: 10 mm mm Quantity readout section:B4-0 Provenance: invented Sections browser fixture" [selected] [ref=e198]':
                      - rowheader "section:B4-0" [ref=e199]:
                        - button "section:B4-0" [ref=e200]
                      - 'gridcell "section:B4-0 Name: Invented 0" [ref=e201]':
                        - 'button "section:B4-0 Name: Invented 0" [ref=e202]': Invented 0
                      - 'gridcell "section:B4-0 Type: p" [selected] [ref=e203]':
                        - 'button "section:B4-0 Type: p" [ref=e204]': p
                      - 'gridcell "section:B4-0 Outside dia.: 2 m m Quantity readout" [ref=e205]':
                        - 'button "section:B4-0 Outside dia.: 2 m" [ref=e206]': "2"
                        - generic "m" [ref=e207]
                        - generic "Quantity readout" [ref=e208]:
                          - generic [ref=e209]: 2 m
                      - 'gridcell "section:B4-0 Wall: 10 mm mm Quantity readout" [ref=e210]':
                        - 'button "section:B4-0 Wall: 10 mm" [ref=e211]': "10"
                        - generic "mm" [ref=e212]
                        - generic "Quantity readout" [ref=e213]:
                          - generic [ref=e214]: 10 mm
                      - 'gridcell "section:B4-0 Provenance: invented Sections browser fixture" [ref=e215]':
                        - 'button "section:B4-0 Provenance: invented Sections browser fixture" [ref=e216]': invented Sections browser fixture
                    - 'row "section:B4-1 section:B4-1 Name: Invented 1 section:B4-1 Type: pipe section:B4-1 Outside dia.: 100 mm mm Quantity readout section:B4-1 Wall: 10 mm mm Quantity readout section:B4-1 Provenance: invented Sections browser fixture" [ref=e218]':
                      - rowheader "section:B4-1" [ref=e219]:
                        - button "section:B4-1" [ref=e220]
                      - 'gridcell "section:B4-1 Name: Invented 1" [ref=e221]':
                        - 'button "section:B4-1 Name: Invented 1" [ref=e222]': Invented 1
                      - 'gridcell "section:B4-1 Type: pipe" [ref=e223]':
                        - 'button "section:B4-1 Type: pipe" [ref=e224]': pipe
                      - 'gridcell "section:B4-1 Outside dia.: 100 mm mm Quantity readout" [ref=e225]':
                        - 'button "section:B4-1 Outside dia.: 100 mm" [ref=e226]': "100"
                        - generic "mm" [ref=e227]
                        - generic "Quantity readout" [ref=e228]:
                          - generic [ref=e229]: 100 mm
                      - 'gridcell "section:B4-1 Wall: 10 mm mm Quantity readout" [ref=e230]':
                        - 'button "section:B4-1 Wall: 10 mm" [ref=e231]': "10"
                        - generic "mm" [ref=e232]
                        - generic "Quantity readout" [ref=e233]:
                          - generic [ref=e234]: 10 mm
                      - 'gridcell "section:B4-1 Provenance: invented Sections browser fixture" [ref=e235]':
                        - 'button "section:B4-1 Provenance: invented Sections browser fixture" [ref=e236]': invented Sections browser fixture
              - group "Section fields footer" [ref=e237]:
                - generic [ref=e238]: 2 of 2 rows
            - button "Review multiple changes" [ref=e239] [cursor=pointer]
      - option "All" [selected]
      - option "Pipes"
      - option "Nodes"
      - option "Supports"
      - option "Components"
      - text: ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾ ▾
    - complementary "Agent" [ref=e240]:
      - button "Agent" [disabled] [ref=e242]:
        - img [ref=e243]
        - generic [ref=e246]: Agent
  - generic "Workspace status" [ref=e247]:
    - generic "Analysis statuses" [ref=e248]:
      - button "Solver · Not solved" [ref=e250] [cursor=pointer]
    - button "2 Issues" [ref=e251] [cursor=pointer]:
      - img [ref=e252]
      - text: 2 Issues
    - generic "Selection" [ref=e254]: "section: section:B4-0"
    - generic "Display units" [ref=e255]: Entered
    - button "About SWBPIPE…" [ref=e256] [cursor=pointer]:
      - img [ref=e257]
```

# Test source

```ts
  1   | import { expect, test, type Page } from "@playwright/test";
  2   | import { attachBrowserIdentity, currentModelHashThroughVisibleExport, gotoModel, readFixture, setAppearance } from "./ui-foundation-workflows";
  3   | import { ensureTreeExpanded } from "./workspace-driver";
  4   | 
  5   | async function inventedSections(count = 2) {
  6   |   const { model } = await readFixture("precision-origin-base.model.json");
  7   |   model.sections = Array.from({ length: count }, (_, index) => ({ id: `section:B4-${index}`, name: `Invented ${index}`, section_type: "pipe",
  8   |     properties: { outside_diameter: { value: index === 0 ? 2 : index === 1 ? 100 : 200 + index, unit: index === 0 ? "m" : "mm" }, wall_thickness: { value: 10, unit: "mm" } }, provenance: "invented Sections browser fixture" }));
  9   |   return model;
  10  | }
  11  | async function openSections(page: Page, model: any) {
  12  |   await gotoModel(page, model); await page.getByTestId("view-switch-table").click(); await ensureTreeExpanded(page);
  13  |   await page.getByTestId("layout-mode-grid").click(); await page.getByTestId("entity-grid-type-sections").click();
  14  |   await expect(page.getByTestId("table-cell-section:B4-0-outside").locator("..")).toHaveAttribute("aria-readonly", "false");
  15  | }
  16  | 
  17  | test("B4 Sections mixed-unit direct editing preserves no-op, rejection, shared-pipe rejection and history", async ({ page, browser }, info) => {
  18  |   await attachBrowserIdentity(browser, info); const model = await inventedSections();
  19  |   // Bind one existing invented pipe with an exact shared cache; retain its other fields.
  20  |   const pipe = model.pipe_segments[0]; pipe.section_ref = model.sections[0].id;
  21  |   pipe.section.outside_diameter = { value: 2, unit: "m" }; pipe.section.wall_thickness = { value: 10, unit: "mm" };
  22  |   pipe.section.mill_tolerance = { value: 4, unit: "mm" };
  23  |   await openSections(page, model); const table = page.getByTestId("section-engineering-table");
  24  |   await table.getByRole("button", { name: "Sort Outside dia.", exact: true }).click(); await expect(table.getByRole("rowheader").first()).toHaveText("section:B4-1");
  25  |   const od = table.getByTestId("table-cell-section:B4-0-outside"), wall = table.getByTestId("table-cell-section:B4-0-wall");
  26  |   await od.dblclick(); const input = table.getByRole("textbox"); await input.fill("2.00"); await table.getByRole("button", { name: "Apply", exact: true }).click();
  27  |   await expect(input).toHaveCount(0); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  28  |   await wall.dblclick(); await expect(input).toHaveAccessibleName("section:B4-0 Wall [mm]"); await input.fill("3"); await table.getByRole("button", { name: "Apply", exact: true }).click();
  29  |   await expect(input).toHaveValue("10"); await expect(table.getByRole("alert")).toContainText("Engine rejected"); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  30  |   await table.getByRole("button", { name: "Cancel", exact: true }).click();
  31  |   await wall.dblclick(); await input.fill("12"); await table.getByRole("button", { name: "Apply", exact: true }).click(); await expect(wall).toHaveText("12");
  32  |   await page.getByTestId("workspace-undo").click(); await expect(wall).toHaveText("10"); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  33  |   await page.getByTestId("workspace-redo").click(); await expect(wall).toHaveText("12");
  34  | });
  35  | 
  36  | test("B4 Sections enum explicit completion, passive cancellation and raw review retention", async ({ page, browser }, info) => {
  37  |   await attachBrowserIdentity(browser, info); const model = await inventedSections(); model.sections[0].section_type = "p";
  38  |   await openSections(page, model); const table = page.getByTestId("section-engineering-table"); const type = table.getByTestId("table-cell-section:B4-0-type");
  39  |   const portalStyles = [];
  40  |   for (const theme of ["light", "dark"] as const) {
  41  |     await setAppearance(page, theme, "comfortable");
  42  |     await type.dblclick(); const themedInput = table.getByRole("combobox"); await themedInput.click(); await themedInput.press("ArrowDown");
  43  |     const popup = page.getByRole("listbox", { name: "Supported values" }); const selected = popup.getByRole("option", { name: "pipe", exact: true });
  44  |     await expect(popup).toBeVisible(); await expect(selected).toBeVisible(); await expect(selected).toHaveAttribute("aria-selected", "true");
  45  |     const observed = await popup.evaluate((element) => {
  46  |       const style = getComputedStyle(element), option = element.querySelector('[role="option"]')!, optionStyle = getComputedStyle(option);
  47  |       const tokens = getComputedStyle(document.documentElement);
  48  |       const rgb = (token: string) => {
  49  |         const hex = tokens.getPropertyValue(token).trim();
  50  |         if (!/^#[0-9a-f]{6}$/i.test(hex)) throw new Error(`Expected an opaque root color token for ${token}: ${hex}`);
  51  |         return `rgb(${[1, 3, 5].map((offset) => parseInt(hex.slice(offset, offset + 2), 16)).join(", ")})`;
  52  |       };
  53  |       const rect = option.getBoundingClientRect();
  54  |       return { background: style.backgroundColor, text: style.color, border: style.borderTopColor,
  55  |         borderStyle: style.borderTopStyle, borderWidth: style.borderTopWidth, optionBackground: optionStyle.backgroundColor, optionText: optionStyle.color,
  56  |         hit: option.contains(document.elementFromPoint(rect.x + rect.width / 2, rect.y + rect.height / 2)),
  57  |         rootSurface: rgb("--surface-panel"), rootText: rgb("--text-primary"), rootBorder: rgb("--border-hairline"), rootSelection: rgb("--selection-band") };
  58  |     });
  59  |     portalStyles.push({ theme, ...observed });
  60  |     await info.attach(`section-enum-${theme}-computed-style`, { body: JSON.stringify(observed, null, 2), contentType: "application/json" });
  61  |     await page.screenshot({ path: info.outputPath(`section-enum-${theme}-selected.png`) });
  62  |     await themedInput.press("Escape"); await expect(themedInput).toHaveValue("p"); await themedInput.press("Escape"); await expect(type).toHaveText("p");
  63  |   }
  64  |   await setAppearance(page, "light", "comfortable");
  65  |   // Inspect both actual themes before asserting, retaining both witnesses even
  66  |   // when the first comparison fails. Equality to opaque resolved tokens also
  67  |   // rejects transparent/unset portal declarations and an invisible selection.
  68  |   for (const style of portalStyles) {
> 69  |     expect(style.background, `${style.theme} opaque popup surface`).toBe(style.rootSurface);
      |                                                                     ^ Error: light opaque popup surface
  70  |     expect(style.text, `${style.theme} popup text`).toBe(style.rootText);
  71  |     expect(style.border, `${style.theme} popup border`).toBe(style.rootBorder);
  72  |     expect(style.borderStyle).toBe("solid"); expect(style.borderWidth).toBe("1px");
  73  |     expect(style.optionBackground, `${style.theme} visible selected option`).toBe(style.rootSelection);
  74  |     expect(style.optionBackground).not.toBe(style.background); expect(style.optionText).toBe(style.rootText); expect(style.hit).toBe(true);
  75  |   }
  76  |   await type.dblclick(); const input = table.getByRole("combobox"); await input.click(); await expect(page.getByRole("listbox", { name: "Supported values" })).toBeVisible();
  77  |   await page.keyboard.press("Enter"); await expect(input).toHaveValue("p"); await expect(input).toHaveAttribute("aria-invalid", "true"); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  78  |   await input.click(); await page.keyboard.press("ArrowDown"); await page.keyboard.press("Escape"); await expect(input).toHaveValue("p"); await expect(page.getByRole("listbox", { name: "Supported values" })).toHaveCount(0);
  79  |   await page.keyboard.press("Escape"); await expect(type).toHaveText("p");
  80  |   await type.dblclick(); await input.fill("pi"); const option = page.getByRole("option", { name: "pipe", exact: true }); await expect(option).toBeVisible();
  81  |   const box = await option.boundingBox(); expect(box).not.toBeNull();
  82  |   const visibleAtCenter = await option.evaluate((node) => { const r = node.getBoundingClientRect(); return node.contains(document.elementFromPoint(r.x + r.width / 2, r.y + r.height / 2)); }); expect(visibleAtCenter).toBe(true);
  83  |   await page.mouse.move(box!.x + box!.width / 2, box!.y + box!.height / 2); await page.mouse.down(); await expect(input).toHaveValue("pi");
  84  |   await page.mouse.move(box!.x + box!.width + 30, box!.y); await page.mouse.up(); await expect(input).toHaveValue("pi");
  85  |   await option.click(); await expect(input).toHaveValue("pipe"); await expect(input).toBeFocused(); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  86  |   await table.getByRole("button", { name: "Cancel", exact: true }).click(); await expect(type).toHaveText("p");
  87  |   await type.dblclick(); await input.fill("pi"); await page.keyboard.press("Tab"); await expect(type).toHaveText("pipe"); await expect(table.getByTestId("table-cell-section:B4-0-outside")).toBeFocused();
  88  |   await page.getByTestId("workspace-undo").click(); await expect(type).toHaveText("p"); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  89  |   await page.getByTestId("section-grid-review-disclosure").click(); const review = page.getByTestId("section-engineering-table-review");
  90  |   await review.getByTestId("review-cell-section:B4-0-type").dblclick(); await review.getByRole("combobox").fill("TBD"); await review.getByRole("button", { name: "Keep draft", exact: true }).click();
  91  |   await expect(page.getByTestId("queue-entity-grid-intents")).toBeDisabled(); await page.getByTestId("section-grid-review-disclosure").click(); await expect(page.getByTestId("section-grid-review-disclosure")).toContainText("1 retained draft");
  92  |   await page.getByTestId("section-grid-review-disclosure").click(); await page.getByTestId("clear-entity-grid-drafts").click(); await expect(review.getByTestId("review-cell-section:B4-0-type")).toHaveText("p");
  93  | });
  94  | 
  95  | test("B4 Sections Apply Tab accepts the next actual key during unrelated quantity reconversion", async ({ page, browser }, info) => {
  96  |   await attachBrowserIdentity(browser, info); const model = await inventedSections();
  97  |   await page.route("**/src/services/displayQuantityService.ts", async (route) => {
  98  |     const response = await route.fetch(); const source = await response.text(); expect(source).toContain("export async function convertDisplayQuantities(");
  99  |     await route.fulfill({ response, body: source.replace("export async function convertDisplayQuantities(", "async function originalConvertDisplayQuantities(") + `
  100 | export async function convertDisplayQuantities(items) {
  101 |  const result = await originalConvertDisplayQuantities(items);
  102 |  const gate = window.__b4SectionGate;
  103 |  if (gate.hold && items.some(item => item.id.includes('section') && item.id.includes('outside'))) await new Promise(resolve => gate.pending.push(resolve));
  104 |  return result;
  105 | }
  106 | ` });
  107 |   });
  108 |   await page.addInitScript(() => { (window as any).__b4SectionGate = { hold: false, pending: [] }; });
  109 |   await openSections(page, model); const table = page.getByTestId("section-engineering-table");
  110 |   const od = table.getByTestId("table-cell-section:B4-0-outside"), wall = table.getByTestId("table-cell-section:B4-0-wall");
  111 |   await table.getByRole("button", { name: "Sort Outside dia.", exact: true }).click(); await expect(table.getByRole("rowheader").first()).toHaveText("section:B4-1");
  112 |   await od.dblclick(); await table.getByRole("textbox").fill("2.1"); await page.evaluate(() => { (window as any).__b4SectionGate.hold = true; }); await page.keyboard.press("Tab");
  113 |   await expect(od).toHaveText("2.1"); await expect(wall).toBeFocused(); await expect.poll(() => page.evaluate(() => (window as any).__b4SectionGate.pending.length)).toBeGreaterThan(0);
  114 |   await expect(table.getByRole("status").filter({ hasText: "Quantity sort unavailable" })).toBeVisible(); await page.keyboard.press("8");
  115 |   const input = table.getByRole("textbox", { name: "section:B4-0 Wall [mm]" }); await expect(input).toHaveValue("8"); await expect(input).toBeFocused(); await page.keyboard.press("7"); await expect(input).toHaveValue("87");
  116 |   await table.getByRole("button", { name: "Cancel", exact: true }).click();
  117 |   await page.evaluate(() => { const gate = (window as any).__b4SectionGate; gate.hold = false; gate.pending.splice(0).forEach((resolve: () => void) => resolve()); });
  118 |   await expect(table.getByRole("rowheader").first()).toHaveText("section:B4-1"); await page.getByTestId("workspace-undo").click(); await expect(od).toHaveText("2"); await expect(wall).toHaveText("10"); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  119 | });
  120 | 
  121 | test("B4 Sections moved review row preserves text Undo, input ownership and virtual hidden lifetimes", async ({ page, browser }, info) => {
  122 |   await attachBrowserIdentity(browser, info); const model = await inventedSections(140);
  123 |   model.sections[0].properties.outside_diameter = { value: 2, unit: "mm" }; model.sections[1].properties.outside_diameter = { value: 15, unit: "mm" };
  124 |   await openSections(page, model); const hashBefore = await currentModelHashThroughVisibleExport(page);
  125 |   await page.getByTestId("section-grid-review-disclosure").click(); const table = page.getByTestId("section-engineering-table-review"); const cell = table.getByTestId("review-cell-section:B4-0-outside");
  126 |   await table.getByRole("button", { name: "Sort Outside dia.", exact: true }).click(); await cell.focus(); await page.keyboard.press("1"); await table.getByRole("button", { name: "Keep draft", exact: true }).click();
  127 |   await cell.dblclick(); const input = table.getByRole("textbox"); await page.keyboard.press("ArrowRight"); const original = await input.elementHandle();
  128 |   const movement = await table.evaluateHandle((root) => {
  129 |     const input = root.querySelector("input")!, row = root.querySelector('[data-testid="review-cell-section:B4-0-wall"]')!.closest("[data-virtual-index]")!;
  130 |     const counts = { rowRemoved: 0, inputRemoved: 0 }; const observer = new MutationObserver((records) => records.forEach((r) => {
  131 |       counts.rowRemoved += [...r.removedNodes].filter((n) => n === row).length; counts.inputRemoved += [...r.removedNodes].filter((n) => n === input || n.contains(input)).length;
  132 |     })); observer.observe(root, { childList: true, subtree: true }); return { counts, observer };
  133 |   });
  134 |   await page.keyboard.press("9"); await expect(table.getByRole("rowheader").first()).toHaveText("section:B4-1"); await expect(input).toHaveValue("19");
  135 |   expect(await input.evaluate((node, prior) => node === prior, original)).toBe(true); await expect(input).toBeFocused();
  136 |   expect(await table.locator('[aria-owns]').count()).toBe(1); expect(await table.locator('[aria-owns]').getAttribute("aria-owns")).toBe(await input.getAttribute("id"));
  137 |   await page.keyboard.press("ControlOrMeta+z"); await expect(input).toHaveValue("1"); await expect(table.getByRole("rowheader").first()).toHaveText("section:B4-0");
  138 |   const moved = await movement.evaluate((state) => { state.observer.disconnect(); return state.counts; }); expect(moved.rowRemoved).toBeGreaterThan(0); expect(moved.inputRemoved).toBe(0);
  139 |   await input.fill("invalid"); const rows = page.getByTestId("section-engineering-table-review-rows"); await rows.hover(); await page.mouse.wheel(0, 2500); await expect(input).toHaveValue("invalid");
  140 |   const filter = page.getByTestId("model-tree-filter-input"); await filter.fill("section:B4-139");
  141 |   // Natural review blur Keeps the raw draft and closes the live editor. The
  142 |   // filter owns focus; clearing it reveals the retained cell without reopening.
  143 |   await expect(input).toHaveCount(0); await expect(filter).toBeFocused(); await filter.fill(""); await expect(cell).toHaveText("invalid");
  144 |   await page.getByTestId("entity-grid-type-nodes").click(); await page.getByTestId("entity-grid-type-sections").click(); await expect(cell).toHaveText("invalid");
  145 |   await page.getByTestId("clear-entity-grid-drafts").click(); await filter.fill(""); await expect(cell).toHaveText("2"); expect(await currentModelHashThroughVisibleExport(page)).toBe(hashBefore);
  146 |   await expect(page.getByTestId("workspace-undo")).toBeDisabled(); await movement.dispose();
  147 | });
  148 | 
```