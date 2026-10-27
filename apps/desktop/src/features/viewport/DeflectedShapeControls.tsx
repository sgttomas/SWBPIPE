// Deflected-shape controls (DEL-07-05, FR-GUI-001; design system 6.8): the case drawn, the
// magnification factor, the undeformed ghost and the animation between the undeformed and the
// deformed shape. Every control is view state of this session. None writes to the model, the
// result or an export, and none changes a result value: the factor and the phase only scale
// how emitted displacement rows are drawn.

import { useEffect, useState } from "react";
import type { DeformationOverlay } from "./deformationOverlay";
import { stepFactor125 } from "./deformationOverlay";
import type { ResultOverlayCase } from "./resultColourOverlay";

/** The phase slider's steps from undeformed (0) to deformed (the maximum). */
export const DEFLECTION_PHASE_STEPS = 24;

export function formatFactor(value: number): string {
  return String(Number(value.toPrecision(6)));
}

/** The on-canvas statement of how the deformed shape is drawn: the magnification actually
 * drawn (the stated factor times the phase, with both named when the phase is not 1) and the
 * case, or the normalized scale and its basis. */
export function deflectionLabel(deformation: DeformationOverlay, caseLabel: string | null): string {
  const basis = deformation.basisKey === null ? "governing case per node" : caseLabel ?? deformation.basisKey;
  const phase = deformation.phase;
  const partial = phase < 1 ? ` (${Math.round(phase * 100)}% of the deflection)` : "";
  if (deformation.scale.mode === "factor") {
    const { factor, isDefault } = deformation.scale;
    const drawn = phase < 1
      ? `×${formatFactor(factor * phase)}, ${Math.round(phase * 100)}% of ×${formatFactor(factor)}${isDefault ? " (default)" : ""}`
      : `×${formatFactor(factor)}${isDefault ? " (default)" : ""}`;
    return `Deformed ${drawn} · ${basis}`;
  }
  return `Deformed · normalized, not to scale${partial} · ${basis}`;
}

/** The popover summary's scale: the magnification drawn now. */
export function deflectionSummaryScale(deformation: DeformationOverlay): string {
  if (deformation.scale.mode !== "factor") return "normalized";
  return `×${formatFactor(deformation.scale.factor * deformation.phase)}`;
}

export function DeflectedShapeControls({
  cases,
  caseKey,
  onCaseChange,
  deformation,
  onFactorChange,
  ghost,
  onGhostChange,
  playing,
  onPlayingChange,
  phase,
  onPhaseChange,
  reducedMotion
}: {
  cases: readonly ResultOverlayCase[];
  caseKey: string | null;
  onCaseChange: (key: string | null) => void;
  /** The overlay at full deflection. */
  deformation: DeformationOverlay;
  /** A positive factor, or null for the case's default. */
  onFactorChange: (factor: number | null) => void;
  ghost: boolean;
  onGhostChange: (ghost: boolean) => void;
  playing: boolean;
  onPlayingChange: (playing: boolean) => void;
  phase: number;
  onPhaseChange: (phase: number) => void;
  reducedMotion: boolean;
}) {
  const factor = deformation.scale.mode === "factor" ? deformation.scale.factor : null;
  const [draft, setDraft] = useState<string>(factor === null ? "" : formatFactor(factor));
  const [draftError, setDraftError] = useState<string | null>(null);
  useEffect(() => {
    setDraft(factor === null ? "" : formatFactor(factor));
    setDraftError(null);
  }, [factor]);

  const factorReady = factor !== null;
  // The field takes a factor whenever one could be drawn, including when there is no default.
  const fieldReady = deformation.state === "available" && deformation.factorAccepted;
  const stated = deformation.state !== "available"
    ? "No displacement rows to draw for this choice."
    : deformation.factorUnavailable
      ? `Factor unavailable: ${deformation.factorUnavailable}.`
      : factorReady
        ? `Displacements are drawn at ×${formatFactor(factor)} their physical length in model units${deformation.scale.mode === "factor" && deformation.scale.isDefault ? "; the default draws the largest at one tenth of the model extent, rounded to 1, 2 or 5" : ""}.`
        : deformation.basisKey === null
          ? "The largest displacement is drawn at a fixed display length; choose one case to set a factor."
          : "The largest displacement is drawn at a fixed display length.";
  const reason = deformation.factorNote ? `${stated} Note: ${deformation.factorNote}.` : stated;

  function commitDraft() {
    const value = Number(draft.trim());
    if (!draft.trim() || !Number.isFinite(value) || value <= 0) {
      setDraftError("Enter a positive number.");
      return;
    }
    setDraftError(null);
    onFactorChange(value);
  }

  return (
    <div className="viewport-deflection-controls" role="group" aria-label="Deflected shape controls" data-testid="viewport-deflection-controls">
      <label>
        <span>Case</span>
        <select
          aria-label="Deflected shape case"
          data-testid="viewport-deflection-case"
          value={caseKey ?? ""}
          onChange={(event) => onCaseChange(event.target.value || null)}
        >
          <option value="">Governing row per node (normalized)</option>
          {cases.map((item) => <option key={item.key} value={item.key}>{item.label}</option>)}
        </select>
      </label>
      <div className="viewport-deflection-factor" role="group" aria-label="Magnification factor">
        <button
          type="button"
          aria-label="Smaller magnification factor"
          data-testid="viewport-deflection-factor-down"
          disabled={!factorReady}
          onClick={() => factor !== null && onFactorChange(stepFactor125(factor, -1))}
        >−</button>
        <label>
          <span>×</span>
          <input
            aria-label="Magnification factor"
            aria-invalid={draftError ? true : undefined}
            data-testid="viewport-deflection-factor"
            disabled={!fieldReady}
            inputMode="decimal"
            onBlur={() => { if (fieldReady && draft !== (factor === null ? "" : formatFactor(factor))) commitDraft(); }}
            onChange={(event) => setDraft(event.target.value)}
            onKeyDown={(event) => {
              if (event.key === "Enter") { event.preventDefault(); commitDraft(); }
              if (event.key === "Escape") { event.preventDefault(); setDraft(factor === null ? "" : formatFactor(factor)); setDraftError(null); }
            }}
            type="text"
            value={draft}
          />
        </label>
        <button
          type="button"
          aria-label="Larger magnification factor"
          data-testid="viewport-deflection-factor-up"
          disabled={!factorReady}
          onClick={() => factor !== null && onFactorChange(stepFactor125(factor, 1))}
        >+</button>
        <button
          type="button"
          data-testid="viewport-deflection-factor-default"
          disabled={!factorReady || (deformation.scale.mode === "factor" && deformation.scale.isDefault)}
          onClick={() => onFactorChange(null)}
          title="Return to the default factor for this case"
        >Default</button>
      </div>
      {draftError ? <small role="alert" data-testid="viewport-deflection-factor-error">{draftError}</small> : null}
      <button
        type="button"
        aria-pressed={ghost}
        className={ghost ? "active" : ""}
        data-testid="toggle-viewport-undeformed-ghost"
        disabled={deformation.state !== "available"}
        onClick={() => onGhostChange(!ghost)}
        title="Draw the undeformed model faintly behind the deformed shape"
      >Undeformed ghost</button>
      <button
        type="button"
        aria-pressed={playing}
        className={playing ? "active" : ""}
        data-testid="toggle-viewport-deflection-animation"
        disabled={!factorReady || reducedMotion}
        onClick={() => onPlayingChange(!playing)}
        title={reducedMotion ? "Reduced motion is set; step the phase with the slider instead" : "Animate between the undeformed and the deformed shape"}
      >{playing ? "Stop animation" : "Animate"}</button>
      <label>
        <span>Undeformed to deformed</span>
        <input
          aria-label="Deflection phase"
          aria-valuetext={`${Math.round(phase * 100)}% of the deflection`}
          data-testid="viewport-deflection-phase"
          disabled={!factorReady}
          max={DEFLECTION_PHASE_STEPS}
          min={0}
          onChange={(event) => onPhaseChange(Number(event.target.value) / DEFLECTION_PHASE_STEPS)}
          step={1}
          type="range"
          value={Math.round(phase * DEFLECTION_PHASE_STEPS)}
        />
      </label>
      <small data-testid="viewport-deflection-reason">{reason}</small>
    </div>
  );
}
