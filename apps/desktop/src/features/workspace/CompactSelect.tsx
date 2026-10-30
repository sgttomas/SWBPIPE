import { createContext, useContext, useId, useLayoutEffect, useRef, useState, type CSSProperties, type KeyboardEvent, type ReactNode } from "react";
import { createPortal } from "react-dom";

export type CompactSelectOption = Readonly<{ value: string; label: string; disabled?: boolean }>;
type Props = Readonly<{
  options: readonly CompactSelectOption[];
  value: string;
  onValueChange: (value: string) => void;
  id?: string;
  "aria-label"?: string;
  "aria-labelledby"?: string;
  "aria-describedby"?: string;
  "data-testid"?: string;
  disabled?: boolean;
  className?: string;
  title?: string;
}>;

const SelectScope = createContext("");
export function CompactSelectScope({ scopeKey, children }: { scopeKey: string; children: ReactNode }) {
  return <SelectScope.Provider value={scopeKey}>{children}</SelectScope.Provider>;
}

/** A select-only control; shell scope changes cancel without remounting drafts. */
export function CompactSelect({ options, value, onValueChange, disabled, className, ...attributes }: Props) {
  const scope = useContext(SelectScope);
  const previousScope = useRef(scope);
  const generatedId = useId();
  const triggerId = attributes.id ?? generatedId;
  const listId = `${generatedId}-list`;
  const trigger = useRef<HTMLButtonElement>(null);
  const popup = useRef<HTMLDivElement>(null);
  const [open, setOpen] = useState(false);
  const openRef = useRef(false);
  const [activeValue, setActiveValue] = useState<string | null>(null);
  const active = options.findIndex((option) => option.value === activeValue);
  const pending = useRef<{ value: string | null; explicit: boolean }>({ value: null, explicit: false });
  const openingBasis = useRef<{ value: string; options: string } | null>(null);
  // Order does not change identity, but label/availability/membership changes do.
  function optionsBasis(items: readonly CompactSelectOption[]) {
    return JSON.stringify(items.map((option) => [option.value, option.label, !!option.disabled]).sort((a, b) => JSON.stringify(a).localeCompare(JSON.stringify(b))));
  }
  const catalogBasis = optionsBasis(options);
  const [placement, setPlacement] = useState<CSSProperties>({});
  const search = useRef({ text: "", time: 0 });
  // Event listeners always use the latest controlled value, options and callback.
  const latest = useRef({ options, value, onValueChange, disabled });
  latest.current = { options, value, onValueChange, disabled };

  function unavailable() {
    const button = trigger.current;
    if (!button || latest.current.disabled || button.matches(":disabled")) return true;
    for (let node: HTMLElement | null = button; node; node = node.parentElement) {
      if (node.hidden || node.hasAttribute("inert") || node.getAttribute("aria-hidden") === "true") return true;
      const style = getComputedStyle(node);
      if (style.display === "none" || style.visibility === "hidden" || style.visibility === "collapse") return true;
    }
    return false;
  }

  function activateValue(optionValue: string | null, explicit = true) {
    pending.current = { value: optionValue, explicit };
    setActiveValue(optionValue);
  }

  // Keyboard indexes are resolved immediately in the same current catalog.
  // Pointer handlers instead retain their rendered option value below.
  function activate(index: number, explicit = true) {
    activateValue(latest.current.options[index]?.value ?? null, explicit);
  }

  function sameBasis() {
    const current = latest.current;
    return openingBasis.current?.value === current.value && openingBasis.current.options === optionsBasis(current.options);
  }

  function activeIndex() {
    return latest.current.options.findIndex((option) => option.value === pending.current.value);
  }

  function close(commit: boolean, returnFocus = false) {
    if (!openRef.current) return;
    openRef.current = false;
    setOpen(false);
    search.current = { text: "", time: 0 };
    const current = latest.current;
    const option = current.options.find((item) => item.value === pending.current.value);
    const available = !unavailable();
    if (commit && pending.current.explicit && sameBasis() && available && option && !option.disabled && option.value !== current.value) current.onValueChange(option.value);
    if (returnFocus && available) trigger.current?.focus();
  }

  useLayoutEffect(() => {
    if (previousScope.current !== scope) close(false);
    previousScope.current = scope;
  }, [scope]);

  function show() {
    if (unavailable()) return;
    trigger.current?.focus();
    openingBasis.current = { value, options: optionsBasis(options) };
    const selected = options.findIndex((option) => option.value === value && !option.disabled);
    activate(selected >= 0 ? selected : options.findIndex((option) => !option.disabled), false);
    openRef.current = true;
    setOpen(true);
  }

  useLayoutEffect(() => {
    if (!open) return;
    function position() {
      if (unavailable()) { close(false); return; }
      const rect = trigger.current!.getBoundingClientRect();
      const margin = 8;
      const width = Math.max(0, Math.min(Math.max(rect.width, 160), window.innerWidth - margin * 2));
      const anchorTop = Math.max(margin, Math.min(rect.top, window.innerHeight - margin));
      const anchorBottom = Math.max(margin, Math.min(rect.bottom, window.innerHeight - margin));
      const below = Math.max(0, window.innerHeight - anchorBottom - margin);
      const above = Math.max(0, anchorTop - margin);
      const upward = below < 160 && above > below;
      setPlacement({
        position: "fixed", left: Math.max(margin, Math.min(rect.left, window.innerWidth - width - margin)),
        width, maxHeight: Math.min(280, upward ? above : below), overflowY: "auto", boxSizing: "border-box",
        ...(upward ? { bottom: Math.max(margin, window.innerHeight - anchorTop) } : { top: anchorBottom })
      });
    }
    function outsidePointer(event: PointerEvent) {
      if (event.target instanceof Node && !trigger.current?.contains(event.target) && !popup.current?.contains(event.target)) close(true);
    }
    function outsideFocus(event: FocusEvent) {
      if (event.target instanceof Node && !trigger.current?.contains(event.target) && !popup.current?.contains(event.target)) close(true);
    }
    function windowBlur() { close(false); }
    position();
    // Observe only our DOM ancestors; this is not a native-popup observer.
    const observer = new MutationObserver(position);
    for (let node: HTMLElement | null = trigger.current; node; node = node.parentElement) {
      observer.observe(node, { attributes: true, attributeFilter: ["hidden", "inert", "aria-hidden", "disabled", "class", "style"] });
    }
    window.addEventListener("blur", windowBlur);
    window.addEventListener("resize", position);
    window.addEventListener("scroll", position, true);
    document.addEventListener("pointerdown", outsidePointer, true);
    document.addEventListener("focusin", outsideFocus);
    return () => {
      observer.disconnect();
      window.removeEventListener("blur", windowBlur);
      window.removeEventListener("resize", position);
      window.removeEventListener("scroll", position, true);
      document.removeEventListener("pointerdown", outsidePointer, true);
      document.removeEventListener("focusin", outsideFocus);
    };
  }, [open]);

  useLayoutEffect(() => {
    if (!open) return;
    if (unavailable() || !sameBasis()) close(false);
  }, [open, catalogBasis, value, disabled]);

  useLayoutEffect(() => {
    if (open && active >= 0) document.getElementById(`${listId}-${active}`)?.scrollIntoView?.({ block: "nearest", inline: "nearest" });
  }, [open, active, listId]);

  function keyDown(event: KeyboardEvent<HTMLButtonElement>) {
    if (unavailable()) return;
    const key = event.key;
    if (key === "Escape") {
      if (openRef.current) {
        event.preventDefault();
        event.stopPropagation();
        close(false, true);
      }
      return;
    }
    if (key === "Tab") { close(true); return; }
    if (event.metaKey || event.ctrlKey || (event.altKey && key !== "ArrowDown" && key !== "ArrowUp")) return;
    const wasOpen = openRef.current;
    const navigation = ["ArrowDown", "ArrowUp", "Home", "End", "PageDown", "PageUp"].includes(key);
    const printable = key.length === 1 && key !== " ";
    if (!navigation && !printable && key !== "Enter" && key !== " ") return;
    event.preventDefault();
    // Avoid browser button key activation and keep shell shortcuts out of navigation.
    event.stopPropagation();
    if (!wasOpen) show();
    if (key === "Enter" || key === " ") { if (wasOpen) { pending.current.explicit = true; close(true, true); } return; }
    const enabled = options.map((option, index) => option.disabled ? -1 : index).filter((index) => index >= 0);
    if (!enabled.length) return;
    if (printable) {
      const now = Date.now();
      const old = now - search.current.time < 700 ? search.current.text : "";
      const text = old + key.toLocaleLowerCase();
      search.current = { text, time: now };
      const repeated = [...text].every((character) => character === text[0]);
      const query = repeated ? text[0] : text;
      const start = repeated ? activeIndex() + 1 : activeIndex();
      const order = [...enabled.filter((index) => index >= start), ...enabled.filter((index) => index < start)];
      const found = order.find((index) => options[index].label.toLocaleLowerCase().startsWith(query));
      if (found !== undefined) activate(found);
    } else if (key === "Home" || (!wasOpen && key === "ArrowUp")) activate(enabled[0]);
    else if (key === "End") activate(enabled[enabled.length - 1]);
    else if (wasOpen && event.altKey && key === "ArrowUp") close(true, true);
    else if (wasOpen) {
      const index = enabled.indexOf(activeIndex());
      const step = key === "PageDown" ? 10 : key === "PageUp" ? -10 : key === "ArrowDown" ? 1 : -1;
      activate(enabled[Math.max(0, Math.min(enabled.length - 1, index + step))]);
    }
  }

  const selected = options.find((option) => option.value === value);
  return <span className="compact-select">
    <button {...attributes} id={triggerId} ref={trigger} className={["compact-select-trigger", className].filter(Boolean).join(" ")}
      type="button" role="combobox" data-value={value} disabled={disabled} aria-haspopup="listbox" aria-expanded={open}
      aria-controls={open ? listId : undefined} aria-activedescendant={open && active >= 0 ? `${listId}-${active}` : undefined}
      onKeyDown={keyDown} onClick={() => { if (openRef.current) close(false, true); else show(); }}>
      <span className="compact-select-value">{selected?.label ?? (value || "No selection")}</span>
    </button>
    {open && createPortal(<div ref={popup} id={listId} role="listbox" className="compact-select-popup" style={placement}
      aria-label={attributes["aria-label"]} aria-labelledby={attributes["aria-labelledby"] ?? (attributes["aria-label"] ? undefined : triggerId)}
      onPointerDown={(event) => { event.preventDefault(); event.stopPropagation(); }} onClick={(event) => event.stopPropagation()}>
      {options.map((option, index) => <div key={`${index}-${option.value}`} id={`${listId}-${index}`} role="option"
        className="compact-select-option" aria-disabled={option.disabled || undefined} aria-selected={option.value === value}
        data-value={option.value} data-active={index === active} data-selected={option.value === value}
        onClick={() => { if (!option.disabled) { activateValue(option.value); close(true, true); } }}>
        {option.label}
      </div>)}
      {!options.length && <div className="compact-select-empty">No options</div>}
    </div>, document.body)}
  </span>;
}
