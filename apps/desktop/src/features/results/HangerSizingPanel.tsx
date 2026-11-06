/** The hanger-sizing table and its details (DEL-04-11 increment 3b; Design `hanger-sizing.md`
 * §8.1, from the document contract of §8.3). Read-only: it renders 3a's display model
 * (`hangerSizingView`) of one result document, with the candidates and solves tables, each with
 * its CSV export (DEL-08-04, `hangerSizingCsv.ts`).
 *
 * Numbers are the published SI values with their units, exactly as 3a gives them (`text`): no
 * rounding, conversion or arithmetic here. Record details come only through 3a's library gate;
 * reader standings only as 3a binds them. Not mounted by this increment: 3c mounts it with the
 * native sizing job. It changes no model, result or session and persists nothing. */

import { useEffect, useRef, useState } from "react";
import { CsvExportControl } from "../result-export/CsvExportControl";
import { CSV_SOFTWARE_VERSION } from "../result-export/csvProvenance";
import type { HangerSizingDocument, HangerSizingSizedDocument } from "./hangerSizingResult";
import {
  HANGER_SIZING_CONSTANT_COUNT_KEYS,
  HANGER_SIZING_CSV_KIND,
  HANGER_SIZING_NO_FAILED_TESTS,
  HANGER_SIZING_NO_FIT,
  HANGER_SIZING_NO_NONLINEAR_SUPPORT,
  HANGER_SIZING_NOT_APPLICABLE_CONSTANT,
  HANGER_SIZING_NOT_PUBLISHED,
  HANGER_SIZING_NOT_TESTED,
  HANGER_SIZING_VARIABLE_COUNT_KEYS,
  hangerSizingCandidatesCsv,
  hangerSizingCsvProvenance,
  hangerSizingNotShown,
  hangerSizingSolvesCsv
} from "./hangerSizingCsv";
import {
  F_COLD_CONDITION_LABEL,
  F_COLD_LABEL,
  hangerSizingView,
  type BasisView,
  type CandidateRowView,
  type CaseView,
  type FailureCountsView,
  type HangerSizingReaderEvidence,
  type HangerSizingRefusedView,
  type HangerSizingSizedView,
  type HangerSizingView,
  type LibraryGateView,
  type NamedFindingView,
  type StandingsView,
  type VariationView
} from "./hangerSizingView";

export const HANGER_SIZING_TITLE = "Hanger sizing";
export const HANGER_SIZING_CANDIDATES_TABLE_LABEL = "Hanger sizing candidates";
export const HANGER_SIZING_SOLVES_TABLE_LABEL = "Hanger sizing derived solves";
export const HANGER_SIZING_NOT_PUBLISHED_INPUT = "not published";

/** §8.3.3 and §8.3.5: what each status and standing means. */
export const HANGER_SIZING_SIZED_TEXT =
  "Status: sized. The sizing algorithm completed. This does not mean that every candidate fits, and it is no engineering acceptance.";
export const HANGER_SIZING_REFUSED_TEXT = "Status: refused. No table was formed; every refusal is listed below, in the engine's order.";
export const HANGER_SIZING_CONDITIONAL_TEXT =
  "Verification standing: conditional. At least one candidate has no fitting record (no_record_fits). The verification solves kept each such candidate as a force equal to its W, so every verified position, load and test outcome below is conditional on that assumption. Applying the selected records alone does not reproduce the verification, because an unresolved candidate stays as it is in the model.";
export const HANGER_SIZING_UNCONDITIONAL_TEXT =
  "Verification standing: unconditional. Every candidate has a selected record, and the verification solves used each selection as its verification assumption states.";
export const HANGER_SIZING_STANDINGS_TEXT =
  "Each standing states only what that reader checked. No standing states that the mechanics are verified, and none is an acceptance of a selection.";

const STANDING_STATUS_TEXT = { passed: "passed", failed: "failed", not_checked: "not checked" } as const;

export type HangerSizingPanelProps = Readonly<{
  document: HangerSizingDocument;
  /** The readers' verdict bound to this exact document (§8.3.7); otherwise every standing is not checked. */
  evidence?: HangerSizingReaderEvidence | null;
  /** The full library document the result binds; record details are shown only when its checked hash matches. */
  library?: unknown;
  /** The model hash the caller records (`sha256:<hex>`) for the CSV preamble; `not_recorded` otherwise. */
  modelHash?: string | null;
  /** A prefix for the panel's test ids, so two embeddings stay distinct. */
  testIdPrefix?: string;
}>;

type Built = { document: HangerSizingDocument; evidence: unknown; library: unknown; view: HangerSizingView };

export function HangerSizingPanel({ document, evidence, library, modelHash, testIdPrefix = "hanger-sizing" }: HangerSizingPanelProps) {
  const [built, setBuilt] = useState<Built | null>(null);
  const [error, setError] = useState<string | null>(null);
  const token = useRef(0);
  const tid = (suffix: string) => `${testIdPrefix}-${suffix}`;

  useEffect(() => {
    const mine = ++token.current;
    setError(null);
    hangerSizingView(document, evidence, library).then(
      (view) => { if (mine === token.current) setBuilt({ document, evidence, library, view }); },
      (caught: unknown) => { if (mine === token.current) setError(caught instanceof Error ? caught.message : String(caught)); }
    );
  }, [document, evidence, library]);

  // A view is shown only for the exact inputs it was built from.
  const current = built !== null && built.document === document && built.evidence === evidence && built.library === library ? built : null;
  return (
    <section className="result-detail-panel hanger-sizing" aria-label={HANGER_SIZING_TITLE} data-testid={tid("panel")}>
      <h3>{HANGER_SIZING_TITLE}</h3>
      {error !== null ? <p role="alert" data-testid={tid("error")}>The sizing result could not be displayed: {error}</p> : null}
      {error === null && current === null ? <p role="status" data-testid={tid("pending")}>Reading the sizing result…</p> : null}
      {current?.view.status === "sized" ? (
        <SizedView view={current.view} document={current.document as HangerSizingSizedDocument} modelHash={modelHash} tid={tid} />
      ) : null}
      {current?.view.status === "refused" ? <RefusedView view={current.view} tid={tid} /> : null}
    </section>
  );
}

// ---------------------------------------------------------------------------------------------
// Shared pieces

type Tid = (suffix: string) => string;

const orNotPublished = (value: string | null) => value ?? HANGER_SIZING_NOT_PUBLISHED_INPUT;
const variationText = (value: VariationView) => (value.state === "ratio" ? value.ratio.text : value.text);
const listOrNone = (items: readonly string[]) => (items.length > 0 ? items.join(", ") : "none");

function Findings({ findings, tid, name }: { findings: readonly NamedFindingView[]; tid: Tid; name: string }) {
  return (
    <ol data-testid={tid(name)}>
      {findings.map((finding, index) => (
        <li key={`${finding.code}-${index}`} data-testid={tid(`${name}-${index}`)}>
          <code>{finding.code}</code>
          {finding.names.length > 0 ? <> ({finding.names.join(", ")})</> : null}: {finding.message}
        </li>
      ))}
    </ol>
  );
}

function BasisBlock({ basis, cases, library, tid }: { basis: BasisView; cases: readonly CaseView[] | null; library: LibraryGateView | null; tid: Tid }) {
  const request = basis.sizingRequest;
  return (
    <section aria-label="Sizing basis" data-testid={tid("basis")}>
      <h4>Basis</h4>
      <dl>
        <dt>Document</dt><dd data-testid={tid("basis-document")}>{basis.documentKind} {basis.schemaVersion} ({basis.semanticContractId})</dd>
        <dt>Route</dt><dd data-testid={tid("basis-route")}>{basis.route}</dd>
        <dt>Canonical profile</dt><dd data-testid={tid("basis-profile")}>{basis.canonicalProfile}</dd>
        <dt>Library id</dt><dd data-testid={tid("basis-library-id")}>{orNotPublished(basis.library.libraryId)}</dd>
        <dt>Library sha256</dt><dd data-testid={tid("basis-library-sha256")}>{orNotPublished(basis.library.sha256)}</dd>
        <dt>Solve request sha256 (identifies the exact model and materials)</dt>
        <dd data-testid={tid("basis-solve-request-sha256")}>{orNotPublished(basis.solveRequestSha256)}</dd>
        <dt>Sizing request sha256</dt><dd data-testid={tid("basis-sizing-request-sha256")}>{orNotPublished(basis.sizingRequestSha256)}</dd>
        {library !== null ? (
          <>
            <dt>Record details</dt>
            <dd data-testid={tid("basis-library-gate")}>
              {library.state === "bound"
                ? `shown from the supplied library ${library.libraryId} (sha256 ${library.sha256})`
                : `not shown: ${library.reason}`}
            </dd>
          </>
        ) : null}
      </dl>
      <h5>Request inputs</h5>
      {request === null ? (
        <p data-testid={tid("basis-request")}>Sizing request: {HANGER_SIZING_NOT_PUBLISHED_INPUT} (the input was not admitted).</p>
      ) : (
        <dl data-testid={tid("basis-request")}>
          <dt>Weight case</dt><dd>{request.weightCase}</dd>
          <dt>Weight support state</dt><dd>{request.weightSupportState}</dd>
          <dt>Design operating case</dt><dd>{request.designOperatingCase}</dd>
          <dt>Check operating cases</dt><dd data-testid={tid("basis-check-cases")}>{listOrNone(request.checkOperatingCases)}</dd>
          <dt>Candidates</dt>
          <dd>
            <ul>
              {request.candidates.map((candidate) => <li key={candidate.support}>{candidate.support}: {candidate.familyPolicy}</li>)}
            </ul>
          </dd>
          <dt>Load design</dt><dd>{request.method.loadDesign}</dd>
          <dt>Variation reference</dt><dd>{request.method.variationReference}</dd>
          <dt>Variation limit</dt><dd data-testid={tid("basis-variation-limit")}>{request.method.variationLimit.text}</dd>
          <dt>Travel allowance</dt><dd data-testid={tid("basis-travel-allowance")}>{request.method.travelAllowance.text}</dd>
          <dt>Selection rule</dt><dd>{request.method.selectionRule}</dd>
          <dt>Solver mode</dt><dd>{request.solverMode}</dd>
        </dl>
      )}
      {cases !== null ? (
        <>
          <h5>Cases</h5>
          <ul data-testid={tid("basis-cases")}>
            {cases.map((entry) => (
              <li key={entry.case} data-testid={tid(`basis-case-${entry.case}`)}>
                {entry.case}: role {entry.role}; load categories {listOrNone(entry.loadCategories)}; uses an operating-condition set: {entry.usesConditionSet ? "yes" : "no"}
              </li>
            ))}
          </ul>
        </>
      ) : null}
      <h5>Basis statement</h5>
      <ul data-testid={tid("basis-statement")}>{basis.basisStatement.map((line, index) => <li key={index}>{line}</li>)}</ul>
      <h5>Limitations</h5>
      <ul data-testid={tid("limitations")}>{basis.limitations.map((line, index) => <li key={index}>{line}</li>)}</ul>
    </section>
  );
}

function ReaderStandings({ standings, tid }: { standings: StandingsView; tid: Tid }) {
  return (
    <section aria-label="Reader standings" data-testid={tid("standings")}>
      <h4>Reader standings</h4>
      {standings.binding.state === "bound" ? (
        <p data-testid={tid("standings-binding")}>Reader evidence is bound to this document by its checked hash (sha256 {standings.binding.documentSha256}).</p>
      ) : (
        <p data-testid={tid("standings-binding")}>Reader evidence not bound: {standings.binding.reason}. Every standing reads not checked.</p>
      )}
      <ul>
        {standings.standings.map((standing) => (
          <li key={standing.name} data-testid={tid(`standing-${standing.name}`)}>
            {standing.name}: {STANDING_STATUS_TEXT[standing.status]}
            {standing.difference !== null ? <> (first difference: {standing.difference})</> : null}
            {standing.notChecked && standing.notChecked.length > 0 ? <>; inputs not checked: {standing.notChecked.join(", ")}</> : null}
          </li>
        ))}
      </ul>
      <small>{HANGER_SIZING_STANDINGS_TEXT}</small>
    </section>
  );
}

// ---------------------------------------------------------------------------------------------
// The sized view

function SizedView({ view, document, modelHash, tid }: { view: HangerSizingSizedView; document: HangerSizingSizedDocument; modelHash?: string | null; tid: Tid }) {
  const conditional = view.verificationStanding === "conditional";
  const noFit = view.rows.filter((row) => row.outcome === "no_record_fits").map((row) => row.support);
  const context = async () => ({ view, provenance: await hangerSizingCsvProvenance({ document, modelHash }), softwareVersion: CSV_SOFTWARE_VERSION });
  return (
    <div data-testid={tid("sized")} data-standing={view.verificationStanding}>
      <p data-testid={tid("status")}>{HANGER_SIZING_SIZED_TEXT}</p>
      <p role="note" className={conditional ? "hanger-sizing-conditional" : undefined} data-testid={tid("verification-standing")}>
        <strong>{conditional ? HANGER_SIZING_CONDITIONAL_TEXT : HANGER_SIZING_UNCONDITIONAL_TEXT}</strong>
        {conditional && noFit.length > 0 ? <> Candidates with no fit: {noFit.join(", ")}.</> : null}
      </p>
      <BasisBlock basis={view.basis} cases={view.cases} library={view.library} tid={tid} />

      <h4>Candidates</h4>
      <CsvExportControl
        tableLabel={HANGER_SIZING_CANDIDATES_TABLE_LABEL}
        testId={tid("export-csv")}
        build={async () => hangerSizingCandidatesCsv(await context())}
      />
      <table aria-label={HANGER_SIZING_CANDIDATES_TABLE_LABEL} data-testid={tid("candidates")} data-csv-export={HANGER_SIZING_CSV_KIND}>
        <thead>
          <tr>
            <th scope="col">Support</th>
            <th scope="col">Node, DOF</th>
            <th scope="col">Family policy</th>
            <th scope="col">Outcome</th>
            <th scope="col">Hanger (library id, hanger id, name, type)</th>
            <th scope="col">W</th>
            <th scope="col">F_cold: {F_COLD_LABEL}</th>
            <th scope="col">F_C: {F_COLD_CONDITION_LABEL}</th>
            <th scope="col">k</th>
            <th scope="col">Envelope</th>
            <th scope="col">Record working load range, travel range</th>
            <th scope="col">u_C</th>
            <th scope="col">Per operating case: u_i, F_i, v_i</th>
            <th scope="col">Governing travel, variation case</th>
            <th scope="col">Span</th>
            <th scope="col">Required travel</th>
          </tr>
        </thead>
        <tbody>
          {view.rows.map((row) => <CandidateRow key={row.support} row={row} libraryId={view.basis.library.libraryId} tid={tid} />)}
        </tbody>
      </table>

      <h4>Candidate details</h4>
      {view.rows.map((row) => <CandidateDetail key={row.support} row={row} tid={tid} />)}

      <h4>Derived solves</h4>
      <CsvExportControl
        tableLabel={HANGER_SIZING_SOLVES_TABLE_LABEL}
        testId={tid("export-csv-solves")}
        build={async () => hangerSizingSolvesCsv(await context())}
      />
      <table aria-label={HANGER_SIZING_SOLVES_TABLE_LABEL} data-testid={tid("solves")} data-csv-export={HANGER_SIZING_CSV_KIND}>
        <thead>
          <tr>
            <th scope="col">Solve</th>
            <th scope="col">Case</th>
            <th scope="col">Derived document sha256</th>
            <th scope="col">Nonlinear support states</th>
          </tr>
        </thead>
        <tbody>
          {view.solves.map((solve) => (
            <tr key={solve.label} data-testid={tid(`solve-${solve.label}`)}>
              <th scope="row">{solve.label}</th>
              <td>{solve.case}</td>
              <td><code>{solve.derivedDocumentSha256}</code></td>
              <td>
                {solve.states.length > 0
                  ? solve.states.map((entry) => `${entry.support}: ${entry.state}`).join("; ")
                  : HANGER_SIZING_NO_NONLINEAR_SUPPORT}
              </td>
            </tr>
          ))}
        </tbody>
      </table>

      <section aria-label="Disclosures" data-testid={tid("disclosures")}>
        <h4>Disclosures</h4>
        {view.disclosures.length > 0 ? <Findings findings={view.disclosures} tid={tid} name="disclosure" /> : <p>No disclosures.</p>}
      </section>
      <ReaderStandings standings={view.readerStandings} tid={tid} />
    </div>
  );
}

function recordText(row: CandidateRowView): { identity: string; ranges: string } {
  const record = row.record;
  if (record === null) return { identity: HANGER_SIZING_NO_FIT, ranges: HANGER_SIZING_NO_FIT };
  if (record.state === "not_shown") {
    const notShown = hangerSizingNotShown(record.reason);
    return { identity: `${record.hangerId}; record details ${notShown}`, ranges: notShown };
  }
  return {
    identity: `${record.hangerId}; ${record.name}; ${record.hangerType}`,
    ranges: `[${record.workingLoadRange.minimum.text}, ${record.workingLoadRange.maximum.text}]; travel ${record.travelRange.text}`
  };
}

function CandidateRow({ row, libraryId, tid }: { row: CandidateRowView; libraryId: string | null; tid: Tid }) {
  const selection = row.selection;
  const variable = selection?.family === "variable_spring" ? selection : null;
  const absent = selection === null ? HANGER_SIZING_NO_FIT : HANGER_SIZING_NOT_APPLICABLE_CONSTANT;
  const record = recordText(row);
  const forces = new Map((variable?.operating ?? []).map((entry) => [entry.case, entry]));
  return (
    <tr data-testid={tid(`row-${row.support}`)} data-outcome={row.outcome}>
      <th scope="row">{row.support}</th>
      <td>{row.node}, {row.dof}</td>
      <td>{row.familyPolicy}</td>
      <td data-testid={tid(`outcome-${row.support}`)}>{row.outcome}</td>
      <td data-testid={tid(`record-${row.support}`)}>{selection === null ? HANGER_SIZING_NO_FIT : `${orNotPublished(libraryId)}; ${selection.family}; ${record.identity}`}</td>
      <td>
        {row.w.text}
        {selection?.family === "constant_effort" ? <> (constant load {selection.load.text})</> : null}
      </td>
      <td data-testid={tid(`f-cold-${row.support}`)}>{variable ? variable.fCold.quantity.text : absent}</td>
      <td data-testid={tid(`f-c-${row.support}`)}>{variable ? variable.fColdCondition.quantity.text : absent}</td>
      <td>{variable ? variable.k.text : absent}</td>
      <td>{variable ? `[${variable.envelope[0].text}, ${variable.envelope[1].text}]` : absent}</td>
      <td data-testid={tid(`record-ranges-${row.support}`)}>{record.ranges}</td>
      <td>{row.uCold.text}</td>
      <td>
        <ul data-testid={tid(`per-case-${row.support}`)}>
          {row.operating.map((entry) => {
            const force = forces.get(entry.case);
            return (
              <li key={entry.case}>
                {entry.case}: u_i {entry.position.text}; F_i {force ? force.force.text : (variable ? HANGER_SIZING_NOT_PUBLISHED : absent)}; v_i {force ? variationText(force.variation) : (variable ? HANGER_SIZING_NOT_PUBLISHED : absent)}
              </li>
            );
          })}
        </ul>
      </td>
      <td>
        {row.governing === null
          ? HANGER_SIZING_NO_FIT
          : `travel ${row.governing.travelCase}; variation ${row.governing.variationCase ?? HANGER_SIZING_NOT_APPLICABLE_CONSTANT}`}
      </td>
      <td>[{row.span[0].text}, {row.span[1].text}]</td>
      <td>{row.requiredTravel.text}</td>
    </tr>
  );
}

function CountsText({ counts, keys }: { counts: FailureCountsView; keys: readonly string[] }) {
  if (counts.state === "not_tested") return <>{HANGER_SIZING_NOT_TESTED}</>;
  return <>{keys.map((key) => `${key} ${typeof counts.counts[key] === "number" ? counts.counts[key] : HANGER_SIZING_NOT_PUBLISHED}`).join(", ")}</>;
}

function CandidateDetail({ row, tid }: { row: CandidateRowView; tid: Tid }) {
  const assumption = row.verificationAssumption;
  const verification = row.verification;
  const record = row.record;
  return (
    <details data-testid={tid(`detail-${row.support}`)}>
      <summary>Details: {row.support} ({row.outcome})</summary>
      <dl>
        <dt>Verification assumption</dt>
        <dd data-testid={tid(`detail-assumption-${row.support}`)}>
          {assumption.role === "spring"
            ? `spring: k ${assumption.k.text}; F_cold (${F_COLD_LABEL}) ${assumption.fCold.quantity.text}`
            : `force: ${assumption.force.text}`}
        </dd>
        <dt>Verification</dt>
        <dd data-testid={tid(`detail-verification-${row.support}`)}>
          {verification === null ? HANGER_SIZING_NO_FIT : (
            <ul>
              <li>u_C {verification.uCold.text}</li>
              {verification.family === "variable_spring"
                ? verification.operating.map((entry) => (
                  <li key={entry.case}>{entry.case}: u_i {entry.position.text}; F_i {entry.force.text}; v_i {variationText(entry.variation)}</li>
                ))
                : verification.operating.map((entry) => <li key={entry.case}>{entry.case}: u_i {entry.position.text}</li>)}
              <li>span [{verification.span[0].text}, {verification.span[1].text}]; required travel {verification.requiredTravel.text}; T pass: {verification.tPass ? "yes" : "no"}</li>
              {verification.family === "variable_spring" ? (
                <li>F_C ({F_COLD_CONDITION_LABEL}) {verification.fColdCondition.quantity.text}; departure {verification.departure.text}</li>
              ) : null}
            </ul>
          )}
        </dd>
        <dt>Failed tests</dt>
        <dd data-testid={tid(`detail-failed-tests-${row.support}`)}>
          {verification === null ? HANGER_SIZING_NO_FIT : (verification.failedTests.length > 0 ? verification.failedTests.join(", ") : HANGER_SIZING_NO_FAILED_TESTS)}
        </dd>
        <dt>Records failing each test: variable family</dt>
        <dd data-testid={tid(`detail-counts-variable-${row.support}`)}><CountsText counts={row.failureCounts.variable} keys={HANGER_SIZING_VARIABLE_COUNT_KEYS} /></dd>
        <dt>Records failing each test: constant family</dt>
        <dd data-testid={tid(`detail-counts-constant-${row.support}`)}><CountsText counts={row.failureCounts.constant} keys={HANGER_SIZING_CONSTANT_COUNT_KEYS} /></dd>
        <dt>Governing cases</dt>
        <dd>
          {row.governing === null
            ? HANGER_SIZING_NO_FIT
            : `travel ${row.governing.travelCase}; variation ${row.governing.variationCase ?? HANGER_SIZING_NOT_APPLICABLE_CONSTANT}`}
        </dd>
        <dt>Span and required travel</dt>
        <dd>[{row.span[0].text}, {row.span[1].text}]; required travel {row.requiredTravel.text}</dd>
        <dt>Record</dt>
        <dd data-testid={tid(`detail-record-${row.support}`)}>
          {record === null ? `${HANGER_SIZING_NO_FIT}: no record is selected` : record.state === "not_shown" ? (
            `${record.hangerId}: record details ${hangerSizingNotShown(record.reason)}`
          ) : (
            <ul>
              <li>{record.hangerId}: {record.name} ({record.hangerType})</li>
              <li>record sha256 {record.recordSha256}</li>
              <li>k {record.stiffness ? record.stiffness.text : HANGER_SIZING_NOT_APPLICABLE_CONSTANT}</li>
              <li>working load range [{record.workingLoadRange.minimum.text}, {record.workingLoadRange.maximum.text}]</li>
              <li>travel range {record.travelRange.text}</li>
            </ul>
          )}
        </dd>
      </dl>
    </details>
  );
}

// ---------------------------------------------------------------------------------------------
// The refused view

function RefusedView({ view, tid }: { view: HangerSizingRefusedView; tid: Tid }) {
  return (
    <div data-testid={tid("refused")}>
      <p role="note" data-testid={tid("status")}><strong>{HANGER_SIZING_REFUSED_TEXT}</strong></p>
      <section aria-label="Refusals" data-testid={tid("refusals-section")}>
        <h4>Refusals</h4>
        <Findings findings={view.refusals} tid={tid} name="refusal" />
      </section>
      <BasisBlock basis={view.basis} cases={null} library={null} tid={tid} />
      <ReaderStandings standings={view.readerStandings} tid={tid} />
    </div>
  );
}
