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

Paths are under `projects/chirality-piping/`. **PF** is the pressure-free solve
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
- **Public repository** (owner, 2026-10-28: "Yes."). The public sanitized
  repository export (DEL-10-04:1) is in v1.0 (RT-V8).
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
"Current" is the state on `main` with its evidence. "Owner" is the owning
deliverable; "new" means a scope change assigns it.

### Analyses

| Row | Capability | Level | Current (evidence) | Open work / owner |
|---|---|---|---|---|
| RT-A1 | Linear static | Must | Built on both routes (W-9 benchmarks) | — / DEL-04-01 |
| RT-A2 | Nonlinear static operating (gaps, lift-off, friction) | Must | PF only. Mechanics combinations of nonlinear models are withheld; differences and envelopes are formed from rows (#1695) | #1654, #1671 / DEL-04-04, DEL-04-07 |
| RT-A3 | One route: EX admits every family PF has (tees, nonlinear and constant-effort supports, reducers in pressure regions) | Must | EX refuses these by name (`EXACT_PRESSURE_FAMILY_NOT_ADMITTED`, `EXACT_PRESSURE_REDUCER_IN_REGION`, PP `exact_admission.rs`) | unscheduled / DEL-04-07 |
| RT-A4 | Combinations: algebraic, difference, envelope, absolute sum, SRSS | Must | Algebraic, difference and envelope on both routes (`load_case_algebra`, #1698). No absolute sum or SRSS | unscheduled / DEL-05-02 |
| RT-A5 | Sustained, operating and expansion sets. The expansion basis is recorded; sustained can take the operating support status | Must (basis), Should (status) | `difference_basis` designed in model document 0.9.0 (#1703) | build unscheduled / DEL-05-02, DEL-02-01 |
| RT-A6 | Hydrostatic test: test pressure, test contents, springs held, marked as a test case | Must | PF blocks hydrotest pressure (PP `validation.rs:62-66`); no test condition exists | unscheduled / DEL-05-01, after RT-G5 |
| RT-A7 | Modal analysis (PRD §12.9) | Must | Design only (DEL-04-09 CLM-003) | M1a–M3 / DEL-04-09 |
| RT-A8 | Response spectrum, uniform support motion: user spectra, mode and directional combination, missing mass; snubbers as dynamic-only restraints. Independent (multi-level) support motion is Should | Must (uniform), Should (independent) | Absent | SCA-014 / new |
| RT-A9 | Static occasional: wind, three-axis seismic g | Must | DEC-068 generators on PF and plain EX documents (#1698). Refused on EX load/reference-state documents | #1678 / DEL-05-01 |
| RT-A10 | Imposed displacement, including seismic anchor movement | Must | EX `boundary_motion`. PF stage 1 (#1615); PF with nonlinear supports is refused | #1654, #1671 / DEL-04-03 |
| RT-A11 | Hanger sizing from the user's private catalogue, with a hanger table | Must | Records import and apply one to a support (`features/hanger-selection`); no sizing procedure | unscheduled / new |
| RT-A12 | Cold spring | Should | Absent | unscheduled / DEL-05-01 |
| RT-A13 | Numerical diagnostics and health | Must | Built (FR-SOL-015; #1544, #1665) | — / DEL-04-06 |
| RT-A14 | Harmonic, time history, force spectrum, buried pipe; verified bounds | Later | Absent; verified bounds dormant and owner-held (DEL-04-08) | — |

### Model, components, supports, loads

| Row | Capability | Level | Current (evidence) | Open work / owner |
|---|---|---|---|---|
| RT-G1 | Model document, persistence, units (SI, US, custom) | Must | Document 0.8.0 (#1649); units at creation and after (#1569, #1627) | 0.9.0 build / DEL-02-01, DEL-02-02 |
| RT-G2 | Authoring: viewport, run sheet, bulk change, split into parts, slope entry, context menus | Must | #1693, #1679, #1667. B1 run-sheet rules and builder (#1699); B5 split and slope (#1700) | run-sheet UI slices / DEL-07-01, DEL-07-02 |
| RT-G3 | Materials, sections, temperature-dependent modulus, mass properties | Must | Libraries with provenance (#1566); per-case modulus basis (DEC-092) | GUI entry of `modulus_basis_records` (DEL-07-02:5) |
| RT-G4 | Model checks | Must | Built (W-8; #1665) | — / DEL-04-06 |
| RT-G5 | Operating-condition sets along runs | Must | Designed (#1703) | build / DEL-02-01, DEL-05-01 |
| RT-K1 | Bends with user k and SIFs | Must | Built (DEC-070; EX realized bends) | — / DEL-03-03 |
| RT-K2 | Tees with user modifiers; branch flexibility | Must (tee), Should (branch flexibility) | PF only, with user SIF; branch flexibility not assembled | RT-A3 / DEL-03-04 |
| RT-K3 | Reducers, valves, flanges, rigid items, weight at CoG | Must | Built (#1604, #1608, #1649) | RT-A3 for reducers in pressure regions / DEL-03-05 |
| RT-K4 | Expansion joints, tie rods, pressure thrust | Must | EX only; `movement_limit` not enforced | — / DEL-03-06 |
| RT-K5 | Timoshenko straights on PF | Should | EX only | #1663 / DEL-05-03 |
| RT-S1 | Linear restraints and springs | Must | Built on both routes | — / DEL-04-03 |
| RT-S2 | Gaps, lift-off, friction | Must | PF only. Friction is static and history-free; rotational gap and rotational friction are refused | RT-A3 / DEL-04-04 |
| RT-S3 | Variable spring and constant-effort supports | Must | Variable spring = user-stiffness spring; whether the installed load acts as a preload is not yet settled (open work). Constant effort on PF only | RT-A3 / DEL-04-03 |
| RT-L1 | Weight, pressure, thermal (many cases), user forces including relief thrust, wind | Must | Pressure on EX only; thermal per case; DEC-068 wind | RT-A3, RT-G5 / DEL-05-01, DEL-05-05 |
| RT-L2 | Bourdon effect (pressure elongation) as an option, off by default and clearly labelled. Pressure-dependent flexibility and SIF corrections are out of scope; users enter their own k and SIF | Should | EX excludes Bourdon opening and pressure stiffening of k/SIF, and discloses it (DEL-04-07 `Design/pressure-contract.md` D-3) | unscheduled / DEL-04-07 |

### Results, reports, interoperation

| Row | Capability | Level | Current (evidence) | Open work / owner |
|---|---|---|---|---|
| RT-R1 | Results review: navigator, contours, deflected shape, key results, support loads, extremes, weight/CoG, sorted stresses | Must | #1687, #1689, #1667, #1694, #1668, #1675, #1677, #1674 | hanger table with RT-A11 / DEL-07-05 |
| RT-R2 | Frequencies, mode shapes and participation views | Must | Absent | M3 / DEL-04-09 |
| RT-R3 | User rule packs as a design aid; outcomes "within / exceeds user limit". Survey extensions C2, E1, E2, E5 and E6 and the plugin/adapter runtime are deferred until after v1.0; E3 and E4 are dropped | Should | Format 0.5.0 (#1629); outcome wording still pass/fail | wording / DEL-06-01, DEL-07-03, DEL-08-01 |
| RT-R4 | Calculation report with a section composer | Must | Local-private render and package on both routes (W-3, #1648); no composer | composer / DEL-08-01 |
| RT-R5 | Shareable report released by the user | Must | W-18b r1–r2b (#1661, #1664, #1673); EX refused on route 010 | #1682 / DEL-08-01 |
| RT-R6 | CSV export of every results table | Must | Absent; result JSON and stress-neutral JSON exist | unscheduled / DEL-08-04 |
| RT-R7 | MBF export from the desktop | Must | Headless SI pilot writer (#1652); the desktop panel is a preview and withholds MBF text | unscheduled / DEL-17-04 |
| RT-R8 | MBF import into the desktop | Must | Headless importer tranches 1–3 (#1670, #1685, #1697) and authored cases (#1702) | unscheduled / DEL-17-04 |
| RT-R9 | PCF, glTF, handoff package, native JSON | Should | Save Handoff (#1600); PCF and glTF text withheld | — / DEL-17-07, DEL-17-08, DEL-15-01 |

### Human factors, validation, release

| Row | Capability | Level | Current (evidence) | Open work / owner |
|---|---|---|---|---|
| RT-U1 | Coherent visual design, keyboard routes, accessibility of touched controls | Must | Bounded D-68 baseline (DEL-07-06) | walkthrough findings |
| RT-U2 | Usability basis: Agent 0 use-case walkthrough, then owner-arranged user testing | Must | Absent | walkthrough / DEL-07-06 |
| RT-U3 | Autosave and backup | Must | Absent | unscheduled / DEL-12-01, DEL-02-05 |
| RT-U4 | Performance on large models | Must | No thresholds (DEL-04-05:3) | harness run / DEL-04-05 |
| RT-U5 | States, runs, comparisons | Should | Built (W-12); mapping editor, tolerance profiles and export are Later | — / PKG-14 |
| RT-V1 | Closed-form benchmarks for every Must analysis | Must | §22.2 and §22.3 suites; none yet for RT-A4 SRSS/absolute sum, RT-A6, RT-A7, RT-A8, RT-A11 | with each row / PKG-09 |
| RT-V2 | Benchmark set against CAEPIPE (§3) | Must | Proposal (validation lane); correlation pilot (#1652, #1672) | §3 / DEL-17-05 |
| RT-V3 | Validation manual and a fresh reproduction bundle | Must | Procedure current; last bundle predates W-12 | W-2 / DEL-09-04 |
| RT-V4 | User guide and theory notes for every Must analysis | Must | Statics only (`docs/theory/centerline_analysis.md`) | DEL-11-01, DEL-11-03 |
| RT-V5 | Claim fence on every surface (notice, GEN-13, release labels) | Must | GEN-13 clean (#1701) and run in hosted CI (#1705) | release labels / DEL-09-05, DEL-16-04 |
| RT-V6 | Signed, notarized macOS build; MIT licence recorded consistently | Must | Signing decided (D-06b), not executed; MIT in `LICENSE.md` (D-74) and DEL-01-01 | DEL-10-04 |
| RT-V7 | Windows build | Later (revisited immediately after v1.0) | macOS Apple Silicon only (DEC-057) | — |
| RT-V8 | Public sanitized repository export | Must | Pipeline exists; not activated (DEL-10-04:1) | owner-signed export scan, public repository and allowlist review / DEL-10-04 |

### Embedded agent

| Row | Capability | Level | Current (evidence) | Open work / owner |
|---|---|---|---|---|
| RT-E1 | The App v4 agent harness embedded in the desktop: a conversation panel, SWBPIPE's capability catalog of reads and changes, and changes through the one operation route, applied on the person's acceptance or within the autonomy the person granted | Must | Operation engine, batch review queue and offline proposal intake built; no embedded harness, catalog or agent transport (DEL-16-03:2) | connection design with App v4 / new, PKG-16 |

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
14. **Claims and content checks.**
    - GEN-13 passes over all product texts in hosted CI.
    - The release-candidate protected-content scan is signed off by the owner
      (DEL-08-05:2).
15. **Build and licence.** The build is signed and notarized, and MIT is
    recorded consistently.
16. **Public repository.** The sanitized export is staged at the release
    candidate, and its scan is signed off by the owner (DEL-10-04:1).
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
