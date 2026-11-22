// Whether the status bar's typed selection is legible: both of its spans (the type and the
// identifier) visible, with a box of at least MIN_SPAN_PX each way, their rendered text complete (not
// clipped by the product's ellipsis: scrollWidth not above clientWidth) and equal to the typed
// identity expected. Self-contained, so that it runs in the page through locator.evaluate.

export type StatusSelectionExpectation = Readonly<{ type: string; id: string }>;

export function judgeStatusSelectionExposure(element: Element, expected: StatusSelectionExpectation): any {
  const MIN_SPAN_PX = 4;
  const view = element.ownerDocument.defaultView;
  const span = (selector: string, text: string) => {
    const target = element.querySelector(selector);
    if (!view || !(target instanceof view.HTMLElement)) return { selector, present: false, exposed: false };
    const box = target.getBoundingClientRect();
    const style = view.getComputedStyle(target);
    const visible = style.display !== "none" && style.visibility !== "hidden" && Number(style.opacity) > 0;
    const sized = box.width >= MIN_SPAN_PX && box.height >= MIN_SPAN_PX;
    const complete = target.scrollWidth <= target.clientWidth && target.clientWidth > 0;
    const observedText = (target.textContent ?? "").trim();
    const textMatches = observedText === text;
    return {
      selector, present: true, expectedText: text, observedText, width: box.width, height: box.height,
      clientWidth: target.clientWidth, scrollWidth: target.scrollWidth, visible, sized, complete, textMatches,
      exposed: visible && sized && complete && textMatches
    };
  };
  const type = span(".shell-status-selection-type", `${expected.type}:`);
  const id = span(".shell-status-selection-id", expected.id);
  return {
    status: type.exposed && id.exposed ? "PASS_STATUS_SELECTION_EXPOSED" : "FAIL_STATUS_SELECTION_NOT_EXPOSED",
    type,
    id
  };
}
