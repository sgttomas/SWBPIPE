// Test support for the raw-body ingress commands (DEL-04-07 memory-budget
// decision). It states the wire contract independently of the production
// helper: each listed command is invoked with the UTF-8 JSON bytes of its
// argument object and the ingress header, never with a JSON argument object.
import { expect } from "vitest";

export const INGRESS_HEADER = "x-swbpipe-ingress";
export const INGRESS_FORMAT = "args-json-v1";
export const INGRESS_COMMANDS: ReadonlySet<string> = new Set([
  "embedded_agent_host",
  "render_calculation_report",
  "sample_agent_proposal",
  "validate_model_operation",
  "apply_model_operation",
  "generate_self_weight_operation_plan",
  "validate_model_operation_batch",
  "apply_model_operation_batch",
  "convert_display_quantities",
  "check_preview_model",
  "create_local_project",
  "save_local_project",
  "validate_rule_pack",
  "run_rule_checks",
  "compute_rule_pack_document_checksum",
  "save_local_rule_pack",
  "validate_library_import",
  "save_local_library",
  "save_report_package",
  "save_local_result_json",
  "begin_local_csv_stream",
  "append_local_csv_stream",
  "finish_local_csv_stream",
  "abort_local_csv_stream",
  "save_model_state",
  "list_model_states",
  "read_model_state",
  "edit_model_state_labels",
  "delete_model_state",
  "admit_model_state_comparison",
  "read_model_state_for_comparison",
  "record_analysis_run",
  "list_analysis_runs",
  "read_analysis_run",
  "edit_analysis_run_labels",
  "delete_analysis_run",
  "admit_analysis_run_comparison",
  "save_model_document_file",
  "save_stored_model_document_file",
  "save_handoff_package",
  "write_project_autosave",
  "list_project_autosaves",
  "read_project_autosave",
  "discard_project_autosave",
  "get_autosave_settings",
  "set_autosave_settings",
  "read_preview_mechanics_job_verdict",
  "read_preview_mechanics_job_summary",
  "query_preview_mechanics_job_rows",
  "lookup_preview_mechanics_job_rows",
  "read_preview_mechanics_job_row_semantics",
  "query_preview_mechanics_job_source_recovery",
  "query_preview_mechanics_job_navigator",
  "read_preview_mechanics_job_analysis_run_preview"
]);

type InvokeOptions = { headers?: Record<string, string> } | undefined;

/** The argument object carried by one ingress invocation, after asserting its
 * raw body and header. Throws (failing the test) for any other call shape. */
export function decodeIngressBody(command: string, body: unknown, options: InvokeOptions): Record<string, unknown> {
  if (!(body instanceof Uint8Array) && !ArrayBuffer.isView(body)) {
    throw new Error(`INGRESS_TEST: ${command} was invoked without a raw body`);
  }
  if (options?.headers?.[INGRESS_HEADER] !== INGRESS_FORMAT) {
    throw new Error(`INGRESS_TEST: ${command} was invoked without the ${INGRESS_HEADER}: ${INGRESS_FORMAT} header`);
  }
  const view = body as ArrayBufferView;
  const text = new TextDecoder("utf-8", { fatal: true }).decode(new Uint8Array(view.buffer, view.byteOffset, view.byteLength));
  const args = JSON.parse(text) as unknown;
  if (!args || typeof args !== "object" || Array.isArray(args)) throw new Error(`INGRESS_TEST: ${command} body is not a JSON object`);
  return args as Record<string, unknown>;
}

/** The parsed argument object of a recorded `[command, body, options]` mock
 * call, asserting that the call is an ingress invocation. */
export function ingressArgs(call: readonly unknown[]): Record<string, unknown> {
  const [command, body, options] = call as [string, unknown, InvokeOptions];
  expect(INGRESS_COMMANDS.has(command), `${command} is an ingress command`).toBe(true);
  return decodeIngressBody(command, body, options);
}

type AnyInvoke = (command: string, args?: unknown, options?: unknown) => unknown;

/** Wraps an invoke mock so the wire contract is checked at every call and the
 * mock records what the command receives: an ingress command must arrive as a
 * raw body with the header, and the mock is then called with
 * `(command, decodedArgs)` exactly as before the commands took raw bodies; a
 * JSON-argument call to an ingress command fails. Every other command passes
 * through unchanged. */
export function ingressDecodingInvoke(mock: AnyInvoke): (...call: unknown[]) => unknown {
  return (...call: unknown[]) => {
    const [command, body, options] = call as [string, unknown, unknown];
    if (!INGRESS_COMMANDS.has(command)) return (mock as (...args: unknown[]) => unknown)(...call);
    let args: Record<string, unknown>;
    try { args = decodeIngressBody(command, body, options as InvokeOptions); }
    catch (error) { return Promise.reject(error); }
    return mock(command, args);
  };
}
