// The Loads page's Hydrostatic test section for the selected case (DEL-05-01 REQ-014, on the
// DEL-07-06 presentation). One switch; when on, the held supports and the test contents, both
// chosen by the person (nothing is defaulted). The test pressure is the case's own pressure
// regions. The operations are lane 2's: `set_load_case_test_condition` /
// `clear_load_case_test_condition` and the case's `condition_use` of `{contents_density}`
// alone; the engine checks them before they are queued and its findings are shown here by code.
// Turning the test off on a case with test contents queues the two clears in order, the
// contents first, as one action.
import { useState } from "react";
import { ListPlus } from "lucide-react";
import type { EditorOperationIntent, PreviewModel } from "../../types";
import type { EngineCheckedQueueProps } from "../condition-sets/engineCheckedQueue";
import { HELD_SUPPORTS, conditionUseIntent, contentsKeys, storedTestCondition, testConditionIntent, type HeldSupports } from "../condition-sets/conditionSetIntents";
import { disabledBy, useReseed, type Canonicalize } from "../model-data/modelDataIntents";
import { SectionFindings, useCheckedSectionQueue } from "./caseSectionQueue";
import { plural } from "./LoadsPageParts";

type LoadCase = PreviewModel["load_cases"][number];

export const HELD_SUPPORTS_LABEL: Record<HeldSupports, string> = {
  springs_and_constant_effort: "Springs and constant-effort supports held",
  none: "None held"
};

/** The findings a person meets for a hydrostatic test, in plain words; the engine's code and
 * message follow each. */
export const HYDROSTATIC_FINDINGS: Readonly<Record<string, string>> = {
  LOAD_CASE_TEST_CONTENTS_UNSELECTED: "test contents not chosen",
  LOAD_CASE_TEST_CONTENTS_SELF_WEIGHT_INVALID: "no self-weight in this case for the test contents",
  LOAD_CASE_TEST_HELD_SUPPORT_DUPLICATE: "two held supports act on one direction at one node",
  LOAD_CASE_TEST_HELD_SUPPORT_RESTRAINED: "a held support is already restrained by another support",
  LOAD_CASE_TEST_HELD_SUPPORT_UNFORMED: "a held support cannot be held",
  LOAD_CASE_TEST_HELD_SUPPORT_MOTION: "a held support has a prescribed movement",
  COMBINATION_HYDROSTATIC_TEST_WITHHELD: "a combination uses this test case",
  LOAD_CASE_TEST_CONDITION_PRESSURE_FREE: "the model has no pressure contract",
  LOAD_CASE_CONDITION_USE_EXACT_UNSUPPORTED: "clear the test contents first",
  EXACT_PRESSURE_REGIONS_REQUIRED: "give this case its pressure regions first, in its Pressure section"
};

type StoredTest = { held: string; contents: string };
function storedOf(loadCase: LoadCase): StoredTest & { on: boolean } {
  const stored = storedTestCondition(loadCase);
  const record = stored && typeof stored === "object" && !Array.isArray(stored) ? stored as Record<string, unknown> : null;
  const contents = loadCase.condition_use?.contents_density;
  return {
    on: stored !== undefined,
    held: typeof record?.held_supports === "string" ? record.held_supports : "",
    contents: typeof contents === "string" ? contents : ""
  };
}

/** The section's closed line: "off", or what the test holds. */
export function hydrostaticSummary(loadCase: LoadCase): string {
  const stored = storedOf(loadCase);
  if (!stored.on) return "off";
  const held = stored.held === "springs_and_constant_effort" ? "springs held" : stored.held === "none" ? "no supports held" : "held supports not chosen";
  return `${held}, ${stored.contents ? `contents ${stored.contents}` : "contents not chosen"}`;
}

export function HydrostaticTestSection(props: EngineCheckedQueueProps & { loadCase: LoadCase }) {
  const { model, loadCase } = props;
  const stored = storedOf(loadCase);
  const [draft, setDraft] = useState(stored);
  useReseed([loadCase.id, stored], () => setDraft(storedOf(loadCase)));
  const queue = useCheckedSectionQueue(props);
  const keys = contentsKeys(model);
  // A case holds its pressure regions before it is made a test (the engine refuses the other
  // order with EXACT_PRESSURE_REGIONS_REQUIRED); the line below says so before the person meets it.
  const regionsGiven = Array.isArray(loadCase.pressure_regions);
  const regions = loadCase.pressure_regions?.length ?? 0;
  const unchanged = draft.on === stored.on && (!draft.on || (draft.held === stored.held && draft.contents === stored.contents));

  async function intents(canonicalize: Canonicalize): Promise<EditorOperationIntent[]> {
    if (!draft.on) {
      // Off: the contents selection goes first; the engine refuses the test's clear while it stands.
      const out: EditorOperationIntent[] = [];
      if (stored.contents) out.push(await conditionUseIntent(canonicalize, loadCase, null));
      if (stored.on) out.push(await testConditionIntent(canonicalize, loadCase, null));
      return out;
    }
    const out: EditorOperationIntent[] = [];
    if (!stored.on || draft.held !== stored.held) {
      out.push(await testConditionIntent(canonicalize, loadCase, { kind: "hydrostatic", held_supports: draft.held as HeldSupports }));
    }
    if (draft.contents !== stored.contents) {
      out.push(await conditionUseIntent(canonicalize, loadCase, draft.contents ? { contents_density: draft.contents } : null));
    }
    return out;
  }

  return (
    <div className="loads-hydrotest" data-testid="load-case-hydrotest">
      <label className="loads-switch">
        <input type="checkbox" data-testid="load-case-hydrotest-switch" checked={draft.on} disabled={queue.busy}
          onChange={(event) => setDraft({ ...draft, on: event.target.checked })} />
        <span>Hydrostatic test</span>
      </label>
      {draft.on ? (
        <>
          <div className="loads-fields">
            <label>
              <span>Held supports</span>
              <select aria-label="Held supports" data-testid="load-case-hydrotest-held" required value={draft.held}
                onChange={(event) => setDraft({ ...draft, held: event.target.value })}>
                <option value="">Choose…</option>
                {draft.held && !(HELD_SUPPORTS as readonly string[]).includes(draft.held) ? <option value={draft.held}>{draft.held} (as stored)</option> : null}
                {HELD_SUPPORTS.map((choice) => <option key={choice} value={choice}>{HELD_SUPPORTS_LABEL[choice]}</option>)}
              </select>
            </label>
            <label>
              <span>Test contents</span>
              <select aria-label="Test contents" data-testid="load-case-hydrotest-contents" required value={draft.contents}
                onChange={(event) => setDraft({ ...draft, contents: event.target.value })}>
                <option value="">{keys.length ? "Choose…" : "No condition states a contents density"}</option>
                {draft.contents && !keys.some((item) => item.key === draft.contents) ? <option value={draft.contents}>{draft.contents} (in no condition set)</option> : null}
                {keys.map((item) => <option key={item.key} value={item.key}>{contentsLabel(item)}</option>)}
              </select>
            </label>
          </div>
          <dl className="loads-values">
            <dt>Test pressure</dt>
            <dd data-testid="load-case-hydrotest-pressure">{regions ? `${plural(regions, "pressure region", "pressure regions")} of this case` : regionsGiven ? "no pressure region in this case" : "not given yet: give this case its pressure regions first, in its Pressure section"}</dd>
          </dl>
          <p className="loads-status" data-testid="load-case-hydrotest-self-weight">Self-weight is taken with the selected test contents at solve.</p>
        </>
      ) : null}
      <div className="loads-form-actions">
        <button type="button" className="loads-primary-button" data-testid="load-case-hydrotest-queue"
          {...disabledBy(queue.busy, [unchanged, "Nothing to queue: the case already holds this."], [draft.on && !draft.held, "Choose the held supports."])}
          onClick={() => void queue.submit("Hydrostatic test", intents)}>
          <ListPlus size={14} aria-hidden="true" />
          Queue hydrostatic test
        </button>
      </div>
      <SectionFindings error={queue.error} notice={queue.notice} diagnostics={queue.diagnostics} testId="load-case-hydrotest" plain={HYDROSTATIC_FINDINGS} />
    </div>
  );
}

function contentsLabel(item: ReturnType<typeof contentsKeys>[number]): string {
  const distinct = [...new Set(item.densities.map((density) => `${density.value} ${density.unit}`))];
  return distinct.length === 1 ? `${item.key} (${distinct[0]})` : `${item.key} (${distinct.length} densities by line)`;
}
