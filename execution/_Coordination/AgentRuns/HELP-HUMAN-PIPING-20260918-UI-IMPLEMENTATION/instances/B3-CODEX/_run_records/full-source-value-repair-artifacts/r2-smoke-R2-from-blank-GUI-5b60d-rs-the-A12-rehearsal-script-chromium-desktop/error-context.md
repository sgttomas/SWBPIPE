# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: r2-smoke.spec.ts >> R2 from-blank GUI journey authors the A12 rehearsal script
- Location: e2e/r2-smoke.spec.ts:865:1

# Error details

```
Error: expect(locator).toHaveValue(expected) failed

Locator: getByTestId('viewport-create-node-unit')
Expected: "m"
Error: Not an input element

Call log:
  - Expect "toHaveValue" with timeout 10000ms
  - waiting for getByTestId('viewport-create-node-unit')

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
      - paragraph [ref=e18]: Blank Local Model
    - group "Editing tools" [ref=e19]:
      - button "Undo model edit" [disabled] [ref=e20]:
        - img [ref=e21]
      - button "Redo model edit" [disabled] [ref=e24]:
        - img [ref=e25]
      - button "Select" [ref=e28] [cursor=pointer]:
        - img [ref=e29]
        - generic [ref=e31]: Select
    - group "View" [ref=e32]:
      - button "Table" [ref=e34] [cursor=pointer]:
        - img [ref=e35]
        - generic [ref=e37]: Table
      - button "Model" [ref=e39] [cursor=pointer]:
        - img [ref=e40]
        - generic [ref=e43]: Model
      - button "Both" [pressed] [ref=e45] [cursor=pointer]:
        - img [ref=e46]
        - generic [ref=e48]: Both
    - generic [ref=e49]:
      - button "Run" [ref=e50] [cursor=pointer]:
        - img [ref=e51]
        - generic [ref=e53]: Run
      - button "Issues, 5" [ref=e54] [cursor=pointer]:
        - img [ref=e55]
        - generic [ref=e57]: Issues
        - generic [ref=e58]: "5"
    - group "Panels" [ref=e59]:
      - button "Inspector" [expanded] [ref=e61] [cursor=pointer]:
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
          - button "Issues, 5" [ref=e128] [cursor=pointer]:
            - img [ref=e129]
            - generic [ref=e131]: Issues
            - generic [ref=e132]: "5"
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
            - button "Tree" [pressed] [ref=e149]:
              - img [ref=e150]
              - text: Tree
            - button "Grid" [ref=e153]:
              - img [ref=e154]
              - text: Grid
          - region "Model tree filtering" [ref=e156]:
            - generic [ref=e157]:
              - img [ref=e158]
              - generic [ref=e161]: Filter model
              - searchbox "Filter model tree" [ref=e162]
            - generic [ref=e163]: 2 of 2 model entities visible
            - button "Clear model tree filter" [disabled] [ref=e164]:
              - img [ref=e165]
          - tree "Model" [ref=e168]:
            - generic [ref=e169]:
              - treeitem "Blank Local Model project:blank-local-20260920t055225z" [level=1] [selected] [ref=e171] [cursor=pointer]:
                - img [ref=e172]
                - generic [ref=e176]:
                  - strong [ref=e177]: Blank Local Model
                  - generic [ref=e178]: project:blank-local-20260920t055225z
              - treeitem "Diagnostics" [expanded] [level=1] [ref=e180] [cursor=pointer]:
                - generic [ref=e181]: ▾
                - strong [ref=e182]: Diagnostics
              - treeitem "BLANK_PROJECT_AUTHORING_TARGET diagnostic:blank-project:authoring-target" [level=2] [ref=e184] [cursor=pointer]:
                - img [ref=e185]
                - generic [ref=e188]:
                  - strong [ref=e189]: BLANK_PROJECT_AUTHORING_TARGET
                  - generic [ref=e190]: diagnostic:blank-project:authoring-target
      - separator "Resize table and canvas" [ref=e191]
      - generic [ref=e193]:
        - generic [ref=e194]:
          - group "Viewport controls" [ref=e195]:
            - generic [ref=e196]: 3D Centerline
            - group "Viewport deformation overlay status" [ref=e197]:
              - generic "Deformation · unavailable" [ref=e198] [cursor=pointer]
            - group "Viewport display toggles" [ref=e199]:
              - button "Labels" [pressed] [ref=e200]
              - button "Loads" [pressed] [ref=e201]
              - button "Grid" [pressed] [ref=e202]
            - group "Viewport selection tools" [ref=e203]:
              - button "Box Select" [ref=e204]
              - generic [ref=e205]:
                - generic [ref=e206]: Selection filter
                - combobox "Selection filter" [ref=e207]:
                  - option "All" [selected]
                  - option "Pipes"
                  - option "Nodes"
                  - option "Supports"
                  - option "Components"
              - button "Hide" [disabled] [ref=e208]
              - button "Isolate" [disabled] [ref=e209]
              - button "Show All" [disabled] [ref=e210]
              - button "Fit Model" [ref=e211]
              - button "Fit Visible" [ref=e212]
              - button "Fit Selection" [disabled] [ref=e213]
            - group "Viewport geometry" [ref=e214]:
              - button "Schematic" [pressed] [ref=e215]
              - button "Actual OD" [ref=e216]
              - button "Measure" [ref=e217]
          - generic "Viewport status" [ref=e218]:
            - 'generic "Selected project: project:blank-local-20260920t055225z" [ref=e219]': "Selected: project:blank-local-20260920t055225z"
            - status "Schematic centerline geometry" [ref=e220]
            - status "View command status" [ref=e221]: No view command dispatched.
        - generic [ref=e222]:
          - generic "Three.js pipe centerline viewport" [ref=e223]
          - generic "Viewport entity selection"
          - img "Orientation gizmo showing X, Y, Z axes"
          - generic "View controls" [ref=e225]:
            - button "Front" [ref=e226] [cursor=pointer]
            - button "Top" [ref=e227] [cursor=pointer]
            - button "Isometric" [pressed] [ref=e228] [cursor=pointer]
          - generic:
            - generic: 1 m
        - region "Command and selection bar" [ref=e229]:
          - generic "Object creation tools" [ref=e230]:
            - button "Node" [pressed] [ref=e231] [cursor=pointer]:
              - img [ref=e232]
              - text: Node
            - button "Pipe" [ref=e234] [cursor=pointer]:
              - img [ref=e235]
              - text: Pipe
            - button "Support" [ref=e239] [cursor=pointer]:
              - img [ref=e240]
              - text: Support
            - button "Component" [ref=e243] [cursor=pointer]:
              - img [ref=e244]
              - text: Component
            - button "Load" [ref=e247] [cursor=pointer]:
              - img [ref=e248]
              - text: Load
          - 'generic "Node tool armed: click empty canvas to fill coordinates, then queue node." [ref=e250]': Node tool armed
          - group [ref=e251]:
            - generic "Selection & navigation" [ref=e252] [cursor=pointer]
      - generic [ref=e253]:
        - button "Close inspector" [ref=e254]:
          - img [ref=e255]
        - region "Viewport editor intents" [ref=e259]:
          - heading "Create node" [level=3] [ref=e260]
          - group [ref=e261]:
            - generic "Explicit node geometry" [ref=e262]:
              - generic [ref=e263]:
                - generic [ref=e264]: Node ID
                - textbox "New node ID" [ref=e265]:
                  - /placeholder: node:N-3
                  - text: node:R2-100
              - generic [ref=e266]:
                - generic [ref=e267]: Label
                - textbox "New node label" [ref=e268]:
                  - /placeholder: Node label
                  - text: R2 anchored node
              - generic [ref=e269]:
                - generic [ref=e270]: X
                - textbox "New node X coordinate" [ref=e271]:
                  - /placeholder: "0"
                  - text: "0"
              - generic [ref=e272]:
                - generic [ref=e273]: "Y"
                - textbox "New node Y coordinate" [ref=e274]:
                  - /placeholder: "0"
                  - text: "0"
              - generic [ref=e275]:
                - generic [ref=e276]: Z
                - textbox "New node Z coordinate" [ref=e277]:
                  - /placeholder: "0"
                  - text: "0"
              - generic [ref=e278]:
                - generic [ref=e279]: Coordinate unit
                - combobox "New node coordinate unit" [ref=e281] [cursor=pointer]:
                  - generic [ref=e282]: m
                  - text: ▾
              - generic [ref=e283]: "Coordinates: m, model metadata"
              - generic [ref=e284]: "Pointer plane: global XZ · Y=0 m"
              - generic "Click within 4 CSS pixels on the visible 3D canvas to capture on global XZ at Y=0." [ref=e285]
              - generic [ref=e286]:
                - generic [ref=e287]: Provenance
                - textbox "New node provenance" [active] [ref=e288]: invented_a12_rehearsal_user_input
              - button "Add node" [ref=e289] [cursor=pointer]:
                - img [ref=e290]
                - text: Add node
              - generic [ref=e292]: Ready to validate and freeze this explicit node create intent.
            - text: ▾ ▾ ▾ ▾ ▾ ▾
            - region "Route review" [ref=e293]:
              - heading "Review and Apply" [level=4] [ref=e294]
              - status [ref=e295]: The affected selection changed. Add again to review the current draft.
              - paragraph [ref=e296]: Add a complete node or route to generate the service validation and exact diff.
              - button "Apply" [disabled] [ref=e297]
            - group [ref=e298]:
              - generic "Unit source" [ref=e299] [cursor=pointer]
          - group [ref=e300]:
            - generic "Pending changes (0)" [ref=e301] [cursor=pointer]
        - region "Property inspector" [ref=e302]:
          - 'heading "Blank Local Model — project: project:blank-local-20260920t055225z" [level=2] [ref=e303]':
            - text: Blank Local Model
            - generic [ref=e304]: "— project: project:blank-local-20260920t055225z"
          - tablist "Inspector views" [ref=e305]:
            - tab "Properties" [selected] [ref=e306]
            - tab "Task" [ref=e307]
          - tabpanel [ref=e308]:
            - group [ref=e309]:
              - generic "All properties" [ref=e310] [cursor=pointer]
          - generic [ref=e311]:
            - group [ref=e312]:
              - generic "Sources and units" [ref=e313] [cursor=pointer]
            - group [ref=e314]:
              - generic "New support configuration" [ref=e315] [cursor=pointer]
              - text: ▾
            - group [ref=e316]:
              - generic "New section" [ref=e317] [cursor=pointer]
              - text: ▾ ▾
            - group [ref=e318]:
              - generic "New material" [ref=e319] [cursor=pointer]
              - text: ▾ ▾
            - group [ref=e320]:
              - generic "New support" [ref=e321] [cursor=pointer]
              - text: ▾
            - group [ref=e322]:
              - generic "New component" [ref=e323] [cursor=pointer]
              - text: ▾ ▾ ▾
    - complementary "Agent" [ref=e324]:
      - button "Agent" [disabled] [ref=e326]:
        - img [ref=e327]
        - generic [ref=e330]: Agent
  - generic "Workspace status" [ref=e331]:
    - generic "Analysis statuses" [ref=e332]:
      - button "Solver · Model incomplete" [ref=e334] [cursor=pointer]
    - button "5 Issues" [ref=e335] [cursor=pointer]:
      - img [ref=e336]
      - text: 5 Issues
    - generic "Selection" [ref=e338]: "project: project:blank-local-20260920t055225z"
    - generic "Display units" [ref=e339]: Entered
    - button "About SWBPIPE…" [ref=e340] [cursor=pointer]:
      - img [ref=e341]
```

# Test source

```ts
  1611 |     "supersedes the run-panel selector"
  1612 |   );
  1613 |   await page.getByTestId("rule-check-solver-preview-actual_stress").click();
  1614 |   await expect(page.getByTestId("rule-check-solver-resolution-actual_stress")).toHaveAttribute(
  1615 |     "data-status",
  1616 |     "no_result_rows"
  1617 |   );
  1618 |   await expect(page.getByTestId("rule-check-solver-browse-actual_stress")).toContainText(
  1619 |     "Run a solve first"
  1620 |   );
  1621 | });
  1622 | 
  1623 | test("R3 guided flow routes private library, rule-pack, solve, binding, and blocked check steps", async ({ page }) => {
  1624 |   await page.goto("/");
  1625 |   await expect(page.getByTestId("desktop-preview-shell")).toBeVisible();
  1626 |   await ensureEngineReady(page);
  1627 | 
  1628 |   await expect(page.getByTestId("app-menu-bar")).toBeVisible();
  1629 |   // Slice B3, specification §5.4 rules 1 and 2: no Rule pack chip before a run; the recorded value
  1630 |   // is read in the Analyze page's readiness summary.
  1631 |   await expectNoStatusChip(page, "status-pill-rule-check");
  1632 |   await expectRecordedStatusOnAnalyzePage(page, "rule", "RULE_INPUTS_INCOMPLETE");
  1633 | 
  1634 |   await openWorkspaceSection(page, "libraries");
  1635 |   await expect(page.getByTestId("workspace-section-libraries")).toBeVisible();
  1636 |   await page.getByTestId("library-load-template").click();
  1637 |   await page.getByTestId("library-validate").click();
  1638 |   await expect(page.getByTestId("library-action-status")).toContainText(
  1639 |     "LIBRARY-IMPORT-BACKEND-DESKTOP-ONLY"
  1640 |   );
  1641 |   await page.getByTestId("library-save").click();
  1642 | 
  1643 |   await openWorkspaceSection(page, "rule-packs");
  1644 |   await expect(page.getByTestId("workspace-section-rule-packs")).toBeVisible();
  1645 |   await page.getByTestId("rule-pack-new-draft").click();
  1646 |   await page.getByTestId("rule-pack-validate").click();
  1647 |   await expect(page.getByTestId("rule-pack-action-status")).toContainText(
  1648 |     "RULE-PACK-BACKEND-DESKTOP-ONLY"
  1649 |   );
  1650 |   await page.getByTestId("rule-pack-compute-checksum").click();
  1651 |   await expect(page.getByTestId("rule-pack-action-status")).toContainText(
  1652 |     "RULE-PACK-BACKEND-DESKTOP-ONLY"
  1653 |   );
  1654 |   await page.getByTestId("rule-pack-save").click();
  1655 | 
  1656 |   await openWorkspaceSection(page, "solve");
  1657 |   await expect(page.getByTestId("workspace-section-solve")).toBeVisible();
  1658 |   await page.getByTestId("run-mechanics-preview").click();
  1659 |   await expect(page.getByTestId("solve-job-summary")).toContainText("state=completed");
  1660 | 
  1661 |   await page.getByTestId("rule-check-load-demo").click();
  1662 |   await expect(page.getByTestId("rule-check-binding-plan")).toBeVisible();
  1663 | 
  1664 |   await page.getByTestId("rule-check-run").click();
  1665 |   await expect(page.getByTestId("rule-check-run-status")).toContainText("RULE-CHECK-BACKEND-DESKTOP-ONLY");
  1666 |   await page.getByTestId("issues-drawer-toggle").click();
  1667 |   await expect(page.getByTestId("issues-home")).toContainText("RULE_INPUTS_INCOMPLETE");
  1668 | 
  1669 |   const horizontalOverflow = await page.evaluate(
  1670 |     () =>
  1671 |       document.documentElement.scrollWidth > document.documentElement.clientWidth ||
  1672 |       document.body.scrollWidth > document.body.clientWidth
  1673 |   );
  1674 |   expect(horizontalOverflow).toBe(false);
  1675 | });
  1676 | 
  1677 | function stepPayload(changeKind: string, ref: string): any {
  1678 |   const step = rehearsal.steps.find(
  1679 |     (candidate) =>
  1680 |       candidate.change_kind === changeKind && (candidate.target?.ref === ref || candidate.payload.id === ref)
  1681 |   );
  1682 |   if (!step) throw new Error(`missing rehearsal step ${changeKind} ${ref}`);
  1683 |   return step.payload;
  1684 | }
  1685 | 
  1686 | async function fillNodeDraft(page: Page, payload: any): Promise<void> {
  1687 |   await ensureCreationToolArmed(page, "command-node", "Node tool armed");
  1688 |   await page.getByTestId("viewport-create-node-id").fill(payload.id);
  1689 |   await page.getByTestId("viewport-create-node-label").fill(payload.label);
  1690 |   await page.getByTestId("viewport-create-node-x").fill(String(payload.position.x));
  1691 |   await page.getByTestId("viewport-create-node-y").fill(String(payload.position.y));
  1692 |   await page.getByTestId("viewport-create-node-z").fill(String(payload.position.z));
  1693 |   await page.getByTestId("viewport-create-node-provenance").fill(payload.provenance);
  1694 | }
  1695 | 
  1696 | async function applyReviewedDraft(
  1697 |   page: Page,
  1698 |   expectation: {
  1699 |     addTestId: string;
  1700 |     unitTestId: string;
  1701 |     operationId: string;
  1702 |     diff: string;
  1703 |     reviewSequence: number;
  1704 |     appliedSequence: number;
  1705 |     publishedType: TreeEntityType;
  1706 |     publishedId: string;
  1707 |     publishedText: string;
  1708 |   }
  1709 | ): Promise<void> {
  1710 |   await expect(page.getByTestId(expectation.unitTestId)).toBeVisible();
> 1711 |   await expect(page.getByTestId(expectation.unitTestId)).toHaveValue("m");
       |                                                          ^ Error: expect(locator).toHaveValue(expected) failed
  1712 |   await page.getByTestId(expectation.addTestId).click();
  1713 | 
  1714 |   const review = page.getByTestId("viewport-draft-review-preview");
  1715 |   await expect(review).toContainText("Single operation");
  1716 |   await expect(review).toContainText(expectation.operationId);
  1717 |   await expect(review).toContainText("Validated model hash: sha256:");
  1718 |   await expect(review).toContainText(expectation.diff);
  1719 |   await expect(review).toContainText("[m]");
  1720 | 
  1721 |   const apply = page.getByTestId("apply-reviewed-draft");
  1722 |   await expect(apply).toBeEnabled();
  1723 |   await openWorkspaceSection(page, "operations");
  1724 |   const applyPanel = page.getByTestId("operation-apply-panel");
  1725 |   await expect(applyPanel.getByTestId("operation-apply-summary")).toContainText(
  1726 |     `0 queued; ${expectation.appliedSequence - 1} applied`
  1727 |   );
  1728 |   await apply.click();
  1729 |   await expect(applyPanel.getByTestId("operation-apply-message")).toContainText(
  1730 |     `Applied reviewed ${expectation.operationId}`
  1731 |   );
  1732 |   await expectTreeEntity(page, expectation.publishedType, expectation.publishedId, expectation.publishedText);
  1733 |   await expect(applyPanel.getByTestId("operation-apply-summary")).toContainText(
  1734 |     `0 queued; ${expectation.appliedSequence} applied`
  1735 |   );
  1736 |   await expect(applyPanel.getByTestId("operation-unit-policy-chip")).toContainText(
  1737 |     `${expectation.appliedSequence} applied receipts`
  1738 |   );
  1739 | 
  1740 |   const receipt = applyPanel.getByTestId(
  1741 |     `applied-operation-route-applied-viewport-draft-review-${expectation.reviewSequence}`
  1742 |   );
  1743 |   await expect(receipt).toContainText("Applied through local_wasm_engine");
  1744 |   await expect(receipt).toContainText("Acceptance basis user_initiated_apply_in_local_session");
  1745 |   await expect(receipt).toContainText("persistence session_state_only_not_yet_saved");
  1746 |   await expect(receipt).toContainText("professional approval not recorded");
  1747 | }
  1748 | 
  1749 | async function applyQueuedIntent(
  1750 |   page: Page,
  1751 |   queuedSequence: number,
  1752 |   appliedSequence: number,
  1753 |   expectedOperation: string
  1754 | ): Promise<void> {
  1755 |   // Authoring forms live in the persistent core or the Load Cases section;
  1756 |   // the Operation Apply section is the receipt and audit surface.
  1757 |   const key = `editor-intent-${queuedSequence}`;
  1758 |   await openWorkspaceSection(page, "operations");
  1759 |   await expect(page.getByTestId("operation-apply-summary")).toContainText(
  1760 |     `1 queued; ${appliedSequence - 1} applied`
  1761 |   );
  1762 |   await page.getByTestId(`apply-intent-${key}`).click();
  1763 |   await expect(page.getByTestId("workspace-section-operations")).toBeVisible();
  1764 |   await expect(page.getByTestId("operation-apply-summary")).toContainText(`0 queued; ${appliedSequence} applied`);
  1765 |   const receipt = page.getByTestId(`applied-operation-route-applied-${appliedSequence}-${key}`);
  1766 |   await expect(receipt).toContainText("Applied through local_wasm_engine");
  1767 |   await expect(receipt).toContainText("Acceptance basis user_initiated_apply_in_local_session");
  1768 |   await expect(receipt).toContainText("persistence session_state_only_not_yet_saved");
  1769 |   await expect(receipt).toContainText("professional approval not recorded");
  1770 |   await expect(page.getByTestId(`applied-operation-applied-${appliedSequence}-${key}`)).toContainText(
  1771 |     expectedOperation
  1772 |   );
  1773 | }
  1774 | 
  1775 | type PngImage = {
  1776 |   bytesPerPixel: number;
  1777 |   data: Uint8Array;
  1778 |   height: number;
  1779 |   width: number;
  1780 | };
  1781 | 
  1782 | function pngStats(buffer: Buffer): { height: number; opaquePixels: number; uniqueColors: number; width: number } {
  1783 |   const image = decodePng(buffer);
  1784 |   const colors = new Set<string>();
  1785 |   let opaquePixels = 0;
  1786 |   for (let offset = 0; offset < image.data.length; offset += image.bytesPerPixel) {
  1787 |     const alpha = image.bytesPerPixel === 4 ? image.data[offset + 3] : 255;
  1788 |     if (alpha > 0) opaquePixels += 1;
  1789 |     if (colors.size < 2_000) {
  1790 |       colors.add(Array.from(image.data.slice(offset, offset + image.bytesPerPixel)).join(","));
  1791 |     }
  1792 |   }
  1793 |   return { height: image.height, opaquePixels, uniqueColors: colors.size, width: image.width };
  1794 | }
  1795 | 
  1796 | function diffPngPixels(leftBuffer: Buffer, rightBuffer: Buffer): number {
  1797 |   const left = decodePng(leftBuffer);
  1798 |   const right = decodePng(rightBuffer);
  1799 |   const compared = Math.min(left.data.length, right.data.length);
  1800 |   const bytesPerPixel = Math.min(left.bytesPerPixel, right.bytesPerPixel);
  1801 |   let changed = 0;
  1802 |   for (let offset = 0; offset + bytesPerPixel <= compared; offset += bytesPerPixel) {
  1803 |     const channelDelta =
  1804 |       Math.abs(left.data[offset] - right.data[offset]) +
  1805 |       Math.abs(left.data[offset + 1] - right.data[offset + 1]) +
  1806 |       Math.abs(left.data[offset + 2] - right.data[offset + 2]);
  1807 |     if (channelDelta > 6) changed += 1;
  1808 |   }
  1809 |   return changed;
  1810 | }
  1811 | 
```