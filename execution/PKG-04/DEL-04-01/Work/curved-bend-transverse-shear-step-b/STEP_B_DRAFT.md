# Step B draft: admitting `timoshenko_annulus_energy_curved_v2`

DRAFT for Agent 0's review. Nothing here is written to any repository. Base:
`f18834a6b` on `codex/bend-transverse-shear` (Step A implemented). Governing design:
`execution/PKG-04/DEL-04-01/Work/curved-bend-transverse-shear.md` §§1, 3.3–3.4, 5–7, 9.
Every identity name, profile, contract treatment and UI string below is a proposal.

**Size.** The drafted diff covers 27 files in 86 hunks (`step-b.diff`,
reproducible with `python3 -I make_diff.py <checkout>`). §2.8 lists further consumer
sites, each with an exact anchor and a one-line rule, which are not drafted line by line
because the pressure-elongation unit rewrites them first (finding F-1).

## 0. Findings that shape the diff

- **F-1. Main's exact identity moves before Step B lands.** The pressure-elongation unit
  (`origin/claude/pressure-elongation`, design `execution/PKG-04/DEL-04-07/Work/pressure-elongation.md`
  §5) retires `pressure-2` (`SOURCE_PRESSURE_2_RETIRED`) and publishes `pressure-3`, re-keying
  every pressure reader, the pressure table and its retained digests. So the EX v2 identity must
  be derived from `pressure-3`, not `pressure-2`. Every EX hunk here (exact_admission,
  semantic_contract, reader_verdict, physics_evidence, generator) is written against
  `pressure-2` at `f18834a6b` and must be re-derived with the same rule. That branch also edits
  `model_settings.rs`, `ModelSettingsSection.tsx`, `modelSettingsIntents.ts`, `types.ts`, the
  operation applier's settings authoring and `schemas/model.schema.yaml`, so those hunks will
  conflict textually and need rebasing.
- **F-2. The EX identity is much larger than the PF one.** PF already keys readers by a list
  (`PREVIEW_PHYSICS_IDENTITIES`), so PF v2 is mostly a third list entry. EX has a single
  `PRESSURE_ID` used across result_export, report_package, the headless runner, the Tauri
  report renderer, analysis-run compatibility, stress-neutral export, the Python verifiers
  and the desktop. It also has bound schemas: `results_pressure_2.v0.3.schema.json`
  (~l.226–240) and `results.v0.3.schema.yaml` `PhysicsGeometry` (~l.2402–2414) force a realized
  arc's `beam_theory` to Euler–Bernoulli. `results_pressure_2` and
  `physics_source_recovery.schema.json` are also bound in the registered retained profile's
  `reviewed_inputs` (`retained_memory.rs:1467`). The draft therefore adds a derived EX schema
  and table and never edits the bound files (§3).
- **F-3. A source decline is needed and is not in design §5.** Without it, an EX v2 document
  with no arcs could select the straight source method and publish `pressure-source-1`, whose
  evidence schema (bound) admits only EB/v1 `beam_theory`. The draft declines v2 by name in
  `source_recovery.rs`.
- **F-4. No retained-route edit is needed.** The D1 namespace already refuses v2 on both
  branches without touching `retained_memory.rs`. On L, `pf_timoshenko::selects` now covers v2,
  giving `FamilyFact::ShearModel`. On E, `contract_is` admits only EB|v1 (`retained_memory.rs:861`),
  giving `FamilyFact::PressureContract`. The pricing D3 still has to restate pins (§6).
- **F-5. The design's Step B deliverable list is incomplete** (`curved-bend-transverse-shear.md:249`
  names DEL-03-03, DEL-05-03, DEL-02-01 and DEL-04-07). By `tools.deliverables`' own matching
  rule (§5), the change also reaches DEL-05-02, DEL-04-08, DEL-04-09, DEL-08-01/04/06,
  DEL-07-03/05/11, DEL-04-06, DEL-14-02, DEL-17-06 and DEL-13-04. DEL-03-03 (bend component
  fields) has no file in this diff.

## 1. Surface inventory

Searched with `rg` over `core/ apps/ schemas/ tools/ tests/ fixtures/` for `euler_bernoulli`,
`timoshenko_annulus_energy_v1`, `ShearModelSetting`, `selects_timoshenko`,
`pf_timoshenko::selects`, the identity IDs, profiles, table names and hashes, `ShearModel` and
`SHEAR_MODELS`. Fixture and recorded-result bytes are excluded: they keep EB/v1 bytes by rule.
"Hunk" means drafted in §2; "Rule" means §2.8; "No" gives the reason.

| # | Surface | v2? | Why |
|---|---|---|---|
| 1 | PP `model_settings.rs` `ShearModelSetting`, decoder, refusal, accessor | Hunk | The vocabulary gains the value. |
| 2 | PP `validation.rs` `validate_model_settings` | No | Calls `shear_model_refusal`, which follows #1. |
| 3 | PP `pf_timoshenko.rs` `selects`, identity, basis, `shear_area`, partial span, arc disclosure | Hunk | PF selection, identity and messages. |
| 4 | PP `pressure_runtime.rs` `selects_timoshenko`, `validate_profile`, region geometry evidence | Hunk | EX vocabulary and selection. |
| 5 | PP `exact_admission.rs` identity constants and v3 limitation 0 | Hunk | EX v2 identity/profile/limitation 0. |
| 6 | PP `lib.rs` `mechanics_producer_for_model`, `formulation_basis_for_model_with_source` | Hunk | Identity and basis selection. |
| 7 | PP `lib.rs` `build_model_for_basis` (A_s), `build_curved_bend_macro_elements` | Hunk | Arc A_s by route rule; `with_transverse_shear_area`. |
| 8 | PP `lib.rs` `exact_section_evidence`/`exact_pipe_sections` (`beam_theory`, ~12203) | Hunk | Arc record and v2 model name. |
| 9 | PP `lib.rs` `curved_formation_of` | No | Step A already copies `transverse_shear_area`. |
| 10 | PP `source_receipt/composite.rs:554` | Hunk | Caller of the changed signature. |
| 11 | PP `source_recovery.rs` straight-family declines | Hunk | F-3. |
| 12 | PP `modal/prepare.rs` | Hunk | Modal v2 refusal naming v2. |
| 13 | PP modal re-formation (`modal_eigen` `reformation/mod.rs`) | No | Passes `None`; modal refuses v2 before it. |
| 14 | PP `retained_memory.rs` D1 `contract_is`, `family_clauses` | No | F-4 (refuses v2 unchanged). |
| 15 | PP `retained_product.rs:2986` beam-model check | No | v2 never reaches W1 (F-4). |
| 16 | PP `bend_pressure.rs` `BEND_LIMITATION` | No | Still true under v2. pressure-elongation revises it anyway. |
| 17 | PP `model_format.rs`, desktop `model_document_version.rs` | No | Test fixtures naming `euler_bernoulli`. |
| 18 | NI `structural_adapter.rs` CurvedFormation sites | No | Step A copies A_s. |
| 19 | FK `retained/assemble.rs:311` v1 refusal text | No | Kernel retained path, unreachable for v2. |
| 20 | RE `semantic_contract.rs` IDs, lists, loaders, dispatch | Hunk | Both identities. |
| 21 | RE `section_basis.rs` `Beam`, `beam_theory`, `selected_model`, `bind_shear_model` | Hunk | v2 records, arcs included. |
| 22 | RE `physics_evidence.rs` section loop | Hunk | Identity ⇔ v2 binding. |
| 23 | RE `arc_evidence.rs:122` arc EB requirement | Hunk | Admit v2 arc record. |
| 24 | RE `reader_verdict.rs` BINDINGS, `route`, `bind_preview_shear_model`, `physics_model_checks` | Hunk | Both identities; crossed-pair refusals. |
| 25 | RE `physics_source.rs` (pressure-source-1 reader, l.38, 1107) | No | v2 is declined from source (#11). |
| 26 | RE `retained_precision.rs` (l.4580, 4948, 5443) | No | Retained routes refuse v2 (F-4). |
| 27 | RE `derivative.rs:161`, `recorded_tables.rs`, `preview_physics_evidence.rs`, `u4b_evidence.rs`, `connector_evidence.rs`, `load_reference.rs` | Rule | Identity lists/table selection (§2.8). |
| 28 | `modal_result/src/model_mass.rs:83` | Hunk | Classes v2 as a modal refusal. |
| 29 | `report_package` `document_form.rs`, `exact_sections.rs`, `user_release.rs`, `pf_timoshenko.rs`, `bindings.rs` | Rule | Report admission and limitation-0 projection (§2.8). |
| 30 | headless `lib.rs:1526`, `result_envelope_binding.rs` | Rule | Identity lists (tests mostly). |
| 31 | Tauri `report_render_exact.rs:72,76`, `analysis_run_v03_validate.rs`, `analysis_run_v03_build.rs:714`, `lib.rs` | Rule | Native render and analysis-run admission. |
| 32 | `core/analysis_runs/compatibility.py`, `core/handoff/stress_neutral/package.py` | Rule | Record builder and export method lists. |
| 33 | `tests/verifiers/{compatibility,reader_verdict,section_basis,physics_evidence,pressure_2,preview_physics_evidence,modal_model_mass}.py` | Rule | Python twins of #20–#28. |
| 34 | `tests/verifiers/physics_source.py:52`, `retained_precision.py` | No | Source and retained exclude v2. |
| 35 | `tools/serialization/semantic_contract_tables.py` | Hunk | Derives both tables; PF schema limitations. |
| 36 | `tools/validation/qualification_*.py`, `qualification_gate.py`, `generate_product_preview_mechanics.mjs` | No | EB-only witness dispatch, by design (as for PF v1 in the inventory guard). |
| 37 | `schemas/model.schema.yaml` (both enums) | Hunk | Vocabulary. |
| 38 | `schemas/physical_transform_model.schema.yaml` | Hunk | Twin of `PressureContract`. |
| 39 | `schemas/results.v0.3.schema.yaml` PF branch | Hunk | PF identity branch. |
| 40 | `schemas/results.v0.3.schema.yaml` EX `oneOf`/`contract_evidence anyOf`, `PhysicsGeometry`; new `results_pressure_curved_shear_1.v0.3.schema.json` | Rule / generated | F-2; derived, never hand-edited (§3). |
| 41 | `schemas/stress_neutral_export.v0.3.schema.json` | Rule | Twin ids, profiles, hashes, `PhysicsBeamTheory` variant. |
| 42 | `schemas/analysis_run.v0.3.schema.json` (l.254–301) | Rule | Admitted ids and hashes. |
| 43 | `schemas/physics_source_recovery.schema.json`, `results_pressure_2.v0.3.schema.json` | No | Bound reviewed inputs; v2 never publishes there. |
| 44 | Operation applier `lib.rs:929` `is_pressure_v3_contract`, remediation l.6007 | Hunk | v2 contracts admitted. |
| 45 | Operation applier `pressure_authoring.rs:299` | Hunk | Refusal lists v2. |
| 46 | Operation applier `model_settings_authoring.rs`, `document_validator.rs` | No | Delegate to `shear_model_refusal`. |
| 47 | Desktop `types.ts`, `pressureContract.ts`, `PressureAuthoringPanel.tsx` | Hunk / No | Types and choices; the panel reads `SHEAR_MODEL_CHOICES`, so it needs no hunk. |
| 48 | Desktop `ModelSettingsSection.tsx`, `modelSettingsIntents.ts` | Hunk | Offer, label, note. |
| 49 | Desktop `numericalResultQuality.ts`, `knownSemanticLimitations.ts` | Hunk | DEL-02-01 reader identities. |
| 50 | Desktop `StressNeutralExportPanel.tsx`, `analysisRunCompatibility.ts`, `recordedPreRtA4.ts`, `nativeResult.ts`, report `exactReportAdmission.ts`, `exactReportSections.ts`, `reportPackageRequest.ts`, `renderableReportInput.ts`, `pfTimoshenkoReport.ts`, `exactReportLimitations.ts` | Rule | Identity lists, tables, limitation statements. |
| 51 | Desktop `projectService.ts:496` blank load-state contract (v1) | No | Default stays v1. |
| 52 | MBF writer `writer/mod.rs:32`, `read.rs:100` `SHEAR_MODELS` | No | Visibly refuses a v2 exact document (`MBF-SI-PRESSURE-UNSUPPORTED`). |
| 53 | MBF `read.rs:1325` `MBF-SI-MODEL-SETTINGS-NOT-EXPORTED` | No | v2 settings export with this loss, as v1. |
| 54 | MBF `sidecar.rs:27`, importer `mod.rs:475`, `importer/pressure.rs:187`, Python `importer.py:102`, `si_writer.py:104` | No | The caller's `shear_model` option is checked against EB/v1 (`MBF-IMPORT-SHEAR-MODEL-INVALID`), so import never produces v2. |
| 55 | `fixtures/results/semantic_contract_v0_3_*` new tables | Generated | §3. |
| 56 | `fixtures/product_preview/*_fixture_generation.json` | Generated | They pin the schema and source hashes. |
| 57 | `fixtures/reports/exact_route_limitations.json`, `exact_report_statements.json` | Generated / Rule | EX v2 limitation 0 statement. |
| 58 | `tools/release/schema_manifest.json` | Generated | Schema hashes. |
| 59 | `tests/test_preview_identity_inventory.py` | Rule | Third identity in the guard; add a matching EX guard. |
| 60 | `tools/retained_pricing/**`, `build_identity.rs`, `retained_memory.rs` `reviewed_inputs` | D3 only | §6; never edited outside a pricing turn. |

## 2. Proposed hunks

All hunks are against `f18834a6b`, generated by `make_diff.py` from `git show f18834a6b:<path>` copies, so each applies exactly there. `diff -u` line numbers are the anchors; each hunk's first context lines name the function. The whole diff is `step-b.diff` beside this file.

### 2.1 Product physics (DEL-04-07; DEL-04-09 for modal; DEL-04-08 for source recovery)

**`core/product_physics/src/model_settings.rs`** (7 hunks). `model_settings.shear_model` gains the third name; the accessor returns it; the unknown-value refusal lists three names (a refusal text change, see §7 O-6).

```diff
diff --git a/core/product_physics/src/model_settings.rs b/core/product_physics/src/model_settings.rs
--- a/core/product_physics/src/model_settings.rs
+++ b/core/product_physics/src/model_settings.rs
@@ -19,6 +19,9 @@
 //! differs. `timoshenko_annulus_energy_v1` selects Timoshenko straight members on the
 //! pressure-free route (DEL-05-03 Design `pf-timoshenko.md`; `crate::pf_timoshenko`),
 //! which removed 0.8.0's interim refusal at solve and at authoring.
+//! `timoshenko_annulus_energy_curved_v2` (DEL-04-01 Design `curved-bend-transverse-shear.md`
+//! §1, §5) forms the straight members exactly as v1 does and adds transverse shear to every
+//! realized arc. It is never inferred: v1 keeps its meaning, and only an explicit set selects v2.
 //!
 //! The topology check (`topology::append_topology_diagnostics`) reads it through
 //! [`PreviewModel::node_proximity_tolerance_m`]. The solve's validation
@@ -48,17 +51,20 @@
 pub const SHEAR_MODEL_EULER_BERNOULLI: &str = "euler_bernoulli";
 /// `timoshenko_annulus_energy_v1`: Timoshenko straight members on a pressure-free document.
 pub const SHEAR_MODEL_TIMOSHENKO: &str = "timoshenko_annulus_energy_v1";
-/// The value is not one of the two strings (a non-string and `null` included).
+/// `timoshenko_annulus_energy_curved_v2`: v1's straight members, and realized arcs with transverse shear.
+pub const SHEAR_MODEL_TIMOSHENKO_CURVED: &str = "timoshenko_annulus_energy_curved_v2";
+/// The value is not one of the three strings (a non-string and `null` included).
 pub const MODEL_SETTING_SHEAR_MODEL_UNKNOWN: &str = "MODEL-SETTING-SHEAR-MODEL-UNKNOWN";
 /// The member is present on a document that declares `pressure_contract`.
 pub const MODEL_SETTING_SHEAR_MODEL_WITH_PRESSURE_CONTRACT: &str = "MODEL-SETTING-SHEAR-MODEL-WITH-PRESSURE-CONTRACT";
 
-/// The authored `model_settings.shear_model`, as decoded: one of the two names, or any other
+/// The authored `model_settings.shear_model`, as decoded: one of the three names, or any other
 /// JSON value (a non-string and `null` included), which is refused by name rather than at decode.
 #[derive(Debug, Clone, Copy, PartialEq, Eq)]
 pub enum ShearModelSetting {
     EulerBernoulli,
     TimoshenkoAnnulusEnergyV1,
+    TimoshenkoAnnulusEnergyCurvedV2,
     Unknown,
 }
 
@@ -68,6 +74,7 @@
         match value.as_str() {
             Some(SHEAR_MODEL_EULER_BERNOULLI) => ShearModelSetting::EulerBernoulli,
             Some(SHEAR_MODEL_TIMOSHENKO) => ShearModelSetting::TimoshenkoAnnulusEnergyV1,
+            Some(SHEAR_MODEL_TIMOSHENKO_CURVED) => ShearModelSetting::TimoshenkoAnnulusEnergyCurvedV2,
             _ => ShearModelSetting::Unknown,
         }
     }
@@ -88,6 +95,7 @@
             Ok(match value {
                 SHEAR_MODEL_EULER_BERNOULLI => ShearModelSetting::EulerBernoulli,
                 SHEAR_MODEL_TIMOSHENKO => ShearModelSetting::TimoshenkoAnnulusEnergyV1,
+                SHEAR_MODEL_TIMOSHENKO_CURVED => ShearModelSetting::TimoshenkoAnnulusEnergyCurvedV2,
                 _ => ShearModelSetting::Unknown,
             })
         }
@@ -121,9 +129,9 @@
     deserializer.deserialize_any(ShearModelVisitor).map(Some)
 }
 
-/// The refusal message for a value that is not one of the two names.
+/// The refusal message for a value that is not one of the three names.
 pub fn shear_model_unknown_message() -> String {
-    format!("{SHEAR_MODEL_PATH} must be `{SHEAR_MODEL_EULER_BERNOULLI}` or `{SHEAR_MODEL_TIMOSHENKO}`.")
+    format!("{SHEAR_MODEL_PATH} must be `{SHEAR_MODEL_EULER_BERNOULLI}`, `{SHEAR_MODEL_TIMOSHENKO}` or `{SHEAR_MODEL_TIMOSHENKO_CURVED}`.")
 }
 
 /// The refusal message for the member on a document that declares `pressure_contract`.
@@ -132,8 +140,8 @@
 }
 
 /// The refusals of an authored setting, at solve, at authoring and in the document
-/// validator alike: a value other than the two names, then the member on a document
-/// that declares `pressure_contract`. Both names are admitted on a pressure-free document.
+/// validator alike: a value other than the three names, then the member on a document
+/// that declares `pressure_contract`. Every name is admitted on a pressure-free document.
 pub fn shear_model_refusal(setting: ShearModelSetting, has_pressure_contract: bool) -> Option<ModelSettingError> {
     if setting == ShearModelSetting::Unknown {
         return Some(ModelSettingError { code: MODEL_SETTING_SHEAR_MODEL_UNKNOWN, message: shear_model_unknown_message() });
@@ -244,10 +252,11 @@
 
     /// The effective shear-model choice: the authored `model_settings.shear_model`, else
     /// Euler–Bernoulli. An unknown value reads as Euler–Bernoulli here and is refused by
-    /// [`shear_model_refusal`]; `crate::pf_timoshenko::selects` consumes the choice.
+    /// [`shear_model_refusal`]; `crate::pf_timoshenko::selects` and `selects_curved` consume the choice.
     pub fn shear_model(&self) -> ShearModelSetting {
         match self.model_settings.as_ref().and_then(|settings| settings.shear_model) {
             Some(ShearModelSetting::TimoshenkoAnnulusEnergyV1) => ShearModelSetting::TimoshenkoAnnulusEnergyV1,
+            Some(ShearModelSetting::TimoshenkoAnnulusEnergyCurvedV2) => ShearModelSetting::TimoshenkoAnnulusEnergyCurvedV2,
             _ => ShearModelSetting::EulerBernoulli,
         }
     }
```

**`core/product_physics/src/pf_timoshenko.rs`** (10 hunks). PF selection: `selects` covers v1 and v2 (identical straights), `selects_curved` is new; v2 identity/profile/limitation 0; diagnostics name the selected model (v1 bytes unchanged); arc disclosure suppressed under v2.

```diff
diff --git a/core/product_physics/src/pf_timoshenko.rs b/core/product_physics/src/pf_timoshenko.rs
--- a/core/product_physics/src/pf_timoshenko.rs
+++ b/core/product_physics/src/pf_timoshenko.rs
@@ -11,6 +11,12 @@
 //! Realized arcs stay
 //! Euler–Bernoulli and are disclosed by name (§4).
 //!
+//! `timoshenko_annulus_energy_curved_v2` (DEL-04-01 Design `curved-bend-transverse-shear.md`
+//! §1, §5) forms the same straight members bit for bit, and each realized arc on a consumed
+//! basis also takes A_s from this module's [`shear_area`] (the arc's own PF annulus, area and
+//! unscaled moduli); its result has its own identity, profile and limitation 0, and no arc
+//! disclosure.
+//!
 //! Admission follows consumption (§2): the bases the cases consume are known
 //! from their temperature selections before any build, so a factor failure or
 //! a ν warning belongs to a consumed basis and names the cases that select it.
@@ -39,6 +45,13 @@
 /// §4: the named disclosure of the realized arcs that stay Euler–Bernoulli.
 pub(crate) const REALIZED_ARCS_EULER_BERNOULLI: &str = "PF_TIMOSHENKO_REALIZED_ARCS_EULER_BERNOULLI";
 
+/// DEL-04-01 `curved-bend-transverse-shear.md` §5: the curved-shear PF result's semantic contract identity.
+pub(crate) const CURVED_ID: &str = "openpipestress.result_semantics/0.3.0/preview-physics-curved-shear-1";
+/// The curved-shear PF result's profile.
+pub(crate) const CURVED_PROFILE_ID: &str = "product_preview_mechanics_curved_shear_v2";
+/// Limitation 0 of the curved-shear PF identity; the other limitations are preview-physics-2's.
+pub(crate) const CURVED_LIMITATION_0: &str = "Small-displacement, linear-elastic frame preview: straight members, including geometry-only bends, and realized arcs are Timoshenko with the energy-matched annular shear factor (timoshenko_annulus_energy_curved_v2) and Poisson's ratio E/(2G) - 1; on a realized arc the straight-annulus factor is applied along the centreline with no curvature correction, the arc's thermal equivalent load is formed with that shear-dependent arc stiffness, as the solve is, and the arc's stress recovery stays the straight-annulus recovery; a straight member's published nodal rotations are section rotations, not slopes; numerical integrity does not establish physical correctness.";
+
 /// §4: the arc ids the disclosure names before "and N more".
 const ARC_IDS_NAMED: usize = 32;
 /// §4: the disclosure's bound in Unicode scalar values (the report renderer's evidence threshold).
@@ -46,21 +59,50 @@
 /// §2: the disclosure threshold of the authored-ν warning (not a numerical tolerance).
 const POISSON_RATIO_DISCLOSURE: f64 = 1e-6;
 
-/// Whether `model` is a pressure-free document selecting Timoshenko straight members.
+/// Whether `model` is a pressure-free document selecting Timoshenko straight members (v1 or v2;
+/// both form them identically).
 pub(crate) fn selects(model: &PreviewModel) -> bool {
-    model.pressure_contract.is_none() && model.shear_model() == ShearModelSetting::TimoshenkoAnnulusEnergyV1
+    model.pressure_contract.is_none()
+        && matches!(
+            model.shear_model(),
+            ShearModelSetting::TimoshenkoAnnulusEnergyV1 | ShearModelSetting::TimoshenkoAnnulusEnergyCurvedV2
+        )
 }
 
+/// Whether `model` is a pressure-free document selecting `timoshenko_annulus_energy_curved_v2`:
+/// realized arcs with transverse shear as well.
+pub(crate) fn selects_curved(model: &PreviewModel) -> bool {
+    model.pressure_contract.is_none() && model.shear_model() == ShearModelSetting::TimoshenkoAnnulusEnergyCurvedV2
+}
+
+/// The selected Timoshenko model's document name, as the diagnostics state it.
+fn shear_model_name(model: &PreviewModel) -> &'static str {
+    if selects_curved(model) {
+        crate::model_settings::SHEAR_MODEL_TIMOSHENKO_CURVED
+    } else {
+        crate::model_settings::SHEAR_MODEL_TIMOSHENKO
+    }
+}
+
 /// The producer's semantic contract identity on the preview route.
 pub(crate) fn semantic_contract_id(model: &PreviewModel) -> &'static str {
-    if selects(model) { ID } else { preview_physics::ID }
+    if selects_curved(model) {
+        CURVED_ID
+    } else if selects(model) {
+        ID
+    } else {
+        preview_physics::ID
+    }
 }
 
 /// The preview route's formulation basis: preview-physics-2's, or for a Timoshenko
 /// document the same with its own profile and limitation 0.
 pub(crate) fn formulation_basis(model: &PreviewModel) -> FormulationBasis {
     let mut basis = preview_physics::formulation_basis();
-    if selects(model) {
+    if selects_curved(model) {
+        basis.profile_id = CURVED_PROFILE_ID.to_string();
+        basis.limitations[0] = CURVED_LIMITATION_0.to_string();
+    } else if selects(model) {
         basis.profile_id = PROFILE_ID.to_string();
         basis.limitations[0] = LIMITATION_0.to_string();
     }
@@ -72,6 +114,8 @@
 pub(crate) struct ConsumedBasis {
     pub(crate) description: String,
     pub(crate) case_ids: Vec<String>,
+    /// The selected Timoshenko model's name (v1 or v2), as the diagnostics state it.
+    pub(crate) shear_model: &'static str,
 }
 
 /// §2, "Construction boundary": the bases the document's cases consume, from their
@@ -101,13 +145,14 @@
                             (None, Some(temperature)) => format!("the material basis interpolated at solve temperature {:?} K", temperature.value),
                             (None, None) => unreachable!("a selected basis has a selection"),
                         };
-                        bases.selected.push((key, ConsumedBasis { description, case_ids: Vec::new() }));
+                        bases.selected.push((key, ConsumedBasis { description, case_ids: Vec::new(), shear_model: shear_model_name(model) }));
                         &mut bases.selected.last_mut().expect("pushed").1
                     }
                 },
                 None if refusals.is_empty() => bases.base.get_or_insert_with(|| ConsumedBasis {
                     description: "the base material basis (no temperature selection)".to_string(),
                     case_ids: Vec::new(),
+                    shear_model: shear_model_name(model),
                 }),
                 None => continue,
             };
@@ -122,7 +167,8 @@
     }
 }
 
-/// §2: one straight member's shear area on a consumed basis, from PF's stiffness annulus
+/// §2: one straight member's shear area on a consumed basis (under v2, also one realized
+/// arc's, by the same rule; `curved-bend-transverse-shear.md` §5), from PF's stiffness annulus
 /// and the ν that gives the member's unscaled G. `None` after a blocking diagnostic: the
 /// member is not formed, and the build blocks. The authored-ν warning is emitted once per
 /// material and basis (`warned`), since every member of a material shares its moduli there.
@@ -163,8 +209,8 @@
                 SHEAR_FACTOR_UNAVAILABLE,
                 "blocking",
                 format!(
-                    "timoshenko_annulus_energy_v1 member {} has no energy-matched annular shear factor on {} (load cases {cases}): nu = E/(2G) - 1 = {nu:?} from material {} E = {elastic_modulus:?} Pa and G = {shear_modulus:?} Pa, r_i = {inner_radius:?} m, r_o = {outer_radius:?} m: {error}; no default factor or Euler-Bernoulli member is used",
-                    pipe.id, basis.description, material.id,
+                    "{} member {} has no energy-matched annular shear factor on {} (load cases {cases}): nu = E/(2G) - 1 = {nu:?} from material {} E = {elastic_modulus:?} Pa and G = {shear_modulus:?} Pa, r_i = {inner_radius:?} m, r_o = {outer_radius:?} m: {error}; no default factor or Euler-Bernoulli member is used",
+                    basis.shear_model, pipe.id, basis.description, material.id,
                 ),
                 refs,
             ));
@@ -181,8 +227,14 @@
                 AUTHORED_POISSON_RATIO_NOT_USED,
                 "warning",
                 format!(
-                    "material {} authors poisson_ratio {authored:?}, which is not used: its timoshenko_annulus_energy_v1 straight members on {} (load cases {cases}) take nu = E/(2G) - 1 = {nu:?} from E = {elastic_modulus:?} Pa and G = {shear_modulus:?} Pa",
-                    material.id, basis.description,
+                    "material {} authors poisson_ratio {authored:?}, which is not used: its {} on {} (load cases {cases}) take nu = E/(2G) - 1 = {nu:?} from E = {elastic_modulus:?} Pa and G = {shear_modulus:?} Pa",
+                    material.id,
+                    if basis.shear_model == crate::model_settings::SHEAR_MODEL_TIMOSHENKO_CURVED {
+                        "timoshenko_annulus_energy_curved_v2 straight members and realized arcs"
+                    } else {
+                        "timoshenko_annulus_energy_v1 straight members"
+                    },
+                    basis.description,
                 ),
                 refs,
             ));
@@ -213,8 +265,9 @@
                 PARTIAL_SPAN_UNSUPPORTED,
                 "blocking",
                 format!(
-                    "load case {} equivalent_static.wind.exposed_spans[{index}] is a partial-span member load on timoshenko_annulus_energy_v1 straight member {pipe}; partial-span loads on Timoshenko members are not yet supported, so mark the whole span (exposed_pipe_refs) or select euler_bernoulli",
+                    "load case {} equivalent_static.wind.exposed_spans[{index}] is a partial-span member load on {} straight member {pipe}; partial-span loads on Timoshenko members are not yet supported, so mark the whole span (exposed_pipe_refs) or select euler_bernoulli",
                     case.id,
+                    shear_model_name(model),
                 ),
                 vec![pipe.to_string(), case.id.clone()],
             ));
@@ -222,9 +275,10 @@
     }
 }
 
-/// §4: the named arc disclosure, once per Timoshenko result that contains a realized arc.
+/// §4: the named arc disclosure, once per v1 Timoshenko result that contains a realized arc.
+/// Under v2 the arcs carry shear, so nothing is disclosed (`curved-bend-transverse-shear.md` §5).
 pub(crate) fn append_arc_disclosure(model: &PreviewModel, built: &BuiltModel, diagnostics: &mut Vec<Diagnostic>) {
-    if !selects(model) {
+    if !selects(model) || selects_curved(model) {
         return;
     }
     let mut arcs: Vec<&str> = built.curved_bend_elements.iter().map(|bend| bend.pipe_id.as_str()).collect();
```

**`core/product_physics/src/pressure_runtime.rs`** (3 hunks). EX vocabulary admits v2; `selects_timoshenko` covers v1|v2; new `selects_arc_shear` and `timoshenko_model`; region geometry's beam_theory names the document's model.

```diff
diff --git a/core/product_physics/src/pressure_runtime.rs b/core/product_physics/src/pressure_runtime.rs
--- a/core/product_physics/src/pressure_runtime.rs
+++ b/core/product_physics/src/pressure_runtime.rs
@@ -39,19 +39,35 @@
 /// T4-U2 (D-D): the named refusal of a direction change above alpha_tan at a
 /// realized bend's end; static text (T3 O-10).
 const MITRE_UNSUPPORTED: &str = "PRESSURE_REGION_MITRE_UNSUPPORTED";
-/// The exact contract's two shear models (DEL-05-03 Design §3, §5).
+/// The exact contract's three shear models (DEL-05-03 Design §3, §5; DEL-04-01 Design
+/// `curved-bend-transverse-shear.md` §1).
 pub(crate) const EULER_BERNOULLI: &str = "euler_bernoulli";
 pub(crate) const TIMOSHENKO_ANNULUS_ENERGY: &str = "timoshenko_annulus_energy_v1";
+pub(crate) const TIMOSHENKO_ANNULUS_ENERGY_CURVED: &str = "timoshenko_annulus_energy_curved_v2";
 
 /// T4-U8 (DEL-05-03 Design §3.6): whether an exact-contract document selects
-/// the Timoshenko straight member (per document; realized arcs stay
-/// Euler-Bernoulli).
+/// the Timoshenko straight member (per document; v1 and v2 form it identically).
 pub(crate) fn selects_timoshenko(model: &PreviewModel) -> bool {
     is_exact(model)
+        && matches!(
+            model.pressure_contract.as_ref().and_then(|contract| contract.shear_model.as_deref()),
+            Some(TIMOSHENKO_ANNULUS_ENERGY | TIMOSHENKO_ANNULUS_ENERGY_CURVED)
+        )
+}
+
+/// `curved-bend-transverse-shear.md` §5: whether an exact-contract document selects
+/// `timoshenko_annulus_energy_curved_v2`, so its realized arcs carry transverse shear too.
+pub(crate) fn selects_arc_shear(model: &PreviewModel) -> bool {
+    is_exact(model)
         && model.pressure_contract.as_ref().and_then(|contract| contract.shear_model.as_deref())
-            == Some(TIMOSHENKO_ANNULUS_ENERGY)
+            == Some(TIMOSHENKO_ANNULUS_ENERGY_CURVED)
 }
 
+/// The Timoshenko model a Timoshenko member's `beam_theory` and diagnostics name.
+pub(crate) fn timoshenko_model(model: &PreviewModel) -> &'static str {
+    if selects_arc_shear(model) { TIMOSHENKO_ANNULUS_ENERGY_CURVED } else { TIMOSHENKO_ANNULUS_ENERGY }
+}
+
 #[derive(Debug, Clone, Default, Deserialize)]
 #[serde(deny_unknown_fields)]
 pub struct PressureContractInput {
@@ -229,8 +245,9 @@
                     match contract.shear_model.as_deref() {
                         None => problem(diagnostics, "EXACT_PRESSURE_SHEAR_MODEL_REQUIRED",
                             &["pressure_contract", "shear_model"], "an exact pressure contract requires an explicit shear_model"),
-                        // T4-U8 (DEL-05-03 Design §5): both shear models are admitted.
-                        Some(EULER_BERNOULLI | TIMOSHENKO_ANNULUS_ENERGY) => {}
+                        // T4-U8 (DEL-05-03 Design §5) and `curved-bend-transverse-shear.md` §5:
+                        // the three shear models are admitted.
+                        Some(EULER_BERNOULLI | TIMOSHENKO_ANNULUS_ENERGY | TIMOSHENKO_ANNULUS_ENERGY_CURVED) => {}
                         Some(_) => problem(diagnostics, "PRESSURE_CONTRACT_UNSUPPORTED",
                             &["pressure_contract", "shear_model"], "unknown shear_model; no fallback is used"),
                     }
@@ -979,7 +996,7 @@
                 },
             };
             let end_directions = arc.map_or([direction, direction], |arc| arc.tangents);
-            let mut member_geometry = super::exact_section_evidence(&pipe.element_id, sections);
+            let mut member_geometry = super::exact_section_evidence(&pipe.element_id, sections, timoshenko_model(model));
             member_geometry["traversal_forward"] = json!(step.forward);
             if let Some(arc) = &arc {
                 super::mark_realized_arc(&mut member_geometry);
```

**`core/product_physics/src/exact_admission.rs`** (2 hunks). EX v2 identity, profile and whole-static limitation 0 (derived text in `ex_limitation_0.txt`); a curved-shear basis builder on top of the unselected v3 basis.

```diff
diff --git a/core/product_physics/src/exact_admission.rs b/core/product_physics/src/exact_admission.rs
--- a/core/product_physics/src/exact_admission.rs
+++ b/core/product_physics/src/exact_admission.rs
@@ -54,6 +54,15 @@
     "openpipestress.result_semantics/0.3.0/pressure-source-1";
 /// The formulation profile of every `pressure-2` envelope.
 pub(crate) const PRESSURE_PROFILE_ID: &str = "exact_pressure_v3";
+/// DEL-04-01 `curved-bend-transverse-shear.md` §5: the ordinary result semantics of a
+/// `3.0.0/exact_pressure_v3` document selecting `timoshenko_annulus_energy_curved_v2`.
+pub(crate) const PRESSURE_CURVED_SHEAR_SEMANTIC_CONTRACT_ID: &str =
+    "openpipestress.result_semantics/0.3.0/pressure-curved-shear-1";
+/// Its formulation profile.
+pub(crate) const PRESSURE_CURVED_SHEAR_PROFILE_ID: &str = "product_pressure_curved_shear_v2";
+/// Its limitation 0: pressure-2's unselected identity with the shear-model parenthesis and
+/// the realized-arc clause replaced. A whole static string, as v3's.
+const PRESSURE_CURVED_SHEAR_IDENTITY: &str = "3.0.0/exact_pressure_v3 under pressure-2 semantics. Admitted families: small-displacement homogeneous-isotropic straight circular pipe members (shear deformation as the contract declares: timoshenko_annulus_energy_curved_v2, the energy-matched annular shear factor kappa_E on the nominal section, whose nodal rotations are section rotations; G is derived from E/nu) and linear restraints or springs; realized curved bends carry transverse shear with the same kappa_E and A_s rule applied along the arc centreline, a one-dimensional approximation with no curvature correction; an arc's thermal and pressure free-strain equivalent loads K_b*u_free are formed with that shear-dependent arc stiffness, as the solve is, and the arc's stress recovery stays the straight-annulus recovery; realized curved bends and, with expansion joints authored as objective connectors, are admitted only as the bend and connector limitations state; valve, flange, rigid and specialty components are admitted as their mapped straight member with the user's DEC-045 stiffness scale, an equivalent pipe of the mapped section whose stress rows are those of that reference section (marked component_body), not a body stress, and reducers when their mapped member lies outside every pressure region or forms a pressure step, the only member of its own region between single straight members of other regions with transfers_to_wall at both ends, either concentric and collinear or eccentric or slanted with W-15c's run evidence and any read offset accepted, whose junction caps at the actual nodes along each member's own axis carry the step's pressure force and moment (EXACT_PRESSURE_REDUCER_IN_REGION otherwise); a branch connection (tee) is a record at its node, whose members meet as a rigid joint with no branch flexibility; a tee consumed as mechanics_geometry_only gives its user-entered header and branch SIFs as intensified rows at the ends of its named straight members on each section basis, not a code stress, and a realized arc it names carries none; a constant-effort support (DEC-049) declaring exactly one translational DOF and a finite positive constant load is consumed: it has zero stiffness and applies that load as a constant nodal force along the DOF's positive global axis in every solved case, and a mechanics combination over a model with one is withheld (CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE) when a factor is not finite or the exact factor sum is not within 64 epsilon of 1; a record that is not consumed applies nothing and warns (SUPPORT_CONSTANT_EFFORT_NOT_CONSUMED); gap, one-way, lift-off and friction supports (friction with an explicit normal or a normal derived from a linear restraint), beside objective connectors and on load/reference-state documents included, are solved in each case by the active-set loop from their authored initial states and publish their states, final displacements and reactions; K-D5 confirmation is attempted for an eligible non-gap state, and the case's published numerical standing reflects its outcome; a case with a gap support is published with the unqualified recovery basis (NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED) and no confirmation, and a mechanics combination over a model with a nonlinear support is withheld (NONLINEAR_COMBINATION_REQUIRES_SOLVE) unless it is one load case at factor exactly 1 plus at least one combination whose fixed-load count is exactly 0, formed from rows as a solved state plus derived differences or bounds, not an equilibrium state. Pressure regions meeting at a node where every terminal transfers to the wall form a junction, whose caps sum to the fitting's pressure load. Section bases: stiffness and pipe metal mass use the nominal wall; contents mass and pressure end-cap areas use the reduced bore; every stress row is published on the nominal section and on the reduced section (wall minus mill tolerance and corrosion allowance) and names its basis, and each basis has its own maximum and headline; no governing basis is chosen. Each basis's Lamé stresses are those of its own annulus, while the axial membrane stress divides the one wall force by that basis's area. Every other component or support family is refused by name (EXACT_PRESSURE_FAMILY_NOT_ADMITTED) until a later contract revision admits it.";
 
 /// The named seam refusal for a family a v3 document may not yet contain.
 pub(crate) const FAMILY_NOT_ADMITTED: &str = "EXACT_PRESSURE_FAMILY_NOT_ADMITTED";
@@ -185,6 +194,16 @@
     FormulationBasis { profile_id: PRESSURE_PROFILE_ID.to_string(), limitations }
 }
 
+/// `curved-bend-transverse-shear.md` §5: the curved-shear identity's basis: the unselected v3
+/// basis with its own profile and limitation 0. A v2 document never selects a retained source
+/// (`source_recovery` declines it by name), so there is no selected variant.
+pub(crate) fn pressure_v3_curved_shear_formulation_basis(load_state: bool, connectors: bool) -> FormulationBasis {
+    let mut basis = pressure_v3_formulation_basis_with_source(load_state, connectors, false);
+    basis.profile_id = PRESSURE_CURVED_SHEAR_PROFILE_ID.to_string();
+    basis.limitations[0] = PRESSURE_CURVED_SHEAR_IDENTITY.to_string();
+    basis
+}
+
 /// W-16 slice c (DEL-05-01 Design `exact-equivalent-static.md` §3): a case's
 /// `equivalent_static` on a load/reference-state document. Generated loads
 /// have no ledger class there, so they are refused for every authored case,
```

**`core/product_physics/src/lib.rs`** (14 hunks). Producer identity and basis selection; arc A_s by the route's straight rule into `member_shear_areas` (beside `member_moduli`); chord section stays EB; `with_transverse_shear_area` in the macro builder; `beam_theory` names the model, so v2 arcs get a Timoshenko record.

```diff
diff --git a/core/product_physics/src/lib.rs b/core/product_physics/src/lib.rs
--- a/core/product_physics/src/lib.rs
+++ b/core/product_physics/src/lib.rs
@@ -1430,7 +1430,10 @@
 /// The identity follows the actual current model route; source selection
 /// changes it only at publication.
 fn mechanics_producer_for_model(model: &PreviewModel) -> MechanicsProducer {
-    mechanics_producer(if pressure_runtime::is_exact(model) {
+    mechanics_producer(if pressure_runtime::selects_arc_shear(model) {
+        // DEL-04-01 `curved-bend-transverse-shear.md` §5: its own exact identity.
+        exact_admission::PRESSURE_CURVED_SHEAR_SEMANTIC_CONTRACT_ID
+    } else if pressure_runtime::is_exact(model) {
         PRESSURE_SEMANTIC_CONTRACT_ID
     } else {
         pf_timoshenko::semantic_contract_id(model)
@@ -1447,11 +1450,11 @@
         // names the publication (pressure-source-1 or pressure-2).
         let load_state = case_state::is_load_state(model);
         let connectors = model.components.iter().any(|c| c.objective_connector.is_some());
-        let mut basis = exact_admission::pressure_v3_formulation_basis_with_source(
-            load_state,
-            connectors,
-            source_selected,
-        );
+        let mut basis = if pressure_runtime::selects_arc_shear(model) {
+            exact_admission::pressure_v3_curved_shear_formulation_basis(load_state, connectors)
+        } else {
+            exact_admission::pressure_v3_formulation_basis_with_source(load_state, connectors, source_selected)
+        };
         // T4-U2: a model with a realized bend states the bend's terms; a
         // straight-only v3 model keeps its text unchanged (SP-1).
         if model.components.iter().any(is_curved_bend_macro_component) {
@@ -7429,7 +7432,7 @@
         "load_case_id":load_case.id,"profile_mode":pressure_runtime::exact_contract(model).map(|c| c.mode()),
         "material_basis":modulus_basis_record.unwrap_or("base_material_common_E_nu"),
         "pressure_rhs_assembly":pressure_assembly_evidence,
-        "pipe_sections":exact_pipe_sections(built),
+        "pipe_sections":exact_pipe_sections(built, pressure_runtime::timoshenko_model(model)),
         "pipe_stress_extrema":pipe_stress_extrema,
         "stress_maximum_coverage":[
             {"section_basis":pressure_exact::SectionBasis::Nominal.token(),"complete":unavailable_stress_maximum_members.is_empty(),"unavailable_pipe_ids":unavailable_stress_maximum_members},
@@ -8823,6 +8826,10 @@
     // Each realized-bend pipe's resolved (E, G): the material's, or the 0.4.0
     // member pair. Straight pipes insert nothing (RV12 S-3).
     let mut member_moduli: HashMap<&str, (f64, f64)> = HashMap::new();
+    // DEL-04-01 `curved-bend-transverse-shear.md` §5: each realized-bend pipe's transverse
+    // shear area A_s under timoshenko_annulus_energy_curved_v2, by its route's straight rule.
+    // Every other document inserts nothing.
+    let mut member_shear_areas: HashMap<&str, f64> = HashMap::new();
     let mut exact_sections = HashMap::new();
     // W-15 (DEL-03-05 Design `rigid-component-stiffness-scaling.md` §2.2): each
     // eligible rigid-family component's member is formed with s*E and s*G.
@@ -8936,9 +8943,12 @@
         // area A_s = kappa_E * A_n, with kappa_E (FK `shear`) from the nominal
         // annulus's binary64 leaves and the member's selected nu, the E/nu basis
         // that gives its G (per resolved case on a load-state document). A
-        // realized arc stays Euler-Bernoulli. A missing basis or a factor out of
-        // range blocks by name; no default factor exists.
-        let shear_area = if pressure_runtime::selects_timoshenko(model) && !curved_bend_pipe_ids.contains(pipe.id.as_str()) {
+        // realized arc stays Euler-Bernoulli, except under
+        // timoshenko_annulus_energy_curved_v2, where it takes A_s by the same rule
+        // (DEL-04-01 `curved-bend-transverse-shear.md` §5). A missing basis or a
+        // factor out of range blocks by name; no default factor exists.
+        let arc = curved_bend_pipe_ids.contains(pipe.id.as_str());
+        let shear_area = if pressure_runtime::selects_timoshenko(model) && (!arc || pressure_runtime::selects_arc_shear(model)) {
             let nu = match member_pairs {
                 Some(pairs) => pairs.get(&pipe.id).map(|pair| pair.poisson_ratio()),
                 None => material.poisson_ratio.as_ref().map(|nu| nu.value),
@@ -8958,14 +8968,15 @@
                 }
                 Err(missing) => {
                     diagnostics.push(diag(&format!("diagnostic:exact-section:{}:shear", stable_suffix(&pipe.id)), "EXACT_PRESSURE_SHEAR_FACTOR_UNAVAILABLE", "blocking",
-                        format!("a timoshenko_annulus_energy_v1 member requires {missing}; no default shear factor is used"), vec![pipe.id.clone()]));
+                        format!("a {} member requires {missing}; no default shear factor is used", pressure_runtime::timoshenko_model(model)), vec![pipe.id.clone()]));
                     continue;
                 }
             }
-        } else if let Some(basis) = pf_shear.filter(|_| !curved_bend_pipe_ids.contains(pipe.id.as_str())) {
+        } else if let Some(basis) = pf_shear.filter(|_| !arc || pf_timoshenko::selects_curved(model)) {
             // PF Timoshenko (DEL-05-03 Design `pf-timoshenko.md` §2): every straight
-            // member on a consumed basis, from PF's stiffness annulus and the nu of
-            // the unscaled (E, G) above; a refused factor blocks the member by name.
+            // member on a consumed basis, and under v2 every realized arc too, from PF's
+            // stiffness annulus and the nu of the unscaled (E, G) above; a refused factor
+            // blocks the member by name.
             let Some(shear_area) = pf_timoshenko::shear_area(basis, pipe, &derived, material, elastic_modulus, shear_modulus, &mut pf_poisson_warned, diagnostics) else {
                 continue;
             };
@@ -9001,7 +9012,9 @@
             }
             None => (elastic_modulus, shear_modulus),
         };
-        let section = match shear_area.map_or_else(|| StraightPipeSectionProperties::new(
+        // A realized arc's chord section stays Euler-Bernoulli: its A_s reaches only the
+        // macro-element, through `member_shear_areas`.
+        let section = match shear_area.filter(|_| !arc).map_or_else(|| StraightPipeSectionProperties::new(
             elastic_modulus,
             shear_modulus,
             derived.area,
@@ -9046,8 +9059,11 @@
                     continue;
                 }
             };
-        if curved_bend_pipe_ids.contains(pipe.id.as_str()) {
+        if arc {
             member_moduli.insert(pipe.id.as_str(), (elastic_modulus, shear_modulus));
+            if let Some(shear_area) = shear_area {
+                member_shear_areas.insert(pipe.id.as_str(), shear_area);
+            }
         } else if !replaced_span_ids.contains(pipe.id.as_str()) {
             frame_elements.push(
                 element
@@ -9089,6 +9105,7 @@
         &node_map,
         &sections,
         &member_moduli,
+        &member_shear_areas,
         diagnostics,
     );
     let nonlinear = build_nonlinear_supports(model, &node_map, diagnostics);
@@ -9273,6 +9290,7 @@
     node_map: &HashMap<&str, usize>,
     sections: &HashMap<String, DerivedSection>,
     member_moduli: &HashMap<&str, (f64, f64)>,
+    member_shear_areas: &HashMap<&str, f64>,
     diagnostics: &mut Vec<Diagnostic>,
 ) -> Vec<CurvedBendMacroBuild> {
     let pipe_map = model
@@ -9497,6 +9515,18 @@
                 ));
                 continue;
             }
+        };
+        // DEL-04-01 `curved-bend-transverse-shear.md` §5: a v2 arc carries its route's A_s;
+        // every other arc is formed exactly as before (no shear arithmetic for `None`).
+        let element = match member_shear_areas.get(pipe_ref) {
+            None => element,
+            Some(&shear_area) => match element.with_transverse_shear_area(shear_area) {
+                Ok(element) => element,
+                Err(error) => {
+                    diagnostics.push(curved_bend_macro_element_diag(component, pipe_ref, &error.to_string()));
+                    continue;
+                }
+            },
         };
         let (global_stiffness, arc_length, included_angle) = match (
             element.global_stiffness(),
@@ -12154,14 +12184,14 @@
 /// region geometry decides it (`realized_arc`), so a reader without the model
 /// knows every arc, including arcs outside every pressure region. A straight
 /// member's entry keeps exactly the `exact_section_evidence` keys.
-fn exact_pipe_sections(built: &BuiltModel) -> Vec<serde_json::Value> {
+fn exact_pipe_sections(built: &BuiltModel, timoshenko: &'static str) -> Vec<serde_json::Value> {
     built
         .pipes
         .iter()
         .enumerate()
         .map(|(index, pipe)| {
             let sections = *built.exact_sections.get(&pipe.element_id).expect("every built exact member has source geometry");
-            let mut entry = exact_section_evidence(&pipe.element_id, sections);
+            let mut entry = exact_section_evidence(&pipe.element_id, sections, timoshenko);
             if realized_arc(built, index).is_some() {
                 mark_realized_arc(&mut entry);
             }
@@ -12187,10 +12217,10 @@
 /// pipe's `pipe_sections` entry, the one shape of every identity's section
 /// evidence and of the realized-arc geometry record: the authored OD, wall,
 /// mill tolerance (absent = 0) and allowance, both sections, the pressure bore
-/// and the beam theory (a realized arc's sections carry no shear leaf, so it
-/// is Euler-Bernoulli under either selection). Callers add the realized-arc
-/// marker.
-fn exact_section_evidence(pipe_id: &str, g: pressure_exact::ExactSectionBases) -> serde_json::Value {
+/// and the beam theory (a realized arc's sections carry a shear leaf only
+/// under timoshenko_annulus_energy_curved_v2; `timoshenko` names the document's Timoshenko
+/// model). Callers add the realized-arc marker.
+fn exact_section_evidence(pipe_id: &str, g: pressure_exact::ExactSectionBases, timoshenko: &'static str) -> serde_json::Value {
     let section = |s: pressure_exact::SourceAnnulus| serde_json::json!({"wall_m":s.effective_wall_thickness_m(),
         "ri_m":s.inner_radius_m(),"As_m2":s.wall_area_m2(),"I_m4":s.second_moment_m4(),"J_m4":s.polar_moment_m4(),"Z_m3":s.section_modulus_m3()});
     serde_json::json!({"pipe_id":pipe_id,"geometry_basis":"authored_od_wall_d5_sections_v1",
@@ -12198,10 +12228,11 @@
         "wall_thickness_m":g.nominal_wall_m(),"mill_tolerance_m":g.mill_tolerance_m(),"corrosion_allowance_m":g.corrosion_allowance_m(),
         "nominal_section":section(g.nominal()),"reduced_section":section(g.reduced()),
         "pressure_bore":{"ri_m":g.pressure_bore_radius_m(),"Ai_m2":g.pressure_bore_area_m2()},
-        // T4-U8 (§2.3): a Timoshenko straight member's kappa_E and A_s; every
-        // other member, realized arcs included, is Euler-Bernoulli.
+        // T4-U8 (§2.3): a Timoshenko member's kappa_E and A_s under the document's
+        // Timoshenko model (under v2, realized arcs too); every other member is
+        // Euler-Bernoulli.
         "beam_theory":match g.timoshenko() {
-            Some((shear_factor, shear_area)) => serde_json::json!({"model":pressure_runtime::TIMOSHENKO_ANNULUS_ENERGY,
+            Some((shear_factor, shear_area)) => serde_json::json!({"model":timoshenko,
                 "shear_factor":shear_factor,"shear_area_m2":shear_area}),
             None => serde_json::json!({"model":pressure_runtime::EULER_BERNOULLI}),
         }})
```

**`core/product_physics/src/source_receipt/composite.rs`** (1 hunk). Caller of the changed `exact_pipe_sections` signature (pressure-source-1 binding; v2 never reaches it).

```diff
diff --git a/core/product_physics/src/source_receipt/composite.rs b/core/product_physics/src/source_receipt/composite.rs
--- a/core/product_physics/src/source_receipt/composite.rs
+++ b/core/product_physics/src/source_receipt/composite.rs
@@ -551,7 +551,7 @@
     {
         return Err(bad("physical case/method/material/coverage binding"));
     }
-    if evidence["pipe_sections"] != json!(exact_pipe_sections(built)) {
+    if evidence["pipe_sections"] != json!(exact_pipe_sections(built, crate::pressure_runtime::timoshenko_model(model))) {
         return Err(bad("physical source geometry binding"));
     }
     let material_evidence: Value = if let Some((state, _)) = load_state {
```

**`core/product_physics/src/source_recovery.rs`** (1 hunk). v2 declined by name from the straight source family, so pressure-source-1 (whose bound schema admits EB/v1 only) never publishes a v2 document.

```diff
diff --git a/core/product_physics/src/source_recovery.rs b/core/product_physics/src/source_recovery.rs
--- a/core/product_physics/src/source_recovery.rs
+++ b/core/product_physics/src/source_recovery.rs
@@ -811,6 +811,12 @@
     }
     if !input.built.connectors.is_empty() {
         return Err(unsupported("objective connector is not a straight source family"));
+    }
+    // DEL-04-01 `curved-bend-transverse-shear.md` §5: pressure-source-1 and its evidence
+    // state euler_bernoulli or timoshenko_annulus_energy_v1 only, so a document selecting
+    // timoshenko_annulus_energy_curved_v2 is declined by name, with or without arcs.
+    if super::pressure_runtime::selects_arc_shear(input.model) {
+        return Err(unsupported("timoshenko_annulus_energy_curved_v2 is not a straight source family"));
     }
     // O4 (DEL-04-08 Design `o4-source-extension.md` §3.1, §3.2): a
     // timoshenko_annulus_energy_v1 document and a nonzero mill tolerance or
```

**`core/product_physics/src/modal/prepare.rs`** (1 hunk). Modal refuses v2 under `MODAL_TIMOSHENKO_MASS_UNSUPPORTED` with a message naming v2; the v1 message is byte-identical.

```diff
diff --git a/core/product_physics/src/modal/prepare.rs b/core/product_physics/src/modal/prepare.rs
--- a/core/product_physics/src/modal/prepare.rs
+++ b/core/product_physics/src/modal/prepare.rs
@@ -256,11 +256,19 @@
 
     // The modal interim refusals, each by name.
     if pf_timoshenko::selects(&model) || pressure_runtime::selects_timoshenko(&model) {
+        // DEL-04-01 `curved-bend-transverse-shear.md` §5: v2 is refused under the same code,
+        // naming its value; its arcs would otherwise pair an Euler-Bernoulli mass with
+        // shear-flexible arcs too.
+        let message = if pf_timoshenko::selects_curved(&model) || pressure_runtime::selects_arc_shear(&model) {
+            "the model selects Timoshenko straight members and realized arcs (timoshenko_annulus_energy_curved_v2); modal analysis forms an Euler-Bernoulli consistent mass, and pairing it with shear-flexible stiffness would be inconsistent, so nothing is formed (a Timoshenko consistent mass is a later slice, M4)"
+        } else {
+            "the model selects Timoshenko straight members (timoshenko_annulus_energy_v1); modal analysis forms an Euler-Bernoulli consistent mass, and pairing it with shear-flexible stiffness would be inconsistent, so nothing is formed (a Timoshenko consistent mass is a later slice, M4)"
+        };
         diagnostics.push(diag(
             "diagnostic:modal:timoshenko-mass",
             MODAL_TIMOSHENKO_MASS_UNSUPPORTED,
             "blocking",
-            "the model selects Timoshenko straight members (timoshenko_annulus_energy_v1); modal analysis forms an Euler-Bernoulli consistent mass, and pairing it with shear-flexible stiffness would be inconsistent, so nothing is formed (a Timoshenko consistent mass is a later slice, M4)",
+            message,
             vec!["shear_model".to_string()],
         ));
         return Err(diagnostics);
```

### 2.2 Result export and semantic contract (DEL-05-02; DEL-04-09 for modal_result)

**`core/reporting/result_export/src/semantic_contract.rs`** (11 hunks). Both new identities, profiles, table loaders (new OnceLock owners, see §6), `PRESSURE_IDENTITIES`/`is_pressure_id`, dispatch arms, fresh set. Table SHA-256 constants are placeholders filled from `derive`.

```diff
diff --git a/core/reporting/result_export/src/semantic_contract.rs b/core/reporting/result_export/src/semantic_contract.rs
--- a/core/reporting/result_export/src/semantic_contract.rs
+++ b/core/reporting/result_export/src/semantic_contract.rs
@@ -54,7 +54,7 @@
 /// Canonical metadata under the vocabulary `for_source` selected for `source`
 /// Connector-specific location/basis rules remain independently checked.
 pub fn canonical_metadata_for(source: &Value, table: &Value, row: &Value) -> Option<Value> {
-    if source["producer"]["semantic_contract_id"] == PRESSURE_ID
+    if source["producer"]["semantic_contract_id"].as_str().is_some_and(is_pressure_id)
         && crate::connector_evidence::is_connector_kind(row["kind"].as_str().unwrap_or_default())
     {
         return crate::connector_evidence::canonical_metadata(row);
@@ -95,6 +95,22 @@
 /// pressure-1; model 0.6.0 renamed it, DEL-04-07 Design `u4b-u5b-implementation.md` §1.2).
 pub const PRESSURE_ID: &str = "openpipestress.result_semantics/0.3.0/pressure-2";
 pub const PRESSURE_PROFILE: &str = "exact_pressure_v3";
+/// DEL-04-01 `curved-bend-transverse-shear.md` §5: the exact curved-shear identity
+/// (`timoshenko_annulus_energy_curved_v2`). Its table is pressure-2's with the identity and
+/// profile replaced (`semantic_contract_tables.py derive`); pressure-2's bytes are unchanged.
+pub const PRESSURE_CURVED_SHEAR_ID: &str = "openpipestress.result_semantics/0.3.0/pressure-curved-shear-1";
+pub const PRESSURE_CURVED_SHEAR_PROFILE: &str = "product_pressure_curved_shear_v2";
+/// Generated by `derive`; never edited by hand.
+pub const PRESSURE_CURVED_SHEAR_TABLE_SHA256: &str = "<sha256 of semantic_contract_v0_3_pressure_curved_shear_1.json>";
+/// The ordinary exact route's identities, `(id, profile)`.
+pub const PRESSURE_IDENTITIES: [(&str, &str); 2] = [
+    (PRESSURE_ID, PRESSURE_PROFILE),
+    (PRESSURE_CURVED_SHEAR_ID, PRESSURE_CURVED_SHEAR_PROFILE),
+];
+/// Either ordinary exact identity.
+pub fn is_pressure_id(id: &str) -> bool {
+    PRESSURE_IDENTITIES.iter().any(|(i, _)| *i == id)
+}
 pub const PRESSURE_SOURCE_ID: &str = crate::physics_source::PRESSURE_CONTRACT_ID;
 /// Independent current resident source-family table, with no ordinary alias.
 /// T4-U6 (D-5, §2.4): both tables carry one stress signature per (recovery
@@ -137,13 +153,21 @@
 pub const PREVIEW_PHYSICS_TIMOSHENKO_PROFILE: &str = "product_preview_mechanics_timoshenko_v1";
 pub const PREVIEW_PHYSICS_TIMOSHENKO_SHA256: &str =
     "8a0abaeac93729b5d6ee6c922f2a60f0caf9332948622bb8b5896747ce41442d";
+/// DEL-04-01 `curved-bend-transverse-shear.md` §5: the pressure-free curved-shear identity
+/// (`timoshenko_annulus_energy_curved_v2`), a third complete identity on the route, derived
+/// as the Timoshenko one is.
+pub const PREVIEW_PHYSICS_CURVED_SHEAR_ID: &str = "openpipestress.result_semantics/0.3.0/preview-physics-curved-shear-1";
+pub const PREVIEW_PHYSICS_CURVED_SHEAR_PROFILE: &str = "product_preview_mechanics_curved_shear_v2";
+/// Generated by `derive`; never edited by hand.
+pub const PREVIEW_PHYSICS_CURVED_SHEAR_SHA256: &str = "<sha256 of semantic_contract_v0_3_preview_physics_curved_shear_1.json>";
 /// The preview-physics route's admitted identities, `(id, profile)`, in one place: every
 /// identity-aware site keys by the envelope's `semantic_contract_id` through this list.
-pub const PREVIEW_PHYSICS_IDENTITIES: [(&str, &str); 2] = [
+pub const PREVIEW_PHYSICS_IDENTITIES: [(&str, &str); 3] = [
     (PREVIEW_PHYSICS_ID, PREVIEW_PHYSICS_PROFILE),
     (PREVIEW_PHYSICS_TIMOSHENKO_ID, PREVIEW_PHYSICS_TIMOSHENKO_PROFILE),
+    (PREVIEW_PHYSICS_CURVED_SHEAR_ID, PREVIEW_PHYSICS_CURVED_SHEAR_PROFILE),
 ];
-/// Either preview-physics identity (Euler–Bernoulli or Timoshenko).
+/// Any preview-physics identity (Euler–Bernoulli, Timoshenko or curved shear).
 pub fn is_preview_physics_id(id: &str) -> bool {
     PREVIEW_PHYSICS_IDENTITIES.iter().any(|(i, _)| *i == id)
 }
@@ -161,6 +185,10 @@
         PREVIEW_PHYSICS_TIMOSHENKO_ID => {
             Some((preview_physics_timoshenko_contract(), PREVIEW_PHYSICS_TIMOSHENKO_PROFILE))
         }
+        // No recorded variant exists for the curved-shear identity: it is newer than every record.
+        PREVIEW_PHYSICS_CURVED_SHEAR_ID => {
+            Some((preview_physics_curved_shear_contract(), PREVIEW_PHYSICS_CURVED_SHEAR_PROFILE))
+        }
         _ => None,
     }
 }
@@ -377,6 +405,54 @@
         .expect("pinned preview-physics-timoshenko semantic contract")
     })
 }
+/// Pinned curved-shear table bytes: identity, profile and sha256 are checked.
+pub fn verify_preview_physics_curved_shear_table(bytes: &[u8]) -> Result<Value, String> {
+    use sha2::{Digest, Sha256};
+    if format!("{:x}", Sha256::digest(bytes)) != PREVIEW_PHYSICS_CURVED_SHEAR_SHA256 {
+        return Err("SOURCE_PREVIEW_PHYSICS_CURVED_SHEAR_TABLE_HASH".into());
+    }
+    let table: Value = serde_json::from_slice(bytes)
+        .map_err(|_| "SOURCE_PREVIEW_PHYSICS_CURVED_SHEAR_TABLE_HASH".to_string())?;
+    if table["semantic_contract_id"] != PREVIEW_PHYSICS_CURVED_SHEAR_ID
+        || table["formulation_profile_id"] != PREVIEW_PHYSICS_CURVED_SHEAR_PROFILE
+    {
+        return Err("SOURCE_PREVIEW_PHYSICS_CURVED_SHEAR_TABLE_IDENTITY".into());
+    }
+    Ok(table)
+}
+pub fn preview_physics_curved_shear_contract() -> &'static Value {
+    static CONTRACT: OnceLock<Value> = OnceLock::new();
+    CONTRACT.get_or_init(|| {
+        verify_preview_physics_curved_shear_table(include_bytes!(
+            "../../../../fixtures/results/semantic_contract_v0_3_preview_physics_curved_shear_1.json"
+        ))
+        .expect("pinned preview-physics-curved-shear semantic contract")
+    })
+}
+/// Pinned exact curved-shear table bytes: identity, profile and sha256 are checked.
+pub fn verify_pressure_curved_shear_table(bytes: &[u8]) -> Result<Value, String> {
+    use sha2::{Digest, Sha256};
+    if format!("{:x}", Sha256::digest(bytes)) != PRESSURE_CURVED_SHEAR_TABLE_SHA256 {
+        return Err("SOURCE_PRESSURE_CURVED_SHEAR_TABLE_HASH".into());
+    }
+    let table: Value = serde_json::from_slice(bytes)
+        .map_err(|_| "SOURCE_PRESSURE_CURVED_SHEAR_TABLE_HASH".to_string())?;
+    if table["semantic_contract_id"] != PRESSURE_CURVED_SHEAR_ID
+        || table["formulation_profile_id"] != PRESSURE_CURVED_SHEAR_PROFILE
+    {
+        return Err("SOURCE_PRESSURE_CURVED_SHEAR_TABLE_IDENTITY".into());
+    }
+    Ok(table)
+}
+pub fn pressure_curved_shear_contract() -> &'static Value {
+    static CONTRACT: OnceLock<Value> = OnceLock::new();
+    CONTRACT.get_or_init(|| {
+        verify_pressure_curved_shear_table(include_bytes!(
+            "../../../../fixtures/results/semantic_contract_v0_3_pressure_curved_shear_1.json"
+        ))
+        .expect("pinned pressure-curved-shear semantic contract")
+    })
+}
 /// Verify the independent current source table. Its full reviewed-byte digest
 /// binds the 94 signatures, vocabulary, bases and hash vectors.
 pub fn verify_pressure_source_table(bytes: &[u8]) -> Result<Value, String> {
@@ -587,7 +663,12 @@
             if !matches!(
                 p["semantic_contract_id"].as_str(),
                 Some(
-                    PRESSURE_ID | PRESSURE_SOURCE_ID | PREVIEW_PHYSICS_ID | PREVIEW_PHYSICS_TIMOSHENKO_ID
+                    PRESSURE_ID
+                        | PRESSURE_CURVED_SHEAR_ID
+                        | PRESSURE_SOURCE_ID
+                        | PREVIEW_PHYSICS_ID
+                        | PREVIEW_PHYSICS_TIMOSHENKO_ID
+                        | PREVIEW_PHYSICS_CURVED_SHEAR_ID
                 )
             ) {
                 return Err("SOURCE_PRODUCER_CONTRACT_UNSUPPORTED".into());
@@ -679,8 +760,10 @@
             let profile = match p["semantic_contract_id"].as_str() {
                 // T4-U2a: the only profile of pressure-1, on 0.3.0 and 0.4.0.
                 Some(PRESSURE_ID | PRESSURE_SOURCE_ID) => PRESSURE_PROFILE,
+                Some(PRESSURE_CURVED_SHEAR_ID) => PRESSURE_CURVED_SHEAR_PROFILE,
                 // pf-timoshenko.md §4: each preview identity has its own profile.
                 Some(PREVIEW_PHYSICS_TIMOSHENKO_ID) => PREVIEW_PHYSICS_TIMOSHENKO_PROFILE,
+                Some(PREVIEW_PHYSICS_CURVED_SHEAR_ID) => PREVIEW_PHYSICS_CURVED_SHEAR_PROFILE,
                 _ => PREVIEW_PHYSICS_PROFILE,
             };
             if !exact_keys(f, &["profile_id", "limitations"])
@@ -695,8 +778,9 @@
             // raw evidence and source-block invocation checks are separate APIs.
             let table = match p["semantic_contract_id"].as_str() {
                 Some(PRESSURE_ID) => pressure_rows_contract(),
+                Some(PRESSURE_CURVED_SHEAR_ID) => pressure_curved_shear_contract(),
                 Some(PRESSURE_SOURCE_ID) => pressure_source_contract(),
-                Some(id @ (PREVIEW_PHYSICS_ID | PREVIEW_PHYSICS_TIMOSHENKO_ID)) => {
+                Some(id @ (PREVIEW_PHYSICS_ID | PREVIEW_PHYSICS_TIMOSHENKO_ID | PREVIEW_PHYSICS_CURVED_SHEAR_ID)) => {
                     preview_physics_identity(id).expect("a preview-physics identity").0
                 }
                 _ => return Err("SOURCE_PRODUCER_CONTRACT_UNSUPPORTED".into()),
@@ -717,8 +801,10 @@
     let selected = for_source_metadata(source)?;
     forbid_retained_rows(source)?;
     match source["producer"]["semantic_contract_id"].as_str() {
-        Some(PREVIEW_PHYSICS_ID | PREVIEW_PHYSICS_TIMOSHENKO_ID) => validate_preview_physics_evidence(source)?,
-        Some(PRESSURE_ID) => validate_pressure_evidence(source)?,
+        Some(PREVIEW_PHYSICS_ID | PREVIEW_PHYSICS_TIMOSHENKO_ID | PREVIEW_PHYSICS_CURVED_SHEAR_ID) => {
+            validate_preview_physics_evidence(source)?
+        }
+        Some(PRESSURE_ID | PRESSURE_CURVED_SHEAR_ID) => validate_pressure_evidence(source)?,
         Some(PRESSURE_SOURCE_ID) => {
             crate::physics_source::validate_in(source, None, crate::physics_source::SourceContract::PressureV3)?;
         }
@@ -742,7 +828,7 @@
 /// place of the reader's call. A change to those steps is a change to both. Any
 /// other source reads as `for_source`.
 fn for_source_given(source: &Value, verdict: &Result<(), String>) -> Result<(&'static Value, &'static str), String> {
-    if is_retained(source) || source["producer"]["semantic_contract_id"] != PRESSURE_ID {
+    if is_retained(source) || !source["producer"]["semantic_contract_id"].as_str().is_some_and(is_pressure_id) {
         return for_source(source);
     }
     let selected = for_source_metadata(source)?;
@@ -760,9 +846,12 @@
     // pf-timoshenko.md §4: a pressure-free document whose `model_settings.shear_model`
     // selects `timoshenko_annulus_energy_v1`.
     PREVIEW_PHYSICS_TIMOSHENKO_ID,
+    // curved-bend-transverse-shear.md §5: either route's `timoshenko_annulus_energy_curved_v2`.
+    PREVIEW_PHYSICS_CURVED_SHEAR_ID,
     PRESSURE_SOURCE_ID,
     // T4-U2: pressure-1 (`3.0.0/exact_pressure_v3`) on 0.3.0 and 0.4.0.
     PRESSURE_ID,
+    PRESSURE_CURVED_SHEAR_ID,
     // U6a (D-U6-6; D2 4.7 S-1): membership is not standing. The successor's
     // standing comes only from the accepted reader's verified receipt.
     PREVIEW_PHYSICS_RETAINED_ID,
```

**`core/reporting/result_export/src/section_basis.rs`** (7 hunks). Reader admits v2 beam_theory (straights and arcs); `selected_model` gives v2 for arcs only under v2; `Beam::Timoshenko` gains `curved`.

```diff
diff --git a/core/reporting/result_export/src/section_basis.rs b/core/reporting/result_export/src/section_basis.rs
--- a/core/reporting/result_export/src/section_basis.rs
+++ b/core/reporting/result_export/src/section_basis.rs
@@ -66,6 +66,9 @@
 pub const BORE_KEYS: [&str; 2] = ["ri_m", "Ai_m2"];
 pub const EULER_BERNOULLI: &str = "euler_bernoulli";
 pub const TIMOSHENKO: &str = "timoshenko_annulus_energy_v1";
+/// DEL-04-01 `curved-bend-transverse-shear.md` §5: v1's straight members, and realized arcs
+/// with the same kappa_E and A_s rule.
+pub const TIMOSHENKO_CURVED: &str = "timoshenko_annulus_energy_curved_v2";
 /// §2.1: the kinds whose pressure-2 table family is `stress` and which the
 /// exact route publishes, once per section basis.
 pub const STRESS_KINDS: [&str; 15] = [
@@ -210,7 +213,9 @@
 #[derive(Clone, Copy, Debug, PartialEq)]
 pub enum Beam {
     EulerBernoulli,
-    Timoshenko { shear_factor: f64, shear_area: f64 },
+    /// `curved`: the member states `timoshenko_annulus_energy_curved_v2` (a realized arc may then
+    /// be Timoshenko too); otherwise `timoshenko_annulus_energy_v1`.
+    Timoshenko { shear_factor: f64, shear_area: f64, curved: bool },
 }
 /// A checked D-5 `pipe_sections` entry.
 #[derive(Clone, Copy, Debug)]
@@ -332,9 +337,10 @@
     let beam = &v["beam_theory"];
     let beam = match beam["model"].as_str() {
         Some(EULER_BERNOULLI) if keys(beam, &["model"]) => Beam::EulerBernoulli,
-        Some(TIMOSHENKO) if keys(beam, &["model", "shear_factor", "shear_area_m2"]) => Beam::Timoshenko {
+        Some(model @ (TIMOSHENKO | TIMOSHENKO_CURVED)) if keys(beam, &["model", "shear_factor", "shear_area_m2"]) => Beam::Timoshenko {
             shear_factor: number(&beam["shear_factor"])?,
             shear_area: number(&beam["shear_area_m2"])?,
+            curved: model == TIMOSHENKO_CURVED,
         },
         _ => return Err("SOURCE_PHYSICS_BEAM_THEORY".into()),
     };
@@ -352,14 +358,14 @@
     })
 }
 
-/// §3.1/§3.6: a realized arc states Euler-Bernoulli whatever the document
-/// selects; a Timoshenko straight states kappa_E and A_s = kappa_E * A_n bit
-/// for bit from its nominal radii and its case's nu.
+/// §3.1/§3.6: a realized arc states Euler-Bernoulli unless the document selects
+/// timoshenko_annulus_energy_curved_v2; a Timoshenko member states kappa_E and
+/// A_s = kappa_E * A_n bit for bit from its nominal radii and its case's nu.
 pub fn beam_theory(sections: &Sections, nu: f64, arc: bool) -> Check {
     match sections.beam {
         Beam::EulerBernoulli => Ok(()),
-        Beam::Timoshenko { .. } if arc => Err("SOURCE_PHYSICS_ARC_BEAM_THEORY".into()),
-        Beam::Timoshenko { shear_factor, shear_area } => {
+        Beam::Timoshenko { curved: false, .. } if arc => Err("SOURCE_PHYSICS_ARC_BEAM_THEORY".into()),
+        Beam::Timoshenko { shear_factor, shear_area, .. } => {
             let kappa = timoshenko_shear_factor(sections.nominal.ri, sections.ro, nu);
             require(
                 kappa.is_finite()
@@ -375,7 +381,8 @@
 pub fn model(sections: &Sections) -> &'static str {
     match sections.beam {
         Beam::EulerBernoulli => EULER_BERNOULLI,
-        Beam::Timoshenko { .. } => TIMOSHENKO,
+        Beam::Timoshenko { curved: false, .. } => TIMOSHENKO,
+        Beam::Timoshenko { curved: true, .. } => TIMOSHENKO_CURVED,
     }
 }
 
@@ -398,7 +405,7 @@
 /// realized arc is Euler-Bernoulli whatever the document selects; a straight
 /// member takes the contract's `pressure_contract.shear_model`.
 pub fn selected_model(shear_model: &str, arc: bool) -> &str {
-    if arc {
+    if arc && shear_model != TIMOSHENKO_CURVED {
         EULER_BERNOULLI
     } else {
         shear_model
@@ -420,7 +427,7 @@
 pub fn bind_shear_model(evidence: &Value, model: &Value) -> Check {
     let shear_model = model["pressure_contract"]["shear_model"]
         .as_str()
-        .filter(|m| [EULER_BERNOULLI, TIMOSHENKO].contains(m))
+        .filter(|m| [EULER_BERNOULLI, TIMOSHENKO, TIMOSHENKO_CURVED].contains(m))
         .ok_or("SOURCE_PHYSICS_SHEAR_MODEL_CONTRACT")?;
     let realized = realized_arcs(model);
     let mut arcs = realized.clone();
```

**`core/reporting/result_export/src/physics_evidence.rs`** (1 hunk). Identity binding: every member of a curved-shear publication states v2, and no member of any other publication does.

```diff
diff --git a/core/reporting/result_export/src/physics_evidence.rs b/core/reporting/result_export/src/physics_evidence.rs
--- a/core/reporting/result_export/src/physics_evidence.rs
+++ b/core/reporting/result_export/src/physics_evidence.rs
@@ -617,6 +617,14 @@
             // §3.1: kappa_E from this case's nu (per resolved case on load-state
             // documents); a marked arc is Euler-Bernoulli.
             crate::section_basis::beam_theory(&sections, number(&materials[pipe]["nu"])?, arc)?;
+            // curved-bend-transverse-shear.md §5: every member of a curved-shear publication
+            // states timoshenko_annulus_energy_curved_v2 (arcs included), and no member of any
+            // other publication does.
+            require(
+                (crate::section_basis::model(&sections) == crate::section_basis::TIMOSHENKO_CURVED)
+                    == (source["producer"]["semantic_contract_id"] == crate::semantic_contract::PRESSURE_CURVED_SHEAR_ID),
+                "SHEAR_MODEL_IDENTITY",
+            )?;
             if arc {
                 case_arcs.insert(pipe);
             } else {
```

**`core/reporting/result_export/src/arc_evidence.rs`** (1 hunk). Region geometry admits a v2 arc record (values and identity bound by the case section).

```diff
diff --git a/core/reporting/result_export/src/arc_evidence.rs b/core/reporting/result_export/src/arc_evidence.rs
--- a/core/reporting/result_export/src/arc_evidence.rs
+++ b/core/reporting/result_export/src/arc_evidence.rs
@@ -117,9 +117,17 @@
             "GEOMETRY",
         )?;
         // T4-U8 (D-6, §2.5, §3.6): a realized arc is Euler-Bernoulli under
-        // either shear model, and its geometry record says so.
+        // euler_bernoulli and timoshenko_annulus_energy_v1, and its geometry record says
+        // so; under timoshenko_annulus_energy_curved_v2 it states that model with its
+        // kappa_E and A_s (curved-bend-transverse-shear.md §5). The case section binds the
+        // values and the publication's identity (`physics_evidence`, `section_basis`).
+        let beam = &g["beam_theory"];
         require(
-            g["beam_theory"] == serde_json::json!({"model": crate::section_basis::EULER_BERNOULLI}),
+            *beam == serde_json::json!({"model": crate::section_basis::EULER_BERNOULLI})
+                || (beam["model"] == crate::section_basis::TIMOSHENKO_CURVED
+                    && beam.as_object().is_some_and(|b| b.len() == 3)
+                    && beam["shear_factor"].as_f64().is_some_and(|k| k.is_finite() && k > 0.0)
+                    && beam["shear_area_m2"].as_f64().is_some_and(|a| a.is_finite() && a > 0.0)),
             "BEAM_THEORY",
         )?;
         arcs.insert(g["pipe_id"].as_str().unwrap_or_default());
```

**`core/reporting/result_export/src/reader_verdict.rs`** (4 hunks). Bindings and routes for both identities; PF model-bound check maps v2 to its identity; EX model-bound check refuses a crossed identity (`SOURCE_PHYSICS_SHEAR_MODEL_IDENTITY`).

```diff
diff --git a/core/reporting/result_export/src/reader_verdict.rs b/core/reporting/result_export/src/reader_verdict.rs
--- a/core/reporting/result_export/src/reader_verdict.rs
+++ b/core/reporting/result_export/src/reader_verdict.rs
@@ -55,11 +55,13 @@
 /// `binding_hashes_are_the_table_files` checks them). A route's first binding is its
 /// default; the preview-physics route admits two identities (pf-timoshenko.md §4), and a
 /// source's binding is keyed by its `semantic_contract_id` (`binding_of`).
-const BINDINGS: [(&str, &str, &str); 6] = [
+const BINDINGS: [(&str, &str, &str); 8] = [
     ("pressure_source", sc::PRESSURE_SOURCE_ID, sc::PRESSURE_SOURCE_TABLE_SHA256),
     ("preview_physics", sc::PREVIEW_PHYSICS_ID, sc::PREVIEW_PHYSICS_SHA256),
     ("preview_physics", sc::PREVIEW_PHYSICS_TIMOSHENKO_ID, sc::PREVIEW_PHYSICS_TIMOSHENKO_SHA256),
+    ("preview_physics", sc::PREVIEW_PHYSICS_CURVED_SHEAR_ID, sc::PREVIEW_PHYSICS_CURVED_SHEAR_SHA256),
     ("pressure", sc::PRESSURE_ID, sc::PRESSURE_TABLE_SHA256),
+    ("pressure", sc::PRESSURE_CURVED_SHEAR_ID, sc::PRESSURE_CURVED_SHEAR_TABLE_SHA256),
     ("retained_preview_physics", sc::PREVIEW_PHYSICS_RETAINED_ID, sc::PREVIEW_PHYSICS_RETAINED_SHA256),
     ("retained_physics", sc::PHYSICS_RETAINED_ID, sc::PHYSICS_RETAINED_SHA256),
 ];
@@ -108,8 +110,8 @@
             crate::physics_source::validate_receipt_shape_in(receipt, crate::physics_source::SourceContract::PressureV3)?;
             "pressure_source"
         }
-        Some(sc::PREVIEW_PHYSICS_ID | sc::PREVIEW_PHYSICS_TIMOSHENKO_ID) => "preview_physics",
-        Some(sc::PRESSURE_ID) => {
+        Some(sc::PREVIEW_PHYSICS_ID | sc::PREVIEW_PHYSICS_TIMOSHENKO_ID | sc::PREVIEW_PHYSICS_CURVED_SHEAR_ID) => "preview_physics",
+        Some(sc::PRESSURE_ID | sc::PRESSURE_CURVED_SHEAR_ID) => {
             if !evidence_object {
                 return Err(SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED.into());
             }
@@ -216,8 +218,9 @@
         None => sc::PREVIEW_PHYSICS_ID,
         Some(Some("euler_bernoulli")) => sc::PREVIEW_PHYSICS_ID,
         Some(Some("timoshenko_annulus_energy_v1")) => sc::PREVIEW_PHYSICS_TIMOSHENKO_ID,
+        Some(Some("timoshenko_annulus_energy_curved_v2")) => sc::PREVIEW_PHYSICS_CURVED_SHEAR_ID,
         Some(_) => {
-            return Err(format!("{SHEAR_MODEL_IDENTITY}: MODEL-SETTING-SHEAR-MODEL-UNKNOWN: model_settings.shear_model must be `euler_bernoulli` or `timoshenko_annulus_energy_v1`"));
+            return Err(format!("{SHEAR_MODEL_IDENTITY}: MODEL-SETTING-SHEAR-MODEL-UNKNOWN: model_settings.shear_model must be `euler_bernoulli`, `timoshenko_annulus_energy_v1` or `timoshenko_annulus_energy_curved_v2`"));
         }
     };
     if source["producer"]["semantic_contract_id"] != expected {
@@ -277,6 +280,15 @@
     // contract selects for its member (a realized arc, Euler-Bernoulli).
     if model.get("pressure_contract").is_some_and(|v| !v.is_null()) {
         crate::section_basis::bind_shear_model(evidence, model)?;
+        // curved-bend-transverse-shear.md §5: the exact identity is the one the contract's
+        // shear model selects (the curved-shear identity iff v2), as on the preview route.
+        let curved = model["pressure_contract"]["shear_model"] == crate::section_basis::TIMOSHENKO_CURVED;
+        let expected = if curved { sc::PRESSURE_CURVED_SHEAR_ID } else { sc::PRESSURE_ID };
+        if sc::is_pressure_id(source["producer"]["semantic_contract_id"].as_str().unwrap_or_default())
+            && source["producer"]["semantic_contract_id"] != expected
+        {
+            return Err("SOURCE_PHYSICS_SHEAR_MODEL_IDENTITY".into());
+        }
     }
     if model.get("supports").is_some_and(|v| !v.is_null()) {
         let supports = ids(&model["supports"], "id");
```

**`core/reporting/modal_result/src/model_mass.rs`** (2 hunks). The modal mass reader classes v2 with v1 (modal refusal) instead of 'outside this reader's mass basis'.

```diff
diff --git a/core/reporting/modal_result/src/model_mass.rs b/core/reporting/modal_result/src/model_mass.rs
--- a/core/reporting/modal_result/src/model_mass.rs
+++ b/core/reporting/modal_result/src/model_mass.rs
@@ -81,6 +81,8 @@
 const MM_GEOMETRY_ONLY: &str = "mechanics_geometry_only";
 const MM_ARC: &str = "curved_bend_macro_element";
 const MM_TIMOSHENKO: &str = "timoshenko_annulus_energy_v1";
+/// DEL-04-01 `curved-bend-transverse-shear.md` §5: refused by the producer under the same code.
+const MM_TIMOSHENKO_CURVED: &str = "timoshenko_annulus_energy_curved_v2";
 /// The component-weight primitives' method (§2.3 item 5): their recorded
 /// acceleration agreement is not re-derived here.
 const MM_COMPONENT_WEIGHT_METHOD: &str = "component_weight_at_center_of_gravity_global_offset/v1";
@@ -680,7 +682,7 @@
     };
     match shear {
         None | Some("euler_bernoulli") => {}
-        Some(MM_TIMOSHENKO) => return mm_fail("the bound model selects Timoshenko members, which modal analysis refuses (MODAL_TIMOSHENKO_MASS_UNSUPPORTED)"),
+        Some(MM_TIMOSHENKO | MM_TIMOSHENKO_CURVED) => return mm_fail("the bound model selects Timoshenko members, which modal analysis refuses (MODAL_TIMOSHENKO_MASS_UNSUPPORTED)"),
         Some(other) => return mm_scope(format!("shear model `{other}` is outside this reader's mass basis")),
     }
```

### 2.3 Generator (DEL-05-02)

**`tools/serialization/semantic_contract_tables.py`** (4 hunks). `derive` writes the two new tables (PF: preview-physics-2 with identity/profile/limitation 0 replaced, via a shared `preview_variant`; EX: pressure-2 with identity/profile replaced); `apply-u4b` fills the PF branch's limitation const.

```diff
diff --git a/tools/serialization/semantic_contract_tables.py b/tools/serialization/semantic_contract_tables.py
--- a/tools/serialization/semantic_contract_tables.py
+++ b/tools/serialization/semantic_contract_tables.py
@@ -94,6 +94,16 @@
     "Timoshenko with the energy-matched annular shear factor (timoshenko_annulus_energy_v1) and Poisson's ratio "
     "E/(2G) - 1, and realized arcs are Euler-Bernoulli; a straight member's published nodal rotations are section "
     "rotations, not slopes; numerical integrity does not establish physical correctness.")
+# DEL-04-01 curved-bend-transverse-shear.md §5: the pressure-free curved-shear identity, preview-physics-2
+# with its identity, profile and limitation 0 replaced, as the Timoshenko one. Never hand-edited.
+CURVED = "fixtures/results/semantic_contract_v0_3_preview_physics_curved_shear_1.json"
+CURVED_ID = "openpipestress.result_semantics/0.3.0/preview-physics-curved-shear-1"
+CURVED_PROFILE = "product_preview_mechanics_curved_shear_v2"
+CURVED_LIMITATION_0 = "Small-displacement, linear-elastic frame preview: straight members, including geometry-only bends, and realized arcs are Timoshenko with the energy-matched annular shear factor (timoshenko_annulus_energy_curved_v2) and Poisson's ratio E/(2G) - 1; on a realized arc the straight-annulus factor is applied along the centreline with no curvature correction, the arc's thermal equivalent load is formed with that shear-dependent arc stiffness, as the solve is, and the arc's stress recovery stays the straight-annulus recovery; a straight member's published nodal rotations are section rotations, not slopes; numerical integrity does not establish physical correctness."
+# The exact curved-shear identity: pressure-2's table with its identity and profile replaced.
+PRESSURE_CURVED = "fixtures/results/semantic_contract_v0_3_pressure_curved_shear_1.json"
+PRESSURE_CURVED_ID = "openpipestress.result_semantics/0.3.0/pressure-curved-shear-1"
+PRESSURE_CURVED_PROFILE = "product_pressure_curved_shear_v2"
 SCHEMA = "schemas/results_pressure_2.v0.3.schema.json"
 # The members physics-retained-1 inherits unchanged from pressure-2. The retained
 # table's own members (identity, profile, limitations, receipt and formation
@@ -817,6 +827,10 @@
             # pf-timoshenko.md §4: preview-physics-2's texts with limitation 0 replaced, as the table.
             branch["properties"]["formulation_basis"]["properties"]["limitations"] = {
                 "const": [TIMOSHENKO_LIMITATION_0] + list(preview_limitations[1:])}
+        elif identity == CURVED_ID:
+            # curved-bend-transverse-shear.md §5: likewise for the curved-shear identity.
+            branch["properties"]["formulation_basis"]["properties"]["limitations"] = {
+                "const": [CURVED_LIMITATION_0] + list(preview_limitations[1:])}
 
 
 def pressure_schema(schema) -> None:
@@ -921,7 +935,14 @@
             raise SystemExit(f"{RETAINED}: inherited member {key} missing")
         retained[key] = pressure[key]
     retained["inherited_semantic_contract_sha256"] = sha256(pressure_bytes)
-    return {PRESSURE: pressure_bytes, RETAINED: dumps(retained), TIMOSHENKO: timoshenko_table()}
+    # curved-bend-transverse-shear.md §5: the exact curved-shear table, pressure-2's members in the
+    # same order with the identity and profile replaced.
+    pressure_curved = json.loads(pressure_bytes)
+    pressure_curved["semantic_contract_id"] = PRESSURE_CURVED_ID
+    pressure_curved["formulation_profile_id"] = PRESSURE_CURVED_PROFILE
+    return {PRESSURE: pressure_bytes, RETAINED: dumps(retained), TIMOSHENKO: timoshenko_table(),
+            CURVED: preview_variant(CURVED_ID, CURVED_PROFILE, CURVED_LIMITATION_0),
+            PRESSURE_CURVED: dumps(pressure_curved)}
 
 
 def timoshenko_table() -> bytes:
@@ -929,14 +950,20 @@
     the same order, with the identity, the profile and limitation 0 replaced. A preview-physics-2
     whose limitation 0 is not the Euler-Bernoulli sentence is refused, so a later edit of that
     sentence is reviewed here rather than carried silently."""
+    return preview_variant(TIMOSHENKO_ID, TIMOSHENKO_PROFILE, TIMOSHENKO_LIMITATION_0)
+
+
+def preview_variant(identity: str, profile: str, limitation_0: str) -> bytes:
+    """A preview-physics-2 variant (Timoshenko, curved shear): identity, profile and limitation 0
+    replaced, every other member and byte preview-physics-2's."""
     preview = json.loads((PROJECT / PREVIEW).read_bytes())
     if preview["semantic_contract_id"] != PREVIEW_IDENTITIES[0] or preview["formulation_profile_id"] != "product_preview_mechanics_v1":
         raise SystemExit(f"{PREVIEW}: unexpected identity or profile")
     if preview["supported_profile_limitations"][0] != PREVIEW_LIMITATION_0:
         raise SystemExit(f"{PREVIEW}: limitation 0 is not the Euler-Bernoulli sentence this derivation replaces")
-    preview["semantic_contract_id"] = TIMOSHENKO_ID
-    preview["formulation_profile_id"] = TIMOSHENKO_PROFILE
-    preview["supported_profile_limitations"][0] = TIMOSHENKO_LIMITATION_0
+    preview["semantic_contract_id"] = identity
+    preview["formulation_profile_id"] = profile
+    preview["supported_profile_limitations"][0] = limitation_0
     return dumps(preview)
```

### 2.4 Schemas (DEL-02-01, DEL-13-04, DEL-00-06/DEL-08-04)

**`schemas/model.schema.yaml`** (2 hunks). Both enums (`pressure_contract.shear_model`, `model_settings.shear_model`) gain v2, with descriptions.

```diff
diff --git a/schemas/model.schema.yaml b/schemas/model.schema.yaml
--- a/schemas/model.schema.yaml
+++ b/schemas/model.schema.yaml
@@ -2226,10 +2226,12 @@
           "const": "exact_pressure_v3"
         },
         "shear_model": {
+          "description": "euler_bernoulli; timoshenko_annulus_energy_v1 (Timoshenko straight members, realized arcs Euler–Bernoulli); or timoshenko_annulus_energy_curved_v2 (v1's straight members, and realized arcs with transverse shear by the same energy-matched annular factor; DEL-04-01 Design curved-bend-transverse-shear.md). Any other value is refused by name.",
           "type": "string",
           "enum": [
             "euler_bernoulli",
-            "timoshenko_annulus_energy_v1"
+            "timoshenko_annulus_energy_v1",
+            "timoshenko_annulus_energy_curved_v2"
           ]
         }
       }
@@ -2946,11 +2948,12 @@
       "additionalProperties": false,
       "properties": {
         "shear_model": {
-          "description": "Model document 0.8.0 (DEL-02-01 Design model-document-0.8.0.md §1, §4): the pressure-free document's beam theory, in the exact contract's vocabulary. Absent: Euler–Bernoulli. An explicit user set of either value writes the member (as for the tolerance); the product never writes the default into a document the user did not author, and clear restores the absent form. timoshenko_annulus_energy_v1 makes every straight member of the pressure-free document Timoshenko, with the exact route's energy-matched annular shear factor and Poisson's ratio E/(2G) - 1; realized arcs stay Euler–Bernoulli (DEL-05-03 Design pf-timoshenko.md). Refused on a document with pressure_contract (MODEL-SETTING-SHEAR-MODEL-WITH-PRESSURE-CONTRACT; that document's choice is pressure_contract.shear_model); any other value, null included, is refused (MODEL-SETTING-SHEAR-MODEL-UNKNOWN). euler_bernoulli solves exactly as absent.",
+          "description": "Model document 0.8.0 (DEL-02-01 Design model-document-0.8.0.md §1, §4): the pressure-free document's beam theory, in the exact contract's vocabulary. Absent: Euler–Bernoulli. An explicit user set of any value writes the member (as for the tolerance); the product never writes the default into a document the user did not author, and clear restores the absent form. timoshenko_annulus_energy_v1 makes every straight member of the pressure-free document Timoshenko, with the exact route's energy-matched annular shear factor and Poisson's ratio E/(2G) - 1; realized arcs stay Euler–Bernoulli (DEL-05-03 Design pf-timoshenko.md). timoshenko_annulus_energy_curved_v2 forms the same straight members and adds transverse shear to every realized arc by the same factor (DEL-04-01 Design curved-bend-transverse-shear.md). Refused on a document with pressure_contract (MODEL-SETTING-SHEAR-MODEL-WITH-PRESSURE-CONTRACT; that document's choice is pressure_contract.shear_model); any other value, null included, is refused (MODEL-SETTING-SHEAR-MODEL-UNKNOWN). euler_bernoulli solves exactly as absent.",
           "type": "string",
           "enum": [
             "euler_bernoulli",
-            "timoshenko_annulus_energy_v1"
+            "timoshenko_annulus_energy_v1",
+            "timoshenko_annulus_energy_curved_v2"
           ]
         },
         "node_proximity_tolerance": {
```

**`schemas/physical_transform_model.schema.yaml`** (2 hunks). The twin's `PressureContract` enum and description (it has no `model_settings`).

```diff
diff --git a/schemas/physical_transform_model.schema.yaml b/schemas/physical_transform_model.schema.yaml
--- a/schemas/physical_transform_model.schema.yaml
+++ b/schemas/physical_transform_model.schema.yaml
@@ -932,7 +932,7 @@
       ]
     },
     "PressureContract": {
-      "description": "Current exact pressure contract: a closed object with explicit version, mode and shear_model. Present null, scalars and arrays are invalid. Operative preview pipe sections require an explicit local corrosion_allowance; zero is authored, never defaulted, and a nonzero allowance is admitted with mill tolerance plus allowance below the wall. shear_model selects the straight members' shear deformation: euler_bernoulli, or timoshenko_annulus_energy_v1 (the energy-matched annular shear factor on the nominal section); realized arcs are Euler–Bernoulli under either. Stiffness and pipe metal mass use the nominal wall; contents mass and pressure end caps use the reduced bore; stresses are published on both the nominal and the reduced section (wall minus mill tolerance and corrosion allowance). Canonical Section.properties uses its separate code-neutral Quantity representation.",
+      "description": "Current exact pressure contract: a closed object with explicit version, mode and shear_model. Present null, scalars and arrays are invalid. Operative preview pipe sections require an explicit local corrosion_allowance; zero is authored, never defaulted, and a nonzero allowance is admitted with mill tolerance plus allowance below the wall. shear_model selects the shear deformation: euler_bernoulli; timoshenko_annulus_energy_v1 (the energy-matched annular shear factor on the nominal section for straight members; realized arcs Euler–Bernoulli); or timoshenko_annulus_energy_curved_v2 (the same straight members, and realized arcs with transverse shear by the same factor). Stiffness and pipe metal mass use the nominal wall; contents mass and pressure end caps use the reduced bore; stresses are published on both the nominal and the reduced section (wall minus mill tolerance and corrosion allowance). Canonical Section.properties uses its separate code-neutral Quantity representation.",
       "type": "object",
       "additionalProperties": false,
       "required": [
@@ -953,7 +953,8 @@
           "type": "string",
           "enum": [
             "euler_bernoulli",
-            "timoshenko_annulus_energy_v1"
+            "timoshenko_annulus_energy_v1",
+            "timoshenko_annulus_energy_curved_v2"
           ]
         }
       }
```

**`schemas/results.v0.3.schema.yaml`** (1 hunk). A PF curved-shear envelope branch, a copy of the Timoshenko branch with id, profile and limitation 0 replaced (limitations regenerated by `apply-u4b`). The EX schema work is in §3 and §2.8, not hand-drafted here.

```diff
diff --git a/schemas/results.v0.3.schema.yaml b/schemas/results.v0.3.schema.yaml
--- a/schemas/results.v0.3.schema.yaml
+++ b/schemas/results.v0.3.schema.yaml
@@ -1411,6 +1411,91 @@
           ]
         },
         {
+          "required": [
+            "contract_evidence"
+          ],
+          "properties": {
+            "producer": {
+              "properties": {
+                "semantic_contract_id": {
+                  "const": "openpipestress.result_semantics/0.3.0/preview-physics-curved-shear-1"
+                }
+              }
+            },
+            "semantic_contract_ref": {
+              "properties": {
+                "ref_id": {
+                  "const": "openpipestress.result_semantics/0.3.0/preview-physics-curved-shear-1"
+                }
+              }
+            },
+            "formulation_basis": {
+              "properties": {
+                "profile_id": {
+                  "const": "product_preview_mechanics_curved_shear_v2"
+                },
+                "limitations": {
+                  "const": [
+                    "Small-displacement, linear-elastic frame preview: straight members, including geometry-only bends, and realized arcs are Timoshenko with the energy-matched annular shear factor (timoshenko_annulus_energy_curved_v2) and Poisson's ratio E/(2G) - 1; on a realized arc the straight-annulus factor is applied along the centreline with no curvature correction, the arc's thermal equivalent load is formed with that shear-dependent arc stiffness, as the solve is, and the arc's stress recovery stays the straight-annulus recovery; a straight member's published nodal rotations are section rotations, not slopes; numerical integrity does not establish physical correctness.",
+                    "Nonzero pressure is refused on this route.",
+                    "On every member the section normal-stress extremes are N/A ± hypot(My,Mz)/Z at each station and the transverse shear maximum is the straight-annulus Saint-Venant value, with ν = E/(2G) − 1 assuming an isotropic material. On straight members the normal-stress maximum is |N/A| + hypot(My,Mz)/Z, bounded over all statics intervals. Torsional shear is separate. No equivalent stress or code stress.",
+                    "On arcs: signed tangent-frame resultants, nominal stress components and section extremes; endpoint force rows are node-on-element actions in the tangent frame at their end. The normal-stress maximum is bounded along the arc and enters the stress headline. The transverse shear uses the straight-annulus field without curvature correction.",
+                    "Intensified measures are i*hypot(My,Mz)/Z with the user's scalar SIF at member ends adjacent to markers and branches, using member Z. They are not code stresses and are never combined. A flexibility factor never multiplies stress. On realized arcs with directional SIFs, intensified bending is hypot(i_ip*Mz, i_op*My)/Z in the tangent frame, with its along-arc maximum.",
+                    "Signed support actions for rigid restraints, springs, consuming constant effort and attributable nonlinear supports. Ambiguous attribution is withheld.",
+                    "Headlines cover all load cases. In a model with nonlinear supports a mechanics combination is withheld unless one term is a solved load case with factor exactly 1, at least one other term exists, and every other term is a combination whose fixed-load count κ is defined and exactly 0; it is then formed from rows, with its operands' standing, as a solved state plus derived differences or bounds, not an equilibrium state. With a consuming constant effort a mechanics combination is withheld when its κ is undefined or differs from 1 by more than 64ε, and an absolute sum or SRSS unless every operand's κ is exactly 0. Mechanics combinations, absolute sums and SRSS are also withheld for mixed moduli over the load cases they reach, and every combination is withheld with a withheld operand combination. Absolute sums and SRSS are unsigned. Subtraction is a labelled difference. No combination maxima and no code compliance.",
+                    "Member actions, section actions, stresses and support reactions of a load case reported NUMERICAL_INTEGRITY_CHECKS_PASSED are recovered at 128-bit precision from the formation check's corrected displacement and rounded once; they are not binary64 recomputations from the solve's own displacement, from which they may differ by about 1e-12 of the body's action scale (at most about 6e-9 in the evaluation sets).",
+                    "The recovery is checked against the formation check's re-formed system (stiffness and loads formed at 128-bit precision from the binary64 inputs). Wherever the formation check produced its member bound for a load case, including a case that the member recovery check then reports as sensitive, each published nodal displacement component is the corrected displacement (the solve's displacement plus the check's two correction terms, summed exactly) rounded once to binary64 in SI units and then converted to the published unit as every other row is, and each displacement magnitude is the correctly rounded norm of the node's three published translation components; where there is no such bound, or where any component of the case cannot be formed, every published displacement of the case is the solve's own, and a selected retained source publishes its recovery's nodal values. Support reactions balance the re-formed loads, not the solve's binary64 load vector."
+                  ]
+                }
+              }
+            },
+            "contract_evidence": {
+              "$ref": "#/$defs/PreviewPhysicsContractEvidence"
+            },
+            "result_sets": {
+              "items": {
+                "$ref": "#/$defs/PreviewPhysicsResultSet"
+              }
+            }
+          },
+          "not": {
+            "anyOf": [
+              {
+                "required": [
+                  "source_block_recovery"
+                ]
+              }
+            ]
+          },
+          "allOf": [
+            {
+              "not": {
+                "required": [
+                  "retained_precision"
+                ]
+              }
+            },
+            {
+              "properties": {
+                "row_disclosures": {
+                  "items": {
+                    "properties": {
+                      "reason_code": {
+                        "not": {
+                          "enum": [
+                            "retained_precision_absolute_verified",
+                            "retained_precision_not_covered"
+                          ]
+                        }
+                      }
+                    }
+                  }
+                }
+              }
+            }
+          ]
+        },
+        {
           "$ref": "results_pressure_2.v0.3.schema.json#/$defs/PressureResultEnvelope"
         },
         {
```

### 2.5 Operation applier (DEL-16-06, DEL-04-07)

**`core/model_operations/operation_applier/src/lib.rs`** (2 hunks). `is_pressure_v3_contract` admits v2; the remediation lists it. `model_settings` authoring and the document validator need no hunk: they call `shear_model_refusal(ShearModelSetting::of(..))`.

```diff
diff --git a/core/model_operations/operation_applier/src/lib.rs b/core/model_operations/operation_applier/src/lib.rs
--- a/core/model_operations/operation_applier/src/lib.rs
+++ b/core/model_operations/operation_applier/src/lib.rs
@@ -927,7 +927,7 @@
             && contract.get("version").and_then(Value::as_str) == Some("3.0.0")
             && contract.get("mode").and_then(Value::as_str) == Some("exact_pressure_v3")
             && matches!(contract.get("shear_model").and_then(Value::as_str),
-                Some("euler_bernoulli" | "timoshenko_annulus_energy_v1")))
+                Some("euler_bernoulli" | "timoshenko_annulus_energy_v1" | "timoshenko_annulus_energy_curved_v2")))
 }
 
 fn collection_for(object_type: &str) -> Option<&'static str> {
@@ -6004,7 +6004,7 @@
             "OP-PRESSURE-PRIMITIVE-RETIRED",
             "blocking",
             "Pressure primitive loads are retired, zero values included; the product refuses them (PRESSURE_MODEL_REAUTHOR_REQUIRED).".to_string(),
-            &format!("Author pressure as exact pressure regions: on model {} select the 3.0.0/exact_pressure_v3 profile (shear_model timoshenko_annulus_energy_v1, the default, or euler_bernoulli) with an explicit section.corrosion_allowance on every pipe, then enter the load case's pressure_regions (an explicit [] for an unpressurized case).", open_pipe_stress_product_physics::MODEL_SCHEMA_VERSION),
+            &format!("Author pressure as exact pressure regions: on model {} select the 3.0.0/exact_pressure_v3 profile (shear_model timoshenko_annulus_energy_v1, the default, timoshenko_annulus_energy_curved_v2 or euler_bernoulli) with an explicit section.corrosion_allowance on every pipe, then enter the load case's pressure_regions (an explicit [] for an unpressurized case).", open_pipe_stress_product_physics::MODEL_SCHEMA_VERSION),
             vec![target_ref.to_string()],
         );
         return None;
```

**`core/model_operations/operation_applier/src/pressure_authoring.rs`** (1 hunk). The successor-required refusal lists v2.

```diff
diff --git a/core/model_operations/operation_applier/src/pressure_authoring.rs b/core/model_operations/operation_applier/src/pressure_authoring.rs
--- a/core/model_operations/operation_applier/src/pressure_authoring.rs
+++ b/core/model_operations/operation_applier/src/pressure_authoring.rs
@@ -296,7 +296,7 @@
             if !crate::is_pressure_v3_contract(&after) {
                 return Err(RichError {
                     code: "OP-PRESSURE-PROFILE-SUCCESSOR-REQUIRED",
-                    message: "New pressure authoring requires the closed 3.0.0/exact_pressure_v3 contract with an explicit shear_model: timoshenko_annulus_energy_v1 (the default) or euler_bernoulli".into(),
+                    message: "New pressure authoring requires the closed 3.0.0/exact_pressure_v3 contract with an explicit shear_model: timoshenko_annulus_energy_v1 (the default), timoshenko_annulus_energy_curved_v2 or euler_bernoulli".into(),
                 });
             }
             if model["schema_version"] != open_pipe_stress_product_physics::MODEL_SCHEMA_VERSION
```

### 2.6 Desktop (DEL-02-01, DEL-04-06, DEL-07-03, DEL-07-05, DEL-07-11)

**`apps/desktop/src/types.ts`** (2 hunks). `pressure_contract.shear_model` union and the `ModelSettings` doc.

```diff
diff --git a/apps/desktop/src/types.ts b/apps/desktop/src/types.ts
--- a/apps/desktop/src/types.ts
+++ b/apps/desktop/src/types.ts
@@ -381,7 +381,7 @@
 export type SupportDirection = { x: number; y: number; z: number };
 
 export type PreviewModel = {
-  pressure_contract?: { version: string; mode: string; shear_model?: "euler_bernoulli" | "timoshenko_annulus_energy_v1" };
+  pressure_contract?: { version: string; mode: string; shear_model?: "euler_bernoulli" | "timoshenko_annulus_energy_v1" | "timoshenko_annulus_energy_curved_v2" };
   schema_version: string;
   document_kind: string;
   data_boundary: Record<string, string>;
@@ -611,8 +611,9 @@
 /** `model_settings`: a closed record. `node_proximity_tolerance` is the near-duplicate node
  * tolerance, a length quantity; absent means 5 mm. `shear_model` (model document 0.8.0) is the
  * pressure-free document's beam theory, absent meaning Euler–Bernoulli, or
- * `timoshenko_annulus_energy_v1` (DEL-05-03 Design `pf-timoshenko.md`); it is refused beside a
- * pressure contract.
+ * `timoshenko_annulus_energy_v1` (DEL-05-03 Design `pf-timoshenko.md`), or
+ * `timoshenko_annulus_energy_curved_v2` (realized arcs too; DEL-04-01 Design
+ * `curved-bend-transverse-shear.md`); it is refused beside a pressure contract.
  * The desktop never writes a default. */
 export type ModelSettings = {
   node_proximity_tolerance?: QuantityValue;
```

**`apps/desktop/src/features/pressure-authoring/pressureContract.ts`** (2 hunks). `ShearModel`, `isShearModel` and the pressure selector's choices gain v2 (default stays v1).

```diff
diff --git a/apps/desktop/src/features/pressure-authoring/pressureContract.ts b/apps/desktop/src/features/pressure-authoring/pressureContract.ts
--- a/apps/desktop/src/features/pressure-authoring/pressureContract.ts
+++ b/apps/desktop/src/features/pressure-authoring/pressureContract.ts
@@ -1,14 +1,17 @@
 import type { PreviewModel } from "../../types";
 import { MODEL_SCHEMA_VERSION } from "../../services/modelSchemaVersion";
 
-/** The two admitted straight-member shear models of the exact contract (T4-U8, D-6/O3). */
-export type ShearModel = "euler_bernoulli" | "timoshenko_annulus_energy_v1";
+/** The admitted shear models of the exact contract (T4-U8, D-6/O3; DEL-04-01
+ * `curved-bend-transverse-shear.md` §5 adds the curved v2). */
+export type ShearModel = "euler_bernoulli" | "timoshenko_annulus_energy_v1" | "timoshenko_annulus_energy_curved_v2";
 /** From T4-U8 on, authoring writes Timoshenko by default; Euler-Bernoulli remains an
- * explicit choice. Realized arcs are Euler-Bernoulli under either model. */
+ * explicit choice. Realized arcs are Euler-Bernoulli under v1 and carry shear only under the
+ * explicit v2, never by default. */
 export const DEFAULT_SHEAR_MODEL: ShearModel = "timoshenko_annulus_energy_v1";
 /** The selector's choices, the default first, with their user-facing labels. */
 export const SHEAR_MODEL_CHOICES: readonly { value: ShearModel; label: string }[] = Object.freeze([
   { value: "timoshenko_annulus_energy_v1", label: "Timoshenko (energy-matched annular shear factor)" },
+  { value: "timoshenko_annulus_energy_curved_v2", label: "Timoshenko, straight members and bends (energy-matched annular factor)" },
   { value: "euler_bernoulli", label: "Euler–Bernoulli" },
 ]);
 
@@ -19,7 +22,7 @@
 type ContractModel = Pick<PreviewModel, "schema_version" | "pressure_contract">;
 
 export function isShearModel(value: unknown): value is ShearModel {
-  return value === "euler_bernoulli" || value === "timoshenko_annulus_energy_v1";
+  return value === "euler_bernoulli" || value === "timoshenko_annulus_energy_v1" || value === "timoshenko_annulus_energy_curved_v2";
 }
 
 /** A document declaring `3.0.0/exact_pressure_v3`. */
```

**`apps/desktop/src/features/model-settings/ModelSettingsSection.tsx`** (1 hunk). The third radio with the approved label; the note states the arc consequence and the modal refusal. The legend change is open (§7 O-5).

```diff
diff --git a/apps/desktop/src/features/model-settings/ModelSettingsSection.tsx b/apps/desktop/src/features/model-settings/ModelSettingsSection.tsx
--- a/apps/desktop/src/features/model-settings/ModelSettingsSection.tsx
+++ b/apps/desktop/src/features/model-settings/ModelSettingsSection.tsx
@@ -37,13 +37,14 @@
   type SettingPath
 } from "./modelSettingsIntents";
 
-/** The straight-member shear deformation choices (pf-timoshenko.md §1), default first. */
+/** The shear deformation choices (pf-timoshenko.md §1; curved-bend-transverse-shear.md §5), default first. */
 export const SHEAR_MODEL_CHOICES = Object.freeze([
   { value: "euler_bernoulli", label: "Euler–Bernoulli (the default)" },
-  { value: "timoshenko_annulus_energy_v1", label: "Timoshenko (energy-matched annular factor)" }
+  { value: "timoshenko_annulus_energy_v1", label: "Timoshenko (energy-matched annular factor)" },
+  { value: "timoshenko_annulus_energy_curved_v2", label: "Timoshenko, straight members and bends (energy-matched annular factor)" }
 ] as const);
 export const SHEAR_MODEL_NOTE =
-  "Applies to every straight member, including tees' straight legs, rigid-family components and geometry-only bends. Realized arcs stay Euler–Bernoulli. Timoshenko members use Poisson's ratio E/(2G) − 1 from each case's moduli; an authored Poisson's ratio is not used.";
+  "Applies to every straight member, including tees' straight legs, rigid-family components and geometry-only bends. Realized arcs stay Euler–Bernoulli unless you choose straight members and bends, which applies the same factor along each arc (no curvature correction; arc stresses stay the straight-annulus recovery). Timoshenko members use Poisson's ratio E/(2G) − 1 from each case's moduli; an authored Poisson's ratio is not used. Modal analysis refuses either Timoshenko choice.";
 export const SHEAR_MODEL_EXACT_NOTE =
   "This document has a pressure contract, so its beam theory is the pressure contract's shear model: set it in the Pressure panel.";
```

**`apps/desktop/src/features/model-settings/modelSettingsIntents.ts`** (1 hunk). Doc only.

```diff
diff --git a/apps/desktop/src/features/model-settings/modelSettingsIntents.ts b/apps/desktop/src/features/model-settings/modelSettingsIntents.ts
--- a/apps/desktop/src/features/model-settings/modelSettingsIntents.ts
+++ b/apps/desktop/src/features/model-settings/modelSettingsIntents.ts
@@ -126,7 +126,8 @@
   return authored === null ? "not_present" : canonicalize(authored);
 }
 
-/** Set the shear model to `name`, `euler_bernoulli` or `timoshenko_annulus_energy_v1` (the
+/** Set the shear model to `name`, `euler_bernoulli`, `timoshenko_annulus_energy_v1` or
+ * `timoshenko_annulus_energy_curved_v2` (the
  * engine refuses by name an unknown name and a document with a pressure contract). */
 export async function setShearModelIntent(canonicalize: Canonicalize, model: PreviewModel, name: string): Promise<EditorOperationIntent> {
   return intent(model, "set_model_setting", await shearBefore(canonicalize, model), name, "none", "dimensionless", "Set shear model", SHEAR_MODEL_PATH);
```

**`apps/desktop/src/features/results/numericalResultQuality.ts`** (1 hunk). The DEL-02-01 reader's identity constants and route lists for both identities.

```diff
diff --git a/apps/desktop/src/features/results/numericalResultQuality.ts b/apps/desktop/src/features/results/numericalResultQuality.ts
--- a/apps/desktop/src/features/results/numericalResultQuality.ts
+++ b/apps/desktop/src/features/results/numericalResultQuality.ts
@@ -15,14 +15,26 @@
 export const PRESSURE_SOURCE_CONTRACT_ID = "openpipestress.result_semantics/0.3.0/pressure-source-1";
 /** T4-U2: the ordinary exact identity (3.0.0/exact_pressure_v3). */
 export const PRESSURE_CONTRACT_ID = "openpipestress.result_semantics/0.3.0/pressure-2";
+/** DEL-04-01 `curved-bend-transverse-shear.md` §5: the exact identity of a document whose
+ * `pressure_contract.shear_model` is `timoshenko_annulus_energy_curved_v2`. */
+export const PRESSURE_CURVED_SHEAR_CONTRACT_ID = "openpipestress.result_semantics/0.3.0/pressure-curved-shear-1";
+/** The ordinary exact route's identities. */
+export const PRESSURE_CONTRACT_IDS: readonly string[] = Object.freeze([PRESSURE_CONTRACT_ID, PRESSURE_CURVED_SHEAR_CONTRACT_ID]);
 export const PREVIEW_PHYSICS_CONTRACT_ID = "openpipestress.result_semantics/0.3.0/preview-physics-2";
 /** DEL-05-03 Design `pf-timoshenko.md` §4: the pressure-free Timoshenko identity, a second
  * complete identity on the `preview_physics` route (a document whose
  * `model_settings.shear_model` selects `timoshenko_annulus_energy_v1`). Its table is
  * preview-physics-2's with the identity, the profile and limitation 0 replaced. */
 export const PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID = "openpipestress.result_semantics/0.3.0/preview-physics-timoshenko-1";
+/** DEL-04-01 `curved-bend-transverse-shear.md` §5: the pressure-free curved-shear identity
+ * (`model_settings.shear_model` `timoshenko_annulus_energy_curved_v2`), derived as the Timoshenko one. */
+export const PREVIEW_PHYSICS_CURVED_SHEAR_CONTRACT_ID = "openpipestress.result_semantics/0.3.0/preview-physics-curved-shear-1";
 /** The `preview_physics` route's identities, Euler–Bernoulli first. */
-export const PREVIEW_PHYSICS_CONTRACT_IDS: readonly string[] = Object.freeze([PREVIEW_PHYSICS_CONTRACT_ID, PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID]);
+export const PREVIEW_PHYSICS_CONTRACT_IDS: readonly string[] = Object.freeze([
+  PREVIEW_PHYSICS_CONTRACT_ID,
+  PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID,
+  PREVIEW_PHYSICS_CURVED_SHEAR_CONTRACT_ID,
+]);
 /** U6d (D-U6-6; D2 4.7 S-1, 4.9.6): the F2a preview successor. Its statement is checked
  * only by the accepted reader (the Rust one, through `retainedPrecisionReader.ts`), and
  * standing comes only from that reader's registered validation
```

**`apps/desktop/src/features/results/knownSemanticLimitations.ts`** (1 hunk). Fresh-result admission of both identities.

```diff
diff --git a/apps/desktop/src/features/results/knownSemanticLimitations.ts b/apps/desktop/src/features/results/knownSemanticLimitations.ts
--- a/apps/desktop/src/features/results/knownSemanticLimitations.ts
+++ b/apps/desktop/src/features/results/knownSemanticLimitations.ts
@@ -21,8 +21,11 @@
   PREVIEW_PHYSICS_CONTRACT_ID,
   // DEL-05-03 pf-timoshenko.md §4: the pressure-free Timoshenko identity (Rust's set too).
   PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID,
+  // DEL-04-01 curved-bend-transverse-shear.md §5: either route's curved-shear identity (Rust's set too).
+  PREVIEW_PHYSICS_CURVED_SHEAR_CONTRACT_ID,
   PRESSURE_SOURCE_CONTRACT_ID,
   PRESSURE_CONTRACT_ID,
+  PRESSURE_CURVED_SHEAR_CONTRACT_ID,
   PREVIEW_PHYSICS_RETAINED_CONTRACT_ID,
 ]);
```

### 2.7 Governing text (DEL-04-01)

**`execution/PKG-04/DEL-04-01/Work/curved-bend-transverse-shear.md`** (1 hunk). §5's status sentence. The other governing edits are listed in §5 of this draft (commitment text needs independent review).

```diff
diff --git a/execution/PKG-04/DEL-04-01/Work/curved-bend-transverse-shear.md b/execution/PKG-04/DEL-04-01/Work/curved-bend-transverse-shear.md
--- a/execution/PKG-04/DEL-04-01/Work/curved-bend-transverse-shear.md
+++ b/execution/PKG-04/DEL-04-01/Work/curved-bend-transverse-shear.md
@@ -179,9 +179,9 @@
 ## 5. Minimal implementation boundary
 
 This boundary was audited at `5f2946ef12`. Step A, the kernel and
-certification, is implemented on branch `codex/bend-transverse-shear`; no
-product path sets the shear area yet. Step B, product admission, is reserved
-until Agent 0 reviews its actual diff.
+certification, is implemented. Step B admits the explicit
+`timoshenko_annulus_energy_curved_v2` on both routes, with the identities,
+refusals and consumers listed below.
 
 ### Step A: kernel and certification (DEL-04-01; FK structural)
```

### 2.8 Rule-derived consumer sites (anchors at `f18834a6b`; re-derive on `pressure-3`)

These sites are not drafted as hunks. Each is a list entry or an arm beside an existing
one, and F-1 rewrites the EX ones first. Two rules apply:

- **Rule P (PF).** Wherever `PREVIEW_PHYSICS_TIMOSHENKO_ID`, `…_PROFILE`, `…_SHA256`, its table
  path or `preview_physics_timoshenko_contract` appears in a list, match or map, add the
  curved-shear counterpart beside it. Where a site keys the Timoshenko report projection, add
  the curved-shear limitation-0 projection.
- **Rule E (EX).** Wherever `PRESSURE_ID` (or the `pressure-2` literal, table or schema)
  admits or dispatches an ordinary exact publication, admit `PRESSURE_CURVED_SHEAR_ID` through
  `is_pressure_id`/`PRESSURE_IDENTITIES` and select its table/profile. Retained and source sites
  (`physics-retained-1` base, `pressure-source-1`) stay `pressure-2`/`pressure-3` only, by design.

| Site (file:line) | Rule | Note |
|---|---|---|
| `result_export/src/derivative.rs:161-164` | P, E | Allowlist that copies contract evidence. |
| `result_export/src/recorded_tables.rs` | none | No recorded variant exists for new identities; confirm `selected_table` returns `None`. |
| `result_export/src/preview_physics_evidence.rs`, `u4b_evidence.rs` `preview_model_checks` | P | Already select by `preview_physics_identity_of`; check no 2-arm match remains. |
| `result_export/src/u4b_evidence.rs:554`, `exact_combination_evidence.rs:1201`, `physics_evidence.rs:16,445` | E | They read `pressure_rows_contract()`; rows are identical, so keep it or select by identity (choose one, state it). |
| `result_export/src/load_reference.rs:335` | E | Projected source must keep `producer`, or the #22 identity check misreads it. Verify. |
| `report_package/src/document_form.rs:24` `ADMITTED_SEMANTIC_CONTRACTS` | P, E | |
| `report_package/src/exact_sections.rs:35` `EXACT_IDENTITIES` | E | Plus EX v2 limitation-0 statement in the statement tables. |
| `report_package/src/user_release.rs:876` `check_pressure_free` | P, E | Refuse both new identities by name (as PF Timoshenko), or admit; open (O-7). |
| `report_package/src/pf_timoshenko.rs:27,41,45,52` and `bindings.rs` | P | Curved-shear projection of limitation 0. |
| `runner/headless/src/lib.rs:1526`, `result_envelope_binding.rs:556,816` | E | Mostly test asserts; production list at 1526. |
| `apps/desktop/src-tauri/src/report_render_exact.rs:72,76`, `rederive_pf_timoshenko` | P, E | |
| `apps/desktop/src-tauri/src/analysis_run_v03_validate.rs`, `analysis_run_v03_build.rs:714`, `src-tauri/src/lib.rs` | P, E | |
| `core/analysis_runs/compatibility.py:495,543-586` | P, E | Ids, hashes, table paths, fresh set. |
| `core/handoff/stress_neutral/package.py:24,100,105` | P, E | `SUPPORTED_METHODS`, `PREVIEW_METHODS`. |
| `tests/verifiers/compatibility.py:53-95` | P, E | |
| `tests/verifiers/reader_verdict.py:62,233-249` | P, E | Twin of #24, including the three-name message. |
| `tests/verifiers/section_basis.py:49,187-188,262` | E | Twin of #21. |
| `tests/verifiers/physics_evidence.py:43`, `pressure_2.py:65,254` | E | Twin of #22–#23; `pressure_2.py` becomes the pressure-3 verifier under F-1. |
| `tests/verifiers/preview_physics_evidence.py` | P | |
| `tests/verifiers/modal_model_mass.py:95,649,655` | modal | Twin of #28. |
| Desktop `StressNeutralExportPanel.tsx`, `analysisRunCompatibility.ts`, `contractBindings.ts` (test support) | P, E | Table file by identity, `RECORD_TABLES`, computed hashes. |
| Desktop `exactReportAdmission.ts`, `exactReportSections.ts`, `reportPackageRequest.ts`, `renderableReportInput.ts`, `pfTimoshenkoReport.ts`, `exactReportLimitations.ts`, `nativeResult.ts` | P, E | Report admission and limitation statements. |
| `schemas/stress_neutral_export.v0.3.schema.json` (`PhysicsBeamTheory` ~2650, ids) | P, E | Generated alongside results.v0.3 where the generator covers it. |
| `schemas/analysis_run.v0.3.schema.json:254-301` | P, E | Ids and table hashes. |
| `tests/test_preview_identity_inventory.py` | P | Third identity marker; add an EX inventory guard keyed on the pressure identity. |

## 3. Generated artefacts (regenerate, never hand-edit)

Run from the checkout root after the source hunks, in this order:

1. `python3 tools/serialization/semantic_contract_tables.py apply-u4b`. This fills the PF
   curved-shear branch's limitation const in `schemas/results.v0.3.schema.yaml`.
2. `python3 tools/serialization/semantic_contract_tables.py derive`. This writes
   `fixtures/results/semantic_contract_v0_3_preview_physics_curved_shear_1.json` and
   `…_pressure_curved_shear_1.json` (on F-1's base, derived from `pressure-3`). Copy the printed
   SHA-256 values into the `*_CURVED_SHEAR_*SHA256` placeholders in `semantic_contract.rs`, and
   into the Python, desktop and analysis-run twins. `derive --check` must then report
   `unchanged` for preview-physics-2, pressure-2/3, physics-retained-1 and
   preview-physics-timoshenko-1. That is the byte check that EB/v1 tables do not move.
3. **EX schema (new generator code, not drafted):** a `derive` output
   `schemas/results_pressure_curved_shear_1.v0.3.schema.json`. It is pressure-2's (pressure-3's)
   schema with `$id` and the identity consts replaced, and with the realized-arc `then` and
   `PhysicsGeometry` `$ref` pointing at a new `results.v0.3.schema.yaml#/$defs/PhysicsBeamTheoryCurvedV2`
   (`{model: timoshenko_annulus_energy_curved_v2, shear_factor, shear_area_m2}`). It also adds a
   `oneOf`/`anyOf` `$ref` to it in `results.v0.3.schema.yaml` and the stress-neutral twin. The
   existing `PhysicsBeamTheory` and every bound schema stay byte-identical.
4. `python3 tools/serialization/semantic_contract_tables.py repin --from-rev <base> --dry-run`,
   then without `--dry-run`. It refuses `tools/retained_pricing/**` and execution/ by design,
   and it prints the carrier writers to run.
5. `node tools/serialization/generate_product_preview_mechanics.mjs` and
   `node tools/serialization/generate_product_preview_mechanics.mjs --preview-physics-1`. These
   refresh `fixtures/product_preview/demo_fixture_generation.json` and
   `preview_physics_fixture_generation.json`, which pin the model schemas and sources. Their
   result outputs must stay byte-identical (EB).
6. `python3 tools/release/generate_schema_manifest.py` for `tools/release/schema_manifest.json`.
7. Report statement tables `fixtures/reports/exact_route_limitations.json` and
   `exact_report_statements.json` (and the desktop `exactReportLimitations.ts` twin) gain the
   EX v2 limitation-0 statement, by their maintained writer. I did not establish whether it is
   a script or a test-printed pin. Verify before running.
8. Retained-pricing generated blocks (`retained_memory.rs` GENERATED PROFILE,
   `retained_memory_law_tests.rs` POINT ATOMS, `build_identity.rs` REVIEWED_INPUTS) change only
   in the D3 turn of §6.

No persisted, recorded or reference fixture is regenerated: EB/v1 bytes must not move, and a
pin that moves is a defect (design §6).

## 4. Tests to add or extend

**Product physics (`core/product_physics/tests/`).**
- `model_settings_shear_model.rs`:
  - v2 decodes, is admitted on a pressure-free document and is refused beside `pressure_contract`;
  - the unknown-value message lists three names;
  - `model_settings` v2 and absent/EB/v1 are unchanged.
- New `curved_shear_runtime.rs`, run on both routes and both solver modes:
  - **v2 straights bit-equal to v1.** The same document with v1 and v2 gives identical
    straight-member K, rows and A_s bits. Only arc rows differ.
  - **Arc stiffness.** Each v2 arc's `global_stiffness` equals a direct
    `CurvedBendMacroElement::new(..).with_transverse_shear_area(A_s)` with A_s by the route
    rule: EX `annulus_energy_shear_factor` on the nominal annulus and the member ν; PF
    `pf_timoshenko::shear_area` on the consumed basis. Include a modulus-basis case selecting
    a non-base basis.
  - **The chord section stays EB.**
  - **Formation.** K-D5 and formation/load certificates pass on v2 arcs (E1/E6 elbows).
  - **Blocking by name.** A_s blocks by name: EX `EXACT_PRESSURE_SHEAR_FACTOR_UNAVAILABLE`
    naming `timoshenko_annulus_energy_curved_v2` for a missing ν on an arc's material; PF
    `PF_TIMOSHENKO_SHEAR_FACTOR_UNAVAILABLE` on an arc with ν outside (−1, 0.5). The v1 message
    bytes are pinned unchanged.
  - **No disclosure under v2.** `PF_TIMOSHENKO_REALIZED_ARCS_EULER_BERNOULLI` is absent under
    v2. Under v1 it is byte-identical to today (pin the existing message).
  - **Identities.** PF publishes `preview-physics-curved-shear-1` with its profile and
    limitation 0; EX publishes `pressure-curved-shear-1`. Blocked envelopes carry the same
    identities.
  - **Arc beam_theory.** EX `pipe_sections` and region geometry carry the v2 record on arcs
    with κ_E and A_s bit-equal to the straight rule.
  - **Source decline.** EX v2 without arcs never publishes `pressure-source-1`; the decline
    reason is present.
  - **Pressure-elongation pairing** (after F-1 lands; at `f18834a6b`, pressure free strain
    is always on). In an EX v2 case with pressure (elongation included), the arc's
    `PressureFreeStrainCurved` and thermal `FreeStrainCurved` equivalent loads equal
    `K_b(v2)·u_free` and differ from the v1 load. Free expansion gives zero
    K·u − p (homothety). A restrained case changes consistently. With elongation not
    included, there is no pressure push but the thermal push still uses `K_b(v2)`. The
    limitation-0 text names the pairing.
- `modal/tests.rs`:
  - PF v2 and EX v2 are refused with `MODAL_TIMOSHENKO_MASS_UNSUPPORTED`, and the message
    names `timoshenko_annulus_energy_curved_v2`;
  - the v1 message is byte-identical.
- Regression (unchanged, must stay green): `pf_timoshenko_references.rs`,
  `t4_u8_timoshenko_references.rs`, `platform_bits_tests`, T4-U1 references, native-smoke
  `expected.json` (owner-reserved; not touched).

**Readers (`result_export/tests/`, Python `tests/`).**
- `preview_timoshenko_identity.rs` / `test_preview_timoshenko_identity.py`, or new
  `curved_shear_identity.*`:
  - the product's own v2 results round-trip through Rust and Python readers;
  - a crossed pair (v2 model with the v1 identity, v1 model with the v2 identity) is refused
    `SOURCE_PREVIEW_PHYSICS_SHEAR_MODEL_IDENTITY` / `SOURCE_PHYSICS_SHEAR_MODEL_IDENTITY`;
  - a pressure-2 envelope stating a v2 member is refused `SHEAR_MODEL_IDENTITY`;
  - a curved-shear envelope with an EB arc is refused;
  - a v1 arc stating Timoshenko is still `SOURCE_PHYSICS_ARC_BEAM_THEORY`.
- `section_bases.rs` / `test_section_bases_readers.py`:
  - a v2 arc's κ and A_s are re-derived bit for bit;
  - `selected_model` returns v2 for arcs only under v2.
- `tests/test_semantic_contract_tables.py`: `derive --check` covers both new tables. Each
  derived table equals its base with only the three (PF) or two (EX) members replaced.
- `tests/test_preview_identity_inventory.py` covers the third identity; add the EX guard.
- `tests/test_modal_model_mass.py`: v2 is classed as the modal refusal.

**Operation applier / document validator.**
- `tests/model_settings_authoring.rs`: set/clear v2; refused beside a contract; unknown
  message.
- `tests/exact_authoring.rs`: a v2 pressure profile is admitted; the successor refusal text
  lists v2.

**Desktop.**
- `ModelSettingsShearControl.test.tsx`: third radio and label; note text; v2 queues
  `set_model_setting`.
- `modelSettingsShearModel.test.ts`: v2 round-trips.
- `PressureAuthoringPanel.test.tsx`: the selector offers v2, and the default stays v1.
- `pfTimoshenkoIdentity.test.tsx` (or a new curved test): both identities are Current.

**MBF (no product change).**
- `caepipe_mbf_engine/tests/fail_closed.rs` and `tests/test_caepipe_mbf_round_trip.py`:
  - a v2 exact document is refused `MBF-SI-PRESSURE-UNSUPPORTED`;
  - a v2 `model_settings` document exports with `MBF-SI-MODEL-SETTINGS-NOT-EXPORTED` and no
    other loss;
  - import with the caller option `shear_model = timoshenko_annulus_energy_curved_v2` is
    refused `MBF-IMPORT-SHEAR-MODEL-INVALID` in both twins;
  - a bound sidecar can never carry v2 (sidecar `SHEAR_MODELS`).
- Parity vectors are unchanged.

## 5. Governing-text reconciliation

Paths were mapped with `tools.deliverables`' own matching rule. `touches` takes a revision
or PR, so `map_paths.py` applies its `load()` and its matching to a path list, read-only.

- **DEL-04-01.**
  - `Work/curved-bend-transverse-shear.md`: the §5 status sentence (drafted). Step B's
    deliverable list (l.249) and §8's test list need the F-3 source decline, the F-5 deliverables
    and the F-1 identity base. §7's sentence "on the retained route the shear area is always
    `None`" stays true; add that D1 refuses v2 on both branches (F-4).
  - `ScopeOfWork.md:62` ("realized arcs are Euler–Bernoulli") becomes "realized arcs are
    Euler–Bernoulli unless the document explicitly selects `timoshenko_annulus_energy_curved_v2`".
  - `deliverable.yaml:105-110` gains the curved-shear condition (kernel option used by the
    product).
- **DEL-05-03.**
  - `ScopeOfWork.md:124` ("realized arcs stay Euler–Bernoulli"): add the v2 exception.
  - `Work/pf-timoshenko.md`: §1 vocabulary, §4 identities and inventory guard (third identity),
    disclosure scope (v1 only).
  - `Work/section-bases-and-timoshenko.md`: §2.3/§2.5/§3.6, where an arc's `beam_theory` is
    v2 under v2.
  - `deliverable.yaml:65`: add v2.
- **DEL-02-01.** `Work/model-document-0.9.0.md`: an amendment adding the v2 value to both
  enums without a version step, as pressure-elongation's member was added (§3.1 amendment
  precedent). `Work/model-document-0.8.0.md` is historical; leave it.
  `ScopeOfWork.md:93`: one clause.
- **DEL-04-07.**
  - `Work/pressure-contract.md`: the 3.0.0 vocabulary amendment (O-1), the curved-shear
    identity beside `pressure-3`, the source decline.
  - `deliverable.yaml:9,37-40,50`: the v2 selection on both routes.
  - `ScopeOfWork.md`: the corresponding current-state line.
- **DEL-03-03.** No change: bend component fields are unaffected.
- **Also affected (F-5):**
  - DEL-05-02 (semantic contracts, readers, generator; `deliverable.yaml:42-51` names D-6
    straights);
  - DEL-04-08 (retained pricing rules entry §8.x; section_basis);
  - DEL-04-09 (modal refusal);
  - DEL-08-01/04/06 (report admission and statements);
  - DEL-07-03/05/11 and DEL-04-06 (desktop);
  - DEL-14-02 (analysis runs);
  - DEL-17-06 (stress-neutral);
  - DEL-13-04 (physical-transform schema).

Changed commitment text (ScopeOfWork, deliverable.yaml, acceptance wording) needs an
independent review under the standing delegation, quoting old and new text in the PR.

## 6. Retained-pricing exposure (name only; nothing priced)

Per `tools/retained_pricing/README.md` and DEL-04-08 `retained-pricing-rules.md` (§8.30 and
§8.32 precedents), the Step B source stops the pricer and needs one actual-source D3 turn with
independent review.

- **Full-file pins.** These files change, and the pricer stops with "reviewed source changed":
  - TIM pins (`o10_rules.py:4063`): `pf_timoshenko.rs`, `model_settings.rs`, `reader_verdict.rs`;
  - the full-file `lib.rs` pin;
  - `solve_prepared_load_case` body pin (the `exact_pipe_sections` call);
  - `retained_product.rs`: unchanged;
  - RE `semantic_contract.rs` full-file pin;
  - `source_recovery.rs` / `source_receipt/composite.rs` (T07/T25/RF owners);
  - `section_basis.rs` / `physics_evidence.rs` / `arc_evidence.rs` reader pins (§3.3).
- **NP-MS** (`o10_rules.py:2470-2508`): pins the `shear_model_unknown_message` template and the
  set of shear constants. The new template, about 40 B longer, and the new
  `SHEAR_MODEL_TIMOSHENKO_CURVED` constant must be restated with their byte bounds.
- **TIM-PF / TIM-ID.** New TEXT sites in zero tails (D1 refuses v2 on L and E, F-4):
  - `shear_model_name` arguments in PF diagnostics;
  - the authored-ν phrase literal (≤ 70 B);
  - the partial-span name argument;
  - the EX `EXACT_PRESSURE_SHEAR_FACTOR_UNAVAILABLE` message, now `format!` with
    `timoshenko_model(model)` (static, ≤ 35 B);
  - the source decline literal "timoshenko_annulus_energy_curved_v2 is not a straight source
    family";
  - the `with_transverse_shear_area` error arm in `build_curved_bend_macro_elements` (an
    existing `CurvedBendError` Display; new edge);
  - the modal v2 message (outside retained roots; confirm).
- **TIM-BN.** `exact_section_evidence` gains a parameter and its match arm text changes; the
  zero edge to PF `shear_area` must be re-keyed.
- **New owner and allocation.** `member_shear_areas: HashMap<&str, f64>` in
  `build_model_for_basis`. It is empty (no allocation) unless v2 is selected, so it is a
  zero-edge premise under D1. `ConsumedBasis` gains a `&'static str`, which is zero under
  TIM-PF (`ConsumedBases::of` returns empty).
- **Static owners (coefficients move).** Two new `OnceLock` tables
  (`preview_physics_curved_shear_contract`, `pressure_curved_shear_contract`) are TIM-OWN-type
  positive owners priced in every phase. §8.32's one table added 322,013 B native and about
  43 KB to the STATICS constant; the EX table is larger (pressure-2 rows). Both tables join
  `REVIEWED_INPUTS` (`build_identity.rs`, `retained_memory.rs` `reviewed_inputs`), so the
  registration moves. By §8.32's practice that means re-registration, native phase pins, and
  AC-004 witnesses under the owner's exclusive grant. TIM-RD needs re-stating: J1's projection
  never selects either new loader.
- **Layout.** `section_basis::Beam` gains a `bool` (expected to stay 24 B; measure, since
  `Sections` is held in reader indices). `ExactSectionBases`, `CurvedFormation`,
  `LoadFormula` and `BuiltModel` are unchanged by Step B.
- **Coordination.** pressure-elongation (F-1) runs its own D3. Pricing Step B on top of that
  registration avoids double re-registration.

## 7. Open choices for Agent 0

- **O-1. EX vocabulary treatment.** Recommend amending `3.0.0/exact_pressure_v3`'s
  `shear_model` enum in place, with no contract or model-version step, as pressure-elongation
  adds its member within 0.9.0. The result identity, not the contract version, distinguishes
  v2 results. Older builds, MBF and retained D1 refuse v2 by name.
  The alternative, `3.1.0`, re-keys `ExactContract`, every "3.0.0/exact_pressure_v3"
  limitation string and every reader.
- **O-2. Identity names.**
  - PF ID `preview-physics-curved-shear-1`: fits the `<base>-<variant>-N` pattern; keep it.
  - PF profile `product_preview_mechanics_curved_shear_v2`: the trailing `_vN` in
    `product_preview_mechanics_timoshenko_v1` is the profile's own version, so `_v2` reads as
    a second version of a profile that has none. Recommend `product_preview_mechanics_curved_shear_v1`.
  - EX ID `pressure-curved-shear-1`: fine, but derive it from `pressure-3` (F-1).
  - EX profile `product_pressure_curved_shear_v2`: clashes with the EX convention, where the
    profile is the contract mode (`exact_pressure_v3`, `exact_admission.rs:56`;
    `physics_source.rs:12,36` uses the profile constant as the mode). Recommend
    `exact_pressure_v3_curved_shear_v1`.
  - The hunks use the proposed names, one constant each, so a rename is a one-line change per
    language.
- **O-3. Sequencing.** Recommend two PRs inside Step B, both routes still delivered:
  - B1: vocabulary on both routes plus PF identity, modal, MBF tests and desktop settings;
    EX v2 refused by name until B2;
  - B2: the EX identity on `pressure-3` after F-1 merges, then one D3 over both.
  - The alternative, one PR after F-1, holds PF back.
- **O-4. `beam_theory.model` on v2 straights.** The draft writes v2 (the document's
  selection; κ and A_s bits equal v1). The alternative, v1 on straights and v2 on arcs, keeps
  straight records byte-identical but splits one document across two names. Recommend v2.
- **O-5. UI.** The label is as approved, on both the settings radio and the pressure
  selector. The legend "Straight-member shear deformation" contradicts the new choice;
  recommend "Shear deformation" with aria-label updated in tests. The draft adds a note
  sentence on arcs and modal.
- **O-6. Refusal texts that enumerate admitted values** (`model_settings.rs:125`,
  `reader_verdict.rs:220`, operation applier `lib.rs:6007`, `pressure_authoring.rs:299`) must
  change bytes to stay true. They are refusals of non-EB/v1 inputs, not EB/v1 diagnostics.
  Recommend changing them; tests and NP-MS move with them.
- **O-7. Report user-release path** (`user_release.rs:876`). Recommend refusing both
  curved-shear identities by name, as PF Timoshenko is today, until a release projection is
  designed.
- **O-8. Source family.** Recommend declining EX v2 from the retained-source method (F-3)
  rather than extending `pressure-source-1`, whose schema is a bound reviewed input.

## What I could not determine

- `pressure-3`'s exact constants, table and schema filenames (unmerged branch; I read only its
  design).
- The derived tables' SHA-256 values (placeholders), and whether `Beam`'s size changes.
- Whether region geometry `beam_theory` values are bound to the case section's (assumed in
  `arc_evidence.rs`'s comment; verify in `physics_evidence.rs`).
- The maintained writer of the report statement tables (§3 item 7).
- Nothing was compiled or run. The hunks are syntax-checked only by inspection.
