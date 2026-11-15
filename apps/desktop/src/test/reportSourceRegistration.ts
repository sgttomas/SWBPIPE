// DEL-08-01 REQ-022: a unit simulation of the desktop process's report source registration
// (`src-tauri/src/rendered_report_routes.rs`: `activate_report_source` starts an incarnation
// at number 0; `advance_report_source` registers the highest number of the current
// incarnation and refuses another incarnation by name). For App tests whose IPC is mocked;
// the registration's own rules are tested in Rust and in `reportSourceSequence.test.ts`.
export const REPORT_SOURCE_COMMANDS = ["activate_report_source", "advance_report_source"];

export function reportSourceRegistration() {
  let incarnation = 0;
  let registered = 0;
  return (command: string, args?: unknown): Promise<{ incarnation: number; sequence: number }> => {
    if (command === "activate_report_source") {
      incarnation += 1;
      registered = 0;
      return Promise.resolve({ incarnation, sequence: 0 });
    }
    const mark = args as { incarnation: number; sequence: number };
    if (mark.incarnation !== incarnation) {
      return Promise.reject({ code: "REPORT-SOURCE-INCARNATION-RETIRED", stage: "source", message: "The source change belongs to a workspace incarnation that is no longer current." });
    }
    registered = Math.max(registered, mark.sequence);
    return Promise.resolve({ incarnation, sequence: registered });
  };
}
