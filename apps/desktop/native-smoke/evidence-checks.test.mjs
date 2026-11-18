// The evidence journeys' pure checks, without the app or a build.
//
//   node --test apps/desktop/native-smoke/evidence-checks.test.mjs
import { test } from "node:test";
import assert from "node:assert/strict";
import * as fs from "node:fs";
import { lstatSync, mkdirSync, mkdtempSync, readFileSync, realpathSync, renameSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import os from "node:os";
import path from "node:path";
import { DatabaseSync } from "node:sqlite";
import {
  autosaveSlot, canonical, crc32, documentSha256, driverTimeout, dumpStore, inspectReportPackage, isCanonicalText,
  judgeAutosaveRecord, packageStatusFields, parseAutosaveName, parseCsvFile, prdRequiredNotice, readStoredZip,
  sha256Hex, timeoutText, ALL_JOURNEYS, parseRunnerArgs, homeRemovalRefusal, removalIdentityRefusal, readHomeMarker, HOME_MARKER,
  UNFINISHED_BY_MACHINE, CONFINEMENT_TERM, NOTHING_ELSE_WATCHED, TRIPWIRE_LIMITS, STRICT_CHOOSER_EXIT,
  FAULT_PHASE, faultDeclaration, faultDeclarationRefusal, ordinaryLaunchRefusal, finalReportAbsence, faultPhaseFailures, autosaveJourneyFailures
} from "./evidence-checks.mjs";
import { STRICT_CHOOSER_ENV, STRICT_CHOOSER_VALUE } from "./launch-env.mjs";
import { launchFailures } from "./launch-outcome.mjs";

test("only the autosave-recovery journey's autosave-end phase may carry the fault declaration, exactly as declared", () => {
  const declared = { phase: "autosave-end", fault: faultDeclaration() };
  assert.deepEqual(FAULT_PHASE, { journey: "autosave-recovery", phase: "autosave-end" });
  assert.equal(faultDeclaration().expected_signal, "SIGKILL");
  assert.equal(faultDeclarationRefusal("autosave-recovery", declared), null);
  assert.match(faultDeclarationRefusal("autosave-recovery", { phase: "autosave-end" }), /^FAULT-UNDECLARED/);
  assert.match(faultDeclarationRefusal("autosave-recovery", { phase: "autosave-end", fault: { ...faultDeclaration(), expected_signal: "SIGTERM" } }), /^FAULT-DECLARATION-WRONG/);
  assert.match(faultDeclarationRefusal("autosave-recovery", { phase: "autosave-reopen", fault: faultDeclaration() }), /^FAULT-NOT-DECLARABLE/);
  assert.match(faultDeclarationRefusal("report-exact", { phase: "autosave-end", fault: faultDeclaration() }), /^FAULT-NOT-DECLARABLE/);
  assert.equal(faultDeclarationRefusal("report-exact", { phase: "report" }), null, "an ordinary plan carries none");
  // The ordinary (judged) launch refuses a declared plan and the declared phase itself, declared or not.
  assert.equal(ordinaryLaunchRefusal("autosave-recovery", { phase: "autosave-reopen" }), null);
  assert.match(ordinaryLaunchRefusal("autosave-recovery", declared), /carries a fault declaration/);
  assert.match(ordinaryLaunchRefusal("autosave-recovery", { phase: "autosave-end" }), /is the declared fault phase/);
  assert.match(ordinaryLaunchRefusal("report-exact", { phase: "report", fault: faultDeclaration() }), /carries a fault declaration/);
});

test("a final report is absent only on ENOENT in a real folder; a file of any contents, or an unknown answer, is not absence", () => {
  const dir = realpathSync(mkdtempSync(path.join(os.tmpdir(), "evidence-final-report-")));
  try {
    const file = path.join(dir, "report-autosave-end.json");
    assert.deepEqual(finalReportAbsence(file, lstatSync), { absent: true });
    for (const [label, bytes] of [["empty", ""], ["malformed", "{\"ok\": tru"], ["parsed", "{\"ok\": false}"]]) {
      writeFileSync(file, bytes);
      assert.equal(finalReportAbsence(file, lstatSync).absent, false, label);
      rmSync(file);
    }
    writeFileSync(file, "{}");
    fs.chmodSync(file, 0o000);
    assert.equal(finalReportAbsence(file, lstatSync).absent, false, "an unreadable file is present");
    fs.chmodSync(file, 0o600);
    rmSync(file);
    mkdirSync(file);
    assert.match(finalReportAbsence(file, lstatSync).reason, /not a regular file/);
    rmSync(file, { recursive: true });
    assert.match(finalReportAbsence(path.join(dir, "gone", "report.json"), lstatSync).reason, /cannot be examined/);
    const eio = (target) => { if (target === file) { const error = new Error("io"); error.code = "EIO"; throw error; } return lstatSync(target); };
    assert.match(finalReportAbsence(file, eio).reason, /could not be established \(EIO\)/);
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
});

// A fault phase that met every condition, as the runner records it; each case below changes one thing.
function faultCase(change = {}) {
  const nonce = "a".repeat(32);
  const plan = { phase: "autosave-end", nonce, fault: faultDeclaration() };
  const base = {
    journey: "autosave-recovery", plan, planRecord: { plan, written_at: 1000 }, startedAt: 1001, nonce,
    ended: { value: { second: { nonce } }, observed_at: 5000, waited_out: false },
    result: { status: null, signal: "SIGKILL", timedOut: false, aborted: true, abort: { requested_at: 5001, target: "group 4242", signal: "SIGKILL", delivered: true, delivered_at: 5002, error: null } },
    census: { resolved: true }, finalReport: { absent: true }
  };
  return { ...base, ...change };
}
const codes = (failures) => failures.map((failure) => failure.split(":")[0]);

test("the declared fault phase passes only when every condition holds; each failed condition is named and none is reclassified", () => {
  assert.deepEqual(faultPhaseFailures(faultCase()), []);
  const undeclared = { phase: "autosave-end", nonce: "a".repeat(32) };
  const cases = {
    "FAULT-UNDECLARED": faultCase({ plan: undeclared, planRecord: { plan: undeclared, written_at: 1000 } }),
    "FAULT-NOT-DECLARABLE": faultCase({ journey: "report-exact" }),
    "FAULT-PLAN-NOT-RECORDED": faultCase({ planRecord: null }),
    "FAULT-DECLARED-LATE": faultCase({ planRecord: { plan: faultCase().plan, written_at: 1002 } }),
    "FAULT-CHECKPOINT-NOT-SEEN": faultCase({ ended: { value: null, observed_at: 5000, waited_out: true } }),
    "FAULT-CHECKPOINT-NOT-THIS-LAUNCH": faultCase({ ended: { value: { second: { nonce: "b".repeat(32) } }, observed_at: 5000, waited_out: false } }),
    "FAULT-SIGKILL-BEFORE-CHECKPOINT": faultCase({ ended: { value: { second: { nonce: "a".repeat(32) } }, observed_at: 6000, waited_out: false } }),
    "FAULT-SIGKILL-NOT-DELIVERED": faultCase({ result: { ...faultCase().result, aborted: false, abort: { ...faultCase().result.abort, delivered: false, delivered_at: null, error: "EPERM" } } }),
    "FAULT-TIMED-OUT": faultCase({ result: { ...faultCase().result, timedOut: true } }),
    "FAULT-CENSUS-UNRESOLVED": faultCase({ census: { resolved: false, reason: "process 7 still ran" } }),
    "FAULT-FINAL-REPORT-NOT-ABSENT": faultCase({ finalReport: { absent: false, reason: "a final report exists (a file of 11 bytes)" } })
  };
  for (const [code, input] of Object.entries(cases)) assert.ok(codes(faultPhaseFailures(input)).includes(code), `${code}: ${JSON.stringify(faultPhaseFailures(input))}`);
  // No SIGKILL attempted, a natural exit, another signal: not the declared ending.
  assert.ok(codes(faultPhaseFailures(faultCase({ result: { status: 0, signal: null, timedOut: false, aborted: false, abort: null } }))).includes("FAULT-SIGKILL-NOT-DELIVERED"));
  assert.ok(codes(faultPhaseFailures(faultCase({ result: { ...faultCase().result, status: 0, signal: null } }))).includes("FAULT-NOT-ENDED-BY-SIGKILL"), "a natural exit");
  assert.ok(codes(faultPhaseFailures(faultCase({ result: { ...faultCase().result, signal: "SIGTERM" } }))).includes("FAULT-NOT-ENDED-BY-SIGKILL"), "another signal");
  // Undelivered to a process rather than the group is not delivery to its own group.
  assert.ok(codes(faultPhaseFailures(faultCase({ result: { ...faultCase().result, abort: { ...faultCase().result.abort, target: "process 4242" } } }))).includes("FAULT-SIGKILL-NOT-DELIVERED"));
});

test("the fault phase alone never passes the autosave journey: an ordinary recovery launch and every recovery check must pass too", () => {
  const ordinary = (outcome) => launchFailures({ status: 0, signal: null, timedOut: false, census: { resolved: true }, report: { nonce: "n" }, nonce: "n", ...outcome });
  assert.deepEqual(autosaveJourneyFailures({ fault: [], reopen: ordinary({}), recovery: [] }), [], "a declared fault and a successful ordinary recovery");
  assert.ok(autosaveJourneyFailures({ fault: [], reopen: null, recovery: [] }).some((failure) => failure.startsWith("RECOVERY-NOT-LAUNCHED")), "no recovery launch");
  assert.ok(autosaveJourneyFailures({ fault: [], reopen: ordinary({ status: 86 }), recovery: [] }).some((failure) => failure.startsWith("LAUNCH-EXIT-NOT-ZERO")), "a failed recovery launch");
  assert.ok(autosaveJourneyFailures({ fault: [], reopen: ordinary({ signal: "SIGKILL", status: null }), recovery: [] }).some((failure) => failure.startsWith("LAUNCH-SIGNALLED")), "a recovery launch ended by a signal is not excused");
  assert.deepEqual(autosaveJourneyFailures({ fault: [], reopen: ordinary({}), recovery: ["the recovered document has the model hash the record's name carries"] }), ["the recovered document has the model hash the record's name carries"]);
  assert.ok(autosaveJourneyFailures({ fault: ["FAULT-NOT-ENDED-BY-SIGKILL: x"], reopen: ordinary({}), recovery: [] }).length > 0, "a failed fault phase with a good recovery");
  assert.ok(autosaveJourneyFailures({ fault: undefined, reopen: ordinary({}), recovery: [] }).includes("FAULT-NOT-JUDGED"));
});

test("the strict chooser's exit code and switch agree with the native-smoke build", () => {
  const rust = readFileSync(new URL("../src-tauri/src/native_smoke.rs", import.meta.url), "utf8");
  const constant = (name) => rust.match(new RegExp(`const ${name}: [^=]+= ([^;]+);`))?.[1]?.trim();
  assert.equal(constant("STRICT_CHOOSER_ENV"), JSON.stringify(STRICT_CHOOSER_ENV));
  assert.equal(constant("STRICT_CHOOSER_VALUE"), JSON.stringify(STRICT_CHOOSER_VALUE));
  assert.equal(Number(constant("STRICT_CHOOSER_EXIT")), STRICT_CHOOSER_EXIT);
});

test("the runner's command line: --all in its order, named journeys, the separate journey alone, --build-only", () => {
  assert.deepEqual(ALL_JOURNEYS[0], "autosave-recovery");
  assert.deepEqual(parseRunnerArgs(["--all"]), { journeys: ALL_JOURNEYS, buildOnly: false });
  assert.deepEqual(parseRunnerArgs(["--journey", "report-exact,report-pressure-free"]), { journeys: ["report-exact", "report-pressure-free"], buildOnly: false });
  assert.deepEqual(parseRunnerArgs(["--journey", "reference-system"]).journeys, ["reference-system"]);
  assert.match(parseRunnerArgs(["--journey", "reference-system,report-exact"]).error, /runs alone/);
  assert.match(parseRunnerArgs(["--all", "--journey", "report-exact"]).error, /not both/);
  assert.match(parseRunnerArgs([]).error, /choose --all/);
  assert.match(parseRunnerArgs(["--journey"]).error, /needs one or more/);
  assert.match(parseRunnerArgs(["--journey", "nope"]).error, /choose --all/);
  assert.match(parseRunnerArgs(["--al"]).error, /unknown arguments: --al/);
  // No lister path, no build during a launch invocation.
  assert.match(parseRunnerArgs(["--all", "--library-listing", "/x/library_listing.py"]).error, /unknown arguments: --library-listing \/x\/library_listing.py/);
  assert.match(parseRunnerArgs(["--all", "--skip-build"]).error, /unknown arguments: --skip-build/);
  assert.deepEqual(parseRunnerArgs(["--build-only"]), { journeys: [], buildOnly: true });
  assert.match(parseRunnerArgs(["--build-only", "--all"]).error, /takes no journey/);
});

test("the record's terms: the confinement term word for word, the tripwire's limits, the act left to a person", () => {
  assert.match(CONFINEMENT_TERM, /a run's record states that nothing else on the disk was watched\.$/);
  assert.equal(NOTHING_ELSE_WATCHED, "Nothing else on the disk was watched.");
  assert.match(TRIPWIRE_LIMITS, /cannot show what was read/);
  assert.match(TRIPWIRE_LIMITS, /does not establish that the app wrote nothing else outside its throwaway home/);
  assert.deepEqual(UNFINISHED_BY_MACHINE.map((item) => item.act), ["Copy"]);
});

test("only a throwaway home this run created is removed", () => {
  const run = "/private/var/folders/ab/T/swbpipe-evidence-journeys-Ab12";
  const home = `${run}/swbpipe-evidence-report-exact-Xy34/swbpipe-native-smoke-home-Qw56`;
  const created = new Set([home]);
  assert.equal(homeRemovalRefusal(home, created, run), null);
  assert.equal(homeRemovalRefusal(home, created, `${run}/`), null);
  assert.match(homeRemovalRefusal(`${run}/swbpipe-evidence-report-exact-Xy34`, created, run), /not a home this run created/);
  assert.match(homeRemovalRefusal("/Users/someone", new Set(["/Users/someone"]), run), /not under the run's folder/);
  assert.match(homeRemovalRefusal(`${run}/other/swbpipe-native-smoke-home-Qw56`, new Set([`${run}/other/swbpipe-native-smoke-home-Qw56`]), run), /not a throwaway home folder/);
  assert.match(homeRemovalRefusal(`${run}/swbpipe-evidence-x-1/home`, new Set([`${run}/swbpipe-evidence-x-1/home`]), run), /not a throwaway home folder/);
  assert.match(homeRemovalRefusal(`${run}/swbpipe-evidence-x-1/../swbpipe-native-smoke-home-Q`, new Set([`${run}/swbpipe-evidence-x-1/../swbpipe-native-smoke-home-Q`]), run), /without \.\./);
  assert.match(homeRemovalRefusal(`${run}/swbpipe-evidence-x-1/swbpipe-native-smoke-home-Q`, new Set(), run), /not a home this run created/);
});

test("a recursive removal needs the recorded identities up to the run's folder and the run's token in the home's marker", () => {
  const scratch = realpathSync(mkdtempSync(path.join(os.tmpdir(), "evidence-removal-")));
  const token = "f".repeat(32);
  const marker = (home) => readHomeMarker(home, fs);
  const identity = (dir) => (({ dev, ino }) => ({ dev, ino }))(lstatSync(dir));
  try {
    const setup = () => {
      const root = path.join(scratch, `swbpipe-evidence-journeys-${Math.random().toString(36).slice(2)}`);
      const journey = path.join(root, "swbpipe-evidence-report-exact-B2");
      const home = path.join(journey, "swbpipe-native-smoke-home-C3");
      mkdirSync(home, { recursive: true });
      writeFileSync(path.join(home, HOME_MARKER), token);
      const recorded = new Map([root, journey, home].map((dir) => [dir, identity(dir)]));
      return { root, journey, home, recorded };
    };
    const refusal = (where, options = {}) => removalIdentityRefusal(where.home, where.root, where.recorded, options.lstat ?? lstatSync, { token: options.token ?? token, readMarker: options.readMarker ?? marker });
    // The intact, owned home.
    const intact = setup();
    assert.equal(refusal(intact), null);
    // The marker missing, or holding another token, with every identity as recorded.
    const missing = setup();
    rmSync(path.join(missing.home, HOME_MARKER));
    assert.match(refusal(missing), /cannot be opened as a file \(ENOENT\)/);
    const wrong = setup();
    writeFileSync(path.join(wrong.home, HOME_MARKER), "0".repeat(32));
    assert.match(refusal(wrong), /does not hold this run's token/);
    // The marker a link to a file holding the token, or a folder: not read as a match.
    const linked = setup();
    const elsewhereToken = path.join(scratch, "token-elsewhere");
    writeFileSync(elsewhereToken, token);
    rmSync(path.join(linked.home, HOME_MARKER));
    symlinkSync(elsewhereToken, path.join(linked.home, HOME_MARKER));
    assert.match(refusal(linked), /cannot be opened as a file/);
    const folder = setup();
    rmSync(path.join(folder.home, HOME_MARKER));
    mkdirSync(path.join(folder.home, HOME_MARKER));
    assert.match(refusal(folder), /cannot be read|not a regular file|cannot be opened/);
    // A marker read failure is reported as such, not as a mismatch.
    assert.match(refusal(intact, { readMarker: () => ({ failure: "its marker cannot be read (EIO)" }) }), /EIO/);
    assert.match(refusal(intact, { token: "short" }), /no run token/);
    // A replacement with the recorded device and inode (established through a controlled lstat that reports the
    // recorded identity, not by assuming the file system reuses one) and no marker of this run is refused.
    const reused = setup();
    rmSync(reused.home, { recursive: true });
    mkdirSync(reused.home);
    const sameIdentity = (dir) => {
      const seen = lstatSync(dir);
      const want = reused.recorded.get(dir);
      return { isSymbolicLink: () => seen.isSymbolicLink(), isDirectory: () => seen.isDirectory(), dev: want?.dev ?? seen.dev, ino: want?.ino ?? seen.ino };
    };
    assert.match(refusal(reused, { lstat: sameIdentity }), /cannot be opened as a file \(ENOENT\)/);
    // A replacement with a different identity, made while the original is still allocated (renamed aside, so the two
    // cannot share an inode number), is refused by identity. Reuse of a freed inode number is not exercised here.
    const replaced = setup();
    renameSync(replaced.home, `${replaced.home}.original`);
    mkdirSync(replaced.home);
    writeFileSync(path.join(replaced.home, HOME_MARKER), token);
    assert.notDeepEqual(identity(replaced.home), replaced.recorded.get(replaced.home), "the replacement is another folder");
    assert.match(refusal(replaced), /not the directory the kit recorded/);
    // A swapped intermediate: the journey folder replaced by a link to elsewhere (holding a marked home of its own).
    const middle = setup();
    const elsewhere = path.join(scratch, "elsewhere-1");
    mkdirSync(path.join(elsewhere, "swbpipe-native-smoke-home-C3"), { recursive: true });
    writeFileSync(path.join(elsewhere, "swbpipe-native-smoke-home-C3", HOME_MARKER), token);
    renameSync(middle.journey, `${middle.journey}.moved`);
    symlinkSync(elsewhere, middle.journey);
    assert.match(refusal(middle), /is now a link|cannot be examined|not the directory/);
    // A swapped final: the home itself replaced by a link.
    const final = setup();
    const target = path.join(scratch, "elsewhere-2");
    mkdirSync(target);
    writeFileSync(path.join(target, HOME_MARKER), token);
    rmSync(final.home, { recursive: true });
    symlinkSync(target, final.home);
    assert.match(refusal(final), /home-C3 is now a link/);
    // A home outside the run's folder, and an unrecorded ancestor, are refused.
    assert.match(removalIdentityRefusal("/Users/someone/home", intact.root, intact.recorded, lstatSync, { token, readMarker: marker }), /not under the run's folder/);
    assert.match(removalIdentityRefusal(intact.home, intact.root, new Map(), lstatSync, { token, readMarker: marker }), /no identity was recorded/);
  } finally {
    rmSync(scratch, { recursive: true, force: true });
  }
});

const project = path.resolve(path.dirname(new URL(import.meta.url).pathname), "..", "..", "..");

// A stored ZIP written by the test's own writer, in the layout the report package uses.
function storedZip(entries, { tamper } = {}) {
  const local = [];
  const central = [];
  let offset = 0;
  for (const [name, text] of entries) {
    const data = Buffer.from(text, "utf8");
    const nameBytes = Buffer.from(name, "latin1");
    const header = Buffer.alloc(30);
    header.writeUInt32LE(0x04034b50, 0); header.writeUInt16LE(20, 4);
    header.writeUInt32LE(crc32(data), 14); header.writeUInt32LE(data.length, 18); header.writeUInt32LE(data.length, 22);
    header.writeUInt16LE(nameBytes.length, 26);
    const entry = Buffer.alloc(46);
    entry.writeUInt32LE(0x02014b50, 0); entry.writeUInt16LE(20, 4); entry.writeUInt16LE(20, 6);
    entry.writeUInt32LE(crc32(data), 16); entry.writeUInt32LE(data.length, 20); entry.writeUInt32LE(data.length, 24);
    entry.writeUInt16LE(nameBytes.length, 28); entry.writeUInt32LE(offset, 42);
    local.push(header, nameBytes, data);
    central.push(entry, nameBytes);
    offset += 30 + nameBytes.length + data.length;
  }
  const centralBytes = Buffer.concat(central);
  const end = Buffer.alloc(22);
  end.writeUInt32LE(0x06054b50, 0); end.writeUInt16LE(entries.length, 8); end.writeUInt16LE(entries.length, 10);
  end.writeUInt32LE(centralBytes.length, 12); end.writeUInt32LE(offset, 16);
  const bytes = Buffer.concat([...local, centralBytes, end]);
  if (tamper) tamper(bytes);
  return bytes;
}

const NOTICE = "SWBPIPE is decision-support software for piping design. Competent human review remains the user's.";
function packageBytes({ userRelease = true, notices = ["required", "user"], bindingSha = null } = {}) {
  const html = [
    "<html><body>",
    notices.includes("user") ? `<p>USER-RELEASED REPORT — the user released … model SHA-256 ${"a".repeat(64)}, result document r (SHA-256 ${"b".repeat(64)}).</p>` : "",
    notices.includes("local") ? "<p>LOCAL PRIVATE REPORT — rendered after explicit local-private export selection.</p>" : "",
    notices.includes("required") ? `<div>${NOTICE.replace(" piping", "\n piping")}</div>` : "",
    "</body></html>"
  ].join("");
  const result = canonical({ result_envelope: { producer: { semantic_contract_id: "openpipestress.result_semantics/0.3.0/preview-physics-2" } } });
  const members = [["report.html", html], ["result.json", result]];
  const rows = members.map(([name, text], index) => ({
    role: ["canonical_rendered_report_html", "result_export"][index], file_name: name,
    sha256_hex: sha256Hex(Buffer.from(text, "utf8")), byte_length: Buffer.byteLength(text)
  }));
  const manifest = { export_profile_id: "p", diagnostics: { export_blocked: false }, members: rows };
  if (userRelease) {
    manifest.user_release = {
      content_binding: { algorithm: "sha256", members: rows.map((row) => ({ file_name: row.file_name, role: row.role, sha256: bindingSha && row.role === "canonical_rendered_report_html" ? bindingSha : row.sha256_hex })) }
    };
  }
  return storedZip([["package_manifest.json", canonical(manifest)], ...members]);
}

test("RFC 8785 text and its hash", () => {
  assert.equal(canonical({ b: [1, 2.5, "x"], a: { d: null, c: true } }), '{"a":{"c":true,"d":null},"b":[1,2.5,"x"]}');
  assert.equal(documentSha256({ a: 1 }), sha256Hex(Buffer.from('{"a":1}')));
  assert.ok(isCanonicalText(Buffer.from('{"a":1,"b":2}')));
  assert.ok(!isCanonicalText(Buffer.from('{"b":2,"a":1}')));
  assert.ok(!isCanonicalText(Buffer.from('{"a":1}\n')));
  assert.ok(!isCanonicalText(Buffer.from("not json")));
});

test("a driver timeout is told apart, with the load average nearest its moment", () => {
  const samples = [{ at: 1700000000000, load: [10, 20, 30] }, { at: 1700000002000, load: [41.5, 38, 35] }];
  const seen = driverTimeout("x\nTIMEOUT waiting for exact solve: 900123 ms elapsed, limit 900000 ms, at 1700000001900\n  at stack", samples);
  assert.deepEqual(seen, { label: "exact solve", elapsed_ms: 900123, limit_ms: 900000, at: 1700000001900, load: [41.5, 38, 35], load_sampled_at: 1700000002000 });
  assert.match(timeoutText(seen), /^TIMEOUT waiting for exact solve: 900 s elapsed of a 900 s limit; load average 41\.50 \/ 38\.00 \/ 35\.00 \(1\/5\/15 min, sampled 0 s from the timeout\), on \d+ processors$/);
  assert.equal(driverTimeout("exact: state=failed", samples), null);
  assert.equal(driverTimeout("TIMEOUT waiting for a: 1 ms elapsed, limit 0 ms, at 1700000000000", []).load, null);
});

test("PRD §19.3's notice is read from the repository's text", () => {
  const notice = prdRequiredNotice(readFileSync(path.join(project, "docs/PRD.md"), "utf8"));
  assert.match(notice, /^SWBPIPE is decision-support software for piping design, flexibility, and stress-analysis workflows\./);
  assert.match(notice, /remain the responsibility of the user and project authority\.$/);
  assert.throws(() => prdRequiredNotice("# PRD\n"), /no §19\.3/);
});

test("CRC-32 and the stored ZIP reader", () => {
  assert.equal(crc32(Buffer.from("123456789")), 0xcbf43926);
  const bytes = storedZip([["a.json", "{}"], ["b.txt", "hello"]]);
  assert.deepEqual(readStoredZip(bytes).map((member) => [member.name, member.bytes.toString()]), [["a.json", "{}"], ["b.txt", "hello"]]);
  // A flipped content byte, a compressed method, a duplicate name and a trailing comment are refused.
  assert.throws(() => readStoredZip(storedZip([["a.json", "{}"]], { tamper: (b) => { b[30 + 6] ^= 1; } })), /CRC-32/);
  assert.throws(() => readStoredZip(storedZip([["a.json", "{}"]], { tamper: (b) => { b.writeUInt16LE(8, 8); } })), /local header differs/);
  assert.throws(() => readStoredZip(storedZip([["a", "1"], ["a", "2"]])), /duplicate/);
  assert.throws(() => readStoredZip(Buffer.concat([storedZip([["a", "1"]]), Buffer.from("x")])), /end-of-central-directory/);
});

test("a package is inspected from its own bytes", () => {
  const released = inspectReportPackage(packageBytes(), NOTICE);
  assert.equal(released.inventory_matches, true);
  assert.equal(released.manifest_canonical, true);
  assert.equal(released.content_binding_matches, true);
  assert.equal(released.html_sha256, released.html_manifest_sha256);
  assert.equal(released.html_sha256, released.html_binding_sha256);
  assert.equal(released.required_notice_count, 1);
  assert.equal(released.user_release_notice_count, 1);
  assert.equal(released.user_release_notice_model_sha256, "a".repeat(64));
  assert.equal(released.local_private_notice_count, 0);
  assert.equal(released.result_contract_id, "openpipestress.result_semantics/0.3.0/preview-physics-2");
  assert.equal(released.package_identity_sha256, sha256Hex(readStoredZip(packageBytes())[0].bytes));
  const local = inspectReportPackage(packageBytes({ userRelease: false, notices: ["required", "local"] }), NOTICE);
  assert.equal(local.user_release, null);
  assert.equal(local.content_binding_matches, null);
  assert.equal(local.user_release_notice_count, 0);
  assert.equal(local.local_private_notice_count, 1);
  assert.equal(inspectReportPackage(packageBytes({ notices: ["user"] }), NOTICE).required_notice_count, 0);
  assert.equal(inspectReportPackage(packageBytes({ bindingSha: "0".repeat(64) }), NOTICE).content_binding_matches, false);
  assert.throws(() => inspectReportPackage(storedZip([["other.json", "{}"]]), NOTICE), /first member/);
});

test("a CSV table file is read in the product's layout", () => {
  const text = "﻿notice,\"a, quoted \"\"notice\"\"\"\r\nmodel_hash,sha256:ab\r\n\r\nId,Label,UX [mm]\r\nr1,\"two\r\nlines\",1.5\r\nr2,plain,-2\r\n";
  const csv = parseCsvFile(text);
  assert.equal(csv.bom, true);
  assert.equal(csv.crlf_only, true);
  assert.deepEqual(csv.preamble, [["notice", 'a, quoted "notice"'], ["model_hash", "sha256:ab"]]);
  assert.deepEqual(csv.header, ["Id", "Label", "UX [mm]"]);
  assert.deepEqual(csv.rows, [["r1", "two\r\nlines", "1.5"], ["r2", "plain", "-2"]]);
  const bare = parseCsvFile("a,b\nc,d\n");
  assert.equal(bare.bom, false);
  assert.equal(bare.crlf_only, false);
  assert.deepEqual(bare.header, ["a", "b"]);
  assert.deepEqual(bare.rows, [["c", "d"]]);
});

test("a package status line's fields", () => {
  assert.deepEqual(packageStatusFields("outcome=saved; code=REPORT-PACKAGE-SAVED; file=a.opsproj bytes=12; container_sha256=ab"),
    { outcome: "saved", code: "REPORT-PACKAGE-SAVED", file: "a.opsproj bytes=12", container_sha256: "ab" });
});

test("an autosave record is judged from its name and bytes", () => {
  const document = { project: { id: "project:x" }, a: 1 };
  const bytes = Buffer.from(canonical(document), "utf8");
  const hash = sha256Hex(bytes);
  const name = `${autosaveSlot("project:x")}-1700000000000-${hash}.json`;
  assert.deepEqual(parseAutosaveName(name), { slot: autosaveSlot("project:x"), written_unix_millis: 1700000000000, model_hash: hash });
  for (const other of [".swbpipe-autosave-1.json", `${autosaveSlot("p")}-170000000000-${hash}.json`, `${autosaveSlot("p")}-1700000000000-${hash}.json.tmp`]) assert.equal(parseAutosaveName(other), null);
  const judged = judgeAutosaveRecord({ name, bytes, projectId: "project:x", endedAt: 1700000030000, intervalMs: 60000, notBefore: 1699999990000 });
  assert.deepEqual(Object.entries(judged).filter(([key, value]) => typeof value === "boolean" && !value), []);
  assert.equal(judged.age_at_end_ms, 30000);
  assert.equal(judgeAutosaveRecord({ name, bytes, projectId: "project:y", endedAt: 1700000030000, intervalMs: 60000, notBefore: 0 }).slot_is_project, false);
  assert.equal(judgeAutosaveRecord({ name, bytes, projectId: "project:x", endedAt: 1700000060001, intervalMs: 60000, notBefore: 0 }).within_one_interval, false);
  assert.equal(judgeAutosaveRecord({ name, bytes: Buffer.from(`${canonical(document)}\n`), projectId: "project:x", endedAt: 1700000000000, intervalMs: 60000, notBefore: 0 }).bytes_hash_is_name_hash, false);
});

test("a store's logical dump ignores layout and sees a changed row", () => {
  const dir = mkdtempSync(path.join(os.tmpdir(), "evidence-checks-"));
  try {
    const make = (file, rows) => {
      const db = new DatabaseSync(path.join(dir, file));
      db.exec("CREATE TABLE t (id TEXT PRIMARY KEY, body BLOB, n INTEGER)");
      const insert = db.prepare("INSERT INTO t VALUES (?, ?, ?)");
      for (const row of rows) insert.run(...row);
      const dump = dumpStore(db);
      db.close();
      return JSON.stringify(dump);
    };
    const a = make("a.sqlite3", [["x", Buffer.from([1, 2]), 1], ["y", null, 2]]);
    const b = make("b.sqlite3", [["y", null, 2], ["x", Buffer.from([1, 2]), 1]]);
    const c = make("c.sqlite3", [["x", Buffer.from([1, 3]), 1], ["y", null, 2]]);
    assert.equal(a, b);
    assert.notEqual(a, c);
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
});
