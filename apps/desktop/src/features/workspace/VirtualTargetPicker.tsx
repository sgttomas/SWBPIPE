import { useId, useMemo, useState, type KeyboardEvent } from "react";
import { VirtualList } from "./VirtualList";

export type TargetPickerOption = Readonly<{
  value: string;
  label: string;
  keywords?: readonly string[];
  disabled?: boolean;
}>;

type Props = Readonly<{
  label: string;
  options: readonly TargetPickerOption[];
  value: string;
  onChange: (value: string) => void;
  allowClear?: boolean;
  disabled?: boolean;
  disabledReason?: string;
  placeholder?: string;
  testId?: string;
}>;

type MultiProps = Readonly<{
  label: string;
  options: readonly TargetPickerOption[];
  values: readonly string[];
  onChange: (values: readonly string[]) => void;
  disabled?: boolean;
  disabledReason?: string;
  placeholder?: string;
  testId?: string;
}>;

export function VirtualTargetPicker({
  label,
  options,
  value,
  onChange,
  allowClear = true,
  disabled = false,
  disabledReason,
  placeholder = "Search targets",
  testId
}: Props) {
  const listId = useId();
  const [query, setQuery] = useState("");
  const [activeIndex, setActiveIndex] = useState(0);
  const [revealActiveRequest, setRevealActiveRequest] = useState(0);
  const normalized = query.trim().toLocaleLowerCase();
  const filtered = useMemo(() => options.filter((option) => !normalized || [
    option.label,
    option.value,
    ...(option.keywords ?? [])
  ].some((part) => part.toLocaleLowerCase().includes(normalized))), [normalized, options]);
  const selected = options.find((option) => option.value === value) ?? null;
  const safeActive = filtered.length === 0 ? -1 : Math.min(activeIndex, filtered.length - 1);

  function moveActive(event: KeyboardEvent<HTMLInputElement>) {
    if (filtered.length === 0) return;
    let next = safeActive;
    if (event.key === "ArrowDown") next = Math.min(filtered.length - 1, safeActive + 1);
    else if (event.key === "ArrowUp") next = Math.max(0, safeActive - 1);
    else if (event.key === "Home") next = 0;
    else if (event.key === "End") next = filtered.length - 1;
    else if (event.key === "PageDown") next = Math.min(filtered.length - 1, safeActive + 8);
    else if (event.key === "PageUp") next = Math.max(0, safeActive - 8);
    else if (event.key === "Enter" && safeActive >= 0) {
      const option = filtered[safeActive];
      if (!disabled && !option.disabled) onChange(option.value);
      event.preventDefault();
      return;
    } else return;
    setActiveIndex(next);
    setRevealActiveRequest((sequence) => sequence + 1);
    event.preventDefault();
  }

  return (
    <div className="virtual-target-picker" data-testid={testId}>
      <label>
        <span>{label}</span>
        <input
          aria-controls={listId}
          aria-expanded="true"
          aria-label={label}
          aria-activedescendant={safeActive >= 0 ? `${listId}-option-${safeActive}` : undefined}
          autoComplete="off"
          disabled={disabled}
          onChange={(event) => { setQuery(event.target.value); setActiveIndex(0); }}
          onKeyDown={moveActive}
          placeholder={placeholder}
          role="combobox"
          type="search"
          value={query}
        />
      </label>
      {disabled && disabledReason ? <p className="control-reason">{disabledReason}</p> : null}
      <div className="virtual-target-picker-current" aria-label={`${label} current value`}>
        <span>{selected ? `${selected.label} (${selected.value})` : value || "No target selected"}</span>
        {allowClear && value ? <button disabled={disabled} type="button" onClick={() => { if (!disabled) onChange(""); }}>Clear</button> : null}
      </div>
      <span aria-live="polite" className="virtual-target-picker-status">
        {filtered.length} of {options.length} targets
      </span>
      <VirtualList
        activeIndex={safeActive}
        revealActiveRequest={revealActiveRequest}
        ariaLabel={`${label} options`}
        className="virtual-target-picker-list"
        height={Math.min(240, Math.max(32, filtered.length * 32))}
        id={listId}
        itemKey={(option) => option.value}
        items={filtered}
        renderItem={(option, index) => (
          <button
            aria-selected={option.value === value}
            className={`virtual-target-picker-option ${index === safeActive ? "active" : ""}`}
            disabled={disabled || option.disabled}
            id={`${listId}-option-${index}`}
            onClick={() => { if (!disabled && !option.disabled) onChange(option.value); }}
            onMouseMove={() => { if (!disabled) setActiveIndex(index); }}
            role="option"
            tabIndex={-1}
            type="button"
          >
            <span>{option.label}</span><small>{option.value}</small>
          </button>
        )}
        role="listbox"
        rowHeight={32}
        testId={`${testId ?? "target-picker"}-list`}
      />
    </div>
  );
}

export function VirtualMultiTargetPicker({
  label,
  options,
  values,
  onChange,
  disabled = false,
  disabledReason,
  placeholder = "Search targets",
  testId
}: MultiProps) {
  const listId = useId();
  const [query, setQuery] = useState("");
  const [activeIndex, setActiveIndex] = useState(0);
  const [revealActiveRequest, setRevealActiveRequest] = useState(0);
  const membership = useMemo(() => new Set(values), [values]);
  const normalized = query.trim().toLocaleLowerCase();
  const filtered = useMemo(() => options.filter((option) => !normalized || [
    option.label,
    option.value,
    ...(option.keywords ?? [])
  ].some((part) => part.toLocaleLowerCase().includes(normalized))), [normalized, options]);
  const safeActive = filtered.length === 0 ? -1 : Math.min(activeIndex, filtered.length - 1);

  function toggle(value: string) {
    if (disabled) return;
    const next = new Set(values);
    if (next.has(value)) next.delete(value);
    else next.add(value);
    onChange(options.flatMap((option) => next.has(option.value) ? [option.value] : []));
  }

  function moveActive(event: KeyboardEvent<HTMLInputElement>) {
    if (filtered.length === 0) return;
    let next = safeActive;
    if (event.key === "ArrowDown") next = Math.min(filtered.length - 1, safeActive + 1);
    else if (event.key === "ArrowUp") next = Math.max(0, safeActive - 1);
    else if (event.key === "Home") next = 0;
    else if (event.key === "End") next = filtered.length - 1;
    else if (event.key === "Enter" && safeActive >= 0) {
      const option = filtered[safeActive];
      if (!option.disabled) toggle(option.value);
      event.preventDefault();
      return;
    } else return;
    setActiveIndex(next);
    setRevealActiveRequest((sequence) => sequence + 1);
    event.preventDefault();
  }

  return (
    <div className="virtual-target-picker virtual-multi-target-picker" data-testid={testId}>
      <label>
        <span>{label}</span>
        <input
          aria-controls={listId}
          aria-expanded="true"
          aria-label={label}
          aria-activedescendant={safeActive >= 0 ? `${listId}-option-${safeActive}` : undefined}
          autoComplete="off"
          disabled={disabled}
          onChange={(event) => { setQuery(event.target.value); setActiveIndex(0); }}
          onKeyDown={moveActive}
          placeholder={placeholder}
          role="combobox"
          type="search"
          value={query}
        />
      </label>
      {disabled && disabledReason ? <p className="control-reason">{disabledReason}</p> : null}
      <div className="virtual-target-picker-current" aria-label={`${label} current values`}>
        <span>{values.length} selected</span>
        {values.length ? <button disabled={disabled} type="button" onClick={() => { if (!disabled) onChange([]); }}>Clear all</button> : null}
      </div>
      <span aria-live="polite" className="virtual-target-picker-status">
        {filtered.length} of {options.length} targets; {values.length} selected
      </span>
      <VirtualList
        activeIndex={safeActive}
        revealActiveRequest={revealActiveRequest}
        ariaActiveDescendant={safeActive >= 0 ? `${listId}-option-${safeActive}` : undefined}
        ariaLabel={`${label} options`}
        ariaMultiselectable
        className="virtual-target-picker-list"
        height={Math.min(240, Math.max(32, filtered.length * 32))}
        id={listId}
        itemKey={(option) => option.value}
        items={filtered}
        renderItem={(option, index) => (
          <button
            aria-selected={membership.has(option.value)}
            className={`virtual-target-picker-option ${index === safeActive ? "active" : ""}`}
            disabled={disabled || option.disabled}
            id={`${listId}-option-${index}`}
            onClick={() => toggle(option.value)}
            onMouseMove={() => { if (!disabled) setActiveIndex(index); }}
            role="option"
            tabIndex={-1}
            type="button"
          >
            <span>{option.label}</span><small>{option.value}</small>
          </button>
        )}
        role="listbox"
        rowHeight={32}
        testId={`${testId ?? "multi-target-picker"}-list`}
      />
    </div>
  );
}
