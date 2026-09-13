# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: r2-smoke.spec.ts >> R2 from-blank GUI journey authors the A12 rehearsal script
- Location: e2e/r2-smoke.spec.ts:860:1

# Error details

```
Error: expect(locator).toBeEnabled() failed

Locator:  getByTestId('rendered-report-render')
Expected: enabled
Received: disabled
Timeout:  10000ms

Call log:
  - Expect "toBeEnabled" with timeout 10000ms
  - waiting for getByTestId('rendered-report-render')
    24 × locator resolved to <button disabled type="button" data-testid="rendered-report-render" title="Disabled: rendering needs a completed mechanics preview and its analysis-run record in this session. Open the Solve section and run the mechanics preview first.">Render report</button>
       - unexpected value "disabled"

```

```yaml
- button "Render report" [disabled]
```

# Test source

```ts
  956  | 
  957  |   const support = stepPayload("create_support", "support:R2-anchor");
  958  |   await page.getByTestId("command-support").click();
  959  |   await expect(page.getByTestId("create-support-id")).toBeFocused();
  960  |   await expect(page.getByTestId("create-support-id")).toBeVisible();
  961  |   await page.getByTestId("create-support-id").fill(support.id);
  962  |   await page.getByTestId("create-support-label").fill(support.label);
  963  |   await page.getByTestId("create-support-node").selectOption(support.node);
  964  |   for (const restraint of ["RX", "RY", "RZ"]) {
  965  |     await page.getByTestId(`create-support-restraint-${restraint}`).setChecked(true);
  966  |   }
  967  |   await page.getByTestId("create-support-provenance").fill(support.provenance);
  968  |   await page.getByTestId("queue-create-support-intent").click();
  969  |   await applyQueuedIntent(page, 3, 6, support.id);
  970  | 
  971  |   const loadCase = stepPayload("create_load_case", "load:R2-L-100");
  972  |   await openWorkspaceSection(page, "loads");
  973  |   await page.getByTestId("load-manager-create-load-id").fill(loadCase.id);
  974  |   await page.getByTestId("load-manager-create-load-label").fill(loadCase.label);
  975  |   await page.getByTestId("load-manager-create-load-kind").fill(loadCase.kind);
  976  |   await page.getByTestId("load-manager-create-load-status").fill(loadCase.status);
  977  |   await page.getByTestId("load-manager-create-load-provenance").fill(loadCase.provenance);
  978  |   await page.getByTestId("queue-create-load-case-intent").click();
  979  |   await applyQueuedIntent(page, 4, 7, loadCase.id);
  980  | 
  981  |   const primitive = stepPayload("create_primitive_load", "load:R2-L-100-FY");
  982  |   await openWorkspaceSection(page, "loads");
  983  |   await page.getByTestId("load-manager-create-primitive-load-case").selectOption(loadCase.id);
  984  |   await page.getByTestId("load-manager-create-primitive-category").selectOption(primitive.category);
  985  |   await page.getByTestId("load-manager-create-primitive-id").fill(primitive.id);
  986  |   await page.getByTestId("load-manager-create-primitive-node").selectOption(primitive.target.node);
  987  |   await page.getByTestId("load-manager-create-primitive-direction").selectOption(primitive.direction);
  988  |   await page.getByTestId("load-manager-create-primitive-magnitude").fill(String(primitive.magnitude.value));
  989  |   await page.getByTestId("load-manager-create-primitive-provenance").fill(primitive.provenance);
  990  |   await page.getByTestId("queue-create-primitive-intent").click();
  991  |   await applyQueuedIntent(page, 5, 8, primitive.id);
  992  | 
  993  |   const combination = stepPayload("create_combination", "combination:R2-C-100");
  994  |   await openWorkspaceSection(page, "loads");
  995  |   await page.getByTestId("load-manager-create-combination-id").fill(combination.id);
  996  |   await page.getByTestId("load-manager-create-combination-label").fill(combination.label);
  997  |   await page.getByTestId("load-manager-create-combination-load-case").selectOption(combination.terms[0].load_case);
  998  |   await page.getByTestId("load-manager-create-combination-factor").fill(String(combination.terms[0].factor));
  999  |   await page.getByTestId("load-manager-create-combination-provenance").fill(combination.provenance);
  1000 |   await page.getByTestId("load-manager-create-combination-rationale").fill("A8 GUI replay of the A12 invented rehearsal.");
  1001 |   await page.getByTestId("queue-create-combination-intent").click();
  1002 |   await applyQueuedIntent(page, 6, 9, combination.id);
  1003 | 
  1004 |   await openWorkspaceSection(page, "loads");
  1005 |   await expect(page.getByTestId("load-case-manager-summary")).toContainText(
  1006 |     "1 load cases; 1 primitive loads; 1 combinations"
  1007 |   );
  1008 |   await openWorkspaceSection(page, "project");
  1009 |   await page.getByRole("button", { name: "Save local" }).click();
  1010 |   await expect(page.getByTestId("local-project-message")).toContainText(
  1011 |     "Saved local browser-preview project snapshot without external file copies."
  1012 |   );
  1013 |   await page.getByRole("button", { name: "Open local" }).click();
  1014 |   await expect(page.getByTestId("local-project-message")).toContainText("Opened local browser-preview project snapshot.");
  1015 |   await expect(page.getByTestId("project-storage-unit-round-trip")).toContainText(
  1016 |     "status=unit_metadata_preserved_in_local_project_envelope"
  1017 |   );
  1018 |   await expect(page.getByTestId("project-storage-unit-round-trip")).toContainText("project.units.length=m");
  1019 |   await expect(page.getByTestId("project-storage-unit-round-trip")).toContainText("conversion=false");
  1020 |   await expect(page.getByTestId("project-validation-unit-round-trip")).toContainText(
  1021 |     "status=unit_metadata_preserved_in_local_project_envelope"
  1022 |   );
  1023 |   await expect(page.getByTestId("project-validation-unit-policy")).toContainText(
  1024 |     "round_trip=unit_metadata_preserved_in_local_project_envelope"
  1025 |   );
  1026 |   await expect(page.getByTestId("project-validation-unit-policy")).toContainText("conversion=false");
  1027 |   await page.getByTestId("audit-drawer-toggle").click();
  1028 |   const auditDrawer = page.getByTestId("audit-boundary-drawer");
  1029 |   await expect(auditDrawer).toBeVisible();
  1030 |   await expect(auditDrawer.getByTestId("validation-evidence-unit-policy")).toContainText("model=");
  1031 |   await expect(auditDrawer.getByTestId("validation-evidence-unit-policy")).toContainText("force=N");
  1032 |   await expect(auditDrawer.getByTestId("validation-evidence-unit-policy")).toContainText("length=m");
  1033 |   await expect(auditDrawer.getByTestId("validation-evidence-unit-policy")).toContainText(
  1034 |     "manual=unit_and_schema_verification"
  1035 |   );
  1036 |   await expect(auditDrawer.getByTestId("validation-evidence-unit-policy")).toContainText("conversion=false");
  1037 |   await expect(auditDrawer.getByTestId("accessibility-baseline-unit-visibility")).toContainText(
  1038 |     "policy=unit_bearing_values_keep_visible_unit_labels_in_review_surfaces"
  1039 |   );
  1040 |   await expect(auditDrawer.getByTestId("accessibility-baseline-unit-visibility")).toContainText("length=m");
  1041 |   await expect(auditDrawer.getByTestId("accessibility-baseline-unit-visibility")).toContainText("conversion=false");
  1042 |   await auditDrawer.getByRole("button", { name: /Close/i }).click();
  1043 |   await openWorkspaceSection(page, "solve");
  1044 |   await page.getByTestId("run-mechanics-preview").click();
  1045 |   await expect(page.getByTestId("solve-job-summary")).toContainText("state=completed");
  1046 |   await expect(page.getByTestId("solve-job-summary")).toContainText("result_rows=0");
  1047 |   await expect(page.getByTestId("solve-job-unit-policy")).toContainText("results=none");
  1048 |   await expect(page.getByTestId("solve-job-unit-policy")).toContainText("rows=0");
  1049 |   await expect(page.getByTestId("solve-job-unit-policy")).toContainText("conversion=false");
  1050 |   await page.getByTestId("issues-drawer-toggle").click();
  1051 |   await expect(page.getByTestId("diagnostic-BROWSER_SOLVE_BACKEND_REQUIRED_FOR_EDITED_MODEL")).toContainText(
  1052 |     "BROWSER_SOLVE_BACKEND_REQUIRED_FOR_EDITED_MODEL"
  1053 |   );
  1054 |   await page.getByTestId("issues-home").getByRole("button", { name: /Close/i }).click();
  1055 |   await openWorkspaceSection(page, "report");
> 1056 |   await expect(page.getByTestId("rendered-report-render")).toBeEnabled();
       |                                                            ^ Error: expect(locator).toBeEnabled() failed
  1057 |   await expect(page.getByTestId("rendered-report-unit-basis")).toContainText(
  1058 |     "unit_system=unit-system:dec-018-si-dual-display"
  1059 |   );
  1060 |   await expect(page.getByTestId("rendered-report-unit-basis")).toContainText("length=m");
  1061 |   await expect(page.getByTestId("rendered-report-unit-basis")).toContainText("results=none");
  1062 |   await expect(page.getByTestId("rendered-report-unit-basis")).toContainText("conversion=false");
  1063 |   await expectWorkspaceStatusClearOfTarget(page, "rendered-report-render");
  1064 |   await page.getByTestId("rendered-report-render").click();
  1065 |   await expect(page.getByTestId("rendered-report-redaction-summary")).toContainText("blocked=true");
  1066 |   await expect(page.getByTestId("rendered-report-route")).toHaveCount(0);
  1067 |   await expect(page.getByTestId("rendered-report-preview")).toHaveCount(0);
  1068 | });
  1069 | 
  1070 | test("diagnostic detail exposes linked result unit context", async ({ page }) => {
  1071 |   await page.goto("/");
  1072 |   await expect(page.getByTestId("desktop-preview-shell")).toBeVisible();
  1073 |   await openWorkspaceSection(page, "solve");
  1074 | 
  1075 |   await page.getByTestId("run-mechanics-preview").click();
  1076 |   await page.getByTestId("issues-drawer-toggle").click();
  1077 |   await page.getByTestId("diagnostic-filter-input").fill("result:stress:pipe-P-130");
  1078 |   const diagnosticButton = page.getByTestId("diagnostic-COMBINATION_STRESS_SUMMARY_SKIPPED");
  1079 |   await expect(diagnosticButton).toBeVisible();
  1080 |   await diagnosticButton.click();
  1081 | 
  1082 |   await expect(page.getByTestId("selected-diagnostic-linked-results")).toContainText("result:stress:pipe-P-130");
  1083 |   await expect(page.getByTestId("diagnostic-unit-context")).toContainText("linked_results=1");
  1084 |   await expect(page.getByTestId("diagnostic-unit-context")).toContainText("units=MPa");
  1085 |   await expect(page.getByTestId("diagnostic-unit-context")).toContainText("source=result_envelope");
  1086 |   await expect(page.getByTestId("diagnostic-unit-context")).toContainText("conversion=false");
  1087 | });
  1088 | 
  1089 | // Phase C2 slice 1 (TP-C2-EDITOR-001): the rule-pack manager authors a
  1090 | // private draft in memory and reports the honest desktop-only seam for
  1091 | // validation, checksum, persistence, and listing in browser mode — the
  1092 | // same recorded boundary pattern as report rendering and the unit catalog.
  1093 | test("rule-pack manager drafts privately and reports the desktop-only backend seam", async ({ page }) => {
  1094 |   await page.goto("/");
  1095 |   await expect(page.getByTestId("desktop-preview-shell")).toBeVisible();
  1096 |   await ensureEngineReady(page);
  1097 | 
  1098 |   await openWorkspaceSection(page, "rule-packs");
  1099 |   await expect(page.getByTestId("rule-pack-scope-status")).toContainText("local SQLite only");
  1100 |   await expect(page.getByTestId("rule-pack-boundary-note")).toContainText("DEC-037");
  1101 | 
  1102 |   await page.getByTestId("rule-pack-new-draft").click();
  1103 |   const draftText = await page.getByTestId("rule-pack-draft-json").inputValue();
  1104 |   const draft = JSON.parse(draftText);
  1105 |   expect(draft.rule_pack_kind).toBe("open_pipe_stress_rule_pack");
  1106 |   expect(draft.grammar_version).toBe("1.0.0");
  1107 |   expect(draft.classification.privacy_class).toBe("private_user_data");
  1108 |   expect(draft.classification.redistribution_status).toBe("private_only");
  1109 |   await expect(page.getByTestId("rule-pack-action-status")).toContainText("private_user_data");
  1110 | 
  1111 |   // Slice 2 (TP-C2-COMPOSER-001) + DEC-037 follow-up: the structured AST
  1112 |   // composer rewrites the selected formula's expression through visible
  1113 |   // controls. It may render display-only text, but ships no writable text
  1114 |   // syntax and no parser. Switching the root node type rewrites the canonical
  1115 |   // document JSON the validate/save flow reads.
  1116 |   await expect(page.getByTestId("rule-pack-expression-composer")).toBeVisible();
  1117 |   await expect(page.getByTestId("rule-pack-variable-browser")).toContainText("user_required_input_1");
  1118 | 	  await expect(page.getByTestId("rule-pack-expression-text-preview")).toContainText(
  1119 | 	    "Read-only AST-to-text preview"
  1120 | 	  );
  1121 | 	  await expect(page.getByTestId("rule-pack-expression-text-preview")).toContainText(
  1122 | 	    "user_required_input_1"
  1123 | 	  );
  1124 |   expect(draft.formula_declarations[0].declaration_payload.expression_ast.node).toBe("variable_ref");
  1125 | 
  1126 |   // Slice 4 (TP-C2-DECLEDITOR-001): the declarations editor authors the
  1127 |   // required_inputs / value_slots the composer's variable_ref binds to. Add a
  1128 |   // required input from blank; the canonical document JSON grows and the
  1129 |   // composer's variable picker reflects the new id — no raw JSON. Still
  1130 |   // structured-only (D-02b).
  1131 |   await expect(page.getByTestId("rule-pack-declarations-editor")).toBeVisible();
  1132 |   await page.getByTestId("rule-pack-input-add").click();
  1133 |   // TP-UNITS-B2-RULEPACKUNITS-001: browser preview cannot call the desktop
  1134 |   // get_unit_catalog command, so declaration unit refs stay editable as stored
  1135 |   // unit text here. The desktop catalog-picker path is covered by mocked-Tauri
  1136 |   // Vitest; this e2e assertion protects the no-fallback/manual-entry route.
  1137 |   await page.getByTestId("rule-pack-input-dimension").last().selectOption("stress");
  1138 |   await page.getByTestId("rule-pack-input-unit").last().fill("MPa");
  1139 |   await expect(page.getByTestId("rule-pack-declarations-unit-policy")).toContainText(
  1140 |     "catalog_route=browser_preview_manual_entry"
  1141 |   );
  1142 |   await expect(page.getByTestId("rule-pack-declarations-unit-policy")).toContainText(
  1143 |     "required_input:user_required_input_2=model_metadata_unit_dimension_declared_catalog_unavailable_browser_preview(unit=MPa;dimension=stress)"
  1144 |   );
  1145 |   const declText = await page.getByTestId("rule-pack-draft-json").inputValue();
  1146 |   expect(JSON.parse(declText).required_inputs).toHaveLength(2);
  1147 |   expect(JSON.parse(declText).required_inputs[1].quantity_intent).toMatchObject({
  1148 |     dimension: "stress",
  1149 |     unit_ref: "MPa"
  1150 |   });
  1151 |   await expect(page.getByTestId("rule-pack-variable-browser")).toContainText(
  1152 |     "user_required_input_2 (required_input)"
  1153 |   );
  1154 |   await page.getByTestId("rule-pack-node-type").first().selectOption("compare");
  1155 |   // TP-UNITS-B2-RULEEXPRUNITS-001: browser preview cannot call the desktop
  1156 |   // catalog command for expression literal/table units either, so expression
```