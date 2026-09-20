# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: workspace-layout.spec.ts >> the table strip clears DOM readouts and the painted orientation frame in Model and narrow Both views
- Location: e2e/workspace-layout.spec.ts:282:1

# Error details

```
Error: center is covered by 

expect(received).toBe(expected) // Object.is equality

Expected: true
Received: false
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
      - button "Table" [ref=e34] [cursor=pointer]:
        - img [ref=e35]
        - generic [ref=e37]: Table
      - button "Model" [pressed] [ref=e39] [cursor=pointer]:
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
      - button "Inspector" [disabled] [expanded] [ref=e61]:
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
      - generic "Appearance" [ref=e74] [cursor=pointer]:
        - img [ref=e75]
      - option "System" [selected]
      - option "Light"
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
      - group "Tables" [ref=e136]:
        - button "Model" [pressed] [ref=e137] [cursor=pointer]
        - button "Review changes" [ref=e138] [cursor=pointer]
        - button "Expand table drawer" [active] [ref=e141] [cursor=pointer]:
          - img [ref=e142]
      - generic [ref=e145]:
        - generic [ref=e146]:
          - group "Viewport controls" [ref=e147]:
            - generic [ref=e148]: 3D Centerline
            - group "Viewport deformation overlay status" [ref=e149]:
              - generic "Deformation · unavailable" [ref=e150] [cursor=pointer]
            - group "Viewport display toggles" [ref=e151]:
              - button "Labels" [pressed] [ref=e152]
              - button "Loads" [pressed] [ref=e153]
              - button "Grid" [pressed] [ref=e154]
            - group "Viewport selection tools" [ref=e155]:
              - button "Box Select" [ref=e156]
              - generic [ref=e157]:
                - generic [ref=e158]: Selection filter
                - combobox "Selection filter" [ref=e159]:
                  - option "All" [selected]
                  - option "Pipes"
                  - option "Nodes"
                  - option "Supports"
                  - option "Components"
              - button "Hide" [ref=e160]
              - button "Isolate" [ref=e161]
              - button "Show All" [disabled] [ref=e162]
              - button "Fit Model" [ref=e163]
              - button "Fit Visible" [ref=e164]
              - button "Fit Selection" [ref=e165]
            - group "Viewport geometry" [ref=e166]:
              - button "Schematic" [pressed] [ref=e167]
              - button "Actual OD" [ref=e168]
              - button "Measure" [pressed] [ref=e169]
          - generic "Viewport status" [ref=e170]:
            - 'generic "Selected pipe: pipe:UIF-PRECISION-00011" [ref=e171]': "Selected: pipe:UIF-PRECISION-00011"
            - status "Schematic centerline geometry" [ref=e172]
            - status "View command status" [ref=e173]: Measurement uses the authored pipe endpoints and does not change the model.
        - generic [ref=e174]:
          - generic "Three.js pipe centerline viewport" [ref=e175]
          - generic "Viewport entity selection":
            - button "Select UI benchmark pipe 00011 in viewport" [pressed] [ref=e177] [cursor=pointer]:
              - img [ref=e178]
              - generic [ref=e182]: UIF-PRECISION-00011
            - button "Select UI benchmark pipe 00007 in viewport" [ref=e183] [cursor=pointer]:
              - img [ref=e184]
              - generic [ref=e188]: UIF-PRECISION-00007
            - button "Select UI benchmark node 00014 in viewport" [ref=e189] [cursor=pointer]:
              - img [ref=e190]
              - generic [ref=e193]: UIF-PRECISION-00014
            - button "Select UI benchmark node 00017 in viewport" [ref=e194] [cursor=pointer]:
              - img [ref=e195]
              - generic [ref=e198]: UIF-PRECISION-00017
            - button "Select UI benchmark node 00002 in viewport" [ref=e199] [cursor=pointer]:
              - img [ref=e200]
              - generic [ref=e203]: UIF-PRECISION-00002
            - button "Select UI benchmark node 00020 in viewport" [ref=e204] [cursor=pointer]:
              - img [ref=e205]
              - generic [ref=e208]: UIF-PRECISION-00020
            - button "Select UI benchmark support 00020 in viewport" [ref=e209] [cursor=pointer]:
              - img [ref=e210]
              - generic [ref=e213]: UIF-PRECISION-00020
          - img "Orientation gizmo showing X, Y, Z axes"
          - generic "View controls" [ref=e214]:
            - button "Front" [ref=e215] [cursor=pointer]
            - button "Top" [ref=e216] [cursor=pointer]
            - button "Isometric" [pressed] [ref=e217] [cursor=pointer]
          - generic:
            - generic: 1 m
        - region "Command and selection bar" [ref=e218]:
          - generic "Object creation tools" [ref=e219]:
            - button "Node" [ref=e220] [cursor=pointer]:
              - img [ref=e221]
              - text: Node
            - button "Pipe" [ref=e223] [cursor=pointer]:
              - img [ref=e224]
              - text: Pipe
            - button "Support" [ref=e228] [cursor=pointer]:
              - img [ref=e229]
              - text: Support
            - button "Component" [ref=e232] [cursor=pointer]:
              - img [ref=e233]
              - text: Component
            - button "Load" [ref=e236] [cursor=pointer]:
              - img [ref=e237]
              - text: Load
          - generic "Model focus" [ref=e239]: Select
          - group [ref=e240]:
            - generic "Selection & navigation" [ref=e241] [cursor=pointer]
        - status [ref=e242]:
          - strong [ref=e243]: "Measure · pipe: pipe:UIF-PRECISION-00011"
          - generic [ref=e244]:
            - 'generic "Distance: 0.100257 m; full-precision numeric value 0.10025736528056192 m." [ref=e245]': Distance 0.100257 m
            - text: ·
            - 'generic "ΔX: 0.100000 m; full-precision numeric value 0.10000000000000007 m." [ref=e246]': ΔX 0.100000 m
            - text: ·
            - 'generic "ΔY: 0.000678 m; full-precision numeric value 0.0006780000000000002 m." [ref=e247]': ΔY 0.000678 m
            - text: ·
            - 'generic "ΔZ: 0.007147 m; full-precision numeric value 0.007147 m." [ref=e248]': ΔZ 0.007147 m
      - region "Property inspector" [ref=e250]:
        - 'heading "UI benchmark pipe 00011 — pipe: pipe:UIF-PRECISION-00011" [level=2] [ref=e251]':
          - text: UI benchmark pipe 00011
          - generic [ref=e252]: "— pipe: pipe:UIF-PRECISION-00011"
        - tablist "Inspector views" [ref=e253]:
          - tab "Properties" [selected] [ref=e254]
          - tab "Task" [ref=e255]
        - tabpanel [ref=e256]:
          - group [ref=e257]:
            - generic "All properties" [ref=e258] [cursor=pointer]
          - paragraph [ref=e259]: "Shared section: section:UIF-OD-120. Edit diameter and wall in Sections."
        - generic [ref=e260]:
          - group [ref=e261]:
            - generic "Sources and units" [ref=e262] [cursor=pointer]
          - region "Pipe section assignment" [ref=e263]:
            - heading "Pipe section" [level=3] [ref=e264]
            - paragraph [ref=e265]: "Shared section: section:UIF-OD-120. Diameter and wall follow that source. Edit the section record to update all assigned pipes, or detach to retain local dimensions. Per-pipe supplements remain local."
            - generic [ref=e266]:
              - text: Shared section
              - combobox "Shared section" [ref=e267]:
                - option "Choose a section" [selected]
                - option "Invented OD 60 mm (section:UIF-OD-060)"
                - option "Invented OD 90 mm (section:UIF-OD-090)"
                - option "Invented OD 120 mm (section:UIF-OD-120)"
                - option "Invented OD 180 mm (section:UIF-OD-180)"
            - button "Queue section assignment" [disabled] [ref=e268]
            - button "Queue detachment to local dimensions" [ref=e269]
            - paragraph [ref=e270]: Review the affected pipes and exact dimensions in Pending changes before applying.
          - group [ref=e271]:
            - generic "New section" [ref=e272] [cursor=pointer]
            - option "pipe" [selected]
            - option "m, model metadata" [selected]
          - group [ref=e273]:
            - generic "New material" [ref=e274] [cursor=pointer]
            - option "Pa, model metadata" [selected]
            - option "1/degC, model metadata" [selected]
          - group [ref=e275]:
            - generic "New support" [ref=e276] [cursor=pointer]
            - option "N/m, model metadata" [selected]
          - group [ref=e277]:
            - generic "New component" [ref=e278] [cursor=pointer]
            - option "bend" [selected]
            - option "tee"
            - option "reducer"
            - option "valve"
            - option "flange"
            - option "expansion_joint"
            - option "m, model metadata" [selected]
            - option "rad, model metadata" [selected]
          - region "Delete pipe intent" [ref=e279]:
            - heading "Delete pipe" [level=3] [ref=e280]
            - button "Queue delete pipe" [ref=e282] [cursor=pointer]:
              - img [ref=e283]
              - text: Queue delete pipe
            - article [ref=e286]:
              - generic [ref=e287]:
                - generic [ref=e288]:
                  - generic [ref=e289]: Operation
                  - strong [ref=e290]: op:delete-pipe-pipe:UIF-PRECISION-00011; delete; proposed
                - generic [ref=e291]:
                  - generic [ref=e292]: Target
                  - strong [ref=e293]: Element; pipe:UIF-PRECISION-00011
                - generic [ref=e294]:
                  - generic [ref=e295]: Change
                  - strong [ref=e296]: delete_pipe_run; Explicit pipe deletion; pipe_segments; before=UI benchmark pipe 00011; node:UIF-PRECISION-00010->node:UIF-PRECISION-00011; material=material:UIF-INVENTED-01; after=not_present
                - generic [ref=e297]:
                  - generic [ref=e298]: Unit basis
                  - strong [ref=e299]: dimensionless; unit=none; explicit user-entered pipe deletion; primitive-load reference integrity required
                - generic [ref=e300]:
                  - generic [ref=e301]: Validation
                  - strong [ref=e302]: not_run; not_run; not_required_dimensionless; not_generated; not_applied
                - generic [ref=e303]:
                  - generic [ref=e304]: Audit boundary
                  - strong [ref=e305]: Routed through the structured operations only; no direct model mutation; requires your acceptance; does not change the accepted model until applied
                - generic [ref=e306]:
                  - generic [ref=e307]: Professional boundary
                  - strong [ref=e308]: Requires human review; no compliance claim; no approval claim
                - generic [ref=e309]:
                  - generic [ref=e310]: Rationale
                  - strong [ref=e311]: explicit user-entered pipe deletion for project:UIF-PRECISION-ORIGIN; requires reference validation before durable model change.
    - complementary "Agent" [ref=e312]:
      - button "Agent" [disabled] [ref=e314]:
        - img [ref=e315]
        - generic [ref=e318]: Agent
  - generic "Workspace status" [ref=e319]:
    - generic "Analysis statuses"
    - button "2 Issues" [ref=e320] [cursor=pointer]:
      - img [ref=e321]
      - text: 2 Issues
    - generic "Selection" [ref=e323]: "pipe: pipe:UIF-PRECISION-00011"
    - generic "Display units" [ref=e324]: Entered
    - button "About SWBPIPE…" [ref=e325] [cursor=pointer]:
      - img [ref=e326]
```

# Test source

```ts
  638 |       const evidence = (window as any).__paletteFocusEvidence;
  639 |       evidence.dispose(); delete (window as any).__paletteFocusEvidence;
  640 |       return evidence.events;
  641 |     });
  642 |     await testInfo.attach(`palette-node-select-${activation}-focus-events`, { body: JSON.stringify(events, null, 2), contentType: "application/json" });
  643 |   }
  644 | }
  645 | 
  646 | export async function expectPassiveOrientationFrame(page: Page, testInfo: TestInfo, name: string) {
  647 |   const marker = page.getByTestId("viewport-axis-triad");
  648 |   const witness = await page.getByTestId("viewport-canvas").evaluate((host) => {
  649 |     const canvases = Array.from(host.querySelectorAll("canvas"));
  650 |     const visible = canvases.filter((canvas) => {
  651 |       const box = canvas.getBoundingClientRect();
  652 |       return box.width > 0 && box.height > 0 && getComputedStyle(canvas).visibility !== "hidden";
  653 |     });
  654 |     const canvas = visible[0];
  655 |     if (!canvas) return null;
  656 |     const box = canvas.getBoundingClientRect();
  657 |     const frame = canvas.closest(".viewport-frame")!.getBoundingClientRect();
  658 |     const width = canvas.clientWidth, height = canvas.clientHeight;
  659 |     const shell = canvas.closest<HTMLElement>(".viewport-shell");
  660 |     const rawBottomInset = Number.parseFloat(shell ? getComputedStyle(shell).getPropertyValue("--viewport-presentation-bottom-inset") : "0");
  661 |     const presentationBottomInset = Number.isFinite(rawBottomInset)
  662 |       ? Math.min(Math.max(0, Math.round(rawBottomInset)), Math.max(0, height - 1))
  663 |       : 0;
  664 |     const availableHeight = Math.max(1, height - presentationBottomInset);
  665 |     const size = Math.min(96, Math.floor(Math.min(width, availableHeight)));
  666 |     const insetX = Math.min(8, Math.max(0, width - size));
  667 |     const insetY = presentationBottomInset + Math.min(8, Math.max(0, availableHeight - size));
  668 |     const clip = { x: box.left + insetX, y: box.top + box.height - insetY - size, width: size, height: size };
  669 |     const offset = 0.5 / devicePixelRatio;
  670 |     const low = offset, high = size - offset, mid = size / 2;
  671 |     const samples = [[mid, mid], [low, low], [high, low], [low, high], [high, high],
  672 |       [mid, low], [mid, high], [low, mid], [high, mid]].map(([x, y]) => {
  673 |       const point = { x: clip.x + x, y: clip.y + y };
  674 |       const hit = document.elementFromPoint(point.x, point.y);
  675 |       return { point, ownedByCanvas: hit === canvas, tag: hit?.tagName ?? null,
  676 |         testId: hit?.getAttribute("data-testid") || null, id: hit?.id || null,
  677 |         owner: hit?.getAttribute("data-testid") || hit?.getAttribute("aria-label") || hit?.id || hit?.tagName || null };
  678 |     });
  679 |     return { canvas: box.toJSON(), frame: frame.toJSON(), client: { width, height }, clip,
  680 |       size, insetX, insetY, presentationBottomInset, dpr: devicePixelRatio, visibleCanvases: visible.length,
  681 |       totalMainCanvases: canvases.length, separateGizmoCanvases: document.querySelectorAll('[data-testid="viewport-axis-triad"] canvas').length,
  682 |       viewport: { width: innerWidth, height: innerHeight }, samples };
  683 |   });
  684 |   await testInfo.attach(`${name}-orientation-witness`, { body: JSON.stringify(witness, null, 2), contentType: "application/json" });
  685 |   await testInfo.attach(`${name}-orientation-context`, { body: await page.screenshot(), contentType: "image/png" });
  686 |   if (witness && witness.size > 0 && witness.clip.x >= 0 && witness.clip.y >= 0 &&
  687 |       witness.clip.x + witness.size <= witness.viewport.width && witness.clip.y + witness.size <= witness.viewport.height) {
  688 |     await testInfo.attach(`${name}-orientation-xyz-crop`, { body: await page.screenshot({ clip: witness.clip }), contentType: "image/png" });
  689 |   }
  690 |   await expect(marker).toBeVisible();
  691 |   await expect(marker).toHaveAccessibleName(/Orientation gizmo.*X.*Y.*Z/i);
  692 |   await expect(marker).toHaveCSS("pointer-events", "none");
  693 |   expect(witness).not.toBeNull();
  694 |   if (!witness) throw new Error("No visible main canvas for passive orientation witness");
  695 |   expect(witness.visibleCanvases).toBe(1);
  696 |   expect(witness.totalMainCanvases).toBe(1);
  697 |   expect(Math.abs(witness.canvas.width - witness.client.width)).toBeLessThanOrEqual(1);
  698 |   expect(Math.abs(witness.canvas.height - witness.client.height)).toBeLessThanOrEqual(1);
  699 |   expect(witness.separateGizmoCanvases).toBe(0);
  700 |   expect(witness.size).toBeGreaterThan(0);
  701 |   if (Math.min(witness.client.width, witness.client.height) >= 96) expect(witness.size).toBe(96);
  702 |   for (const bounds of [witness.canvas, witness.frame,
  703 |     { left: 0, top: 0, right: witness.viewport.width, bottom: witness.viewport.height }]) {
  704 |     expect(witness.clip.x).toBeGreaterThanOrEqual(bounds.left);
  705 |     expect(witness.clip.y).toBeGreaterThanOrEqual(bounds.top);
  706 |     expect(witness.clip.x + witness.size).toBeLessThanOrEqual(bounds.right);
  707 |     expect(witness.clip.y + witness.size).toBeLessThanOrEqual(bounds.bottom);
  708 |   }
  709 |   for (const sample of witness.samples) expect(sample.ownedByCanvas, `orientation sample owned by ${sample.owner}`).toBe(true);
  710 |   return witness;
  711 | }
  712 | 
  713 | export async function expectCenterUnobscured(
  714 |   locator: Locator,
  715 |   options: Readonly<{ minimumTarget?: boolean }> = {},
  716 | ) {
  717 |   const witness = await locator.evaluate((element) => {
  718 |     const box = element.getBoundingClientRect();
  719 |     const center = { x: box.left + box.width / 2, y: box.top + box.height / 2 };
  720 |     const top = document.elementFromPoint(center.x, center.y);
  721 |     return {
  722 |       box: { left: box.left, top: box.top, right: box.right, bottom: box.bottom, width: box.width, height: box.height },
  723 |       center,
  724 |       centerOwner: top instanceof Element
  725 |         ? top.getAttribute("data-testid")
  726 |           ?? top.getAttribute("aria-label")
  727 |           ?? top.id
  728 |           ?? (typeof top.className === "string" && top.className.trim() ? `.${top.className.trim().replace(/\s+/g, ".")}` : top.tagName)
  729 |         : null,
  730 |       centerOwnedByTarget: top instanceof Node && (element === top || element.contains(top)),
  731 |       viewport: { width: innerWidth, height: innerHeight },
  732 |     };
  733 |   });
  734 |   expect(witness.box.left).toBeGreaterThanOrEqual(0);
  735 |   expect(witness.box.top).toBeGreaterThanOrEqual(0);
  736 |   expect(witness.box.right).toBeLessThanOrEqual(witness.viewport.width);
  737 |   expect(witness.box.bottom).toBeLessThanOrEqual(witness.viewport.height);
> 738 |   expect(witness.centerOwnedByTarget, `center is covered by ${witness.centerOwner ?? "no element"}`).toBe(true);
      |                                                                                                      ^ Error: center is covered by 
  739 |   if (options.minimumTarget) {
  740 |     expect(witness.box.width).toBeGreaterThanOrEqual(24);
  741 |     expect(witness.box.height).toBeGreaterThanOrEqual(24);
  742 |   }
  743 |   return witness;
  744 | }
  745 | 
  746 | export async function expectClearOfClosedRailHandles(
  747 |   page: Page,
  748 |   areas: readonly Readonly<{ name: string; locator: Locator }>[],
  749 | ): Promise<void> {
  750 |   const handles = [
  751 |     { name: "closed Model handle", locator: page.getByTestId("toggle-tree") },
  752 |     { name: "closed Properties handle", locator: page.getByTestId("toggle-inspector") },
  753 |   ];
  754 |   for (const handle of handles) {
  755 |     await expect(handle.locator).toHaveAttribute("aria-expanded", "false");
  756 |     await expectCenterUnobscured(handle.locator, { minimumTarget: true });
  757 |   }
  758 |   for (const area of areas) {
  759 |     await expect(area.locator, `${area.name} is visible`).toBeVisible();
  760 |     await expectCenterUnobscured(area.locator);
  761 |     const areaBox = await area.locator.boundingBox();
  762 |     expect(areaBox, `${area.name} has layout geometry`).not.toBeNull();
  763 |     for (const handle of handles) {
  764 |       const handleBox = await handle.locator.boundingBox();
  765 |       expect(handleBox, `${handle.name} has layout geometry`).not.toBeNull();
  766 |       const overlapWidth = Math.max(0, Math.min(areaBox!.x + areaBox!.width, handleBox!.x + handleBox!.width) - Math.max(areaBox!.x, handleBox!.x));
  767 |       const overlapHeight = Math.max(0, Math.min(areaBox!.y + areaBox!.height, handleBox!.y + handleBox!.height) - Math.max(areaBox!.y, handleBox!.y));
  768 |       expect(overlapWidth * overlapHeight, `${handle.name} does not overlap ${area.name}`).toBe(0);
  769 |     }
  770 |   }
  771 | }
  772 | 
  773 | export async function expectContentFits(locator: Locator, name: string): Promise<void> {
  774 |   const metrics = await locator.evaluate((element) => ({
  775 |     clientHeight: element.clientHeight,
  776 |     clientWidth: element.clientWidth,
  777 |     scrollHeight: element.scrollHeight,
  778 |     scrollWidth: element.scrollWidth,
  779 |   }));
  780 |   expect(metrics.clientWidth, `${name} has rendered width`).toBeGreaterThan(0);
  781 |   expect(metrics.clientHeight, `${name} has rendered height`).toBeGreaterThan(0);
  782 |   expect(metrics.scrollWidth, `${name} is horizontally complete`).toBeLessThanOrEqual(metrics.clientWidth + 1);
  783 |   expect(metrics.scrollHeight, `${name} is vertically complete`).toBeLessThanOrEqual(metrics.clientHeight + 1);
  784 | }
  785 | 
  786 | export async function captureState(page: Page, testInfo: TestInfo, name: string): Promise<void> {
  787 |   const fileName = `${name.replace(/[^a-z0-9-]+/gi, "-").toLowerCase()}.png`;
  788 |   const outputPath = testInfo.outputPath(fileName);
  789 |   await page.screenshot({ path: outputPath, animations: "disabled" });
  790 |   await testInfo.attach(name, { path: outputPath, contentType: "image/png" });
  791 | }
  792 | 
  793 | export async function capturePageClip(
  794 |   page: Page,
  795 |   testInfo: TestInfo,
  796 |   name: string,
  797 |   clip: { x: number; y: number; width: number; height: number },
  798 | ): Promise<string> {
  799 |   const fileName = `${name.replace(/[^a-z0-9-]+/gi, "-").toLowerCase()}.png`;
  800 |   const outputPath = testInfo.outputPath(fileName);
  801 |   await page.screenshot({ path: outputPath, animations: "disabled", clip });
  802 |   await testInfo.attach(name, { path: outputPath, contentType: "image/png" });
  803 |   return outputPath;
  804 | }
  805 | 
  806 | export async function captureElementState(locator: Locator, testInfo: TestInfo, name: string): Promise<string> {
  807 |   const fileName = `${name.replace(/[^a-z0-9-]+/gi, "-").toLowerCase()}.png`;
  808 |   const outputPath = testInfo.outputPath(fileName);
  809 |   await locator.screenshot({ path: outputPath, animations: "disabled" });
  810 |   await testInfo.attach(name, { path: outputPath, contentType: "image/png" });
  811 |   return outputPath;
  812 | }
  813 | 
  814 | export async function keyboardMeasureTargets(page: Page, labels: readonly string[]): Promise<Locator> {
  815 |   const measure = page.getByRole("button", { name: "Measure", exact: true });
  816 |   await activateWithKeyboard(page, measure);
  817 |   await expect(measure).toHaveAttribute("aria-pressed", "true");
  818 |   for (const label of labels) {
  819 |     await activateWithKeyboard(page, page.getByRole("button", { name: label, exact: true }));
  820 |   }
  821 |   const readout = page.getByTestId("viewport-measurement-readout");
  822 |   await expect(readout).toContainText("Distance");
  823 |   await expect(readout).toContainText("ΔX");
  824 |   await expect(readout).toContainText("ΔY");
  825 |   await expect(readout).toContainText("ΔZ");
  826 |   return readout;
  827 | }
  828 | 
  829 | export function withTypedCollision(model: any, sharedId = "same/id?typed"): any {
  830 |   const copy = structuredClone(model);
  831 |   const oldNode = copy.nodes[0].id;
  832 |   const oldPipe = copy.pipe_segments[0].id;
  833 |   copy.nodes[0].id = sharedId;
  834 |   copy.pipe_segments[0].id = sharedId;
  835 |   for (const pipe of copy.pipe_segments) {
  836 |     if (pipe.from === oldNode) pipe.from = sharedId;
  837 |     if (pipe.to === oldNode) pipe.to = sharedId;
  838 |   }
```