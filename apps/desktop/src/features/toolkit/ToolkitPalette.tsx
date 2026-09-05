import { useEffect, useRef, useState } from "react";
import { Box, CircleDot, SlidersHorizontal, Layers, Pencil, MousePointer2, ClipboardCheck, Search, X } from "lucide-react";
import {
  capabilityAvailability,
  toolkitCapabilities,
  toolkitGroups,
  toolkitRoadmap,
  type ToolkitCapability,
  type ToolkitContext
} from "./capabilityCatalog";

type Props = {
  context: ToolkitContext;
  onChoose: (capability: ToolkitCapability) => void;
};

export function ToolkitPalette({ context, onChoose }: Props) {
  const [open, setOpen] = useState(false);
  const [query, setQuery] = useState("");
  const toggle = useRef<HTMLButtonElement>(null);
  const search = useRef<HTMLInputElement>(null);
  const dialog = useRef<HTMLDivElement>(null);
  useEffect(() => {
    if (!open) return;
    search.current?.focus();
    function dismiss(event: PointerEvent) {
      if (event.target instanceof Node && !dialog.current?.contains(event.target) && !toggle.current?.contains(event.target)) close();
    }
    document.addEventListener("pointerdown", dismiss);
    return () => document.removeEventListener("pointerdown", dismiss);
  }, [open]);
  const matches = toolkitCapabilities.filter((item) =>
    `${item.label} ${item.group} ${item.description}`.toLowerCase().includes(query.trim().toLowerCase())
  );

  function close() {
    setOpen(false);
    toggle.current?.focus();
  }

  return (
    <section
      className="toolkit-palette"
      aria-label="Human toolkit"
      onKeyDown={(event) => {
        if (event.key === "Escape" && open) {
          event.preventDefault();
          event.stopPropagation();
          close();
        }
      }}
    >
      <button
        data-testid="toolkit-entry"
        ref={toggle}
        type="button"
        aria-expanded={open}
        aria-controls="toolkit-commands"
        onClick={() => {
          setOpen(!open);
          if (!open) requestAnimationFrame(() => search.current?.focus());
        }}
      >
        <Search size={16} aria-hidden="true" /> Toolkit
      </button>
      {open ? (
        <>
        <div className="toolkit-backdrop" aria-hidden="true" onPointerDown={close} />
        <div id="toolkit-commands" className="toolkit-commands" ref={dialog} role="dialog" aria-label="Find a modeling tool" aria-modal="true" onKeyDown={(event) => {
          if (event.key !== "Tab") return;
          const focusable = Array.from(dialog.current?.querySelectorAll<HTMLElement>('button:not(:disabled), input, summary') ?? []);
          const first = focusable[0];
          const last = focusable[focusable.length - 1];
          if (event.shiftKey && document.activeElement === first) { event.preventDefault(); last?.focus(); }
          if (!event.shiftKey && document.activeElement === last) { event.preventDefault(); first?.focus(); }
        }}>
          <div className="toolkit-search-header">
          <label>
            Find a tool
            <input
              ref={search}
              type="search"
              aria-label="Find a tool"
              value={query}
              onChange={(event) => setQuery(event.target.value)}
              placeholder="Pipe, wind, material…"
            />
          </label>
          <button type="button" onClick={close} aria-label="Close toolkit"><X size={18} aria-hidden="true" /></button>
          </div>
          <div className="toolkit-command-groups">
            {toolkitGroups.map((group) => {
              const entries = matches.filter((entry) => entry.group === group);
              if (!entries.length) return null;
              return (
                <section key={group} aria-label={group}>
                  <h3>{group}</h3>
                  {entries.map((entry) => {
                    const availability = capabilityAvailability(entry, context);
                    const Icon = { Build: Box, Supports: CircleDot, Properties: SlidersHorizontal, Loads: Layers, Edit: Pencil, "Select and View": MousePointer2, Review: ClipboardCheck }[entry.group];
                    return (
                      <div className="toolkit-command" key={entry.id}>
                        <button
                          type="button"
                          data-testid={`toolkit-${entry.id}`}
                          disabled={!availability.enabled}
                          aria-describedby={`toolkit-help-${entry.id}`}
                          onClick={() => {
                            if (!capabilityAvailability(entry, context).enabled) return;
                            setOpen(false);
                            onChoose(entry);
                          }}
                        >
                          <Icon size={15} aria-hidden="true" />{entry.label}
                        </button>
                        {availability.enabled ? (
                          <details className="toolkit-command-help"><summary>Details</summary><small id={`toolkit-help-${entry.id}`}>{availability.reason}</small></details>
                        ) : <small id={`toolkit-help-${entry.id}`}>{availability.reason}</small>}
                      </div>
                    );
                  })}
                </section>
              );
            })}
          </div>
          {!matches.length ? <p role="status">No tools match this search.</p> : null}
          <details>
            <summary>Deferred roadmap</summary>
            <ul>{toolkitRoadmap.map((label) => <li key={label}>{label}</li>)}</ul>
          </details>
        </div>
        </>
      ) : null}
    </section>
  );
}
