/** A press of a sample proposal button ("Generate review proposal" in the proposal panel, the Agent
 * tab's "Propose"). Both run the session's one proposal action, which reads a native result's full
 * copy in the webview before it builds the proposal. Every press ends in one line at the button
 * pressed: the copy's size refusal in the result gates' own sentence, another failure, a result that
 * changed while the press worked, or what the press made (a proposal, a replacement, or the same
 * proposal as the one already shown). While a press works, both buttons refuse another. */
import { useRef, useState } from "react";
import { describeFullResultRefused, FullResultRefused } from "../../services/nativeResult";
import { nativeFailureText } from "../../services/nativeIngress";
import { ResourceUnavailableError } from "../../services/memoryPool";
import type { AgentProposal } from "../../types";

/** The proposal's name in the result gates' refusal ("… so the review proposal cannot read it …"). */
export const PROPOSAL_FEATURE = "the review proposal";
export const PROPOSAL_MADE = "Review proposal made from this result.";
export const PROPOSAL_REPLACED = "Review proposal made from this result; it replaces the one shown before.";
export const PROPOSAL_UNCHANGED = "Unchanged: the proposal made is the same as the one shown.";
export const PROPOSAL_SUPERSEDED = "The result changed while the proposal was being made, so it was not made from this result. Press again to make one from this result.";
export const PROPOSAL_BUSY_TITLE = "A review proposal is being made.";

export type ProposalPressSource = "panel" | "workbench";

type PressState =
  | { status: "idle" }
  | { status: "working"; by: ProposalPressSource }
  /** `after` is the proposal shown when the outcome was first rendered; the line stays while it is. */
  | { status: "done"; by: ProposalPressSource; before: AgentProposal | null; after?: AgentProposal | null }
  | { status: "superseded"; by: ProposalPressSource; of: unknown }
  | { status: "refused" | "failed"; by: ProposalPressSource; text: string; of: unknown };

export type ProposalPressLine = { tone: "status" | "refused" | "failed"; text: string };
export type ProposalPress = {
  working: boolean;
  /** Whether this button's press is the one working. */
  workingAt: (by: ProposalPressSource) => boolean;
  /** The line at this button, or null. */
  lineAt: (by: ProposalPressSource) => ProposalPressLine | null;
  press: (by: ProposalPressSource) => void;
};

export function useProposalPress({
  generate,
  proposal,
  result
}: {
  /** The session's proposal action (sets the proposal it makes). */
  generate: () => Promise<void>;
  proposal: AgentProposal | null;
  /** The session's result, so a press that outlives it says so. */
  result: unknown;
}): ProposalPress {
  const [state, setState] = useState<PressState>({ status: "idle" });
  const inFlight = useRef(false);
  const latest = useRef({ proposal, result, generate });
  latest.current = { proposal, result, generate };

  // The outcome describes the proposal shown when it was reached; the proposal set by the press is
  // published with or before the outcome, so the first render of the outcome shows it.
  if (state.status === "done" && state.after === undefined) setState({ ...state, after: proposal });

  const press = (by: ProposalPressSource) => {
    if (inFlight.current) return;
    inFlight.current = true;
    const before = latest.current.proposal;
    const basis = latest.current.result;
    setState({ status: "working", by });
    void (async () => {
      try {
        await latest.current.generate();
        setState(latest.current.result !== basis
          ? { status: "superseded", by, of: latest.current.result }
          : { status: "done", by, before });
      } catch (error) {
        setState(error instanceof FullResultRefused
          ? { status: "refused", by, text: describeFullResultRefused(error, PROPOSAL_FEATURE), of: latest.current.result }
          : { status: "failed", by, text: `The review proposal could not be made: ${failureWords(error)}`, of: latest.current.result });
      } finally {
        inFlight.current = false;
      }
    })();
  };

  const line = ((): ProposalPressLine | null => {
    switch (state.status) {
      case "done": {
        const after = state.after === undefined ? proposal : state.after;
        if (after !== proposal || after === null) return null;
        if (state.before === null) return { tone: "status", text: PROPOSAL_MADE };
        return { tone: "status", text: sameProposal(state.before, after) ? PROPOSAL_UNCHANGED : PROPOSAL_REPLACED };
      }
      case "superseded":
        return state.of === result ? { tone: "status", text: PROPOSAL_SUPERSEDED } : null;
      case "refused":
      case "failed":
        return state.of === result ? { tone: state.status, text: state.text } : null;
      default:
        return null;
    }
  })();

  const working = state.status === "working";
  return {
    working,
    workingAt: (by) => working && state.by === by,
    lineAt: (by) => (state.status !== "idle" && state.status !== "working" && state.by === by ? line : null),
    press
  };
}

/** The line beside a proposal button: a refusal or outcome as a status, another failure as an alert. */
export function ProposalPressStatus({ line, testId }: { line: ProposalPressLine | null; testId: string }) {
  if (!line) return null;
  if (line.tone === "failed") return <p role="alert" className="proposal-press-line" data-testid={testId}>{line.text}</p>;
  return <p role="status" className={line.tone === "refused" ? "proposal-press-line result-refusal" : "proposal-press-line"} data-testid={testId}>{line.text}</p>;
}

function sameProposal(left: AgentProposal, right: AgentProposal): boolean {
  return left === right || JSON.stringify(left) === JSON.stringify(right);
}

/** The failure's own words: its message, without the program's error name. */
function failureWords(error: unknown): string {
  if (error instanceof ResourceUnavailableError) return nativeFailureText(error);
  if (error instanceof Error) return error.message;
  if (error && typeof error === "object" && typeof (error as { message?: unknown }).message === "string") return (error as { message: string }).message;
  return String(error);
}
