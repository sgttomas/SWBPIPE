import { act, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { CompactSelect, CompactSelectScope } from "./CompactSelect";

const options = [{ value: "a", label: "Alpha" }, { value: "b", label: "Beta", disabled: true }, { value: "c", label: "Charlie" }, { value: "d", label: "Delta" }];
function setup(value = "a") {
  const change = vi.fn();
  const result = render(<CompactSelect aria-label="Type" value={value} options={options} onValueChange={change} />);
  return { ...result, change, control: screen.getByRole("combobox", { name: "Type" }) };
}
function active(control: HTMLElement) { return document.getElementById(control.getAttribute("aria-activedescendant") ?? ""); }

describe("CompactSelect", () => {
  it("consumes the first Escape before duplicate React/window shell consumers, but leaves closed Escape untouched", () => {
    const reactShell = vi.fn();
    const windowShell = vi.fn();
    const change = vi.fn();
    window.addEventListener("keydown", windowShell);
    try {
      render(<div onKeyDown={reactShell}><CompactSelect aria-label="Type" options={options} value="a" onValueChange={change} /></div>);
      const control = screen.getByRole("combobox");
      fireEvent.click(control);
      fireEvent.keyDown(control, { key: "End" });
      expect(active(control)).toHaveTextContent("Delta");
      expect(fireEvent.keyDown(control, { key: "Escape" })).toBe(false);
      expect(reactShell).not.toHaveBeenCalled();
      expect(windowShell).not.toHaveBeenCalled();
      expect(change).not.toHaveBeenCalled();
      expect(control).toHaveFocus();
      expect(screen.queryByRole("listbox")).not.toBeInTheDocument();
      expect(fireEvent.keyDown(control, { key: "Escape" })).toBe(true);
      expect(reactShell).toHaveBeenCalledTimes(1);
      expect(windowShell).toHaveBeenCalledTimes(1);
    } finally { window.removeEventListener("keydown", windowShell); }
  });

  it("closes same-value pointer and keyboard commits without duplicate changes", () => {
    const { control, change } = setup();
    fireEvent.click(control);
    fireEvent.click(screen.getByRole("option", { name: "Alpha" }));
    expect(control).toHaveAttribute("aria-expanded", "false");
    fireEvent.keyDown(control, { key: "Enter" });
    fireEvent.keyDown(control, { key: "Enter" });
    expect(change).not.toHaveBeenCalled();
    fireEvent.click(control);
    fireEvent.click(screen.getByRole("option", { name: "Charlie" }));
    expect(change).toHaveBeenCalledExactlyOnceWith("c");
  });

  it("keeps selected and active distinct, skips disabled options, and commits once after rapid keys", () => {
    const { control, change } = setup();
    fireEvent.keyDown(control, { key: " " });
    act(() => {
      control.dispatchEvent(new KeyboardEvent("keydown", { key: "ArrowDown", bubbles: true }));
      control.dispatchEvent(new KeyboardEvent("keydown", { key: "ArrowDown", bubbles: true }));
    });
    expect(active(control)).toHaveTextContent("Delta");
    expect(screen.getByRole("option", { name: "Alpha" })).toHaveAttribute("aria-selected", "true");
    expect(change).not.toHaveBeenCalled();
    fireEvent.keyDown(control, { key: "Home" });
    fireEvent.keyDown(control, { key: "ArrowDown" });
    fireEvent.keyDown(control, { key: " " });
    expect(change).toHaveBeenCalledExactlyOnceWith("c");
  });

  it("supports typeahead, preserves unavailable current values, and handles empty options", () => {
    const { control, change, rerender } = setup("missing");
    expect(control).toHaveTextContent("missing");
    fireEvent.keyDown(control, { key: "d" });
    fireEvent.keyDown(control, { key: "e" });
    expect(active(control)).toHaveTextContent("Delta");
    fireEvent.keyDown(control, { key: "Escape" });
    expect(control).toHaveTextContent("missing");
    expect(change).not.toHaveBeenCalled();
    rerender(<CompactSelect aria-label="Type" value="" options={[]} onValueChange={change} />);
    fireEvent.click(control);
    expect(screen.getByText("No options")).toBeInTheDocument();
    expect(control).not.toHaveAttribute("aria-activedescendant");
    fireEvent.keyDown(control, { key: "Enter" });
    expect(change).not.toHaveBeenCalled();
  });

  it("commits on Tab without preventing native focus traversal and on outside focus without stealing it", () => {
    const change = vi.fn();
    render(<><CompactSelect aria-label="Type" value="a" options={options} onValueChange={change} /><button>Outside</button></>);
    const control = screen.getByRole("combobox");
    const outside = screen.getByRole("button");
    fireEvent.click(control);
    fireEvent.keyDown(control, { key: "End" });
    expect(fireEvent.keyDown(control, { key: "Tab" })).toBe(true);
    act(() => outside.focus());
    expect(outside).toHaveFocus();
    expect(change).toHaveBeenCalledExactlyOnceWith("d");
    change.mockClear();
    fireEvent.click(control);
    fireEvent.keyDown(control, { key: "ArrowDown" });
    act(() => outside.focus());
    expect(change).toHaveBeenCalledExactlyOnceWith("c");
    expect(outside).toHaveFocus();
  });

  it("portals beyond clipping ancestors, preserves trigger focus on option pointerdown, and cancels trigger closing", () => {
    const { control, container, change } = setup();
    fireEvent.click(control);
    const list = screen.getByRole("listbox");
    expect(container.contains(list)).toBe(false);
    expect(list.parentElement).toBe(document.body);
    expect(control).toHaveAttribute("aria-controls", list.id);
    fireEvent.pointerDown(screen.getByRole("option", { name: "Charlie" }));
    expect(control).toHaveFocus();
    fireEvent.keyDown(control, { key: "End" });
    fireEvent.click(control);
    expect(change).not.toHaveBeenCalled();
    expect(screen.queryByRole("listbox")).not.toBeInTheDocument();
  });

  it("honors explicit and inherited disabled states including the fieldset legend exception", () => {
    const change = vi.fn();
    render(<fieldset disabled><legend><CompactSelect aria-label="Legend" value="a" options={options} onValueChange={change} /></legend><CompactSelect aria-label="Inherited" value="a" options={options} onValueChange={change} /><CompactSelect disabled aria-label="Disabled" value="a" options={options} onValueChange={change} /></fieldset>);
    for (const name of ["Inherited", "Disabled"]) {
      const control = screen.getByRole("combobox", { name });
      fireEvent.click(control);
      fireEvent.keyDown(control, { key: "Enter" });
      expect(control).toHaveAttribute("aria-expanded", "false");
    }
    fireEvent.click(screen.getByRole("combobox", { name: "Legend" }));
    expect(screen.getByRole("listbox")).toBeInTheDocument();
    fireEvent.click(screen.getByRole("option", { name: "Beta" }));
    expect(change).not.toHaveBeenCalled();
  });

  it("closes earlier controls on outside pointer/focus and removes portalled state on unmount", () => {
    const change = vi.fn();
    const { unmount } = render(<><CompactSelect aria-label="First" value="a" options={options} onValueChange={change} /><CompactSelect aria-label="Second" value="a" options={options} onValueChange={change} /></>);
    const first = screen.getByRole("combobox", { name: "First" });
    const second = screen.getByRole("combobox", { name: "Second" });
    fireEvent.click(first);
    fireEvent.keyDown(first, { key: "End" });
    fireEvent.pointerDown(second);
    fireEvent.click(second);
    expect(first).toHaveAttribute("aria-expanded", "false");
    expect(second).toHaveAttribute("aria-expanded", "true");
    expect(screen.getAllByRole("listbox")).toHaveLength(1);
    expect(change).toHaveBeenCalledExactlyOnceWith("d");
    unmount();
    expect(screen.queryByRole("listbox")).not.toBeInTheDocument();
    fireEvent.pointerDown(document.body);
    expect(change).toHaveBeenCalledTimes(1);
  });

  it.each(["hidden", "inert", "aria-hidden"])("cancels a popup when its ancestor becomes %s", async (attribute) => {
    const change = vi.fn();
    const { container } = render(<div><CompactSelect aria-label="Type" value="a" options={options} onValueChange={change} /></div>);
    const control = screen.getByRole("combobox");
    fireEvent.click(control);
    fireEvent.keyDown(control, { key: "End" });
    container.firstElementChild!.setAttribute(attribute, attribute === "aria-hidden" ? "true" : "");
    await waitFor(() => expect(screen.queryByRole("listbox")).not.toBeInTheDocument());
    expect(change).not.toHaveBeenCalled();
  });

  it("retains labels/descriptions, bounds a lower-edge popup and reveals keyboard-active options", () => {
    const scroll = vi.fn();
    const original = Object.getOwnPropertyDescriptor(HTMLElement.prototype, "scrollIntoView");
    Object.defineProperty(HTMLElement.prototype, "scrollIntoView", { configurable: true, value: scroll });
    try {
      render(<><label htmlFor="kind">Kind</label><p id="help">Choose a kind</p><CompactSelect id="kind" aria-describedby="help" data-testid="kind" title="Kind control" value="a" options={options} onValueChange={vi.fn()} /></>);
      const control = screen.getByRole("combobox", { name: "Kind" });
      vi.spyOn(control, "getBoundingClientRect").mockReturnValue({ left: 1000, right: 1300, top: 740, bottom: 770, width: 300, height: 30, x: 1000, y: 740, toJSON() {} });
      fireEvent.click(control);
      const popup = screen.getByRole("listbox");
      expect(control).toHaveAccessibleDescription("Choose a kind");
      expect(popup.style.position).toBe("fixed");
      expect(parseFloat(popup.style.left) + parseFloat(popup.style.width)).toBeLessThanOrEqual(window.innerWidth);
      expect(parseFloat(popup.style.maxHeight)).toBeLessThanOrEqual(280);
      expect(popup.style.bottom).not.toBe("");
      fireEvent.keyDown(control, { key: "End" });
      expect(scroll.mock.instances.at(-1)).toBe(active(control));
      expect(control).toHaveFocus();
    } finally {
      if (original) Object.defineProperty(HTMLElement.prototype, "scrollIntoView", original);
      else delete (HTMLElement.prototype as { scrollIntoView?: unknown }).scrollIntoView;
    }
  });

  it("cancels when disabled after opening and cycles repeated typeahead without committing", async () => {
    const change = vi.fn();
    const choices = [{ value: "a", label: "Apple" }, { value: "b", label: "Apricot" }];
    const { rerender } = render(<fieldset><CompactSelect aria-label="Fruit" value="a" options={choices} onValueChange={change} /></fieldset>);
    const control = screen.getByRole("combobox");
    fireEvent.keyDown(control, { key: "a" });
    expect(active(control)).toHaveTextContent("Apricot");
    fireEvent.keyDown(control, { key: "a" });
    expect(active(control)).toHaveTextContent("Apple");
    rerender(<fieldset disabled><CompactSelect aria-label="Fruit" value="a" options={choices} onValueChange={change} /></fieldset>);
    await waitFor(() => expect(screen.queryByRole("listbox")).not.toBeInTheDocument());
    expect(change).not.toHaveBeenCalled();
  });

  it("cancels on window blur without committing or refocusing the trigger", () => {
    const { control, change } = setup();
    fireEvent.click(control);
    fireEvent.keyDown(control, { key: "End" });
    const focus = vi.spyOn(control, "focus");
    fireEvent.blur(window);
    expect(screen.queryByRole("listbox")).not.toBeInTheDocument();
    expect(change).not.toHaveBeenCalled();
    expect(focus).not.toHaveBeenCalled();
  });

  it("cancels on scope changes while retaining the same control and unrelated draft DOM", () => {
    const change = vi.fn();
    const content = <><input aria-label="Draft" defaultValue="pending" /><CompactSelect aria-label="Type" value="a" options={options} onValueChange={change} /></>;
    const { rerender } = render(<CompactSelectScope scopeKey="build">{content}</CompactSelectScope>);
    const control = screen.getByRole("combobox");
    const draft = screen.getByRole("textbox");
    fireEvent.change(draft, { target: { value: "edited" } });
    fireEvent.click(control);
    fireEvent.keyDown(control, { key: "End" });
    rerender(<CompactSelectScope scopeKey="review">{content}</CompactSelectScope>);
    expect(screen.queryByRole("listbox")).not.toBeInTheDocument();
    expect(screen.getByRole("combobox")).toBe(control);
    expect(screen.getByRole("textbox")).toBe(draft);
    expect(draft).toHaveValue("edited");
    expect(change).not.toHaveBeenCalled();
  });
});
