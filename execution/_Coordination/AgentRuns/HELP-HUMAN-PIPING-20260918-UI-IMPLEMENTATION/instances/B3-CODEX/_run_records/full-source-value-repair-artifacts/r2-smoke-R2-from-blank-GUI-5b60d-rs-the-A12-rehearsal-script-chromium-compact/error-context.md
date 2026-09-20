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
      - button "Select (⎋)" [ref=e28] [cursor=pointer]:
        - img [ref=e29]
    - group "View" [ref=e31]:
      - button "Table" [ref=e33] [cursor=pointer]:
        - img [ref=e34]
        - generic [ref=e36]: Table
      - button "Model" [ref=e38] [cursor=pointer]:
        - img [ref=e39]
        - generic [ref=e42]: Model
      - button "Both" [pressed] [ref=e44] [cursor=pointer]:
        - img [ref=e45]
        - generic [ref=e47]: Both
    - generic [ref=e48]:
      - button "Run" [ref=e49] [cursor=pointer]:
        - img [ref=e50]
        - generic [ref=e52]: Run
      - button "Issues, 5" [ref=e53] [cursor=pointer]:
        - img [ref=e54]
        - generic [ref=e56]: Issues
        - generic [ref=e57]: "5"
    - group "Panels" [ref=e58]:
      - button "Inspector" [expanded] [ref=e60] [cursor=pointer]:
        - img [ref=e61]
      - button "Agent" [disabled] [ref=e64]:
        - img [ref=e65]
    - generic "Display units" [ref=e68]:
      - combobox "Display units" [ref=e69]:
        - option "Entered" [selected]
        - option "SI"
        - option "US"
    - group [ref=e70]:
      - generic "Appearance" [ref=e71] [cursor=pointer]:
        - img [ref=e72]
      - option "System" [selected]
      - option "Light"
      - option "Dark"
      - option "Comfortable" [selected]
      - option "Compact"
    - region "Human toolkit" [ref=e74]:
      - button "Find modeling commands" [ref=e75] [cursor=pointer]:
        - img [ref=e76]
        - generic [ref=e79]: Search or command…
        - generic "Command K" [ref=e80]: ⌘K
  - generic [ref=e81]:
    - navigation "Stages" [ref=e82]:
      - list [ref=e83]:
        - listitem [ref=e84]:
          - button "Model" [pressed] [ref=e85] [cursor=pointer]:
            - img [ref=e86]
            - generic [ref=e89]: Model
        - listitem [ref=e90]:
          - button "Loads" [ref=e91] [cursor=pointer]:
            - img [ref=e92]
            - generic [ref=e96]: Loads
        - listitem [ref=e97]:
          - button "Results" [disabled] [ref=e98]:
            - img [ref=e99]
            - generic [ref=e102]: Results
        - listitem [ref=e103]:
          - button "Review" [disabled] [ref=e104]:
            - img [ref=e105]
            - generic [ref=e109]: Review
      - separator [ref=e110]
      - list [ref=e111]:
        - listitem [ref=e112]:
          - button "Libraries" [ref=e113] [cursor=pointer]:
            - img [ref=e114]
            - generic [ref=e116]: Libraries
        - listitem [ref=e117]:
          - button "Rules" [ref=e118] [cursor=pointer]:
            - img [ref=e119]
            - generic [ref=e123]: Rules
        - listitem [ref=e124]:
          - button "Issues, 5" [ref=e125] [cursor=pointer]:
            - img [ref=e126]
            - generic [ref=e128]: Issues
            - generic [ref=e129]: "5"
    - region "Modeling workspace" [ref=e131]:
      - generic [ref=e132]:
        - group "Tables" [ref=e133]:
          - button "Model" [pressed] [ref=e134] [cursor=pointer]
          - button "Review changes" [ref=e135] [cursor=pointer]
          - button "Collapse table drawer" [disabled] [expanded] [ref=e138]:
            - img [ref=e139]
        - generic "Model tree" [ref=e143]:
          - generic [ref=e144]: Model
          - region "Layout grid mode" [ref=e145]:
            - button "Tree" [pressed] [ref=e146]:
              - img [ref=e147]
              - text: Tree
            - button "Grid" [ref=e150]:
              - img [ref=e151]
              - text: Grid
          - region "Model tree filtering" [ref=e153]:
            - generic [ref=e154]:
              - img [ref=e155]
              - generic [ref=e158]: Filter model
              - searchbox "Filter model tree" [ref=e159]
            - generic [ref=e160]: 2 of 2 model entities visible
            - button "Clear model tree filter" [disabled] [ref=e161]:
              - img [ref=e162]
          - tree "Model" [ref=e165]:
            - generic [ref=e166]:
              - treeitem "Blank Local Model project:blank-local-20260920t055758z" [level=1] [selected] [ref=e168] [cursor=pointer]:
                - img [ref=e169]
                - generic [ref=e173]:
                  - strong [ref=e174]: Blank Local Model
                  - generic [ref=e175]: project:blank-local-20260920t055758z
              - treeitem "Diagnostics" [expanded] [level=1] [ref=e177] [cursor=pointer]:
                - generic [ref=e178]: ▾
                - strong [ref=e179]: Diagnostics
              - treeitem "BLANK_PROJECT_AUTHORING_TARGET diagnostic:blank-project:authoring-target" [level=2] [ref=e181] [cursor=pointer]:
                - img [ref=e182]
                - generic [ref=e185]:
                  - strong [ref=e186]: BLANK_PROJECT_AUTHORING_TARGET
                  - generic [ref=e187]: diagnostic:blank-project:authoring-target
      - separator "Resize table and canvas" [ref=e188]
      - generic [ref=e190]:
        - generic [ref=e191]:
          - group "Viewport controls" [ref=e192]:
            - generic [ref=e193]: 3D Centerline
            - group "Viewport deformation overlay status" [ref=e194]:
              - generic "Deformation · unavailable" [ref=e195] [cursor=pointer]
            - group "Viewport display toggles" [ref=e196]:
              - button "Labels" [pressed] [ref=e197]
              - button "Loads" [pressed] [ref=e198]
              - button "Grid" [pressed] [ref=e199]
            - group "Viewport selection tools" [ref=e200]:
              - button "Box Select" [ref=e201]
              - generic [ref=e202]:
                - generic [ref=e203]: Selection filter
                - combobox "Selection filter" [ref=e204]:
                  - option "All" [selected]
                  - option "Pipes"
                  - option "Nodes"
                  - option "Supports"
                  - option "Components"
              - button "Hide" [disabled] [ref=e205]
              - button "Isolate" [disabled] [ref=e206]
              - button "Show All" [disabled] [ref=e207]
              - button "Fit Model" [ref=e208]
              - button "Fit Visible" [ref=e209]
              - button "Fit Selection" [disabled] [ref=e210]
            - group "Viewport geometry" [ref=e211]:
              - button "Schematic" [pressed] [ref=e212]
              - button "Actual OD" [ref=e213]
              - button "Measure" [ref=e214]
          - generic "Viewport status" [ref=e215]:
            - 'generic "Selected project: project:blank-local-20260920t055758z" [ref=e216]': "Selected: project:blank-local-20260920t055758z"
            - status "Schematic centerline geometry" [ref=e217]
            - status "View command status" [ref=e218]: No view command dispatched.
        - generic [ref=e219]:
          - generic "Three.js pipe centerline viewport" [ref=e220]
          - generic "Viewport entity selection"
          - img "Orientation gizmo showing X, Y, Z axes"
          - generic "View controls" [ref=e222]:
            - button "Front" [ref=e223] [cursor=pointer]
            - button "Top" [ref=e224] [cursor=pointer]
            - button "Isometric" [pressed] [ref=e225] [cursor=pointer]
          - generic:
            - generic: 1 m
        - region "Command and selection bar" [ref=e226]:
          - generic "Object creation tools" [ref=e227]:
            - button "Node" [pressed] [ref=e228] [cursor=pointer]:
              - img [ref=e229]
              - text: Node
            - button "Pipe" [ref=e231] [cursor=pointer]:
              - img [ref=e232]
              - text: Pipe
            - button "Support" [ref=e236] [cursor=pointer]:
              - img [ref=e237]
              - text: Support
            - button "Component" [ref=e240] [cursor=pointer]:
              - img [ref=e241]
              - text: Component
            - button "Load" [ref=e244] [cursor=pointer]:
              - img [ref=e245]
              - text: Load
          - 'generic "Node tool armed: click empty canvas to fill coordinates, then queue node." [ref=e247]': Node tool armed
          - group [ref=e248]:
            - generic "Selection & navigation" [ref=e249] [cursor=pointer]
      - generic [ref=e250]:
        - button "Close inspector" [ref=e251]:
          - img [ref=e252]
        - region "Viewport editor intents" [ref=e256]:
          - heading "Create node" [level=3] [ref=e257]
          - group [ref=e258]:
            - generic "Explicit node geometry" [ref=e259]:
              - generic [ref=e260]:
                - generic [ref=e261]: Node ID
                - textbox "New node ID" [ref=e262]:
                  - /placeholder: node:N-3
                  - text: node:R2-100
              - generic [ref=e263]:
                - generic [ref=e264]: Label
                - textbox "New node label" [ref=e265]:
                  - /placeholder: Node label
                  - text: R2 anchored node
              - generic [ref=e266]:
                - generic [ref=e267]: X
                - textbox "New node X coordinate" [ref=e268]:
                  - /placeholder: "0"
                  - text: "0"
              - generic [ref=e269]:
                - generic [ref=e270]: "Y"
                - textbox "New node Y coordinate" [ref=e271]:
                  - /placeholder: "0"
                  - text: "0"
              - generic [ref=e272]:
                - generic [ref=e273]: Z
                - textbox "New node Z coordinate" [ref=e274]:
                  - /placeholder: "0"
                  - text: "0"
              - generic [ref=e275]:
                - generic [ref=e276]: Coordinate unit
                - combobox "New node coordinate unit" [ref=e278] [cursor=pointer]:
                  - generic [ref=e279]: m
                  - text: ▾
              - generic [ref=e280]: "Coordinates: m, model metadata"
              - generic [ref=e281]: "Pointer plane: global XZ · Y=0 m"
              - generic "Click within 4 CSS pixels on the visible 3D canvas to capture on global XZ at Y=0." [ref=e282]
              - generic [ref=e283]:
                - generic [ref=e284]: Provenance
                - textbox "New node provenance" [active] [ref=e285]: invented_a12_rehearsal_user_input
              - button "Add node" [ref=e286] [cursor=pointer]:
                - img [ref=e287]
                - text: Add node
              - generic [ref=e289]: Ready to validate and freeze this explicit node create intent.
            - text: ▾ ▾ ▾ ▾ ▾ ▾
            - region "Route review" [ref=e290]:
              - heading "Review and Apply" [level=4] [ref=e291]
              - status [ref=e292]: The affected selection changed. Add again to review the current draft.
              - paragraph [ref=e293]: Add a complete node or route to generate the service validation and exact diff.
              - button "Apply" [disabled] [ref=e294]
            - group [ref=e295]:
              - generic "Unit source" [ref=e296] [cursor=pointer]
          - group [ref=e297]:
            - generic "Pending changes (0)" [ref=e298] [cursor=pointer]
        - region "Property inspector" [ref=e299]:
          - 'heading "Blank Local Model — project: project:blank-local-20260920t055758z" [level=2] [ref=e300]':
            - text: Blank Local Model
            - generic [ref=e301]: "— project: project:blank-local-20260920t055758z"
          - tablist "Inspector views" [ref=e302]:
            - tab "Properties" [selected] [ref=e303]
            - tab "Task" [ref=e304]
          - tabpanel [ref=e305]:
            - group [ref=e306]:
              - generic "All properties" [ref=e307] [cursor=pointer]
          - generic [ref=e308]:
            - group [ref=e309]:
              - generic "Sources and units" [ref=e310] [cursor=pointer]
            - group [ref=e311]:
              - generic "New support configuration" [ref=e312] [cursor=pointer]
              - text: ▾
            - group [ref=e313]:
              - generic "New section" [ref=e314] [cursor=pointer]
              - text: ▾ ▾
            - group [ref=e315]:
              - generic "New material" [ref=e316] [cursor=pointer]
              - text: ▾ ▾
            - group [ref=e317]:
              - generic "New support" [ref=e318] [cursor=pointer]
              - text: ▾
            - group [ref=e319]:
              - generic "New component" [ref=e320] [cursor=pointer]
              - text: ▾ ▾ ▾
    - complementary "Agent" [ref=e321]:
      - button "Agent" [disabled] [ref=e323]:
        - img [ref=e324]
        - generic [ref=e327]: Agent
  - generic "Workspace status" [ref=e328]:
    - generic "Analysis statuses" [ref=e329]:
      - button "Solver · Model incomplete" [ref=e331] [cursor=pointer]
    - button "5 Issues" [ref=e332] [cursor=pointer]:
      - img [ref=e333]
      - text: 5 Issues
    - generic "Selection" [ref=e335]: "project: project:blank-local-20260920t055758z"
    - generic "Display units" [ref=e336]: Entered
    - button "About SWBPIPE…" [ref=e337] [cursor=pointer]:
      - img [ref=e338]
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