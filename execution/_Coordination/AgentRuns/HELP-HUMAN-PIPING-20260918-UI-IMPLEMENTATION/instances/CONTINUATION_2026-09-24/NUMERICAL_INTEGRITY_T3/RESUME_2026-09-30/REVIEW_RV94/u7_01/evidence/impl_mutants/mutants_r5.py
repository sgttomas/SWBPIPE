#!/usr/bin/env python3
"""I67 U7 slice T mutant programme (scratch): the U6d set on the U7 basis (N17 rewritten for the
status table), plus B01-B05 (live-capture binding), G01-G03 (panel gates), T01s-T03s (token and
status) and H01-H02 . U6d round 04: round 03's set plus X01-X05 (the carrier
transport route, RV88/RV92 N-1). Round 03 (merged carriers head): round 02's set,
K17-K20 rewritten for the SI table (S-2), plus U01-U05. Round 02: round 01's set on the repaired
candidate, S29 rewritten for the restructured standing text, plus the repair-round mutants
(SF-1, N-4) and RV91's RV03, RV04, RV08 and RV09. One textual mutation per new branch, applied
in the mutant lane only (WT/scratch/i67_u6d/lanes/mut), run against the U6d tests
plus the related existing tests, then restored. Usage: mutants.py <out.json> [ids...]"""
import json, os, subprocess, sys, time
T3 = "WT"
LANE = f"WT/rv94/mut_ts/projects/chirality-piping/apps/desktop"
OUT_DIR = f"WT/scratch/rv94_u7_01/impl/runs/r5"
ENV = dict(os.environ, TMPDIR=f"WT/scratch/rv94_u7_01/tmp")
R = "src/features/results/"
S = "src/services/"
NEW = [R + "retainedPrecisionIntegration.test.tsx", S + "retainedPrecisionAnalysisRun.test.ts", R + "retainedPrecisionOutputRefusal.test.tsx"]
RELATED = [R + "numericalResultQuality.test.ts", R + "knownSemanticLimitations.test.ts", R + "resultSemantics.test.ts", S + "analysisRunCompatibility.test.ts",
           S + "previewService.test.ts", S + "ruleCheckService.test.ts", R + "HistoricalRunContext.test.tsx", R + "ResultsPanel.test.tsx"]
NRQ, RPS, KSL, LROA, RS, ARC, PS, RCS, RP, HRC = (R + "numericalResultQuality.ts", R + "retainedPrecisionStanding.ts", R + "knownSemanticLimitations.ts",
    R + "loadReferenceOutputAvailability.ts", R + "resultSemantics.ts", S + "analysisRunCompatibility.ts", S + "previewService.ts", S + "ruleCheckService.ts",
    R + "ResultsPanel.tsx", R + "HistoricalRunContext.tsx")
REP = "src/features/result-export/ResultExportPanel.tsx"; SNP = "src/features/stress-neutral/StressNeutralExportPanel.tsx"; ITEST = R + "retainedPrecisionIntegration.test.tsx"
M = [
  # numericalResultQuality.ts: dispatch, binding, downgrade guard, standing branch, shared predicate
  ("N01", NRQ, 'if (source.producer?.semantic_contract_id === PREVIEW_PHYSICS_RETAINED_CONTRACT_ID) return false;\n', '', "guard: the successor itself is not exempt"),
  ("N02", NRQ, 'return Object.hasOwn(source, "retained_precision")\n    ||', 'return false\n    ||', "guard: receipt member ignored"),
  ("N03", NRQ, '|| (Array.isArray(source.results) && source.results.some(row =>', '|| (false && source.results.some(row =>', "guard: token rows ignored"),
  ("N04", NRQ, 'source.results.some(row => (row as', 'source.results.every(row => (row as', "guard: every row instead of any row"),
  ("N05", NRQ, ')?.recovery_method === RETAINED_METHOD));', ')?.recovery_method !== undefined));', "guard: any method string"),
  ("N06", NRQ, '  if (retainedPrecisionDowngrade(source)) return "unsupported";\n', '', "dispatch: guard not applied"),
  ("N07", NRQ, 'retained ? f?.profile_id === PREVIEW_PHYSICS_RETAINED_PROFILE && evidenceObject', 'retained ? evidenceObject', "dispatch: profile unchecked"),
  ("N08", NRQ, 'f?.profile_id === PREVIEW_PHYSICS_RETAINED_PROFILE && evidenceObject && !!source', 'f?.profile_id === PREVIEW_PHYSICS_RETAINED_PROFILE && !!source', "dispatch: contract evidence unchecked"),
  ("N09", NRQ, '&& !!source.retained_precision && typeof', '&& typeof', "dispatch: null receipt admitted"),
  ("N10", NRQ, '&& typeof source.retained_precision === "object" && !Array', '&& !Array', "dispatch: non-object receipt admitted"),
  ("N11", NRQ, ' && !Array.isArray(source.retained_precision)\n', '\n', "dispatch: array receipt admitted"),
  ("N12", NRQ, 'LOAD_REFERENCE_SOURCE_CONTRACT_ID, PREVIEW_PHYSICS_RETAINED_CONTRACT_ID].includes', 'LOAD_REFERENCE_SOURCE_CONTRACT_ID].includes', "dispatch: id not admitted"),
  ("N13", NRQ, 'retained ? "retained_preview_physics" :', 'retained ? "preview_physics" :', "dispatch: routed as preview-physics-1"),
  ("N14", NRQ, 'return { id: PREVIEW_PHYSICS_RETAINED_CONTRACT_ID, sha256: PREVIEW_PHYSICS_RETAINED_CONTRACT_SHA256 };', 'return { id: PREVIEW_PHYSICS_RETAINED_CONTRACT_ID, sha256: PREVIEW_PHYSICS_CONTRACT_SHA256 };', "binding: wrong table hash"),
  ("N15", NRQ, 'findings.push(retainedPrecisionDowngrade(source) ? RETAINED_PRECISION_DOWNGRADE_FORBIDDEN : "SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED")', 'findings.push("SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED")', "standing: downgrade code not reported"),
  ("N16", NRQ, '  else if (contract === "retained_preview_physics") {\n', '  else if (contract === "retained_preview_physics" && false) {\n', "standing: successor routed to the generic numerical_quality branch"),
  ("N17", NRQ, 'status: RETAINED_STANDING_STATUS[standing.standing]', 'status: "needs_recompute" as const', "standing: eligible status never shown"),
  ("N18", NRQ, '!!c && c.structural_status === "passive_model_basis" && c.solve_quality', '!!c && c.solve_quality', "predicate: structural status dropped"),
  ("N19", NRQ, '&& c.solve_quality === "checks_passed"\n', '\n', "predicate: solve quality dropped"),
  ("N20", NRQ, '&& c.model_matrix_fidelity === "represented_equations_retained" &&', '&&', "predicate: fidelity dropped"),
  ("N21", NRQ, '&& ["not_claimed", "reference_verified"].includes(c.accuracy_evidence)\n', '\n', "predicate: accuracy evidence dropped"),
  ("N22", NRQ, '&& !!c.evidence_refs.length &&', '&&', "predicate: empty evidence refs admitted"),
  ("N23", NRQ, 'c.evidence_refs.every(id => emitted.has(id))', 'c.evidence_refs.some(id => emitted.has(id))', "predicate: unresolved refs admitted"),
  # retainedPrecisionStanding.ts: registration, standing, seams, classes, summary, text
  ("S01", RPS, '    record({ validation, error: null });\n', '', "register: success not recorded"),
  ("S02", RPS, "    record({ validation: null, error: (error as RetainedPrecisionError).message });\n", '', "register: refusal not recorded"),
  ("S03", RPS, '    record({ validation: null, error: (error as RetainedPrecisionError).message });\n    throw error;', '    record({ validation: null, error: (error as RetainedPrecisionError).message });\n    return undefined as never;', "register: refusal not rethrown (native registration proceeds)"),
  ("S04", RPS, 'const record = (outcome: Outcome) => { if (sourcePrint) registrations', 'const record = (outcome: Outcome) => { if (true) registrations', "register: unchecked bytes recorded"),
  ("S05", RPS, 'return current && current.text === registration.source.text && current.negativeZeros', 'return current && current.negativeZeros', "registered: text not compared"),
  ("S06", RPS, ' && current.negativeZeros === registration.source.negativeZeros ? registration', ' ? registration', "registered: zero signs not compared"),
  ("S07", RPS, 'if (registration.outcome.error !== null) return { standing: "unsupported", eligible: false, findings: [registration.outcome.error] };', 'if (registration.outcome.error !== null) return { standing: "needs_recompute", eligible: false, findings: [registration.outcome.error] };', "standing: refusal not unsupported"),
  ("S08", RPS, 'findings: [registration.outcome.error] };', 'findings: [RETAINED_PRECISION_VALIDATION_REQUIRED] };', "standing: reader code not reported"),
  ("S09", RPS, 'if (!validation.invocation_bound || !validation.numerical_eligible', 'if (!validation.numerical_eligible', "seam: invocation binding not required"),
  ("S10", RPS, 'if (!validation.invocation_bound || !validation.numerical_eligible\n', 'if (!validation.invocation_bound\n', "seam: reader eligibility not required"),
  ("S11", RPS, '    || cases.length !== requested.length || cases.some(', '    || cases.some(', "seam: requested count not compared"),
  ("S12", RPS, ' || cases.some((c, index) => !sameJson(c?.basis_ref, requested[index]))', '', "seam: requested order not compared"),
  ("S13", RPS, '    || source.status?.mechanics !== "MECHANICS_SOLVED"\n', '', "seam: mechanics status not required"),
  ("S14", RPS, '    || !notRequiredCasesOrdinarilyEligible(source, cases)) return "needs_recompute";', '    ) return "needs_recompute";', "seam: not_required conjunct dropped"),
  ("S15", RPS, '    if (c?.status === "selected") return true;\n', '', "not_required: selected cases checked as not_required"),
  ("S16", RPS, '    if (c?.status !== "not_required") return false;\n', '', "not_required: other statuses admitted"),
  ("S17", RPS, 'return !!q && sameJson(q.basis_ref, c.basis_ref) && ordinaryCaseEligible', 'return !!q && ordinaryCaseEligible', "not_required: basis ref not compared"),
  ("S18", RPS, 'return !!q && sameJson', 'return sameJson', "not_required: missing quality case admitted"),
  ("S19", RPS, 'if (typeof id !== "string" || !id || ids.has(id)) return false;', 'if (typeof id !== "string" || !id) return false;', "not_required: duplicate ids admitted"),
  ("S20", RPS, 'if (typeof id !== "string" || !id || ids.has(id)) return false;', 'if (ids.has(id)) return false;', "not_required: missing or empty ids admitted"),
  ("S21", RPS, 'map(c => ({ ref_type: "load_case", ref_id: c.id }))', 'map(c => ({ ref_type: "combination", ref_id: c.id }))', "requested: wrong ref type"),
  ("S22", RPS, '  if (!retainedPrecisionStanding(source, model).eligible) return null;\n', '', "invocation: returned without eligible standing"),
  ("S23", RPS, 'return validation ? new Map(validation.classifications.map(row => [row.result_id, row])) : null;', 'return null;', "classes: never available"),
  ("S24", RPS, 'relative_verified: 0, absolute_verified: 1, not_covered: 2,', 'relative_verified: 0, absolute_verified: 2, not_covered: 1,', "summary: absolute and not_covered swapped"),
  ("S25", RPS, 'const withheld = current ? n[1] + n[2] : n[0] + n[1] + n[2] + n[3];', 'const withheld = n[0] + n[1] + n[2] + n[3];', "summary: current standing ignored"),
  ("S26", RPS, 'const withheld = current ? n[1] + n[2] :', 'const withheld = current ? n[1] :', "summary: not_covered not withheld when current"),
  ("S27", RPS, 'if (sameJson(row.basis_ref?.ref_id, id)) n[', 'if (true) n[', "summary: rows not filtered by case"),
  ("S28", RPS, "return validation ? classificationSummaryFrom(validation, source!, requestedRefs(model)) : [];", "return validation ? classificationSummaryFrom(validation, source!, []) : [];", "summary: model requested cases ignored"),
  ("S29", RPS, '  if (outcome.error !== null) return `Retained precision: receipt refused', '  if (outcome.error === null) return `Retained precision: receipt refused', "text: refusal and validation swapped"),
  ("S30", RPS, '${outcome.validation.invocation_bound ? "against the actual invocation" : "without an invocation"}', 'against the actual invocation', "text: invocation binding not stated"),
  ("S31", RPS, 'const selected = cases.filter(c => c?.status === "selected").length;', 'const selected = cases.length;', "text: selected count not counted"),
  ("S32", RPS, 'return registered(source)?.outcome ?? null;', 'return registered(source)?.outcome;', "registration query: undefined for none"),
  # Repair round 02: N-4 (the case count only from a validated registration)
  ("T01", RPS, 'needs recompute. ${tail}`;', 'needs recompute. Selected cases: ${receiptCases(source).length} of ${receiptCases(source).length}. ${tail}`;', "text: case count from an unvalidated receipt"),
  ("T02", RPS, 'values shown for inspection only. ${tail}`;', 'values shown for inspection only. Selected cases: ${receiptCases(source).length} of ${receiptCases(source).length}. ${tail}`;', "text: case count from a refused receipt"),
  # RV91's surviving and probe-only mutants (their text, on the current code)
  ("RV03", RPS, 'captured = structuredClone(invocation);', 'captured = invocation;', "registration: holds the caller's live invocation object"),
  ("RV04", RPS, 'return structuredClone(registered(source)!.invocation);', 'return registered(source)!.invocation;', "registered invocation handed out by reference"),
  ("RV09", RPS, 'const record = (outcome: Outcome) => { if (sourcePrint) registrations.set(source, { source: sourcePrint,', 'const record = (outcome: Outcome) => { const late = fingerprint(source); if (late && sourcePrint) registrations.set(source, { source: late,', "registration: fingerprint taken after the reader's await"),
  # knownSemanticLimitations.ts
  ("K01", KSL, '  LOAD_REFERENCE_SOURCE_CONTRACT_ID,\n  PREVIEW_PHYSICS_RETAINED_CONTRACT_ID,\n]);', '  LOAD_REFERENCE_SOURCE_CONTRACT_ID,\n]);', "fresh set: successor absent"),
  ("K02", KSL, '  if (source.producer?.semantic_contract_id === PREVIEW_PHYSICS_RETAINED_CONTRACT_ID) {\n    const classes', '  if (false) {\n    const classes', "binding: successor not class-refused"),
  ("K03", KSL, 'return classes ? classBindingRefusal(classes.get(row.id)?.class) : RULE_QUANTITY_NOT_COVERED;', 'return classes ? classBindingRefusal(classes.get(row.id)?.class) : null;', "binding: unregistered rows bind"),
  ("K04", KSL, 'return cls === "absolute_verified" ? RULE_QUANTITY_BELOW_VERIFIED_FLOOR : cls === "not_covered" ? RULE_QUANTITY_NOT_COVERED : null;', 'return cls === "absolute_verified" ? RULE_QUANTITY_NOT_COVERED : cls === "not_covered" ? RULE_QUANTITY_BELOW_VERIFIED_FLOOR : null;', "binding: codes swapped"),
  ("K05", KSL, 'return cls === "absolute_verified" ? RULE_QUANTITY_BELOW_VERIFIED_FLOOR : cls === "not_covered" ? RULE_QUANTITY_NOT_COVERED : null;', 'return cls === "absolute_verified" ? RULE_QUANTITY_BELOW_VERIFIED_FLOOR : null;', "binding: not_covered binds"),
  ("K06", KSL, 'if (route === "preview_physics" || route === "retained_preview_physics") {', 'if (route === "preview_physics") {', "notices: preview notices not reused"),
  ("K07", KSL, '    if (!retainedRowClasses(source)) notices.push({ id: "retained-precision-unvalidated"', '    if (false) notices.push({ id: "retained-precision-unvalidated"', "notices: unvalidated notice absent"),
  ("K08", KSL, 'if (c.absolute_verified > 0) notices', 'if (c.absolute_verified >= 0) notices', "notices: empty absolute notice"),
  ("K09", KSL, 'if (c.not_covered > 0) notices', 'if (c.not_covered >= 0) notices', "notices: empty not-covered notice"),
  ("K10", KSL, 'if (c.absolute_verified > 0) notices', 'if (false) notices', "notices: absolute summary absent"),
  ("K11", KSL, 'if (c.not_covered > 0) notices', 'if (false) notices', "notices: not-covered summary absent"),
  ("K12", KSL, '  if (b === 0) return "0";\n', '', "bound text: zero not exact"),
  ("K13", KSL, 'if (Number(`${mantissa}e${exponent}`) > b) return', 'if (Number(`${mantissa}e${exponent}`) >= b) return', "bound text: equal printed value not stepped up"),
  ("K14", KSL, 'if (Number(`${mantissa}e${exponent}`) > b) return', 'if (true) return', "bound text: nearest, not upward"),
  ("K15", KSL, '  if (digits >= 1000) { digits = 100; power += 1; }\n', '', "bound text: mantissa overflow not carried"),
  ("K16", KSL, '${power < 0 ? "-" : "+"}', '${power < 0 ? "+" : "+"}', "bound text: exponent sign"),
  ("K17", KSL, 'Object.freeze({ m: "m", mm: "m", rad: "rad", N: "N", kN: "N", "N*m": "N*m", "kN*m": "N*m", Pa: "Pa", MPa: "Pa" })', 'Object.freeze({ m: "m", mm: "m", rad: "rad", N: "N", kN: "N", "N*m": "N*m", "kN*m": "N*m", Pa: "Pa" })', "units: MPa not normalized"),
  ("K18", KSL, 'Object.freeze({ m: "m", mm: "m", rad: "rad", N: "N", kN: "N", "N*m": "N*m", "kN*m": "N*m", Pa: "Pa", MPa: "Pa" })', 'Object.freeze({ m: "m", rad: "rad", N: "N", kN: "N", "N*m": "N*m", "kN*m": "N*m", Pa: "Pa", MPa: "Pa" })', "units: mm not normalized"),
  ("K19", KSL, 'Object.freeze({ m: "m", mm: "m", rad: "rad", N: "N", kN: "N", "N*m": "N*m", "kN*m": "N*m", Pa: "Pa", MPa: "Pa" })', 'Object.freeze({ m: "m", mm: "m", rad: "rad", N: "N", "N*m": "N*m", Pa: "Pa", MPa: "Pa" })', "units: kN and kN*m not normalized"),
  ("K20", KSL, 'return si === null ? N_RP_NOT_COVERED : N_RP_ABSOLUTE', 'return si === null ? N_RP_ABSOLUTE.replace("{b}", upwardBoundText(decodeBinary64(boundBits))).replace("{unit}", unit) : N_RP_ABSOLUTE', "units: an unnormalized unit names a bound in the published unit"),
  ("U01", KSL, 'Object.freeze({ m: "m", mm: "m", rad: "rad", N: "N", kN: "N", "N*m": "N*m", "kN*m": "N*m", Pa: "Pa", MPa: "Pa" })', 'Object.freeze({ mm: "m", rad: "rad", N: "N", kN: "N", "N*m": "N*m", "kN*m": "N*m", Pa: "Pa", MPa: "Pa" })', "units: m not in the table"),
  ("U02", KSL, 'Object.freeze({ m: "m", mm: "m", rad: "rad", N: "N", kN: "N", "N*m": "N*m", "kN*m": "N*m", Pa: "Pa", MPa: "Pa" })', 'Object.freeze({ m: "m", mm: "m", N: "N", kN: "N", "N*m": "N*m", "kN*m": "N*m", Pa: "Pa", MPa: "Pa" })', "units: rad not in the table"),
  ("U03", KSL, 'Object.freeze({ m: "m", mm: "m", rad: "rad", N: "N", kN: "N", "N*m": "N*m", "kN*m": "N*m", Pa: "Pa", MPa: "Pa" })', 'Object.freeze({ m: "m", mm: "m", rad: "rad", kN: "N", "kN*m": "N*m", MPa: "Pa" })', "units: SI units themselves not in the table"),
  ("U04", KSL, 'const si = Object.hasOwn(RETAINED_SI_UNIT, unit) ? RETAINED_SI_UNIT[unit] : null;', 'const si = RETAINED_SI_UNIT[unit] ?? null;', "units: inherited object members read as units"),
  ("U05", KSL, 'return si === null ? N_RP_NOT_COVERED : N_RP_ABSOLUTE', 'return si === null ? N_RP_UNVALIDATED : N_RP_ABSOLUTE', "units: an unnormalized unit labelled unvalidated, not uncovered"),
  ("K21", KSL, '  if (classified?.class === "absolute_verified") return', '  if (false) return', "label: absolute rows unlabelled"),
  ("K22", KSL, '  if (classified?.class === "not_covered") return N_RP_NOT_COVERED;\n', '', "label: not_covered rows unlabelled"),
  ("K23", KSL, 'if (route !== "preview_physics" && route !== "retained_preview_physics") return null;', 'if (route !== "preview_physics") return null;', "label: successor not labelled"),
  ("K24", KSL, '  const classLabel = retainedRowClassLabel(row, source);', '  const classLabel = null as string | null;', "label: class label never shown"),
  ("K25", KSL, 'return classLabel && kindLabel ? `${classLabel} ${kindLabel}` : classLabel ?? kindLabel;', 'return classLabel ?? kindLabel;', "label: kind label dropped with class label"),
  ("RV08", KSL, 'if (source.producer?.semantic_contract_id === PREVIEW_PHYSICS_RETAINED_CONTRACT_ID) {\n    const classes', 'if (sourceContract(source) === "retained_preview_physics") {\n    const classes', "binding: successor selected by route, not producer id"),
  # Round 04: the carrier transport route (RV88 and RV92 N-1)
  ("X01", NRQ, '  if (route === "retained_preview_physics") await validateRetainedPrecisionTransport(source);\n', '', "transport: the reader's transport checks not run (header shape only)"),
  ("X02", NRQ, '  if (route === "retained_preview_physics") await validateRetainedPrecisionTransport(source);', '  await validateRetainedPrecisionTransport(source);', "transport: the reader's checks applied to every identity"),
  ("X03", NRQ, 'throw new Error(retainedPrecisionDowngrade(source) ? RETAINED_PRECISION_DOWNGRADE_FORBIDDEN : "SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED");', 'throw new Error("SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED");', "transport: downgrade code not reported"),
  ("X04", NRQ, '  if (route === "unsupported") throw new Error(retainedPrecisionDowngrade(source)', '  if (route === "unsupported" && false) throw new Error(retainedPrecisionDowngrade(source)', "transport: an unsupported header admitted"),
  ("X05", NRQ, '  if (route === "retained_preview_physics") await validateRetainedPrecisionTransport(source);', '  if (route === "retained_preview_physics" && Array.isArray(source.results)) await validateRetainedPrecisionTransport(source);', "transport: a header-only (row-less) transport not checked"),
  # U7 slice T: N-2, the live native capture and the current model
  ("B01", RPS, '  if (registration.live?.(model) !== true) return { standing: "needs_recompute", eligible: false, findings: [RETAINED_PRECISION_NATIVE_CAPTURE_REQUIRED] };\n', '', "binding: the live-capture check removed"),
  ("B02", RPS, 'if (registration.live?.(model) !== true) return', 'if (registration.live === null) return', "binding: a capture's presence, not its liveness for this model"),
  ("B03", PS, 'await registerRetainedPrecision(source, capture.invocation, model => hasNativeMechanicsInvocation(source, model as PreviewModel));', 'await registerRetainedPrecision(source, capture.invocation);', "binding: the native capture never passed"),
  ("B04", PS, 'model => hasNativeMechanicsInvocation(source, model as PreviewModel)', '() => true', "binding: a capture that is always live, for any model"),
  ("B05", RPS, '  if (registration.outcome.error !== null) return { standing: "unsupported", eligible: false, findings: [registration.outcome.error] };\n', '  if (registration.outcome.error !== null) return { standing: "unsupported", eligible: false, findings: [registration.outcome.error] };\n  if (registration.live?.(model) !== true) return { standing: "needs_recompute", eligible: false, findings: [RETAINED_PRECISION_NATIVE_CAPTURE_REQUIRED] };\n', "binding: checked before eligibility (unforced outcomes change)"),
  # U7 slice T: N-5, the panels' explicit gates
  ("G01", REP, '!hasCurrentSourceContract(result) || loadReferenceOutputRefusal(result) !== null', '!hasCurrentSourceContract(result)', "result export: the gate removed"),
  ("G02", SNP, '!hasCurrentSourceContract(result) || loadReferenceOutputRefusal(result) !== null', '!hasCurrentSourceContract(result)', "stress-neutral: the gate removed"),
  ("G03", SNP, ') : loadReferenceOutputRefusal(result) ? (', ') : loadReferenceOutputRefusal(result) && false ? (', "stress-neutral: the refusal not displayed (generic empty text)"),
  # U7 slice T: RV92 N-8, the token and TS's status mapping
  ("T01s", NRQ, 'numerically_eligible: "integrity_checked", needs_recompute', 'numerically_eligible: "needs_recompute", needs_recompute', "status: eligible mapped to needs_recompute"),
  ("T02s", NRQ, 'status: RETAINED_STANDING_STATUS[standing.standing]', 'status: standing.standing as "needs_recompute"', "status: the token reported as the status string"),
  ("T03s", NRQ, 'needs_recompute: "needs_recompute", unsupported: "needs_recompute" } as const', 'needs_recompute: "needs_recompute", unsupported: "unsupported" } as unknown as { numerically_eligible: "integrity_checked"; needs_recompute: "needs_recompute"; unsupported: "needs_recompute" }', "status: an unsupported status invented"),
  ("Q01", ITEST, '    const token = route === "unsupported" ? "unsupported" : retainedPrecisionStanding(received, model).standing;\n    const expected = ELIGIBLE_AFTER_FLIP', '    const token = route === "unsupported" ? "unsupported" : numericalResultStanding(received, model).status;\n    const expected = ELIGIBLE_AFTER_FLIP', "harness: forced-eligibility parity compared by status string"),
  ("Q02", ITEST, '    const standing = route === "unsupported" ? "unsupported" : route === "retained_preview_physics" ? retainedPrecisionStanding(received, model).standing : "unexpected";', '    const standing = route === "unsupported" ? "unsupported" : route === "retained_preview_physics" ? numericalResultStanding(received, model).status : "unexpected";', "harness: shared parity compared by status string"),
  # loadReferenceOutputAvailability.ts
  ("L01", LROA, 'try { return sourceContract(source!) === "retained_preview_physics"; } catch { return false; }', 'return false;', "refusal: successor not refused"),
  ("L02", LROA, 'try { return sourceContract(source!) === "retained_preview_physics"; } catch { return false; }', 'return sourceContract(source!) === "retained_preview_physics";', "refusal: null source throws"),
  ("L03", LROA, ': isRetainedPrecisionRoute(source) ? RETAINED_PRECISION_OUTPUT_REFUSAL : null;', ': isRetainedPrecisionRoute(source) ? LOAD_REFERENCE_OUTPUT_REFUSAL : null;', "refusal: load-reference text shown"),
  # resultSemantics.ts
  ("R01", RS, "if (sourceContract(source) === 'retained_preview_physics') return previewPhysicsRetainedContract;", "if (sourceContract(source) === 'retained_preview_physics') return previewPhysicsContract;", "table: inherited table returned"),
  # analysisRunCompatibility.ts
  ("A01", ARC, '  if (route === "retained_preview_physics") await validateRetainedPrecision(result);\n', '', "build: recorded before the reader"),
  ("A02", ARC, '  if (route === "retained_preview_physics") record.analysis_run.retained_precision = structuredClone(result.retained_precision);\n', '', "build: receipt not copied"),
  ("A03", ARC, '    await validateRetainedPrecision(source);\n    if (!same(run.retained_precision', '    if (!same(run.retained_precision', "validate: source not revalidated"),
  ("A04", ARC, '    if (!same(run.retained_precision, source.retained_precision)) throw new Error(ANALYSIS_RETAINED_PRECISION_RECEIPT_MISMATCH);\n', '', "validate: copy equality not checked"),
  ("A05", ARC, '  } else if (Object.hasOwn(run, "retained_precision")) throw new Error(ANALYSIS_RETAINED_PRECISION_DOWNGRADE_FORBIDDEN);\n', '  }\n', "validate: receipt on another identity admitted"),
  # Repair round 02: SF-1 (the historical v0.2 builder never drops a receipt)
  ("V01", ARC, '  if (retainedPrecisionDowngrade(result)) throw new Error(ANALYSIS_LEGACY_SOURCE_DOWNGRADE_FORBIDDEN);\n', '', "v0.2: receipt and tokens dropped silently"),
  ("V02", ARC, '  if (retainedPrecisionDowngrade(result)) throw new Error(ANALYSIS_LEGACY_SOURCE_DOWNGRADE_FORBIDDEN);', '  if (Object.hasOwn(result, "retained_precision")) throw new Error(ANALYSIS_LEGACY_SOURCE_DOWNGRADE_FORBIDDEN);', "v0.2: token rows admitted"),
  ("V03", ARC, '  if (retainedPrecisionDowngrade(result)) throw new Error(ANALYSIS_LEGACY_SOURCE_DOWNGRADE_FORBIDDEN);', '  if (!!(result as { retained_precision?: unknown }).retained_precision) throw new Error(ANALYSIS_LEGACY_SOURCE_DOWNGRADE_FORBIDDEN);', "v0.2: a null member and token rows admitted"),
  ("V04", ARC, '  if (retainedPrecisionDowngrade(result)) throw new Error(ANALYSIS_LEGACY_SOURCE_DOWNGRADE_FORBIDDEN);', '  if (retainedPrecisionDowngrade(result)) throw new Error("HISTORICAL_ANALYSIS_SOURCE_UNSUPPORTED");', "v0.2: the twin code not used"),
  # previewService.ts
  ("P01", PS, '    if (sourceContract(source) === "retained_preview_physics") await registerRetainedPrecision(source, capture.invocation, model => hasNativeMechanicsInvocation(source, model as PreviewModel));\n', '', "capture: successor registered without the reader"),
  ("P02", PS, 'await registerRetainedPrecision(source, capture.invocation, model =>', 'await registerRetainedPrecision(source, undefined, model =>', "capture: invocation not bound"),
  # ruleCheckService.ts
  ("C01", RCS, '    if (!standing.eligible) throw new Error(', '    if (false) throw new Error(', "gate: successor not gated"),
  ("C02", RCS, 'if (sourceContract(source) === "retained_preview_physics") invokeArgs.sourceBlockInvocation = retainedPrecisionInvocation(source, args.model);', '', "gate: invocation not passed"),
  ("C03", RCS, 'reason === RULE_QUANTITY_BELOW_VERIFIED_FLOOR || reason === RULE_QUANTITY_NOT_COVERED ? retainedRowClassLabel', 'reason === RULE_QUANTITY_BELOW_VERIFIED_FLOOR ? retainedRowClassLabel', "precheck: not_covered shows N-SB"),
  ("C04", RCS, 'retainedRowClassLabel(row, source) ?? N_RP_UNVALIDATED : N_SB;', 'retainedRowClassLabel(row, source) ?? N_SB : N_SB;', "precheck: unvalidated shows N-SB"),
  ("C05", RCS, 'throw new Error(`${standing.findings[0]}: the retained', 'throw new Error(`RULE_SOURCE_IDENTITY_NOT_FRESH: the retained', "gate: standing finding not reported"),
  # ResultsPanel.tsx
  ("RP1", RP, ': sourceContract(result) === "retained_preview_physics" ? retainedPrecisionStandingText(result) :', ':', "panel: generic numerical_quality text for the successor"),
  ("RP2", RP, '["preview_physics", "retained_preview_physics"].includes(sourceContract(result))', '["preview_physics"].includes(sourceContract(result))', "panel: successor rows unlabelled"),
  # HistoricalRunContext.tsx
  ("H01", HRC, '  if (mechanicsResult && sourceContract(mechanicsResult) === "retained_preview_physics") {', '  if (false) {', "reopen: successor not revalidated"),
  ("H02", HRC, '    try { await validateRetainedPrecision(mechanicsResult); } catch (error) { findings.push((error as Error).message); }\n', '', "reopen: reader not run"),
  ("H03", HRC, '    if (record && !(await sameSavedReceipt(record.retained_precision, mechanicsResult.retained_precision))) findings.push(ANALYSIS_RETAINED_PRECISION_RECEIPT_MISMATCH);\n', '', "reopen: copy not compared"),
  ("H04", HRC, '    if (record && !(await sameSavedReceipt(', '    if (!(await sameSavedReceipt(', "reopen: absent record compared"),
  ("H05", HRC, 'try { return await canonicalSha256HexCheckedV1(copy) === await canonicalSha256HexCheckedV1(receipt); } catch { return false; }', 'try { return await canonicalSha256HexCheckedV1(copy) === await canonicalSha256HexCheckedV1(receipt); } catch { return true; }', "reopen: absent copy matches"),
]

def run(tag, files):
    os.makedirs(OUT_DIR, exist_ok=True)
    out = f"{OUT_DIR}/{tag}.json"
    if os.path.exists(out): os.remove(out)
    p = subprocess.run(["../../node_modules/.bin/vitest", "run", "--maxWorkers=4", "--reporter=json", f"--outputFile={out}", *files], cwd=LANE, env=ENV, capture_output=True, text=True, timeout=1800)
    try: d = json.load(open(out))
    except Exception: return {"exit": p.returncode, "load_error": True, "failed": None, "passed": None, "first": p.stderr[-400:]}
    failed = [(f["name"].split("/src/")[1], a["fullName"]) for f in d["testResults"] for a in f["assertionResults"] if a["status"] == "failed"]
    load = [f["name"].split("/src/")[1] + ": " + (f.get("message") or "")[:200] for f in d["testResults"] if f["status"] == "failed" and not any(a["status"] == "failed" for a in f["assertionResults"])]
    return {"exit": p.returncode, "failed": len(failed), "passed": d["numPassedTests"], "load_errors": load, "first": failed[:3]}

def main():
    out_path, ids = sys.argv[1], set(sys.argv[2:])
    files = NEW + RELATED
    results = {"lane": "WT/scratch/i67_u6d/lanes/mut", "files": files, "mutants": []}
    t0 = time.time()
    ctrl = run("control", files)
    results["control"] = ctrl
    assert ctrl.get("failed") == 0 and not ctrl.get("load_errors"), ctrl
    for mid, path, old, new, desc in M:
        if ids and mid not in ids: continue
        full = f"{LANE}/{path}"
        text = open(full).read()
        n = text.count(old)
        if n != 1:
            results["mutants"].append({"id": mid, "file": path, "desc": desc, "error": f"match count {n}"}); continue
        open(full, "w").write(text.replace(old, new))
        try: r = run(mid, files)
        finally: open(full, "w").write(text)
        killed = bool(r.get("failed")) or bool(r.get("load_errors")) or r.get("load_error", False)
        results["mutants"].append({"id": mid, "file": path, "desc": desc, "killed": killed, "by_assertion": bool(r.get("failed")), **r})
        print(mid, "KILLED" if killed else "SURVIVED", r.get("failed"), (r.get("first") or [""])[0], flush=True)
    results["seconds"] = round(time.time() - t0)
    json.dump(results, open(out_path, "w"), indent=1)
    s = [m["id"] for m in results["mutants"] if not m.get("killed")]
    print("total", len(results["mutants"]), "survivors", s)

if __name__ == "__main__":
    main()
