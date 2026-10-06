"""I75 T6S mutant programme (final: T6S-3, T6S-4, T6S-5). Optional argv[3]: a comma-separated subset of mutant ids. Each mutant is one exact, single-occurrence
replacement in a scratch copy; the control run must pass; a mutant is killed when the selected
test files fail. Usage: python3 mutants_cp1.py <copy>/projects/chirality-piping <log-dir>"""
import json, pathlib, subprocess, sys, shutil
P = pathlib.Path(sys.argv[1]); LOG = pathlib.Path(sys.argv[2]); LOG.mkdir(parents=True, exist_ok=True)
F = "apps/desktop/src/features/"
POL, DIS, SN, LRA, REP = F+"results/outputPolicy.ts", F+"results/retainedPrecisionDisclosure.ts", F+"stress-neutral/StressNeutralExportPanel.tsx", F+"results/loadReferenceOutputAvailability.ts", F+"result-export/ResultExportPanel.tsx"
TESTS = ["src/features/stress-neutral/retainedPrecisionStressNeutral.test.tsx", "src/features/results/outputPolicy.test.ts",
         "src/features/results/retainedPrecisionOutputRefusal.test.tsx", "src/features/results/loadReferenceOutputRefusal.test.tsx",
         "src/features/stress-neutral/StressNeutralExportPanel.test.tsx", "src/features/result-export/ResultExportPanel.test.tsx",
         "src/features/result-export/retainedPrecisionResultExport.test.tsx", "src/features/result-export/resultExportAdapter.test.ts",
         "src/features/result-export/physicsResultExport.test.ts", "src/features/results/retainedPrecisionIntegration.test.tsx"]
RE_ = F+"result-export/resultExportAdapter.ts"
M = [
 ("M01", "gate admits a non-panel surface (rule-check)", POL, '"rule-check": "refused", "report-lint"', '"rule-check": "admitted_when_eligible", "report-lint"'),
 ("M02", "admission without the live capture (standing not read)", POL, 'return standing.eligible ? null : `', 'return true ? null : `'),
 ("M03", "a route without an entry fails open", POL, 'if (policy === null) return { decision: "refused", reason: OUTPUT_ROUTE_NOT_REGISTERED_REFUSAL };', 'if (policy === null) return null;'),
 ("M04", "a surface the entry does not name is admitted", POL, 'policy.surfaces[surface as OutputSurface] : "refused";', 'policy.surfaces[surface as OutputSurface] : "admitted_when_eligible";'),
 ("M05", "load_reference route ungated", POL, '  load_reference: Object.freeze({ gate: "per_surface", reason: LOAD_REFERENCE_OUTPUT_REFUSAL, surfaces: everySurfaceRefused() }),', '  load_reference: UNGATED,'),
 ("M06", "the shared function stops refusing the successor on the eighteen surfaces", LRA, 'return routeOutputRefusal(source);', 'return isLoadReferenceRoute(source) ? routeOutputRefusal(source) : null;'),
 ("M07", "stress-neutral panel shows no policy refusal", SN, ') : outputRefusal ? (', ') : false ? ('),
 ("M08", "summary line swaps the two counts", DIS, '${c.absolute_verified} verified only to an absolute bound; ${c.not_covered} uncovered', '${c.not_covered} verified only to an absolute bound; ${c.absolute_verified} uncovered'),
 ("M09", "e+ exponent left unnormalized", DIS, '.toExponential().replace("e+", "e")', '.toExponential()'),
 ("M10", "negative-zero sign dropped", DIS, 'value < 0 || Object.is(value, -0) ? "-" : ""', 'value < 0 ? "-" : ""'),
 ("M11", "SI unit table maps mm to mm", DIS, 'mm: "m", rad', 'mm: "mm", rad'),
 ("M12", "an unnormalized unit still names a bound", DIS, 'if (si === null) return { code: RETAINED_NOT_COVERED, message: retainedNotCoveredMessage(kind) };', 'if (si === null) return { code: RETAINED_ABSOLUTE_VERIFIED, message: retainedNotCoveredMessage(kind) };'),
 ("M13", "a dropped disclosure (absolute class not disclosed)", DIS, 'if (classified.class === "absolute_verified") {', 'if (classified.class === "absolute_verified" && false) {'),
 ("M14", "classes never read from the reader", DIS, 'if (source?.producer?.semantic_contract_id !== PREVIEW_PHYSICS_RETAINED_CONTRACT_ID) return null;', 'return null;'),
 ("M15", "UTF-8 CSV policy omits the successor", SN, '"preview_physics", "retained_preview_physics"].includes(sourceContract(source));', '"preview_physics"].includes(sourceContract(source));'),
 ("M16", "successor semantic table path wrong", SN, 'retained_preview_physics: "semantic_contract_v0_3_preview_physics_retained_1.json"', 'retained_preview_physics: "semantic_contract_v0_3_preview_physics_1.json"'),
 ("M17", "contract_evidence not copied for the successor", SN, 'route === "preview_physics" || retained) packet.contract_evidence', 'route === "preview_physics") packet.contract_evidence'),
 ("M18", "receipt not copied into the package", SN, 'if (retained) packet.retained_precision = structuredClone(args.result.retained_precision);', ''),
 ("M19", "missing receipt in the transport header", SN, ', ...(Object.hasOwn(packet,"retained_precision") ? {retained_precision:packet.retained_precision} : {}) }', ' }'),
 ("M20", "transport validation skipped for the successor", SN, 'else if (route === "retained_preview_physics") await validateRetainedPrecisionTransport(header);', 'else if (route === "retained_preview_physics") { /* skipped */ }'),
 ("M21", "withheld-witness code emitted with the wrong severity", SN, 'code: WITHHOLDING_CODES[disposition], class: "unit_preservation_witness", severity: "info",', 'code: WITHHOLDING_CODES[disposition], class: "unit_preservation_witness", severity: "blocking",'),
 ("M22", "validator accepts the wrong severity for the new codes", SN, 'new Set(["diagnostic_work", "retained_absolute_verified", "retained_not_covered"])', 'new Set(["diagnostic_work"])'),
 ("M23", "receipt equality not checked against the source", SN, 'if (!await same(packet.retained_precision, source.retained_precision)) throw new Error("SN-RETAINED-PRECISION-RECEIPT-MISMATCH");', '/* receipt unchecked */'),
 ("M24", "a receipt on another identity not refused (with a source)", SN, '} else if (Object.hasOwn(packet, "retained_precision")) throw new Error(RETAINED_PRECISION_DOWNGRADE_FORBIDDEN);', '}'),
 ("M25", "loss-report class count not checked", SN, 'if (exported.length !== 1 || exported[0].reason !== reason) throw new Error("SN-RETAINED-PRECISION-LOSS-COUNT-MISMATCH");', '/* count unchecked */'),
 ("M26", "class findings not counted in the loss report", SN, '${retained ? retainedLossCount(', '${false ? retainedLossCount('),
 ("M27", "builder ignores classes (no class findings)", SN, 'strictWitnessDisposition(source, args.result, classes) :', 'strictWitnessDisposition(source, args.result, null) :'),
 ("M28", "a valued absolute row (witness kept alongside the finding)", SN, 'const strictWitnesses = dispositions.filter((item) => item.disposition === "eligible")', 'const strictWitnesses = dispositions.filter((item) => item.disposition === "eligible" || item.disposition === "retained_absolute_verified")'),
 ("M29", "class finding message replaced by a generic text", SN, '      message: classMessage,\n', '      message: "Retained precision class.",\n'),
 ("M30", "builder admits without eligibility (route level only)", SN, 'refuseSurfaceOutput(args.result, args.model, "stress-neutral");', 'refuseSurfaceRoute(args.result, "stress-neutral");'),
 ("M31", "validator keeps the old shared refusal", SN, '  refuseSurfaceRoute(source, "stress-neutral");\n  if (source !== undefined) {', '  if (source && sourceContract(source) === "retained_preview_physics") throw new Error(RETAINED_PRECISION_OUTPUT_REFUSAL_TEXT);\n  refuseSurfaceRoute(source, "stress-neutral");\n  if (source !== undefined) {'),
 ("M32", "legacy 0.2.0 packet may carry a receipt", SN, '"contract_evidence", "source_annotations", "retained_precision"].some(key => Object.hasOwn(packet, key))', '"contract_evidence", "source_annotations"].some(key => Object.hasOwn(packet, key))'),
 ("M33", "a blocking disposition loses precedence to a class (fail-safe order)", SN, 'if (classes === null || BLOCKING_WITHHOLDINGS.has(ordinary.disposition)) return ordinary;', 'if (classes === null) return ordinary;'),
 ("M34", "diagnostic work takes precedence over a class", SN, 'if (classes === null || BLOCKING_WITHHOLDINGS.has(ordinary.disposition)) return ordinary;', 'if (classes === null || BLOCKING_WITHHOLDINGS.has(ordinary.disposition) || ordinary.disposition === "diagnostic_work") return ordinary;'),
 ("R01", "derivative: a dropped disclosure (classes ignored per row)", RE_, "const classDisclosure=retainedClassDisclosure(row.kind,row.unit,classes?.get(row.id));\n    const disposition=", "const classDisclosure=null as any;\n    const disposition="),
 ("R02", "derivative: a valued absolute_verified row (disposition ignores the class)", RE_, "const disposition=reviewMissing||physicalMissing||classDisclosure?'disclosed'", "const disposition=reviewMissing||physicalMissing?'disclosed'"),
 ("R03", "derivative: the class row keeps the ordinary reason code", RE_, "const reason=classDisclosure?classDisclosure.code:reviewMissing", "const reason=reviewMissing"),
 ("R04", "derivative: the class row keeps the ordinary message", RE_, "message:classDisclosure?classDisclosure.message:`", "message:`"),
 ("R05", "derivative: the receipt not copied", RE_, "if(route==='retained_preview_physics')e.retained_precision=structuredClone(source.retained_precision);", ""),
 ("R06", "derivative: contract_evidence not copied for the successor", RE_, "||route==='preview_physics'||route==='retained_preview_physics')e.contract_evidence=", "||route==='preview_physics')e.contract_evidence="),
 ("R07", "derivative: classes never read from the reader", RE_, "const classes=await retainedRowClassesFromReader(source);\n  if(route==='physics')validatePhysicsEvidence(source,model);", "const classes=null as any;\n  if(route==='physics')validatePhysicsEvidence(source,model);"),
 ("R08", "validator: receipt equality unchecked", RE_, "if(route==='retained_preview_physics')requireEqual(doc.result_envelope.retained_precision,source.retained_precision,'RETAINED_PRECISION_RECEIPT_BINDING_MISMATCH');", "if(route==='retained_preview_physics'){}"),
 ("R09", "validator: a receipt on another identity unrefused", RE_, "else if(Object.hasOwn(doc.result_envelope,'retained_precision'))throw new Error(RETAINED_PRECISION_DOWNGRADE_FORBIDDEN);", ""),
 ("R10", "validator: class code and message unchecked", RE_, "if(classes){const claimed=", "if(false&&classes){const claimed="),
 ("R11", "validator: a non-class disclosure may claim a class code", RE_, ":claimed)throw new Error('DISCLOSURE_SEMANTICS');", ":false)throw new Error('DISCLOSURE_SEMANTICS');"),
 ("R12", "validator: the class disposition ignored", RE_, "if(classDisclosure)disposition='disclosed';", ""),
 ("R13", "validator: classes never read from the reader", RE_, "const classes=await retainedRowClassesFromReader(source);\n  // D2 4.9.7", "const classes=null as any;\n  // D2 4.9.7"),
 ("R14", "Current builder: admission at route level only (no live capture)", RE_, "refuseSurfaceOutput(result,model,'result-export');", "refuseSurfaceRoute(result,'result-export');"),
 ("R15", "Current builder: the policy gate removed", RE_, "refuseSurfaceOutput(result,model,'result-export');", ""),
 ("R16", "origin: the limit argument ignored", RE_, "origin_limit:originLimit,", "origin_limit:CURRENT_ORIGIN_LIMIT,"),
 ("R17", "derivative: the successor still refused (the old shared refusal)", RE_, "refuseSurfaceRoute(source,'result-export');const version", "if(route==='retained_preview_physics')throw new Error('RETAINED-PRECISION-OUTPUT-NOT-YET-AVAILABLE');refuseSurfaceRoute(source,'result-export');const version"),
 ("R18", "derivative: the route gate removed (load/reference admitted)", RE_, "refuseSurfaceRoute(source,'result-export');const version", "const version"),
 ("R19", "validator: the route gate removed", RE_, "throw new Error(\"SOURCE_SEMANTIC_CONTRACT_UNSUPPORTED\");refuseSurfaceRoute(source,'result-export');", "throw new Error(\"SOURCE_SEMANTIC_CONTRACT_UNSUPPORTED\");"),
 ("R20", "result-export panel: the policy gate removed", REP, ' || surfaceOutputRefusal(result, model, "result-export") !== null', ''),
]
def run(tag):
    r = subprocess.run(["../../node_modules/.bin/vitest", "run", "--maxWorkers=6", "--reporter=json", f"--outputFile={LOG}/{tag}.json", *TESTS], cwd=P / "apps/desktop", capture_output=True, text=True)
    try: d = json.load(open(LOG / f"{tag}.json")); return d["numFailedTests"], d["numTotalTests"], [t["fullName"] for f in d["testResults"] for t in f["assertionResults"] if t["status"] == "failed"][:3], d["numFailedTestSuites"]
    except Exception as e: return None, None, [r.stdout[-500:], r.stderr[-500:]], None
results = []
ctl = run("control"); print("control", ctl[:2], ctl[3]); results.append({"id": "control", "failed": ctl[0], "total": ctl[1]})
assert ctl[0] == 0, ctl
ONLY = set(sys.argv[3].split(",")) if len(sys.argv) > 3 else None
for mid, desc, rel, old, new in M:
    if ONLY is not None and mid not in ONLY: continue
    path = P / rel; orig = path.read_text()
    if mid == "M31": new = new.replace("RETAINED_PRECISION_OUTPUT_REFUSAL_TEXT", '"RETAINED-PRECISION-OUTPUT-NOT-YET-AVAILABLE"')
    n = orig.count(old)
    if n != 1: print(mid, "SKIP occurrences", n); results.append({"id": mid, "desc": desc, "status": f"not applied ({n})"}); continue
    path.write_text(orig.replace(old, new))
    try: failed, total, names, suites = run(mid)
    finally: path.write_text(orig)
    status = "killed" if failed else ("killed (suite error)" if suites else "SURVIVED")
    print(mid, status, failed, total, names[:2]); results.append({"id": mid, "desc": desc, "file": rel, "status": status, "failed": failed, "total": total, "first_failures": names})
json.dump(results, open(LOG / ("mutants_final.json" if ONLY is None else "mutants_final_subset.json"), "w"), indent=1)
