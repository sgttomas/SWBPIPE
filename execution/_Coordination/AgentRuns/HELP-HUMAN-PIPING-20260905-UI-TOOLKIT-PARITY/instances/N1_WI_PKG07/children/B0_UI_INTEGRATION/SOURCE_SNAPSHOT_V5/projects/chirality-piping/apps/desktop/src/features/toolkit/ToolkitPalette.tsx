import { useRef, useState } from "react";
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
        Toolkit
      </button>
      {open ? (
        <div id="toolkit-commands" className="toolkit-commands">
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
          <button type="button" onClick={close}>Close toolkit</button>
          <div className="toolkit-command-groups">
            {toolkitGroups.map((group) => {
              const entries = matches.filter((entry) => entry.group === group);
              if (!entries.length) return null;
              return (
                <section key={group} aria-label={group}>
                  <h3>{group}</h3>
                  {entries.map((entry) => {
                    const availability = capabilityAvailability(entry, context);
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
                          {entry.label}
                        </button>
                        <small id={`toolkit-help-${entry.id}`}>
                          <strong>
                            {entry.status === "supported" ? "Available"
                              : entry.status === "partial" ? "Partial"
                              : entry.status === "gated" ? "Project gate" : "Planned"}.
                          </strong>{" "}
                          {availability.reason}
                        </small>
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
      ) : null}
    </section>
  );
}
