# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: r2-smoke.spec.ts >> R2 desktop preview smoke covers solve, results, report, and viewport overlay
- Location: e2e/r2-smoke.spec.ts:265:1

# Error details

```
Error: expect(locator).toContainText(expected) failed

Locator: getByTestId('result-family-count-reaction')
Expected substring: "33"
Received string:    "29"
Timeout: 10000ms

Call log:
  - Expect "toContainText" with timeout 10000ms
  - waiting for getByTestId('result-family-count-reaction')
    23 × locator resolved to <strong data-testid="result-family-count-reaction">29</strong>
       - unexpected value "29"

```

```yaml
- strong: "29"
```

# Test source

```ts
  531 |   );
  532 |   await expect(page.getByTestId("delete-pipe-intent-panel")).toContainText("after=not_present");
  533 | 
  534 |   const canvas = page.locator(".viewport-canvas canvas");
  535 |   await expect(canvas).toBeVisible();
  536 |   await expect(page.getByTestId("viewport-editor-intents")).toHaveClass(/collapsed/);
  537 |   await ensureCreationToolArmed(page, "command-node", "Node tool armed");
  538 |   await openNamedDisclosure(page.getByTestId("viewport-editor-intents"), "Unit source");
  539 |   await expect(page.getByTestId("viewport-unit-catalog-status")).toContainText(
  540 |     "browser preview uses model metadata"
  541 |   );
  542 |   await expect(page.getByTestId("viewport-create-node-unit")).toHaveValue("m");
  543 |   await expect(page.getByTestId("viewport-create-node-unit-basis")).toContainText(
  544 |     "Coordinates: m, model metadata"
  545 |   );
  546 |   await canvas.click({ position: { x: 64, y: 64 } });
  547 |   await expect(page.getByTestId("viewport-create-node-id")).toHaveValue("node:V-001");
  548 |   await expect(page.getByTestId("viewport-create-node-label")).toHaveValue("Viewport node V-001");
  549 |   await expect(page.getByTestId("viewport-create-node-x")).toHaveValue(/^-?\d/);
  550 |   await expect(page.getByTestId("viewport-create-node-y")).toHaveValue("0");
  551 |   await expect(page.getByTestId("viewport-create-node-z")).toHaveValue(/^-?\d/);
  552 |   await expect(page.getByTestId("queue-explicit-node-intent")).toBeDisabled();
  553 |   await page.getByTestId("viewport-create-node-provenance").fill("invented_synthetic_ui_acceptance_input");
  554 |   await expect(page.getByTestId("queue-explicit-node-intent")).toBeEnabled();
  555 | 
  556 |   await ensureCreationToolArmed(page, "command-pipe", "Pipe tool armed");
  557 |   await expect(page.getByTestId("viewport-create-pipe-length-unit")).toHaveValue("m");
  558 |   await expect(page.getByTestId("viewport-create-pipe-unit-basis")).toContainText(
  559 |     "Pipe geometry: m, model metadata"
  560 |   );
  561 | 
  562 |   await ensurePipeEndpointPick(page, "viewport-pick-pipe-from");
  563 |   await page.getByTestId("viewport-select-node:N-100").click();
  564 |   await expect(page.getByTestId("viewport-create-pipe-from")).toHaveValue("node:N-100");
  565 |   await expect(page.getByTestId("viewport-pick-pipe-to")).toHaveAttribute("aria-pressed", "true");
  566 |   await page.getByTestId("viewport-select-node:N-140").click();
  567 |   await expect(page.getByTestId("viewport-create-pipe-to")).toHaveValue("node:N-140");
  568 |   await expect(page.getByTestId("viewport-pick-pipe-to")).toHaveAttribute("aria-pressed", "false");
  569 | 
  570 |   const before = await canvas.screenshot();
  571 |   expect(pngStats(before).uniqueColors).toBeGreaterThan(100);
  572 |   await expect(page.getByTestId("viewport-axis-triad")).toBeVisible();
  573 |   await expect(page.getByTestId("viewport-view-cube")).toBeVisible();
  574 |   await expect(page.getByTestId("viewport-scale-bar")).toContainText("m");
  575 |   await expect(page.getByTestId("command-bar")).toBeVisible();
  576 | 
  577 |   await openWorkspaceSection(page, "solve");
  578 |   await page.getByTestId("issues-drawer-toggle").click();
  579 |   await expect(page.getByTestId("missing-data-unit-policy")).toContainText("required=true");
  580 |   await expect(page.getByTestId("missing-data-unit-policy")).toContainText("default_units=false");
  581 |   await expect(page.getByTestId("missing-data-unit-policy")).toContainText("conversion=false");
  582 |   await page.getByTestId("issues-home").getByRole("button", { name: /Close/i }).click();
  583 |   await page.getByTestId("run-mechanics-preview").click();
  584 |   await expect(page.getByTestId("solve-job-summary")).toContainText("state=completed");
  585 |   await expect(page.getByTestId("solve-job-summary")).toContainText("result_rows=830");
  586 |   await expect(page.getByTestId("solve-job-unit-policy")).toContainText("model=angle=rad,force=N,length=m");
  587 |   await expect(page.getByTestId("solve-job-unit-policy")).toContainText("N*m/rad,N/m");
  588 |   await expect(page.getByTestId("solve-job-unit-policy")).toContainText("rows=830");
  589 |   await expect(page.getByTestId("solve-job-unit-policy")).toContainText("conversion=false");
  590 |   await setDisclosure(page.getByTestId("viewport-deformation-status"));
  591 |   await expect(page.getByTestId("viewport-deformation-status")).toContainText("available; nodes=5; max=4.927112 mm");
  592 |   await expect(page.getByTestId("viewport-deformation-boundary")).toContainText(
  593 |     "scale=normalized_display_offset_not_physical_length"
  594 |   );
  595 |   // TP-APP-R2-DEFORMEDDIR-001: the canned preview fixture now carries signed
  596 |   // global ux/uy/uz rows, so the overlay must disclose true directional
  597 |   // rendering instead of vector_direction=TBD.
  598 |   await expect(page.getByTestId("viewport-deformation-boundary")).toContainText(
  599 |     "vector_direction=global_cartesian_displacement_components"
  600 |   );
  601 |   await setDisclosure(page.getByTestId("viewport-deformation-status"), false);
  602 | 
  603 |   await page.getByTestId("audit-drawer-toggle").click();
  604 |   const auditDrawer = page.getByTestId("audit-boundary-drawer");
  605 |   await expect(auditDrawer).toBeVisible();
  606 |   await expect(auditDrawer.getByTestId("secret-private-library-unit-policy")).toContainText("unit_refs=2");
  607 |   await expect(auditDrawer.getByTestId("secret-private-library-unit-policy")).toContainText("required=true");
  608 |   await expect(auditDrawer.getByTestId("secret-private-library-unit-policy")).toContainText("payload=false");
  609 |   await expect(auditDrawer.getByTestId("secret-private-library-unit-policy")).toContainText("conversion=false");
  610 |   await expect(auditDrawer.getByTestId("run-audit-units")).toContainText("model=angle=rad,force=N,length=m");
  611 |   await expect(auditDrawer.getByTestId("run-audit-units")).toContainText("N*m/rad,N/m");
  612 |   await expect(auditDrawer.getByTestId("run-audit-units")).toContainText("rows=830");
  613 |   await expect(auditDrawer.getByTestId("run-audit-units")).toContainText("source=result_envelope");
  614 |   await expect(auditDrawer.getByTestId("run-audit-units")).toContainText("conversion=false");
  615 |   await auditDrawer.getByRole("button", { name: /Close/i }).click();
  616 |   await openWorkspaceSection(page, "solve");
  617 |   await expect(page.getByTestId("knowledge-unit-context")).toContainText("computed_unit_refs=2");
  618 |   await expect(page.getByTestId("knowledge-unit-context")).toContainText("units=N,mm");
  619 |   await expect(page.getByTestId("knowledge-unit-context")).toContainText("source=computed_preview_result");
  620 |   await expect(page.getByTestId("knowledge-unit-context")).toContainText("conversion=false");
  621 | 
  622 |   const solvedCanvas = await canvas.screenshot();
  623 |   expect(pngStats(solvedCanvas).uniqueColors).toBeGreaterThan(100);
  624 | 
  625 |   await openWorkspaceSection(page, "results");
  626 |   await expect(page.getByTestId("results-panel")).toBeVisible();
  627 |   await expect(page.getByTestId("result-unit-policy")).toContainText("MPa, N, N*m, mm, rad");
  628 |   await expect(page.getByTestId("result-unit-policy")).toContainText("830 rows");
  629 |   await expect(page.getByTestId("result-unit-policy")).toContainText("entered units preserved");
  630 |   await expect(page.getByTestId("result-filter-summary")).toContainText("830 of 830 results match filter");
> 631 |   await expect(page.getByTestId("result-family-count-reaction")).toContainText("33");
      |                                                                  ^ Error: expect(locator).toContainText(expected) failed
  632 |   await page.getByTestId("result-family-reaction").click();
  633 |   await expect(page.getByTestId("result-filter-summary")).toContainText("33 of 830 results match filter");
  634 |   await expect(page.getByTestId("result-page-summary")).toContainText(
  635 |     "Showing 1 to 33 of 33 matching results; page 1 of 1"
  636 |   );
  637 |   await expect(page.getByTestId("result-row-result:reaction:support-S-120")).toBeVisible();
  638 |   await page.getByTestId("result-family-all").click();
  639 |   await expect(page.getByTestId("result-filter-summary")).toContainText("830 of 830 results match filter");
  640 |   await page.getByTestId("result-filter-input").fill("pipe-P-120");
  641 |   await expect(page.getByTestId("result-filter-summary")).toContainText("170 of 830 results match filter");
  642 |   await expect(page.getByTestId("result-page-summary")).toContainText(
  643 |     "Showing 1 to 50 of 170 matching results; page 1 of 4"
  644 |   );
  645 |   await expectWorkspaceStatusClearOfTarget(page, "result-row-result:force:pipe-P-120:axial");
  646 |   await page.getByTestId("result-row-result:force:pipe-P-120:axial").click();
  647 |   await expect(page.getByTestId("result-detail-panel")).toContainText("pipe:P-120");
  648 |   await expect(page.getByTestId("result-detail-panel")).toContainText("recovered_from_local_element_stiffness");
  649 |   await expect(page.getByTestId("comparison-unit-policy")).toContainText("units=MPa,N,N*m,mm,rad");
  650 |   await expect(page.getByTestId("comparison-unit-policy")).toContainText("conversion=false");
  651 |   await expect(page.getByTestId("comparison-unit-policy")).toContainText("tolerance=not_tolerance_checked");
  652 |   await expect(page.getByTestId("design-workspace-units")).toContainText("N*m/rad,N/m");
  653 |   await expect(page.getByTestId("design-workspace-units")).toContainText("comparison=MPa,N,N*m,mm,rad");
  654 |   await expect(page.getByTestId("design-workspace-units")).toContainText("conversion=false");
  655 | 
  656 |   await openWorkspaceSection(page, "report");
  657 |   const report = page.getByLabel("Report packet");
  658 |   await expect(report.getByTestId("report-redaction-blocked")).toContainText(
  659 |     "Raw report DOM suppressed by redaction controls"
  660 |   );
  661 |   await expect(report.getByTestId("report-export-link")).toHaveAttribute("href", /data:application\/json/);
  662 |   await expect(report.getByTestId("report-packet-body")).toHaveCount(0);
  663 |   await expect(page.getByTestId("rendered-report-unit-basis")).toContainText(
  664 |     "unit_system=unit-system:dec-018-si-dual-display"
  665 |   );
  666 |   await expect(page.getByTestId("rendered-report-unit-basis")).toContainText("model=angle=rad,force=N,length=m");
  667 |   await expect(page.getByTestId("rendered-report-unit-basis")).toContainText("N*m/rad,N/m");
  668 |   await expect(page.getByTestId("rendered-report-unit-basis")).toContainText("conversion=false");
  669 |   await expect(page.getByTestId("report-package-private-intent")).not.toBeChecked();
  670 |   await page.getByTestId("report-package-save").click();
  671 |   await expect(page.getByTestId("report-package-save-status")).toContainText(
  672 |     "REPORT-PACKAGE-REDACTION-BLOCKED"
  673 |   );
  674 |   await page.getByTestId("report-package-private-intent").check();
  675 |   await page.getByTestId("report-package-save").click();
  676 |   await expect(page.getByTestId("report-package-redaction-summary")).toContainText(
  677 |     "route=DREP-PACKAGE-SAVE-009"
  678 |   );
  679 |   await expect(page.getByTestId("report-package-redaction-summary")).toContainText("blocked=false");
  680 |   await expect(page.getByTestId("report-package-save-status")).toContainText(
  681 |     "REPORT-PACKAGE-SAVE-DESKTOP-ONLY"
  682 |   );
  683 |   const reportLint = page.getByLabel("Report content lint");
  684 |   await expect(reportLint.getByTestId("report-lint-unit-policy")).toContainText("unit_targets=44");
  685 |   await expect(reportLint.getByTestId("report-lint-unit-policy")).toContainText(
  686 |     "conversion_witness_targets=2"
  687 |   );
  688 |   await expect(reportLint.getByTestId("report-lint-unit-policy")).toContainText("lint_conversion=false");
  689 |   const lintHref = await reportLint.getByTestId("report-lint-export-link").getAttribute("href");
  690 |   expect(lintHref).toBeTruthy();
  691 |   const lintPacket = JSON.parse(decodeURIComponent(lintHref!.split(",", 2)[1]));
  692 |   expect(lintPacket.unit_policy_evidence.unit_policy_target_count).toBe("[REDACTED]");
  693 |   expect(lintPacket.unit_policy_evidence.conversion_witness_target_count).toBe("[REDACTED]");
  694 |   expect(lintPacket.unit_policy_evidence.lint_performs_conversion).toBe("[REDACTED]");
  695 |   expect(lintPacket.unit_policy_evidence.lint_asserts_target_format_compatibility).toBe("[REDACTED]");
  696 |   expect(
  697 |     lintPacket.unit_policy_evidence.target_refs.every(
  698 |       (item: { source_path: string }) => item.source_path === "[REDACTED]"
  699 |     )
  700 |   ).toBe(true);
  701 |   expect(lintPacket.unit_policy_evidence.target_refs).toHaveLength(44);
  702 |   expect(
  703 |     lintPacket.unit_policy_evidence.target_refs.every(
  704 |       (item: { unit_policy_surface_id: string }) => item.unit_policy_surface_id === "[REDACTED]"
  705 |     )
  706 |   ).toBe(true);
  707 |   await openWorkspaceSection(page, "exports");
  708 |   const pcfExport = page.getByLabel("Conservative PCF export");
  709 |   await expect(pcfExport.getByTestId("pcf-export-conversion-witnesses")).toContainText("count=23");
  710 |   await expect(pcfExport.getByTestId("pcf-export-conversion-witnesses")).toContainText(
  711 |     "target_length=MM"
  712 |   );
  713 |   const caepipeMbfExport = page.getByLabel("CAEPIPE MBF export");
  714 |   await expect(caepipeMbfExport.getByTestId("caepipe-mbf-conversion-witnesses")).toContainText("count=15");
  715 |   await expect(caepipeMbfExport.getByTestId("caepipe-mbf-conversion-witnesses")).toContainText(
  716 |     "target_length=mm"
  717 |   );
  718 |   const caepipeExternalHarness = page.getByLabel("CAEPIPE external harness");
  719 |   await expect(caepipeExternalHarness.getByTestId("caepipe-external-units")).toContainText(
  720 |     "unit-system:dec-018-si-dual-display"
  721 |   );
  722 |   await expect(caepipeExternalHarness.getByTestId("caepipe-external-unit-witnesses")).toContainText(
  723 |     "count=3"
  724 |   );
  725 |   await expect(caepipeExternalHarness.getByTestId("caepipe-external-unit-witnesses")).toContainText(
  726 |     "conversion=false"
  727 |   );
  728 |   const stressNeutralExport = page.getByLabel("Stress-neutral CSV JSON export");
  729 |   await expect(stressNeutralExport.getByTestId("stress-neutral-unit-witnesses")).toContainText("count=830");
  730 |   await expect(stressNeutralExport.getByTestId("stress-neutral-unit-witnesses")).toContainText(
  731 |     "conversion=false"
```