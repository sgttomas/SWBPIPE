// The evidence journeys' pure checks, without the app or a build.
//
//   node --test apps/desktop/native-smoke/evidence-checks.test.mjs
import { test } from "node:test";
import assert from "node:assert/strict";
import { lstatSync, mkdirSync, mkdtempSync, readFileSync, realpathSync, renameSync, rmSync, symlinkSync } from "node:fs";
import os from "node:os";
import path from "node:path";
import { DatabaseSync } from "node:sqlite";
import {
  autosaveSlot, canonical, crc32, documentSha256, driverTimeout, dumpStore, inspectReportPackage, isCanonicalText,
  judgeAutosaveRecord, packageStatusFields, parseAutosaveName, parseCsvFile, prdRequiredNotice, readStoredZip,
  sha256Hex, timeoutText, homeRedirection, homeRedirectionRefusal, webviewDataEntries,
  ALL_JOURNEYS, parseRunnerArgs, homeRemovalRefusal, screenState, parentsFromPs, lsofFiles, heldSlotAncestor,
  launchGuardRefusal, LAUNCH_GUARD, systemTempRoot, removalIdentityRefusal, applicationEnvironment, applicationEnvironmentRefusal,
  APP_PATH, groupRunning, UNFINISHED_BY_MACHINE, CONFINEMENT_TERM, NOTHING_ELSE_WATCHED, LISTING_LIMITS, STRICT_CHOOSER
} from "./evidence-checks.mjs";

const STRICT = STRICT_CHOOSER;

test("finding 5: the strict chooser constants agree between the runner and the native-smoke build", () => {
  const rust = readFileSync(new URL("../src-tauri/src/native_smoke.rs", import.meta.url), "utf8");
  const constant = (name) => rust.match(new RegExp(`const ${name}: [^=]+= ([^;]+);`))?.[1]?.trim();
  assert.equal(constant("STRICT_CHOOSER_ENV"), JSON.stringify(STRICT_CHOOSER.variable));
  assert.equal(constant("STRICT_CHOOSER_VALUE"), JSON.stringify(STRICT_CHOOSER.value));
  assert.equal(Number(constant("STRICT_CHOOSER_EXIT")), STRICT_CHOOSER.exit_code);
  assert.ok(![0, 1, 2, 97, 101].includes(STRICT_CHOOSER.exit_code) && !(STRICT_CHOOSER.exit_code >= 126 && STRICT_CHOOSER.exit_code <= 165) && !(STRICT_CHOOSER.exit_code >= 64 && STRICT_CHOOSER.exit_code <= 78));
});

test("the runner's command line: --all in its order, named journeys, the separate journey alone, --build-only, and no lister path", () => {
  assert.deepEqual(ALL_JOURNEYS[0], "autosave-recovery");
  assert.deepEqual(parseRunnerArgs(["--all"]), { journeys: ALL_JOURNEYS, buildOnly: false, skipBuild: false });
  assert.deepEqual(parseRunnerArgs(["--journey", "report-exact,report-pressure-free", "--skip-build"]), { journeys: ["report-exact", "report-pressure-free"], buildOnly: false, skipBuild: true });
  assert.deepEqual(parseRunnerArgs(["--journey", "reference-system", "--skip-build"]).journeys, ["reference-system"]);
  assert.match(parseRunnerArgs(["--journey", "reference-system,report-exact"]).error, /runs alone/);
  assert.match(parseRunnerArgs(["--all", "--journey", "report-exact"]).error, /not both/);
  assert.match(parseRunnerArgs([]).error, /choose --all/);
  assert.match(parseRunnerArgs(["--journey"]).error, /needs one or more/);
  assert.match(parseRunnerArgs(["--journey", "nope"]).error, /choose --all/);
  assert.match(parseRunnerArgs(["--al"]).error, /unknown arguments: --al/);
  // An arbitrary lister path is not accepted in any form.
  assert.match(parseRunnerArgs(["--all", "--library-listing", "/x/library_listing.py"]).error, /unknown arguments: --library-listing \/x\/library_listing.py/);
  assert.match(parseRunnerArgs(["--all", "--keep"]).error, /unknown arguments: --keep/);
  assert.deepEqual(parseRunnerArgs(["--build-only"]), { journeys: [], buildOnly: true, skipBuild: false });
  assert.match(parseRunnerArgs(["--build-only", "--all"]).error, /takes no journey/);
  assert.match(parseRunnerArgs(["--build-only", "--skip-build"]).error, /takes no journey/);
});

test("the launch guard seam refuses while the committed helper is absent, and while it is not wired", () => {
  assert.equal(LAUNCH_GUARD, "tools/native_launch_guard.py");
  assert.equal(launchGuardRefusal(false), "refused: the committed launch guard is not present");
  assert.match(launchGuardRefusal(true), /^refused: the committed launch guard is present but this kit is not yet wired to it$/);
  assert.match(LISTING_LIMITS, /cannot show what was read/);
  assert.match(LISTING_LIMITS, /watches no writes outside the Library/);
  assert.match(CONFINEMENT_TERM, /a run's record states that nothing else on the disk was watched\.$/);
  assert.equal(NOTHING_ELSE_WATCHED, "Nothing else on the disk was watched.");
});

test("finding 7: the screen state needs a recognisable on-console session; anything else is not established", () => {
  const plist = (session) => `<?xml version="1.0"?><plist><dict><key>IOConsoleUsers</key><array><dict>${session}</dict></array></dict></plist>`;
  assert.equal(screenState(plist("<key>kCGSSessionOnConsoleKey</key><true/><key>kCGSSessionUserNameKey</key><string>u</string>")), "unlocked");
  assert.equal(screenState(plist("<key>kCGSSessionOnConsoleKey</key><true/><key>CGSSessionScreenIsLocked</key><true/>")), "locked");
  assert.equal(screenState(plist("<key>kCGSSessionOnConsoleKey</key><false/>")), "unknown");
  // Two sessions on the console: which one paints is not established.
  assert.equal(screenState(plist("<key>kCGSSessionOnConsoleKey</key><true/></dict><dict><key>kCGSSessionOnConsoleKey</key><true/>")), "unknown");
  assert.equal(screenState("<plist><dict><key>Other</key><string>x</string></dict></plist>"), "unknown");
  assert.equal(screenState(""), "unknown");
  assert.equal(screenState("some successful output that is not a property list"), "unknown");
});

test("finding 7: a held slot is the lockf ancestor holding a slot file, not an argument naming the wrapper", () => {
  const slot = "/Users/ryan/dev/chirality-t3/guard/cargo_job.slot2.lock";
  const parents = parentsFromPs(" 63 62\n 62 61\n 61 60\n 60 50\n 70 69\n 69 50\n");
  const lsof = {
    61: "p61\nftxt\nn/usr/bin/lockf\nf3\nn" + slot + "\n",
    62: "p62\nftxt\nn/bin/bash\nf255\nn/Users/ryan/dev/chirality-t3/tools/t3_slot_run.sh\n",
    // A process whose arguments name the wrapper and a slot file, but which is not lockf.
    69: "p69\nftxt\nn/usr/local/bin/node\nf3\nn" + slot + "\n"
  };
  const filesOf = (pid) => (lsof[pid] ? lsofFiles(lsof[pid]) : []);
  assert.deepEqual(heldSlotAncestor(parents, 63, filesOf, [slot]), { pid: 61, slot });
  assert.equal(heldSlotAncestor(parents, 70, filesOf, [slot]), null);
  // lockf holding some other file is not a held slot.
  assert.equal(heldSlotAncestor(parents, 63, (pid) => (pid === 61 ? lsofFiles("p61\nftxt\nn/usr/bin/lockf\nf3\nn/tmp/other.lock\n") : []), [slot]), null);
  assert.deepEqual(lsofFiles("p1\nftxt\nn/usr/bin/lockf\nf3\nn/a\n"), [{ fd: "txt", name: "/usr/bin/lockf" }, { fd: "3", name: "/a" }]);
});

test("finding 1: the application's environment is an allowlist built from nothing", () => {
  const inherited = {
    SWBPIPE_NATIVE_SMOKE_INJECT_RESULT: "/Users/someone/secret.json", SWBPIPE_NATIVE_SMOKE_INJECT_ROOT: "/Users/someone",
    SWBPIPE_NATIVE_SMOKE_PROVIDER_KEY: "stdin", ANTHROPIC_API_KEY: "sk-invented", OPENAI_API_KEY: "sk-invented",
    HTTPS_PROXY: "http://proxy.invalid:8080", http_proxy: "http://proxy.invalid:8080", XDG_CONFIG_HOME: "/Users/someone/.config",
    DYLD_INSERT_LIBRARIES: "/tmp/evil.dylib", DYLD_LIBRARY_PATH: "/tmp", TMPDIR: "/Users/someone/tmp", PATH: "/Users/someone/bin:/usr/bin",
    HOME: "/Users/someone", LANG: "en_GB.UTF-8", LC_ALL: "$(touch /tmp/x)"
  };
  const home = "/private/var/folders/ab/T/swbpipe-evidence-run-1/swbpipe-evidence-report-exact-2/home";
  const env = applicationEnvironment({ home, tmp: `${home}/tmp`, driver: "/d.js", plan: "{}", report: "/r.json", strict: STRICT, inherited });
  assert.deepEqual(Object.keys(env).sort(), ["CFFIXED_USER_HOME", "HOME", "LANG", "PATH", STRICT.variable, "SWBPIPE_NATIVE_SMOKE_DRIVER", "SWBPIPE_NATIVE_SMOKE_PLAN", "SWBPIPE_NATIVE_SMOKE_REPORT", "TMPDIR"].sort());
  for (const name of ["SWBPIPE_NATIVE_SMOKE_INJECT_RESULT", "SWBPIPE_NATIVE_SMOKE_INJECT_ROOT", "SWBPIPE_NATIVE_SMOKE_PROVIDER_KEY", "ANTHROPIC_API_KEY", "OPENAI_API_KEY", "HTTPS_PROXY", "http_proxy", "XDG_CONFIG_HOME", "DYLD_INSERT_LIBRARIES", "DYLD_LIBRARY_PATH", "LC_ALL"]) {
    assert.equal(env[name], undefined, name);
  }
  assert.equal(env.HOME, home);
  assert.equal(env.CFFIXED_USER_HOME, home);
  assert.equal(env.TMPDIR, `${home}/tmp`);
  assert.equal(env.PATH, APP_PATH);
  assert.equal(env[STRICT.variable], "1");
  assert.equal(applicationEnvironmentRefusal(env, STRICT), null);
  assert.match(applicationEnvironmentRefusal({ ...env, SWBPIPE_NATIVE_SMOKE_INJECT_RESULT: "/x" }, STRICT), /outside the allowlist: SWBPIPE_NATIVE_SMOKE_INJECT_RESULT/);
  assert.match(applicationEnvironmentRefusal({ ...env, TMPDIR: "/tmp" }, STRICT), /TMPDIR is not inside the home/);
  assert.match(applicationEnvironmentRefusal({ ...env, [STRICT.variable]: "0" }, STRICT), /is not set to 1/);
  assert.match(applicationEnvironmentRefusal({ ...env, PATH: "/usr/bin" }, STRICT), /fixed application PATH/);
});

test("finding 4: the temporary root is the system's answer, not TMPDIR", () => {
  const realpath = (value) => value.replace(/^\/var\//, "/private/var/");
  assert.deepEqual(systemTempRoot("/var/folders/ab/cd123/T/\n", realpath), { root: "/private/var/folders/ab/cd123/T" });
  // An inherited TMPDIR plays no part: the answer comes from getconf, and a home under the inherited folder is refused.
  const { root } = systemTempRoot("/var/folders/ab/cd123/T/", realpath);
  const inheritedTmp = "/Users/someone/tmp";
  assert.match(homeRedirectionRefusal(homeRedirection(`${inheritedTmp}/swbpipe-evidence-x/home`), root), /not under the system temporary folder/);
  assert.match(systemTempRoot("/Users/someone/tmp", realpath).refusal, /not recognisable/);
  assert.match(systemTempRoot("", realpath).refusal, /not recognisable/);
  assert.match(systemTempRoot("/var/folders/ab/cd123/T/", () => "/Users/someone/tmp").refusal, /resolves somewhere unexpected/);
});

test("finding 4: a recursive removal is refused when the home or an ancestor was swapped for a link or another directory", () => {
  const scratch = realpathSync(mkdtempSync(path.join(os.tmpdir(), "evidence-removal-")));
  try {
    const setup = () => {
      const root = path.join(scratch, `T-${Math.random().toString(36).slice(2)}`);
      const run = path.join(root, "swbpipe-evidence-run-A1");
      const journey = path.join(run, "swbpipe-evidence-report-exact-B2");
      const home = path.join(journey, "home");
      mkdirSync(home, { recursive: true });
      const recorded = new Map([root, run, journey, home].map((dir) => [dir, (({ dev, ino }) => ({ dev, ino }))(lstatSync(dir))]));
      return { root, run, journey, home, recorded };
    };
    const intact = setup();
    assert.equal(removalIdentityRefusal(intact.home, intact.root, intact.recorded, lstatSync), null);
    // A swapped intermediate: the journey folder replaced by a link to elsewhere (holding a home of its own).
    const middle = setup();
    const elsewhere = path.join(scratch, "elsewhere-1");
    mkdirSync(path.join(elsewhere, "home"), { recursive: true });
    renameSync(middle.journey, `${middle.journey}.moved`);
    symlinkSync(elsewhere, middle.journey);
    assert.match(removalIdentityRefusal(middle.home, middle.root, middle.recorded, lstatSync), /is now a link|cannot be examined|not the directory/);
    // A swapped final: the home itself replaced by a link.
    const final = setup();
    const target = path.join(scratch, "elsewhere-2");
    mkdirSync(target);
    rmSync(final.home, { recursive: true });
    symlinkSync(target, final.home);
    assert.match(removalIdentityRefusal(final.home, final.root, final.recorded, lstatSync), /home is now a link/);
    // A replaced directory of the same name is not the one recorded.
    const replaced = setup();
    rmSync(replaced.home, { recursive: true });
    mkdirSync(replaced.home);
    assert.match(removalIdentityRefusal(replaced.home, replaced.root, replaced.recorded, lstatSync), /not the directory the kit recorded/);
    // A home outside the root, and an unrecorded ancestor, are refused.
    assert.match(removalIdentityRefusal("/Users/someone/home", intact.root, intact.recorded, lstatSync), /not under the temporary root/);
    assert.match(removalIdentityRefusal(intact.home, intact.root, new Map(), lstatSync), /no identity was recorded/);
  } finally {
    rmSync(scratch, { recursive: true, force: true });
  }
});

test("finding 6: whether the started process's group still runs is read, not signalled", () => {
  const ps = " 101 101 S\n 102 101 Z\n 200 200 R\n";
  assert.equal(groupRunning(ps, 101), true);
  assert.equal(groupRunning(" 102 101 Z\n", 101), false);
  assert.equal(groupRunning(ps, 300), false);
});

test("finding 5: the three acts no machine finishes are named", () => {
  assert.deepEqual(UNFINISHED_BY_MACHINE.map((item) => item.act), ["Save canonical HTML", "Copy", "Print"]);
});

test("only a throwaway home this run created is removed", () => {
  const tmp = "/var/folders/ab/T";
  const home = `${tmp}/swbpipe-evidence-run-Ab12/swbpipe-evidence-report-exact-Xy34/home`;
  const created = new Set([home]);
  assert.equal(homeRemovalRefusal(home, created, tmp), null);
  assert.equal(homeRemovalRefusal(home, created, `${tmp}/`), null);
  assert.match(homeRemovalRefusal(`${tmp}/swbpipe-evidence-run-Ab12/swbpipe-evidence-report-exact-Xy34`, created, tmp), /not a home this run created/);
  assert.match(homeRemovalRefusal("/Users/someone", new Set(["/Users/someone"]), tmp), /not under the system temporary folder/);
  assert.match(homeRemovalRefusal(`${tmp}/other/home`, new Set([`${tmp}/other/home`]), tmp), /not a throwaway home folder/);
  assert.match(homeRemovalRefusal(`${tmp}/swbpipe-evidence-x-1/../home`, new Set([`${tmp}/swbpipe-evidence-x-1/../home`]), tmp), /without \.\./);
  assert.match(homeRemovalRefusal(`${tmp}/swbpipe-evidence-x-1/home`, new Set(), tmp), /not a home this run created/);
});

const project = path.resolve(path.dirname(new URL(import.meta.url).pathname), "..", "..", "..");

test("the launch redirects both homes to one throwaway folder under the temporary folder, or is refused by name", () => {
  const tmp = "/var/folders/ab/T";
  const home = `${tmp}/swbpipe-evidence-x/home`;
  assert.deepEqual(homeRedirection(home), { HOME: home, CFFIXED_USER_HOME: home });
  assert.equal(homeRedirectionRefusal(homeRedirection(home), tmp), null);
  assert.equal(homeRedirectionRefusal(homeRedirection(home), `${tmp}/`), null);
  assert.match(homeRedirectionRefusal({ HOME: home }, tmp), /CFFIXED_USER_HOME is not set/);
  assert.match(homeRedirectionRefusal({ CFFIXED_USER_HOME: home }, tmp), /HOME is not set/);
  assert.match(homeRedirectionRefusal({ HOME: home, CFFIXED_USER_HOME: `${home}2` }, tmp), /differ/);
  assert.match(homeRedirectionRefusal(homeRedirection("/Users/someone"), tmp), /not under the system temporary folder/);
  assert.match(homeRedirectionRefusal(homeRedirection(tmp), tmp), /not under the system temporary folder/);
  assert.match(homeRedirectionRefusal(homeRedirection(`${tmp}/../../Users/someone`), tmp), /without \.\./);
  assert.match(homeRedirectionRefusal(homeRedirection("relative/home"), tmp), /absolute/);
});

test("the web view's data is found in a listing of the throwaway home", () => {
  const name = "openpipestress-desktop";
  const listing = [
    "Library/Application Support/com.example.app/openpipestress-projects.sqlite3",
    "Library/WebKit/openpipestress-desktop/WebsiteData/LocalStorage/x.sqlite3",
    "Library/Caches/openpipestress-desktop/WebKit/NetworkCache/a",
    "Library/HTTPStorages/openpipestress-desktop.binarycookies",
    "Library/Caches/other-app/x",
    "Downloads/a.csv"
  ];
  assert.deepEqual(webviewDataEntries(listing, name), listing.slice(1, 4));
  assert.deepEqual(webviewDataEntries(["Library/HTTPStorages/openpipestress-desktop/x"], name), ["Library/HTTPStorages/openpipestress-desktop/x"]);
  assert.deepEqual(webviewDataEntries(["Library/Application Support/openpipestress-desktop/x"], name), []);
  assert.deepEqual(webviewDataEntries(["Downloads/a.csv"], name), []);
});

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
