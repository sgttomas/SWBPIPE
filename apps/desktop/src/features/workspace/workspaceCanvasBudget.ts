const CANVAS_MINIMUM_PX = 200;
const RESERVE_PROPERTY = "--workspace-modeling-reserve";

// Observe chrome only: neither the flexible canvas nor the dock can feed their
// allocated height back into this reserve. App owns this subscription's lifetime.
export function observeWorkspaceCanvasBudget(workspace: HTMLElement): () => void {
  let disposed = false;
  let observed: Element[] = [];
  let previous = "";
  let mutationRoot: Element | null = null;
  const number = (value: string) => Number.parseFloat(value) || 0;
  const verticalEdges = (element: Element) => {
    const style = getComputedStyle(element);
    return number(style.borderTopWidth) + number(style.borderBottomWidth) +
      number(style.paddingTop) + number(style.paddingBottom);
  };
  const update = () => {
    if (disposed) return;
    const shell = workspace.querySelector<HTMLElement>(".viewport-shell");
    const modeling = workspace.querySelector<HTMLElement>(".modeling-workspace");
    if (!shell || !modeling) return;
    const rows = Array.from(shell.querySelectorAll<HTMLElement>(":scope > .command-bar, :scope > .viewport-toolbar, :scope > .viewport-measurement-strip"));
    let required = CANVAS_MINIMUM_PX + rows.reduce((total, row) => total + row.getBoundingClientRect().height, 0);
    // The shell declares four tracks, including the optional measurement track.
    required += 3 * number(getComputedStyle(shell).rowGap);
    for (let parent: HTMLElement | null = shell; parent && parent !== workspace; parent = parent.parentElement) {
      required += verticalEdges(parent);
      if (parent === modeling) break;
    }
    required += number(getComputedStyle(workspace).rowGap);
    const value = `${Math.ceil(required)}px`;
    if (value !== previous) {
      workspace.style.setProperty(RESERVE_PROPERTY, value);
      previous = value;
    }
  };
  const resize = typeof ResizeObserver === "undefined" ? null : new ResizeObserver(update);
  const rebind = () => {
    if (disposed) return;
    const shell = workspace.querySelector(".viewport-shell");
    const nextRoot = shell ?? workspace;
    if (mutationRoot !== nextRoot) {
      children.disconnect();
      // After loading, only direct viewport rows can add/remove measurement.
      children.observe(nextRoot, { childList: true, subtree: !shell });
      mutationRoot = nextRoot;
    }
    const next: Element[] = [workspace, ...workspace.querySelectorAll(
      ".viewport-shell > .command-bar, .viewport-shell > .viewport-toolbar, .viewport-shell > .viewport-measurement-strip",
    )];
    if (next.length !== observed.length || next.some((element, index) => element !== observed[index])) {
      resize?.disconnect();
      for (const element of next) resize?.observe(element, { box: "border-box" });
      observed = next;
    }
    update();
  };
  const children = new MutationObserver(rebind);
  rebind();
  return () => {
    disposed = true;
    children.disconnect();
    resize?.disconnect();
    workspace.style.removeProperty(RESERVE_PROPERTY);
  };
}
