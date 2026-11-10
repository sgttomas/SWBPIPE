---
name: swbpipe-release-target-v1-0
description: Set the v1.0 capability target, verification criteria and reserved decisions.
metadata:
  kind: instruction
  surface: requirements
---
# SWBPIPE v1.0 (publication-ready): release target

**Status:** PRD companion (`docs/PRD.md` §24 V1). It is the target for the
first publication-ready release, accepted by the owner on 2026-10-28
("Accepted."). It comprises the owner's definition and rulings (§1), and
HELP_HUMAN's capability rows (§2), benchmark method (§3) and exit criteria (§4)
drafted from them. It is also the record on `main` of the owner's 2026-10-28
rulings quoted in §1, given in conversation with HELP_HUMAN. Meeting the exit criteria
is evidence for the owner's release decision, not the decision: acceptance,
issuance and release remain the owner's acts (F-PIP-3). SWBPIPE computes mechanics from recorded inputs and evaluates
the user's own rule packs; it does not certify, approve or establish code
compliance. Standard claim fence applies (claims taxonomy per DEC-081).

Paths are relative to the repository root. **PF** is the pressure-free solve
route (a model with no `pressure_contract`); **EX** is the exact pressure route
(`3.0.0/exact_pressure_v3`). Row ids carry the prefix `RT-` so they do not
collide with survey item or benchmark family ids.

## 1. Target

**Definition** (owner, 2026-10-27):
- Publication-ready means "a version good enough to publish engineering results
  with".
- Analyses: "We should be able to do seismic plus hydrostatic, and the
  sustained, expansion, operation as you clearly know too. Anything else that
  would be considered standard?"
- "For now, it won't proclaim code compliance. I actually want the user to go
  to CAEPIPE for that purpose. This software does all the design iteration."
- CAEPIPE is the validation to SWBPIPE: features on par or better, "given real
  thought for user experience and human factors in the design and taste in the
  aesthetics", with a benchmark set comparing non-code outputs "across as wide a
  range as practicable".

**Rulings that shape it.** The owner answered ten open choices (OC-1 to
OC-10) on 2026-10-28. Where the owner chose an option as drafted, the item
quotes the option's drafted text; the owner's own words are quoted as such.
- **Published results (OC-1).** The owner chose (a) (owner, 2026-10-28), drafted as: "The
  user's engineer issues SWBPIPE's mechanics results under their own
  responsibility: support and equipment loads, displacements, frequencies,
  fundamental stresses."
- **One route (OC-2).** The owner chose (a) (owner, 2026-10-28: "OC-2, a, is
  anything desirable or necessary omitted without also selecting b?").
  HELP_HUMAN answered that nothing necessary is omitted provided EX admits
  every family PF has, and that (b), pressure on PF, would be revisited only if
  EX failed the performance targets (§4 item 9). EX is extended; PF gets no
  pressure.
- **Seismic (OC-3).** The owner chose (a) (owner, 2026-10-28), drafted as: "Must:
  user-entered spectra, modal and directional combination, a missing-mass
  correction, and snubbers as a dynamic-only restraint." The rule (owner,
  2026-10-28): "I said you should only develop a response spectrum capability
  for seismic analysis if that's a functionality that CAEPIPE has, otherwise
  you can go with only static for now." That condition is met: the vendor's
  public manual describes response-spectrum analysis with mode and directional
  combination and missing mass
  ([dynamic analysis](https://docs.sstcae.com/tech_manual/dynamic_analysis.htm),
  [spectrums](https://docs.sstcae.com/users_manual/spectrums.htm),
  [load cases](https://docs.sstcae.com/users_manual/load_cases.htm)). The
  governed scope change DEC-078 requires is SCA-014.
- **Hangers (OC-4).** The owner chose (a) (owner, 2026-10-28), drafted as: "Must: size
  hangers from the user's private catalogue (invented example records only)
  and produce a hanger table."
- **Hydrotest (OC-5).** The owner chose (a) (owner, 2026-10-28), drafted as: "A flagged
  test condition with test pressure, test contents density and spring supports
  held, marked as a test case in results and reports."
- **Rule packs (OC-6).** The owner answered "OC-6, as stated" (owner,
  2026-10-28). OC-6 as drafted: "Keep what is built as a design aid, and defer the extensions:
  C2, E1, E2, E5 and E6. Drop E3 (fatigue usage) and E4 (pressure-design
  thickness) from the roadmap. … Show user-rule outcomes as "within / exceeds
  user limit" rather than pass/fail, so that no outcome reads as compliance. …
  It covers 16 ScopeOfWork items: keep 11, split 1, defer 4."
- **Usability (OC-7)** (owner, 2026-10-28): "I want you to use your own
  judgment after concocting some typical use cases an engineer would go
  through (I will also get actual user testing but that's after you put this
  through the wringer)."
- **Release path (OC-8)** (owner, 2026-10-28): "go straight to v1.0 and first
  macOS but we will revisit immediately after for Windows too."
- **Licence (OC-9)** (owner, 2026-10-28): "it's MIT license."
- **Benchmarks (OC-10)** (owner, 2026-10-28): "first screen the results with
  the tolerance profile and flag anything of merit, then I will review the full
  record." Benchmark agreement with CAEPIPE will be published (owner,
  2026-10-28: "Benchmark agreement with CAEPIPE will indeed be published. That
  part should be out in the open because the whole point is to feed into
  CAEPIPE and expect the results to come out the same (but validated to Code
  and to the reputation of CAEPIPE)."), subject to the owner's check of the
  CAEPIPE licence for benchmark clauses.
  The 23 human-disposition and review holds (RF-001/RF-002, PDU-019, PDU-035
  and PDU-060 findings) do not gate v1.0; they are dispositioned in one batch
  (OC-10 (b); owner, 2026-10-28: "As recommended").
- **Pressure effects** (owner, 2026-10-28: "As recommended"). The Bourdon
  effect is a Should option for v1.0, off by default and clearly labelled
  (RT-L2). Pressure-dependent flexibility and SIF corrections are out of scope:
  users enter their own k and SIF.
- **Public repository** (owner, 2026-10-29: "Split repo replaces RT-V8" and
  "Push now, sign before v1.0"). The independent public repository
  `sgttomas/SWBPIPE` is SWBPIPE's public home, replacing the sanitized-export
  mechanism. The verified import may be published before scan signature. The
  owner-signed D-20/DEC-058 repository scan, including the full private-term
  scan and finding dispositions, gates v1.0 (DEL-10-04:1; RT-V8).
- **Exact-route results under user release** (owner, 2026-10-29: "Open it, as recommended"). Route 010 admits exact-route results with exact source binding, exact sections and limitations, Diagnostics binding and producer item 9 (RT-R5; DEL-08-01 Design `report-user-release.md` §11).
- **Embedded agent** (owner, 2026-10-28: "I want you to add the App v4
  integration into SWBPIPE for the v1.0 target. I want you two to discuss how
  to make it connect, but I thought this had already been decided on (check
  the deliverables). I'm not interested in having all the governance for the
  workflows working perfectly, my focus is just on getting the agent harness
  into SWBPIPE and able to operate that application effectively."). The
  Chirality App v4 agent harness is embedded in the desktop (RT-E1). Earlier
  designs, DEC-041 among them, are historical context only (owner, 2026-10-28,
  relayed from the App session: "take note of what was done in the past, but
  don't take from it. There's better examples now elsewhere and just from your
  own ideas."). The connection is designed afresh with App v4's HELP_HUMAN and
  recorded in the owning Design.

## 2. Capability rows

**Must**: required by the exit criteria; a gap ships only as a limitation the
owner accepts (§4 item 3). **Should**: expected; may ship
declared as a limitation with the owner's acceptance. **Later**: after v1.0.
The "Current" and "Open work" columns state each row's standing. The PR that
changes a row's state edits that row in the same change. Cells stay short: what
is built, what is open and the owning deliverable; the detail is in that
deliverable's `Work/`. They summarize implementation evidence and remaining work; they do not amend
scope, accept limitations, establish release readiness or schedule work.
"Owner" is the owning deliverable; "new" means a scope change assigns it.

### Analyses

| Row | Capability | Level | Current (evidence) | Open work / owner |
|---|---|---|---|---|
| RT-A1 | Linear static | Must | Built on both routes (W-9 benchmarks) | — / DEL-04-01 |
| RT-A2 | Nonlinear static operating (gaps, lift-off, friction) | Must | Built on PF and on plain EX documents without objective connectors (#108); P repairs the shared loop's scale (#138). EX publishes gap cases with an unqualified recovery basis, without confirmation. Mechanics combinations retain OQ-1's one solved case at factor exactly 1 plus at least one zero-fixed-load-count combination; derived rows are not a new equilibrium state. Operand standing and combination certificates are published (#98, #138). | S6 (connectors and load/reference-state documents), R-3 (gap qualification), S7/S8 / DEL-04-04, DEL-04-07 |
| RT-A3 | One route: EX admits every family PF has (tees, nonlinear and constant-effort supports, reducers in pressure regions) | Must | EX now admits tees and implicit pressure junctions (#45), bounded concentric and W-15c eccentric reducer steps (#73, #80), constant-effort supports (#101) and plain-document nonlinear supports (#108). P and the combination certificate are integrated (#138); the tie-rod axial-share reader uses an exact dot rounded once (#150). Compensated shared terminals and contexts outside the bounded admissions remain refused. | R-4 S3/S4/S6–S8, R-3 and remaining reference-system integration / DEL-04-07 |
| RT-A4 | Combinations: algebraic, difference, envelope, absolute sum, SRSS | Must | All five combination kinds and nested combinations built on both ordinary routes, with fixed-load-count gates and OQ-1 (#98). Desktop authoring includes typed nested operands and a three-axis static seismic template (#114); deletes and indexed restores cover every basis (#115, #117). Combination certificates are integrated (#138). Retained routes still decline these combinations. | Remaining reference-system coverage outside criterion 6 (owner, 2026-11-08); retained-route refusal remains / DEL-05-02 |
| RT-A5 | Sustained, operating and expansion sets. The expansion basis is recorded; sustained can take the operating support status | Must (basis), Should (status) | PF operand standing and the declared operating-minus-sustained basis are built (#107). Frozen operating support status (`minuend_final`) is merged and registered, with combination-owned held-operand rows, held-system standing and readers (#122, §8.54). EX still refuses the declared basis. | EX declared-basis admission; hanger sizing's `design_operating_final` and other downstream consumers / DEL-05-02, DEL-02-01, DEL-04-11 |
| RT-A6 | Hydrostatic test: test pressure, test contents, springs held, marked as a test case | Must | Model 0.9.0 carries `test_condition` and its readers (#50), but authoring and solving it remain refused. Hydrotest pressure loads, per-case contents resolution, held-device mechanics and result/report marking are not built. A read-only next-increment brief is complete; no implementation followed. | Mechanics design reconciliation, EX contents resolution, held-device seam, desktop authoring/marking and references / DEL-05-01, DEL-04-07 |
| RT-A7 | Modal analysis (PRD §12.9) | Must | Internal dense product path, state-case binding and result contract/readers are built (#94, #96, #97, #113), with workspace reuse (#110) and an unwired production-parameter constructor (#131). P0's checked counts and S1 allotment wiring are merged (#155, §8.58, AC-004 7/7). Six unbounded resource terms make every budgeted request refuse; no production resource values are adopted. Public/native admission and UI remain off. | Bound remaining terms and memory, set resource values, complete S2/S2b/S3 and projection limits; full reader, recording policy, I3a numerical review, subject census/Bessel; R-3/S6 state-case dependencies / DEL-04-09 |
| RT-A8 | Response spectrum, uniform support motion: user spectra, mode and directional combination, missing mass; snubbers as dynamic-only restraints. Independent (multi-level) support motion is Should | Must (uniform), Should (independent) | Scope and design placed in DEL-04-10 (#22); spectrum members are carried and refused by model 0.9.0 (#50). No product response-spectrum analysis is implemented. | I5 after modal prerequisites; combinations, missing mass, dynamic restraints, views and references / DEL-04-10 |
| RT-A9 | Static occasional: wind, three-axis seismic g | Must | DEC-068 generators on PF and plain EX documents (#1698); partial-span wind on Timoshenko members (#1742). Refused on EX load/reference-state documents | EX load/reference-state documents / DEL-05-01 |
| RT-A10 | Imposed displacement, including seismic anchor movement | Must | EX `boundary_motion` and PF support-targeted motion, including PF nonlinear supports, are built. Constant-effort targets remain refused; plain-EX support-targeted motion is still R-4 S3. Skewed imposed motion awaits S-K3. | R-4 S3/S6 and the remaining skew-restraint slices / DEL-04-03, DEL-04-07 |
| RT-A11 | Hanger sizing from the user's private catalogue, with a hanger table | Must | PF sizing arithmetic, derived solves, library 1.1.0 and the headless result/readers are built (#76, #79, #81, #83). The desktop form, bounded native worker, table/details with standings bound to the document, and CSV are mounted and usable (#145, #148, #152). Sizing-run recording is not implemented. | EX sizing/reference-system verification, recording decisions, preload, progress and pre-fill; held operating-status integration / DEL-04-11 |
| RT-A12 | Cold spring | Should | Absent | unscheduled / DEL-05-01 |
| RT-A13 | Numerical diagnostics and health | Must | Built (FR-SOL-015; #1544, #1665) | — / DEL-04-06 |
| RT-A14 | Harmonic, time history, force spectrum, buried pipe; verified bounds | Later | Absent; verified bounds dormant and owner-held (DEL-04-08) | — |

### Model, components, supports, loads

| Row | Capability | Level | Current (evidence) | Open work / owner |
|---|---|---|---|---|
| RT-G1 | Model document, persistence, units (SI, US, custom) | Must | Model 0.9.0 is current; 0.8.0 is retired, with no migration (#50). Units remain editable. Native result recording, save and reopen are integrated (#59); the project store is v14 (#146). Carried future-analysis members do not imply their solve admission. | Remaining member consumers appear in RT-G5/RT-A5–A8; backup in RT-U3 / DEL-02-01, DEL-02-02, DEL-02-05 |
| RT-G2 | Authoring: viewport, run sheet, bulk change, split into parts, slope entry, context menus | Must | Existing viewport/grid/bulk/split/slope tools plus run-sheet bend/support IDs, valve/flange entry and directional SIF cells are built (#41, #62, #121). Load/reference-state fills/copies and batch review are integrated (#38, #54). The connected browser/wasm journey enters all 82 reference-fixture records and preserves them through save/reopen (#128, #130, #135); this is functional evidence, not timing, native usability or a solve. | Timed/native walkthrough; remaining run-sheet/editor coverage and support-direction edit/remove/display paths / DEL-07-01, DEL-07-02 |
| RT-G3 | Materials, sections, temperature-dependent modulus, mass properties | Must | Libraries with provenance and per-case modulus basis remain built. E/nu material thermal-expansion entry and editing are implemented (#36, #37); the reference-entry journey exercises material/section forms (#135). Model-level `modulus_basis_records` has no GUI entry. | GUI entry of `modulus_basis_records`; remaining reference-system/MBF property coverage / DEL-07-02, DEL-17-04 |
| RT-G4 | Model checks | Must | Built (W-8; #1665) | — / DEL-04-06 |
| RT-G5 | Operating-condition sets along runs | Must | Model 0.9.0 storage and PF solve-time resolution are built (#50, #84), as are applier operations and desktop forms, assignment, grid, bulk change and run-sheet entry (#92, #93, #111, #112). The load manager exposes resolved previews. Contents density is carried but unused; EX `condition_use` remains refused. | S4 EX resolution; contents-density consumption and hydrotest / DEL-02-01, DEL-05-01 |
| RT-K1 | Bends with user k and SIFs | Must | Built (DEC-070; EX realized bends) | — / DEL-03-03 |
| RT-K2 | Tees with user modifiers; branch flexibility | Must (tee), Should (branch flexibility) | Tees are admitted on PF and EX, with user-entered header/branch SIFs and no assembled branch flexibility (#45). The inspector authors the modifier source (#75); the reference fixture includes RS-T1 (#55). | Branch flexibility; remaining MBF tee coverage / DEL-03-04, DEL-17-04 |
| RT-K3 | Reducers, valves, flanges, rigid items, weight at CoG | Must | Existing mapped-member mechanics plus bounded concentric and eccentric/slanted pressure-step reducers on EX are built (#73, #80), with independent references. Run-sheet valve/flange entry is integrated (#62). Reducers outside the admitted pressure-step topology remain refused. | RS-R1 fixture integration and remaining MBF family coverage / DEL-03-05, DEL-09-01, DEL-17-04 |
| RT-K4 | Expansion joints, tie rods, pressure thrust | Must | EX connectors, tie rods and pressure thrust remain built; tie-rod axial-share reader corrections are integrated (#141, #142, #150). `movement_limit` is not enforced. Nonlinear supports with connectors still await S6. | Movement-limit enforcement; S6 interaction / DEL-03-06, DEL-04-07 |
| RT-K5 | Timoshenko straights on PF | Should | Built: opt-in Timoshenko straights on PF (#1663, DEL-05-03 `Work/pf-timoshenko.md`) | — / DEL-05-03 |
| RT-S1 | Linear restraints and springs | Must | Axis restraints/springs remain built on both routes. S-K1 adds PF skewed springs and one skewed rigid line stop per node on models without nonlinear supports, with along-direction reactions and native result presentation (#140). The support form authors Direction on creation (#132). | S-K2/S-K3/S-K5/S-K4; direction edit/remove, other creation paths and viewport/grid display / DEL-04-03, DEL-07-02 |
| RT-S2 | Gaps, lift-off, friction | Must | Built on PF and plain EX without objective connectors (#108). Friction remains static/history-free; rotational gap/friction are refused. EX gap states publish unqualified recovery standing, not confirmation. Friction normals from nonlinear supports and the broader coupling cases remain outside current scope. | S6, R-3, S7 and S8 / DEL-04-04, DEL-04-07 |
| RT-S3 | Variable spring and constant-effort supports | Must | User-stiffness variable springs remain built, with entered installed/cold/hot loads disclosed as not applied preload. Constant-effort supports now work on PF and EX, including active devices on load/reference-state documents (#101). PF sizing is usable (RT-A11); variable hangers can be authored without a rigid restraint (#130). | Preload (DEL-04-11 increment 6), held-device mechanics for hydrotest, EX sizing and RS-CE1 fixture integration / DEL-04-03, DEL-04-11, DEL-09-01 |
| RT-L1 | Weight, pressure, thermal (many cases), user forces including relief thrust, wind | Must | Pressure remains EX-only; authored thermal cases and DEC-068 wind remain built. PF condition sets resolve temperature, thermal strain and added weight through one plan (#84), with desktop authoring (#111). EX condition-set resolution and per-case contents are not built. | RT-G5 S4, test contents/hydrotest and load/reference-state equivalent-static generation / DEL-05-01, DEL-05-05 |
| RT-L2 | Bourdon effect (pressure elongation) as an option, off by default and clearly labelled. Pressure-dependent flexibility and SIF corrections are out of scope; users enter their own k and SIF | Should | EX excludes Bourdon opening and pressure stiffening of k/SIF, and discloses it (DEL-04-07 `Work/pressure-contract.md` D-3) | unscheduled / DEL-04-07 |

### Results, reports, interoperation

| Row | Capability | Level | Current (evidence) | Open work / owner |
|---|---|---|---|---|
| RT-R1 | Results review: navigator, contours, deflected shape, key results, support loads, extremes, weight/CoG, sorted stresses | Must | Existing result views now use native custody, summaries and paged details, including saved-result reopen (#59). Diagnostics and skew-support reactions are mirrored in native presentation (#132, #140, #143). Hanger sizing's table/details and CSV are mounted (#145, #152). | Current-source native walkthrough and large-model qualification; modal views remain RT-R2 / DEL-07-05, DEL-04-05 |
| RT-R2 | Frequencies, mode shapes and participation views | Must | Internal modal documents and readers exist (#113), but there are no admitted desktop modal views. Public/native modal admission remains disabled. | M3 after modal admission prerequisites / DEL-04-09 |
| RT-R3 | User rule packs as a design aid; outcomes "within / exceeds user limit". Survey extensions C2, E1, E2, E5 and E6 and the plugin/adapter runtime are deferred until after v1.0; E3 and E4 are dropped | Should | Format 0.5.0 (#1629); outcome wording still pass/fail | wording / DEL-06-01, DEL-07-03, DEL-08-01 |
| RT-R4 | Calculation report with a section composer | Must | Local-private render and package on both routes (W-3, #1648); section composer designed (DEL-08-01 `Work/report-section-composer.md`), not built | composer build / DEL-08-01 |
| RT-R5 | Shareable report released by the user | Must | Built on both routes: route 010 native admission (W-18b r2c, #1682) and exact-route results (D-6, #1745); privacy fixes (#1692, #1722) | later slice in `report-user-release.md` §10 / DEL-08-01, DEL-12-02 |
| RT-R6 | CSV export of every results table | Must | Built: CSV serializer and export on every results and comparison table, with a coverage guard; saving requires explicit local-private intent (#1720, #1726) | — / DEL-08-04 |
| RT-R7 | MBF export from the desktop | Must | Native capped-worker export is usable from File > Export MBF and the Exports panel, with bounded review, MBF/identity-file transactions and keyed cancellation (#32, #147, #151). It is model-only and independent of result-output gating (#153); an unknown save blocks further exports in that page until reload/restart (#154). Explicit private intent and writer refusals remain. Combinations are outside criterion 6's families (owner, 2026-11-08; §4, criterion 6); other refusals remain, so criterion 6 is not met. | Reference-system coverage tranches (including supplier-dependent thermal mapping), import and round-trip checks; C6-2 metadata exemption list / DEL-17-04, DEL-07-12 |
| RT-R8 | MBF import into the desktop | Must | Rust importer parity remains built; sidecar format 4 preserves section identities through engine round trips (#47). File > Import MBF is connected through the native read-only chooser, bounded review and Adopt through the existing Open transaction (DEL-17-04 ScopeOfWork CLM-003 and Work/desktop-mbf-export-import.md §4.9). This implemented subset does not establish the criterion-6 reference-system round trip. | Slice 6 supported-subset round trip and remaining reference-family coverage; criterion 6 remains unmet / DEL-17-04, DEL-07-12 |
| RT-R9 | PCF, glTF, handoff package, native JSON | Should | Save Handoff (#1600); PCF and glTF text withheld | — / DEL-17-07, DEL-17-08, DEL-15-01 |

### Human factors, validation, release

| Row | Capability | Level | Current (evidence) | Open work / owner |
|---|---|---|---|---|
| RT-U1 | Coherent visual design, keyboard routes, accessibility of touched controls | Must | Existing D-68 baseline and work-loss guards remain. New authoring, MBF and hanger controls have focused UI checks; the reference-entry journey establishes functional entry/save/reopen (#135), not native usability or accessibility of the whole application. | Current-source native walkthrough and its findings / DEL-07-06 |
| RT-U2 | Usability basis: Agent 0 use-case walkthrough, then owner-arranged user testing | Must | Connected browser/wasm reference entry matches 82/82 records (#135). The run-sheet time remains an estimate; no qualifying timed native walkthrough or owner-arranged user testing is established. | Agent 0 walkthrough and dispositions, then owner-arranged testing / DEL-07-06 |
| RT-U3 | Autosave and backup | Must | Autosave built: native writer, timer, settings and recovery offer (#1709, #1718). Backup copy (slice 3) not built | slice 3 backup / DEL-02-05 |
| RT-U4 | Performance on large models | Must | Criterion-9 racks/drivers and historical measurements exist (#31, #53). Native custody, paging, recording/reopen and reader/loop repairs are integrated (#59, #95, #102, #106, #138); v14 removes unused FTS (#146). SQLite upsert/COMMIT memory at v14 remains unmeasured (U1); builder/reader/validator memory (U3) and no App-wide cap (U4) remain. No current 500/2,000-node fit or 16 GiB qualification is established. | Fresh current-source end-to-end timing/memory measurements against §4 item 9; close residuals / DEL-04-05 |
| RT-U5 | States, runs, comparisons | Should | Existing W-12 features plus native Record Run, save and historical result reopen are integrated (#59, #133). Mapping editor, tolerance profiles and export remain Later. | Existing Later work; no current completion claim beyond the implemented paths / PKG-14, DEL-02-05 |
| RT-U6 | Delete a stored project | Later | No delete-project command; stored projects are not removed from the desktop (owner, 2026-10-26: "I agree with your recommendation and approve (a)", where (a) was no delete-project command in v0.1, listed as a later candidate; recorded from superseded #1642) | — / new |
| RT-V1 | Closed-form benchmarks for every Must analysis | Must | Existing static/M3/W-16t and combination references are joined by tee, reducer, nonlinear EX, held-status and skew-restraint references (#45, #80, #108, #122, #140). Modal `f1_modal` and PF hanger-sizing arithmetic/derived-solve references exist (#22, #76, #81). These do not complete all Must analyses or the reference system. | Hydrotest and response-spectrum references; complete modal/published piping and EX hanger/reference-system coverage / PKG-09, DEL-04-09, DEL-04-11 |
| RT-V2 | Benchmark set against CAEPIPE (§3) | Must | Tranche 1 built: 60 invented models, two-way screening against the §3 profile, settings guard and print parser (#1733). Method and harness only; no agreement established | further tranches, publication checker / DEL-17-05 |
| RT-V3 | Validation manual and a fresh reproduction bundle | Must | Procedure current; last bundle predates W-12 | W-2 / DEL-09-04 |
| RT-V4 | User guide and theory notes for every Must analysis | Must | Existing statics material plus load-combination and hanger-sizing theory notes and the headless sizing guide are present (#82, #86; `docs/theory/`). Some notes predate the desktop connections; coverage of every Must analysis is not complete. | Reconcile docs with current desktop paths; complete hydrotest/modal/spectrum and remaining Must coverage / DEL-11-01, DEL-11-03 |
| RT-V5 | Claim fence on every surface (notice, release labels) | Must | The GEN-13 claims-language lint was removed from hosted CI and the repository (owner's test-and-CI review direction) | release labels / DEL-09-05, DEL-16-04 |
| RT-V6 | Signed, notarized macOS build; MIT licence recorded consistently | Must | Signing decided (D-06b), not executed. MIT in `LICENSE.md` (D-74) and DEL-01-01; the repository's `LICENSE` (initial commit) and `LICENSE.md` spell the holder's name differently | DEL-10-04; the holder's name is the owner's to state |
| RT-V7 | Windows build | Later (revisited immediately after v1.0) | Release baseline: macOS Apple Silicon only (DEC-057); Windows development authorized alongside macOS v1.0; native x64 desktop unqualified | DEL-10-04 Work/windows-port.md |
| RT-V8 | Independent public SWBPIPE repository | Must | Split done: verified import (SWBPIPE #1) and standalone governance; unsigned scan findings remain; no signed scan or release acceptance | owner-signed D-20/DEC-058 repository scan including full private-term scan and finding dispositions before v1.0 / DEL-10-04 |

### Embedded agent

| Row | Capability | Level | Current (evidence) | Open work / owner |
|---|---|---|---|---|
| RT-E1 | The App v4 agent harness embedded in the desktop: a conversation panel, SWBPIPE's capability catalog of reads and changes, and changes through the one operation route, applied on the person's acceptance or within the autonomy the person granted | Must | Host/live panel and catalog v4 are integrated, including W9-A construction, components, support configuration and corrections (#2, #3, #15, #27). Agent/imported operations cannot impersonate manual modifier entry (#23). A native W9-A build/apply/solve/explain journey passed with a scripted provider (#46), not a live model or criterion-17 reference system. HC-1 disables supplier helpers (#69); old native witness bytes do not qualify that changed configuration. | Fresh joined live-model D4 qualification and memory evidence under separate owner authority; remaining catalog/reference-system journey and owner-attended Apply. Manual k/SIF restrictions remain / DEL-16-06 |

## 3. Benchmark method (summary)

The validation lane proposes the set; this section records the method only:
- **Scope.** Mechanics only: displacements, rotations, element forces and
  moments, support loads, frequencies, mode shapes and spectrum responses.
  Stresses and code checks are left out.
- **Models.** All invented. A seeded generator covers model families from
  straight spans to nonlinear supports, hangers and dynamics. Closed-form
  members give an independent third value.
- **Inputs.** Each code-dependent input that changes mechanics is held equal on
  both sides. Where it cannot be held equal, it is named and carried by a
  one-parameter series. SWBPIPE is never tuned to match.
- **Screening profile.** Agent 0 screens with these values, proposed by the
  validation lane, under the owner's ruling to "first screen the results with
  the tolerance profile and flag anything of merit, then I will review the full
  record". The owner's review of the full record decides, not the profile. Each
  is a relative tolerance, plus a printed-resolution floor per result kind:

  | Class | Tolerance |
  |---|---|
  | Every input held exactly | 1e-4 |
  | A named inexact input remains | 5e-3 |
  | Nonlinear | 2e-2, and support status must match exactly |
  | Modal | 5e-3 on frequencies, MAC ≥ 0.99 |

  The spectrum class is set before any comparison run, from its closed-form
  references, never from comparison results, and reported to the owner.
  Values are only tightened.
- **Findings.** Any result outside the profile, an unmatched row, or a status,
  refusal or mode-count mismatch is a finding.
  - Triage order: mapping check, closed form, then SWBPIPE's own invariants,
    then a convention series.
  - Each finding is classed as SWBPIPE's to fix, a documented convention
    difference, or unexplained.
- **Review and publication.** Agent 0 screens every run against the profile
  and flags anything of merit; the owner then reviews the full record.
  Agreement is published once the owner has checked the CAEPIPE licence for
  benchmark clauses, and `docs/IP_AND_DATA_BOUNDARY.md` §6.2 is amended to
  match. Other evaluation stays private.

## 4. Exit criteria

Thresholds marked *(provisional)* are Agent 0's targets. The RT-U2
walkthrough informs them; they become final only on the owner's acceptance
after the walkthrough.

1. **One model, every Must analysis.** An invented reference system solves on
   EX in the desktop with no admitted family refused. It contains:
   - pressure, realized bends, a tee with user modifiers, an eccentric reducer,
     valves and flanges, and an expansion joint with tie rods;
   - linear restraints, a gap, lift-off, friction, a variable spring and a
     constant-effort support.

   The analyses are: sustained, operating, expansion with its basis,
   hydrotest, wind, three-axis static seismic with SRSS and absolute-sum
   combination, seismic anchor movement, modal, response spectrum and hanger
   sizing.
2. **Benchmarks.** Each Must analysis passes closed-form or hand-calculation
   references frozen in `validation/references/` with an independent generator.
   Modal also passes at least one published piping benchmark (DEL-04-09
   AC-010).
3. **Limitations.** Every remaining refusal in a Must row is listed in the
   release limitations and accepted by the owner.
4. **Shareable report.** A user releases, saves and prints a report for results
   on either route, with the §19.3 notice and a reproducible canonical hash.
5. **CSV export.** Every results table in the desktop exports to CSV.
6. **MBF round trip.**
   - The desktop saves the reference system as MBF with no Must family omitted
     or unsupported in the loss report.
   - The desktop imports it back with the same entity identities.
   - **Family list (owner, 2026-11-08; option 3, "Family list amended").**
     A-3 in DEL-17-04 `Work/desktop-mbf-export-import.md` §8.1 excludes
     combinations, which combine solved results, from criterion 6's families.
     RT-A4 coverage is judged elsewhere. The other refusals in DEL-17-04
     `Work/caepipe-mbf-import.md` §20.5.10 still stand; criterion 6 remains
     unmet, and no reference-system round-trip claim is made.
7. **Authoring time.** From blank, the reference system is entered by run sheet
   and solved in ≤ 30 minutes *(provisional)*.
8. **Usability.**
   - The RT-U2 walkthrough is recorded, with its use cases, findings and
     dispositions. Its Must findings are closed.
   - Owner-arranged user testing follows. Its findings are the owner's to
     disposition.
9. **Performance.** On a 2,000-node rack model, on the 16 GiB base machine,
   measured by the DEL-04-05 harness *(provisional)*:
   - linear static ≤ 10 s;
   - nonlinear operating ≤ 60 s.
10. **Autosave.** It is on by default; a crash loses at most one interval.
11. **Benchmark set.** Every case is screened against the §3 profile, and the
    findings are triaged. The owner has reviewed the full record.
12. **Reproduction.** A fresh reproduction bundle exists at the release
    candidate.
13. **Documentation.** The user guide and theory notes cover every Must
    analysis: inputs, method, assumptions and limitations. They state that
    SWBPIPE does not determine code compliance.
14. **Content check.** The release-candidate protected-content scan is signed
    off by the owner (DEL-08-05:2).
15. **Build and licence.** The build is signed and notarized, and MIT is
    recorded consistently.
16. **Public repository.** `sgttomas/SWBPIPE` is SWBPIPE's independent public
    home, with the release candidate bound to its repository revision and
    standalone build/test/CI verification. Before v1.0 release, the owner has
    completed and signed the D-20/DEC-058 scan of that repository, including
    the full private-term scan and per-finding dispositions; unresolved
    blocking findings prevent release. The verified migration import may be
    pushed before signature (owner, 2026-10-29: "Split repo replaces RT-V8";
    "Push now, sign before v1.0"). A merge, passing check or unsigned scan
    does not supply the owner's signature or release acceptance (DEL-10-04:1).
17. **Embedded agent.** From a person's request in the desktop's conversation
    panel, the embedded agent builds the reference system, solves it and
    reports its key results, through SWBPIPE's capability catalog. Every change
    goes through the one operation route and is applied on the person's
    acceptance or within autonomy the person granted; the agent records no
    acceptance, approval or reliance (F-PIP-3).

## 5. Maintaining this target

- **Changing rows.** HELP_HUMAN may add rows at Should or Later only, and
  adjust Should and Later rows, under the owner's delegation of 2026-10-27
  (`loop/LOOP_INIT.md`) by editing this file, recording the reason in the row
  and reporting a summary to the owner.
  - A row change that alters commitment or acceptance text in a ScopeOfWork
    takes the independent review `LOOP_INIT.md` requires.
  - A row change that alters numerical meaning takes targeted regression checks
    and independent scrutiny.
- **Citing rows.** Briefs and PRs for work toward the target name the `RT-`
  row they serve. This is HELP_HUMAN's coordination rule, adopted in its
  HELP_HUMAN role, not an owner ruling.
- **Reserved to the owner.**
  - Changing an exit criterion or its value.
  - Adding a Must row, or promoting a row to Must, because it raises the exit
    criteria.
  - Downgrading a Must row.
  - Declaring a limitation acceptable.
  - Acceptance, issuance and release.
  - Risk acceptance and holds.
  - Purchases.
  - The native-smoke `expected.json` values.
- **Edits.** Decisions edit the row they govern. This file is not a log; Git
  keeps its history.
- **Limitations list.** The release limitations that criterion 3 refers to
  are kept in `docs/RELEASE_LIMITATIONS_v1.0.md`. Lanes draft entries there;
  accepting one is the owner's act.
