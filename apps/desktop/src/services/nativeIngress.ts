// Raw-body ingress for the model-sized native commands (DEL-04-07
// memory-budget decision). Tauri would parse a JSON invoke body into native
// values before a command runs; these commands instead receive the UTF-8 JSON
// bytes of their argument object, so the native host can reserve memory for
// the request before it parses it. The argument object, its camelCase keys and
// its JSON text are exactly what Tauri itself sent (JSON.stringify semantics:
// undefined members are left out and -0 is written as 0). Responses are
// unchanged.
import { invoke } from "@tauri-apps/api/core";
import { describeResourceRefusal, nativeInvokeError, ResourceUnavailableError } from "./memoryPool";

/** The header that marks a raw-body invocation and names its body format. */
export const NATIVE_INGRESS_HEADER = "x-swbpipe-ingress";
export const NATIVE_INGRESS_FORMAT = "args-json-v1";

/** The commands the native host takes as a raw body. */
export const NATIVE_INGRESS_COMMANDS = [
  "render_calculation_report",
  "sample_agent_proposal",
  "validate_model_operation",
  "apply_model_operation",
  "generate_self_weight_operation_plan",
  "validate_model_operation_batch",
  "apply_model_operation_batch",
  "convert_display_quantities",
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
  "save_handoff_package"
] as const;
export type NativeIngressCommand = (typeof NATIVE_INGRESS_COMMANDS)[number];

/** How long a command refused at admission only because this Mac's free
 * memory queue was short (`headroom_insufficient`) keeps being re-sent, and
 * the pause between sends. On a loaded Mac that queue swings between about
 * zero and gigabytes within a second at normal pressure. The native host
 * refuses before it parses the arguments, so nothing has run; each re-send
 * is judged afresh by the unchanged native rule (DEL-04-07 memory-budget
 * decision). */
export const HEADROOM_RESEND_WINDOW_MS = 2000;
export const HEADROOM_RESEND_INTERVAL_MS = 100;

/** A request its caller retired before a send (see `invokeWithIngress`):
 * nothing was sent for that attempt. */
export class IngressRequestRetiredError extends Error {
  constructor(readonly command: NativeIngressCommand) {
    super(`INGRESS-REQUEST-RETIRED: ${command} is no longer current; it was not sent.`);
    this.name = "IngressRequestRetiredError";
  }
}

/** Invokes a raw-body command with the UTF-8 JSON of `args`. A typed native
 * rejection becomes a `ResourceUnavailableError` (or an `INVALID_REQUEST`
 * error); a plain string rejection is the command's own failure and passes
 * through unchanged, as does any other rejection. A `headroom_insufficient`
 * refusal is re-sent within `HEADROOM_RESEND_WINDOW_MS`; the last refusal
 * stands after it. When `stillCurrent` is given it is checked immediately
 * before every send, the first and each re-send; a retired request is never
 * sent again and rejects with `IngressRequestRetiredError`. */
export async function invokeWithIngress<T>(
  command: NativeIngressCommand,
  args: Record<string, unknown>,
  options: { stillCurrent?: () => boolean } = {}
): Promise<T> {
  const body = new TextEncoder().encode(JSON.stringify(args));
  const deadline = performance.now() + HEADROOM_RESEND_WINDOW_MS;
  for (;;) {
    let refusal: ResourceUnavailableError;
    if (options.stillCurrent && !options.stillCurrent()) throw new IngressRequestRetiredError(command);
    try {
      return await invoke<T>(command, body, { headers: { [NATIVE_INGRESS_HEADER]: NATIVE_INGRESS_FORMAT } });
    } catch (error) {
      const read = nativeInvokeError(error);
      if (!(read instanceof ResourceUnavailableError) || read.refusal.kind !== "headroom_insufficient") throw read;
      refusal = read;
    }
    if (performance.now() + HEADROOM_RESEND_INTERVAL_MS > deadline) throw refusal;
    await new Promise((resolve) => setTimeout(resolve, HEADROOM_RESEND_INTERVAL_MS));
    // A late timer must not start a send after the window: the last refusal stands.
    if (performance.now() > deadline) throw refusal;
  }
}

/** The text a feature shows for a failed native request: a resource refusal
 * reads as the plain-language refusal sentence, anything else as before. */
export function nativeFailureText(error: unknown): string {
  if (error instanceof ResourceUnavailableError) return `Refused by the native memory pool: ${describeResourceRefusal(error.refusal)}`;
  return String(error);
}
