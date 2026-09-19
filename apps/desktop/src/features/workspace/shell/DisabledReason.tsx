/**
 * The reason a control is disabled, shown to the pointer (hover) and to the
 * keyboard (focus) by the stylesheet, and read with the control through
 * `aria-describedby`. The control itself stays focusable: it is `aria-disabled`,
 * never `disabled`, so that its reason can be reached.
 */
export function DisabledReason({ id, text }: { id: string; text: string }) {
  return <span className="shell-reason" id={id} role="tooltip">{text}</span>;
}
