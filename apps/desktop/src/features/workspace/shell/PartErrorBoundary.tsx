import { Component, Fragment, type ErrorInfo, type ReactNode } from "react";
import { useSessionChrome } from "../WorkspaceSessionContext";

/**
 * A render error in one page or pane of the window shows a plain message in
 * that part's place, and everything outside it keeps working. The message names
 * the part by the name a person knows it by, and states nothing the boundary
 * cannot know: not the cause, and not whether anything was kept or lost.
 */

/** The text a thrown value carries, so that a person can pass it on. */
function errorText(error: unknown): string {
  if (error instanceof Error) return error.message ? `${error.name}: ${error.message}` : error.name;
  try {
    return String(error);
  } catch {
    return "The error has no text.";
  }
}

function ErrorDetails({ error }: { error: unknown }) {
  return (
    <details className="shell-part-error-details" data-testid="part-error-details">
      <summary>Error details</summary>
      <pre>{errorText(error)}</pre>
    </details>
  );
}

/** The way back from a failed part: the rail's Model control, by another hand. */
function ShowModelStage() {
  const { enterStage } = useSessionChrome();
  return <button type="button" data-testid="part-error-show-model" onClick={() => enterStage("model")}>Show the Model stage</button>;
}

/** The way back from a failed Issues drawer: close it, as the rail's Issues control does. */
function CloseIssues() {
  const { setIssuesDrawerOpen } = useSessionChrome();
  return <button type="button" data-testid="part-error-close-issues" onClick={() => setIssuesDrawerOpen(false)}>Close Issues</button>;
}

type PartErrorBoundaryProps = Readonly<{
  /** The part as a person knows it, with its article: "The Project page", "The inspector". */
  subject: string;
  /** A change of this value tries a failed part again (a project opened or created). */
  resetKey?: unknown;
  /** The way back beside Try again: the Model stage, or closing the Issues drawer. None by default. */
  back?: "model-stage" | "close-issues";
  /** Classes for the message's box, so that it stands where the part stood. */
  className?: string;
  children?: ReactNode;
}>;

type PartErrorBoundaryState = {
  failed: boolean;
  error: unknown;
  resetKey: unknown;
  /** Mounts the part afresh on Try again or a new session. */
  attempt: number;
};

export class PartErrorBoundary extends Component<PartErrorBoundaryProps, PartErrorBoundaryState> {
  state: PartErrorBoundaryState = { failed: false, error: null, resetKey: this.props.resetKey, attempt: 0 };

  static getDerivedStateFromError(error: unknown): Partial<PartErrorBoundaryState> {
    return { failed: true, error };
  }

  static getDerivedStateFromProps(props: PartErrorBoundaryProps, state: PartErrorBoundaryState): Partial<PartErrorBoundaryState> | null {
    if (Object.is(props.resetKey, state.resetKey)) return null;
    // A part that has not failed is left mounted as it is; only a failed one is tried again.
    return state.failed
      ? { resetKey: props.resetKey, failed: false, error: null, attempt: state.attempt + 1 }
      : { resetKey: props.resetKey };
  }

  componentDidCatch(error: unknown, _info: ErrorInfo) {
    console.error(`${this.props.subject} could not be shown.`, error);
  }

  private readonly tryAgain = () => {
    this.setState((state) => ({ failed: false, error: null, attempt: state.attempt + 1 }));
  };

  render() {
    if (!this.state.failed) return <Fragment key={this.state.attempt}>{this.props.children}</Fragment>;
    const { subject, back, className } = this.props;
    return (
      <div className={className ? `shell-part-error ${className}` : "shell-part-error"} data-testid="part-error">
        <p role="alert" className="shell-part-error-message">{subject} could not be shown.</p>
        <div className="shell-part-error-actions">
          <button type="button" data-testid="part-error-try-again" onClick={this.tryAgain}>Try again</button>
          {back === "model-stage" ? <ShowModelStage /> : back === "close-issues" ? <CloseIssues /> : null}
        </div>
        <ErrorDetails error={this.state.error} />
      </div>
    );
  }
}

type WindowErrorBoundaryState = { failed: boolean; error: unknown };

/**
 * The last boundary, where the app is mounted: an error outside every page and
 * pane still leaves words in the window, and the one way on it offers is a reload.
 */
export class WindowErrorBoundary extends Component<{ children?: ReactNode }, WindowErrorBoundaryState> {
  state: WindowErrorBoundaryState = { failed: false, error: null };

  static getDerivedStateFromError(error: unknown): WindowErrorBoundaryState {
    return { failed: true, error };
  }

  componentDidCatch(error: unknown, _info: ErrorInfo) {
    console.error("The window could not be shown.", error);
  }

  render() {
    if (!this.state.failed) return this.props.children;
    return (
      <main className="shell-window-error" data-testid="window-error">
        <div className="shell-part-error">
          <p role="alert" className="shell-part-error-message">The window could not be shown.</p>
          <div className="shell-part-error-actions">
            <button type="button" data-testid="window-error-reload" aria-describedby="window-error-reload-note" onClick={() => window.location.reload()}>Reload the window</button>
            <span id="window-error-reload-note" className="shell-part-error-note">Changes that were not saved are lost on reload.</span>
          </div>
          <ErrorDetails error={this.state.error} />
        </div>
      </main>
    );
  }
}
