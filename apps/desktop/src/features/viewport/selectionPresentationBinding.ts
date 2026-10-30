import type { EntityKey } from "../workspace/selectionState";

type AppliedSelection = Readonly<{
  resourceGeneration: number;
  modelGeneration: number;
  revision: number;
  appliedAfterSubmissionSequence: number;
  orderedKeys: readonly EntityKey[];
}>;
export type RenderedSelection = AppliedSelection & Readonly<{ renderedSubmissionSequence: number }>;
type SelectionResource = {
  readonly contextSnapshot: Readonly<{ generation: number }>;
  readonly submissionSequence: number;
  setSelectionPresentation(keys: readonly EntityKey[]): void;
};
const positive = (value: number): boolean => Number.isSafeInteger(value) && value > 0;
const nonnegative = (value: number): boolean => Number.isSafeInteger(value) && value >= 0;
const sameKeys = (left: readonly EntityKey[], right: readonly EntityKey[]): boolean =>
  left.length === right.length && left.every((key, index) => key === right[index]);

/** Bounded bookkeeping around the existing synchronous setter and actual after-render hook. */
export class SelectionPresentationBinding {
  private applied: AppliedSelection | null = null;
  private rendered: RenderedSelection | null = null;
  private resourceGeneration: number | null = null;
  private revision = 0;

  clear(): void {
    this.applied = null;
    this.rendered = null;
  }

  apply(resource: SelectionResource, modelGeneration: number | null, keys: readonly EntityKey[]): void {
    // A thrown setter cannot publish an applied record. No scheduling is added here.
    try {
      resource.setSelectionPresentation(keys);
    } catch (error) {
      this.clear();
      throw error;
    }
    const resourceGeneration = resource.contextSnapshot.generation;
    const after = resource.submissionSequence;
    if (!positive(resourceGeneration) || modelGeneration === null || !positive(modelGeneration) || !nonnegative(after)) {
      this.clear();
      return;
    }
    if (this.resourceGeneration !== resourceGeneration) {
      this.resourceGeneration = resourceGeneration;
      this.revision = 0;
    }
    this.applied = Object.freeze({
      resourceGeneration, modelGeneration, revision: ++this.revision,
      appliedAfterSubmissionSequence: after, orderedKeys: Object.freeze([...keys])
    });
    this.rendered = null;
  }

  afterFrame(resourceGeneration: number, modelGeneration: number | null, submissionSequence: number): void {
    const applied = this.applied;
    this.rendered = applied && positive(resourceGeneration) && modelGeneration !== null && positive(modelGeneration) &&
      positive(submissionSequence) && applied.resourceGeneration === resourceGeneration &&
      applied.modelGeneration === modelGeneration && submissionSequence > applied.appliedAfterSubmissionSequence
      ? Object.freeze({ ...applied, renderedSubmissionSequence: submissionSequence })
      : null;
  }

  current(resourceGeneration: number, modelGeneration: number, submissionSequence: number): RenderedSelection | null {
    const rendered = this.rendered;
    return rendered && rendered.resourceGeneration === resourceGeneration && rendered.modelGeneration === modelGeneration &&
      rendered.renderedSubmissionSequence === submissionSequence ? rendered : null;
  }

  qualifyingSubmission(resourceGeneration: number, modelGeneration: number, submissionSequence: number,
    actionAfterSubmissionSequence: number, keys: readonly EntityKey[]): number {
    const rendered = this.current(resourceGeneration, modelGeneration, submissionSequence);
    return rendered && nonnegative(actionAfterSubmissionSequence) && submissionSequence > actionAfterSubmissionSequence &&
      sameKeys(rendered.orderedKeys, keys) ? submissionSequence : 0;
  }
}
