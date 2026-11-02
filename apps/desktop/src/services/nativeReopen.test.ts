/** C1 slice 6: the reopen replies' strict decoders, on the actual commands' committed replies
 * (`native_reopen_replies.json`): the parsed JSON body is accepted; text, bytes, a whole run, a
 * row-scale header member, a foreign handle or origin, and an unknown state are refused by name. */
import { describe, expect, it } from "vitest";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { decodeNativeProjectOpen, decodeNativeRunRead, NATIVE_REOPEN_MALFORMED, restoredRecordDiagnostics } from "./nativeReopen";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const document = JSON.parse(readFileSync(resolve(__dirname, "../test-support/native-reopen/native_reopen_replies.json"), "utf8")) as Json;
const handle = (value: unknown) => JSON.parse(JSON.stringify(value).replaceAll("$HANDLE", "restored-stored-result-7"));
const open = () => handle(document.open.reply.value);
const read = () => handle(document.read.reply.value);
const refused = (decode: () => unknown) => expect(decode).toThrow(NATIVE_REOPEN_MALFORMED);

describe("the reopen replies' decoders", () => {
  it("accept the commands' parsed replies", () => {
    const opened = decodeNativeProjectOpen(open())!;
    expect(opened.stored_run.state).toBe("restored");
    expect(decodeNativeProjectOpen(null)).toBeNull();
    expect(decodeNativeProjectOpen(handle(document.open_refused.reply.value))!.stored_run.state).toBe("refused");
    const run = decodeNativeRunRead(read());
    expect(run.restored.origin).toBe("restored_recorded_run");
    expect(restoredRecordDiagnostics(run.restored).analysis_run.diagnostics).toEqual(run.restored.record_header.rule_pack_ref_diagnostics);
  });

  it("refuse every other delivery and shape by name", () => {
    refused(() => decodeNativeProjectOpen(JSON.stringify(open())));
    refused(() => decodeNativeProjectOpen(new TextEncoder().encode(JSON.stringify(open())).buffer));
    refused(() => decodeNativeProjectOpen({ ...open(), mechanics_result: {} }));
    refused(() => decodeNativeProjectOpen({ ...open(), stored_run: { state: "withheld" } }));
    refused(() => decodeNativeProjectOpen({ ...open(), stored_run: { ...open().stored_run, handle: "backend-solve-job-1" } }));
    refused(() => decodeNativeProjectOpen({ ...open(), stored_run: { ...open().stored_run, origin: "fresh_job" } }));
    const leaking = open();
    leaking.stored_run.record_header.record.analysis_run.result_refs = [];
    refused(() => decodeNativeProjectOpen(leaking));
    refused(() => decodeNativeRunRead({ ...read(), result_json: "{}" }));
    refused(() => decodeNativeRunRead({ ...read(), restored: { ...read().restored, origin: "restored_saved_run" } }));
    refused(() => decodeNativeRunRead(JSON.stringify(read())));
  });
});
