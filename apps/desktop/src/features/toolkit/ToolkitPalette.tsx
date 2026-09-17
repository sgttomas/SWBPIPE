import { useEffect, useLayoutEffect, useRef, useState } from "react";
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
  const [groupFilter, setGroupFilter] = useState<(typeof toolkitGroups)[number] | null>(null);
  const toggle = useRef<HTMLButtonElement>(null);
  const returnFocus = useRef<HTMLButtonElement | null>(null);
  const search = useRef<HTMLInputElement>(null);
  const dialog = useRef<HTMLDivElement>(null);
  useLayoutEffect(() => {
    if (open) search.current?.focus();
  }, [open]);
  useEffect(() => {
    function openFromShortcut(event: globalThis.KeyboardEvent) {
      if (event.key.toLowerCase() !== "k" || (!event.metaKey && !event.ctrlKey)) return;
      event.preventDefault();
      returnFocus.current = toggle.current;
      setGroupFilter(null);
      setOpen(true);
    }
    window.addEventListener("keydown", openFromShortcut);
    return () => window.removeEventListener("keydown", openFromShortcut);
  }, []);
  const matches = toolkitCapabilities.filter((item) =>
    (!groupFilter || item.group === groupFilter) &&
    `${item.label} ${item.group} ${item.description}`.toLowerCase().includes(query.trim().toLowerCase())
  );

  function close() {
    setOpen(false);
    returnFocus.current?.focus();
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
        aria-label="Find modeling commands"
        title="Search Build, Supports, Properties, Loads, Edit, Select and View, and Review commands"
        onClick={() => {
          returnFocus.current = toggle.current;
          setGroupFilter(null);
          setOpen(!open);
        }}
      >
        <Search size={16} aria-hidden="true" /> Commands <kbd aria-label="Command K">⌘K</kbd>
      </button>
      <div className="toolkit-group-shortcuts" role="group" aria-label="Command groups">
        {toolkitGroups.map((group) => (
          <button
            aria-pressed={open && groupFilter === group}
            data-testid={`toolkit-group-${group.toLowerCase().replaceAll(" ", "-").replace("-and-", "-")}`}
            key={group}
            onClick={(event) => {
              returnFocus.current = event.currentTarget;
              setGroupFilter(group);
              setQuery("");
              setOpen(true);
            }}
            title={`Open ${group} commands`}
            type="button"
          >{group === "Select and View" ? "Select/View" : group}</button>
        ))}
      </div>
      {open ? (
        <>
        <div className="toolkit-backdrop" aria-hidden="true" onPointerDown={(event) => {
          event.preventDefault();
          event.stopPropagation();
          close();
        }} />
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
          {groupFilter ? <p className="toolkit-active-group" role="status">Showing {groupFilter} commands. Search stays within this group.</p> : null}
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
