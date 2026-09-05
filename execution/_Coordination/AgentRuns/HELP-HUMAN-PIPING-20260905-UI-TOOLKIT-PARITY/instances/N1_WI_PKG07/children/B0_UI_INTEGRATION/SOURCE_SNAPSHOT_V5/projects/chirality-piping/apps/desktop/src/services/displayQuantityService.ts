import { invoke } from "@tauri-apps/api/core";
import { loadWasmEngine } from "./wasmEngine/loadWasmEngine";
export type DisplayQuantityRequest = {
  id: string;
  value: number;
  from_unit: string;
  to_unit: string;
  dimension_id: string;
};
export type DisplayQuantityResult = {
  id: string;
  status: "converted";
  value: number;
  unit: string;
} | {
  id: string | null;
  status: "unavailable";
  message: string;
};
export async function convertDisplayQuantities(
  items: DisplayQuantityRequest[]
): Promise<DisplayQuantityResult[]> {
  const input = { items };
  const result = typeof window !== "undefined" && "__TAURI_INTERNALS__" in window
    ? await invoke<{
      items: DisplayQuantityResult[];
      error?: {
        code: string;
        message: string;
      };
    }>("convert_display_quantities", { input })
    : JSON.parse((await loadWasmEngine()).convertDisplayQuantitiesJson(JSON.stringify(input)));
  if (result.error) {
    throw new Error(`${result.error.code}: ${result.error.message}`);
  }
  return result.items;
}
