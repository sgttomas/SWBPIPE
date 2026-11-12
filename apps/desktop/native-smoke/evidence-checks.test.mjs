// The evidence journeys' pure checks, without the app or a build.
//
//   node --test apps/desktop/native-smoke/evidence-checks.test.mjs
import { test } from "node:test";
import assert from "node:assert/strict";
import { existsSync, mkdtempSync, readFileSync, rmSync } from "node:fs";
import os from "node:os";
import path from "node:path";
import { DatabaseSync } from "node:sqlite";
import {
  autosaveIntervalTerms, autosaveSlot, canonical, crc32, documentSha256, driverTimeout, dumpStore, inspectReportPackage, isCanonicalText,
  judgeAutosaveRecord, launchedPid, packageStatusFields, parseAutosaveName, parseCsvFile, prdRequiredNotice, readStoredZip,
  screenLockedFromIoreg, sha256Hex, timeoutText, underHostSlot
} from "./evidence-checks.mjs";

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

test("the session-lock guard reads ioreg's property", () => {
  const plist = (value) => `<plist><dict><key>IOConsoleUsers</key><array><dict><key>CGSSessionScreenIsLocked</key>\n\t\t<${value}/></dict></array></dict></plist>`;
  assert.equal(screenLockedFromIoreg(plist("true")), true);
  assert.equal(screenLockedFromIoreg(plist("false")), false);
  assert.equal(screenLockedFromIoreg("<plist><dict><key>IOConsoleUsers</key></dict></plist>"), false);
});

test("the runner knows whether it runs under the host slot", () => {
  const ps = [
    "  1     0 /sbin/launchd",
    " 50     1 /bin/zsh",
    " 60    50 /bin/bash /x/tools/t3_slot.sh node runner.mjs",
    " 61    60 /usr/bin/lockf -s -k -t 0 /x/guard/cargo_job.slot2.lock /x/tools/t3_slot_run.sh /x/guard/run.60 cargo_job.slot2.lock node runner.mjs",
    " 62    61 /bin/bash /x/tools/t3_slot_run.sh /x/guard/run.60 cargo_job.slot2.lock node runner.mjs",
    " 63    62 node runner.mjs",
    " 70    50 node runner.mjs",
    " 80    50 node t3_slot_run.sh.bak"
  ].join("\n");
  assert.equal(underHostSlot(ps, 63), true);
  assert.equal(underHostSlot(ps, 70), false);
  assert.equal(underHostSlot(ps, 999), false);
});

test("a driver timeout is told apart, with the load average nearest its moment", () => {
  const samples = [{ at: 1700000000000, load: [10, 20, 30] }, { at: 1700000002000, load: [41.5, 38, 35] }];
  const seen = driverTimeout("x\nTIMEOUT waiting for exact solve: 900123 ms elapsed, limit 900000 ms, at 1700000001900\n  at stack", samples);
  assert.deepEqual(seen, { label: "exact solve", elapsed_ms: 900123, limit_ms: 900000, at: 1700000001900, load: [41.5, 38, 35], load_sampled_at: 1700000002000 });
  assert.match(timeoutText(seen), /^TIMEOUT waiting for exact solve: 900 s elapsed of a 900 s limit; load average 41\.50 \/ 38\.00 \/ 35\.00 \(1\/5\/15 min, sampled 0 s from the timeout\), on \d+ processors$/);
  assert.equal(driverTimeout("exact: state=failed", samples), null);
  assert.equal(driverTimeout("TIMEOUT waiting for a: 1 ms elapsed, limit 0 ms, at 1700000000000", []).load, null);
});

test("PRD §19.3's notice and DEL-02-05's interval terms are read from the repository's text", () => {
  const notice = prdRequiredNotice(readFileSync(path.join(project, "docs/PRD.md"), "utf8"));
  assert.match(notice, /^SWBPIPE is decision-support software for piping design, flexibility, and stress-analysis workflows\./);
  assert.match(notice, /remain the responsibility of the user and project authority\.$/);
  assert.throws(() => prdRequiredNotice("# PRD\n"), /no §19\.3/);
  // The hosted desktop job's sparse checkout omits execution/; the runner itself always reads the file.
  const scope = path.join(project, "execution/PKG-02/DEL-02-05/ScopeOfWork.md");
  if (existsSync(scope)) assert.deepEqual(autosaveIntervalTerms(readFileSync(scope, "utf8")), { default_minutes: 5, choices: [1, 2, 5, 10, 15, 30], shortest_minutes: 1 });
  assert.deepEqual(autosaveIntervalTerms("- *Autosave interval.* x: 10 minutes by default, with choices of 3, 10 and 20, and off."), { default_minutes: 10, choices: [3, 10, 20], shortest_minutes: 3 });
  assert.throws(() => autosaveIntervalTerms("nothing"), /defines no autosave interval/);
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

test("the process to end is the private launch copy alone", () => {
  const exe = "/tmp/target/native-smoke/run-1/openpipestress-desktop";
  const ps = ` 101 /usr/bin/caffeinate -di ${exe}\n 102 ${exe}\n 103 ${exe}-other\n`;
  assert.equal(launchedPid(ps, exe), 102);
  assert.equal(launchedPid(` 101 /usr/bin/caffeinate -di ${exe}\n`, exe), null);
  assert.equal(launchedPid(` 102 ${exe}\n 104 ${exe} --x\n`, exe), null);
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
